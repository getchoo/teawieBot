use crate::{storage, utils, Data};
use storage::{ReactBoardEntry, REACT_BOARD_KEY};

use color_eyre::eyre::{eyre, Context as _, Result};
use log::*;
use poise::serenity_prelude::{Context, GuildId, Message, MessageReaction, Reaction};

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

	send_to_reactboard(
		ctx,
		&matched,
		&msg,
		&reaction.guild_id.unwrap_or_default(),
		data,
	)
	.await?;

	Ok(())
}

async fn send_to_reactboard(
	ctx: &Context,
	reaction: &MessageReaction,
	msg: &Message,
	guild_id: &GuildId,
	data: &Data,
) -> Result<()> {
	let storage = &data.storage;
	let settings = storage.get_guild_settings(guild_id).await?;

	// make sure everything is in order...
	if !settings.reactboard_enabled {
		debug!("ReactBoard is disabled in {guild_id}, ignoring");
		return Ok(());
	}

	let target = if let Some(target) = settings.reactboard_channel {
		target
	} else {
		debug!("ReactBoard is disabled in {guild_id}, ignoring");
		return Ok(());
	};

	if !settings.can_use_reaction(&reaction.reaction_type) {
		debug!("Reaction {} can't be used!", reaction.reaction_type);
		return Ok(());
	}

	if reaction.count < settings.reactboard_requirement.unwrap_or(5) {
		debug!(
			"Ignoring message {} on ReactBoard, not enough reactions",
			msg.id
		);
		return Ok(());
	}

	let mut reactboard = storage.get_reactboard_info().await?;

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
			debug!("Message {} doesn't need updating", msg.id);
			return Ok(());
		}

		debug!(
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

		debug!(
			"Updating ReactBoard entry {}\nOld entry:\n{old_entry:#?}\n\nNew:\n{new_entry:#?}\n",
			msg.id
		);
		storage.create_reactboard_info_key(reactboard).await?;
	// make new message and add entry to redis otherwise
	} else {
		let embed = utils::resolve_message_to_embed(ctx, msg).await;

		let resp = target
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

		debug!(
			"Creating new ReactBoard entry {} in {REACT_BOARD_KEY}:\n{:#?}",
			msg.id, entry
		);
		storage.create_reactboard_info_key(reactboard).await?;
	}

	Ok(())
}
