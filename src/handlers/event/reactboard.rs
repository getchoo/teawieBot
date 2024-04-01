use crate::{storage, utils, Data};
use storage::reactboard::ReactBoardEntry;

use eyre::{eyre, Context as _, Result};
use log::{debug, warn};
use poise::serenity_prelude::{
	Context, CreateMessage, EditMessage, GuildId, Message, MessageReaction, Reaction,
};

pub async fn handle(ctx: &Context, reaction: &Reaction, data: &Data) -> Result<()> {
	let msg = reaction
		.message(&ctx.http)
		.await
		.wrap_err("Couldn't get reaction from message!")?;

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
	let Some(storage) = &data.storage else {
		warn!("Can't make ReactBoard entry; no storage backend found!");
		return Ok(());
	};

	let settings = storage.get_guild_settings(guild_id).await?;

	// make sure everything is in order...
	if !settings.reactboard_enabled {
		debug!("ReactBoard is disabled in {guild_id}, ignoring");
		return Ok(());
	}

	let Some(target) = settings.reactboard_channel else {
		debug!("ReactBoard is disabled in {guild_id}, ignoring");
		return Ok(());
	};

	if !settings.can_use_reaction(&reaction.reaction_type) {
		debug!("Reaction {} can't be used!", reaction.reaction_type);
		return Ok(());
	}

	let count = if msg
		.reaction_users(ctx, reaction.reaction_type.clone(), None, None)
		.await?
		.contains(&msg.author)
	{
		reaction.count - 1
	} else {
		reaction.count
	};

	if count < settings.reactboard_requirement.unwrap_or(5) {
		debug!(
			"Ignoring message {} on ReactBoard, not enough reactions",
			msg.id
		);
		return Ok(());
	}

	let content = format!("{} **#{}**", reaction.reaction_type, count);

	let entry = if storage.reactboard_entry_exists(guild_id, &msg.id).await? {
		// bump reaction count if previous entry exists
		let mut entry = storage.get_reactboard_entry(guild_id, &msg.id).await?;

		// bail if we don't need to edit anything
		if entry.reaction_count >= count {
			debug!("Message {} doesn't need updating", msg.id);
			return Ok(());
		}

		debug!(
			"Bumping {} reaction count from {} to {}",
			msg.id, entry.reaction_count, count
		);

		let edited = EditMessage::new().content(content);

		ctx.http
			.get_message(entry.posted_channel_id, entry.posted_message_id)
			.await
			.wrap_err_with(|| {
				format!(
					"Couldn't get previous message from ReactBoardEntry {} in Redis DB!",
					entry.original_message_id
				)
			})?
			.edit(ctx, edited)
			.await?;

		// update reaction count in redis
		entry.reaction_count = count;
		entry
	} else {
		// make new message and add entry to redis otherwise
		let embed = utils::resolve_message_to_embed(ctx, msg).await;
		let message = CreateMessage::default().content(content).embed(embed);

		let resp = target.send_message(ctx, message).await?;

		ReactBoardEntry {
			original_message_id: msg.id,
			reaction_count: count,
			posted_channel_id: resp.channel_id,
			posted_message_id: resp.id,
		}
	};

	debug!("Creating new ReactBoard entry:\n{entry:#?}");
	storage.create_reactboard_entry(guild_id, entry).await?;

	Ok(())
}
