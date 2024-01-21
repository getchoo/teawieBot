use crate::{storage, utils, Data};
use storage::ReactBoardEntry;

use color_eyre::eyre::{eyre, Context as _, Result};
use log::debug;
use poise::serenity_prelude::{
	Context, CreateMessage, EditMessage, GuildId, Message, MessageReaction, Reaction,
};

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

	let Some(target) = settings.reactboard_channel else {
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

	let content = format!("{} **#{}**", reaction.reaction_type, reaction.count);

	// bump reaction count if previous entry exists
	if storage.reactboard_entry_exists(guild_id, &msg.id).await? {
		let old_entry = storage.get_reactboard_entry(guild_id, &msg.id).await?;

		// bail if we don't need to edit anything
		if old_entry.reaction_count >= reaction.count {
			debug!("Message {} doesn't need updating", msg.id);
			return Ok(());
		}

		debug!(
			"Bumping {} reaction count from {} to {}",
			msg.id, old_entry.reaction_count, reaction.count
		);

		let edited = EditMessage::new().content(content);

		ctx.http
			.get_message(old_entry.posted_channel_id, old_entry.posted_message_id)
			.await
			.wrap_err_with(|| {
				format!(
					"Couldn't get previous message from ReactBoardEntry {} in Redis DB!",
					old_entry.original_message_id
				)
			})?
			.edit(ctx, edited)
			.await?;

		// update reaction count in redis
		let mut new_entry = old_entry.clone();
		new_entry.reaction_count = reaction.count;

		debug!("Updating ReactBoard entry\nOld entry:\n{old_entry:#?}\n\nNew:\n{new_entry:#?}\n",);
		storage.create_reactboard_entry(guild_id, new_entry).await?;
	// make new message and add entry to redis otherwise
	} else {
		let embed = utils::resolve_message_to_embed(ctx, msg).await;
		let message = CreateMessage::default().content(content).embed(embed);

		let resp = target.send_message(ctx, message).await?;

		let entry = ReactBoardEntry {
			original_message_id: msg.id,
			reaction_count: reaction.count,
			posted_channel_id: resp.channel_id,
			posted_message_id: resp.id,
		};

		debug!("Creating new ReactBoard entry:\n{entry:#?}");
		storage.create_reactboard_entry(guild_id, entry).await?;
	}

	Ok(())
}
