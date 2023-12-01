use crate::{utils, Data};

use color_eyre::eyre::{eyre, Context as _, Result};
use log::*;
use poise::serenity_prelude::{ChannelId, Context, Message, MessageId, MessageReaction, Reaction};
use redis::AsyncCommands as _;
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
struct ReactBoardEntry {
	original_id: MessageId,
	reaction_count: u64,
	channel_id: ChannelId,
	message_id: MessageId,
}

#[derive(Default, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
struct ReactBoardInfo {
	reactions: Vec<ReactBoardEntry>,
}

const REACT_BOARD_KEY: &str = "reactboard-v1";

pub async fn handle(ctx: &Context, reaction: &Reaction, data: &Data) -> Result<()> {
	let msg = reaction
		.message(&ctx.http)
		.await
		.wrap_err_with(|| "Couldn't get reaction from message!")?;

	let matched = msg
		.clone()
		.reactions
		.into_iter()
		.find(|r| r.reaction_type == reaction.emoji)
		.ok_or_else(|| {
			eyre!(
				"Couldn't find any matching reactions for {} in message {}!",
				reaction.emoji.as_data(),
				msg.id
			)
		})?;

	send_to_reactboard(ctx, &matched, &msg, data).await?;

	Ok(())
}

async fn send_to_reactboard(
	ctx: &Context,
	reaction: &MessageReaction,
	msg: &Message,
	data: &Data,
) -> Result<()> {
	// make sure everything is in order...
	if !data.settings.can_use_reaction(reaction) {
		info!("Reaction {} can't be used!", reaction.reaction_type);
		return Ok(());
	}

	if reaction.count < data.settings.reactboard_requirement.unwrap_or(5) {
		info!(
			"Ignoring message {} on reactboard, not enough reactions",
			msg.id
		);
		return Ok(());
	}

	let mut con = data.redis.get_async_connection().await?;
	let req = con.get(REACT_BOARD_KEY).await;

	let mut reactboard: ReactBoardInfo = if let Err(why) = req {
		// set the value to the default if the key is uninitialized
		match why.kind() {
			redis::ErrorKind::TypeError => {
				warn!("Initializing {REACT_BOARD_KEY} key in Redis...");
				con.set(REACT_BOARD_KEY, ReactBoardInfo::default()).await?;
				con.get(REACT_BOARD_KEY).await?
			}
			_ => return Err(why.into()),
		}
	} else {
		req?
	};

	// try to find previous reactboard entry by the id of the original message
	let old_index = reactboard
		.reactions
		.iter()
		.position(|r| r.original_id == msg.id);

	let content = format!("{} **#{}**", reaction.reaction_type, reaction.count);

	// bump reaction count if previous entry exists
	if let Some(old_index) = old_index {
		let old_entry = reactboard.reactions[old_index].clone();

		// bail if we don't need to edit anything
		if old_entry.reaction_count >= reaction.count {
			info!("Message {} doesn't need updating", msg.id);
			return Ok(());
		}

		info!(
			"Bumping {} reaction count from {} to {}",
			msg.id, old_entry.reaction_count, reaction.count
		);

		ctx.http
			.get_message(
				*old_entry.channel_id.as_u64(),
				*old_entry.message_id.as_u64(),
			)
			.await
			.wrap_err_with(|| {
				format!(
					"Couldn't get previous message from ReactBoardEntry {} in Redis DB!",
					old_entry.original_id
				)
			})?
			.edit(ctx, |m| m.content(content))
			.await?;

		// update reaction count in redis
		let mut new_entry = old_entry.clone();
		new_entry.reaction_count = reaction.count;

		reactboard.reactions.remove(old_index);
		reactboard.reactions.push(new_entry.clone());

		info!(
			"Updating ReactBoard entry {} in {REACT_BOARD_KEY}\nOld:\n{old_entry:#?}\nNew:\n{new_entry:#?}",
			msg.id
		);
		con.set(REACT_BOARD_KEY, reactboard).await?;
	// make new message and add entry to redis otherwise
	} else {
		let embed = utils::resolve_message_to_embed(ctx, msg).await;

		let resp = data
			.settings
			.reactboard_target
			.send_message(ctx, |m| {
				m.allowed_mentions(|am| am.empty_parse())
					.content(content)
					.set_embed(embed)
			})
			.await?;

		let entry = ReactBoardEntry {
			original_id: msg.id,
			reaction_count: reaction.count,
			channel_id: resp.channel_id,
			message_id: resp.id,
		};

		reactboard.reactions.push(entry.clone());

		info!(
			"Creating new ReactBoard entry {} in {REACT_BOARD_KEY}:\n{:#?}",
			msg.id, entry
		);
		con.set(REACT_BOARD_KEY, reactboard).await?;
	}

	Ok(())
}
