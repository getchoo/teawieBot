use crate::{settings::Settings, utils};

use color_eyre::eyre::{eyre, Context as _, Result};
use log::*;
use poise::serenity_prelude::{Context, Message, MessageReaction, Reaction};

pub async fn handle(ctx: &Context, reaction: &Reaction, settings: &Settings) -> Result<()> {
	let msg = reaction
		.message(&ctx.http)
		.await
		.wrap_err("couldn't get reaction from message!")?;

	let matched = msg
		.clone()
		.reactions
		.into_iter()
		.find(|r| r.reaction_type == reaction.emoji)
		.ok_or_else(|| {
			eyre!(
				"couldn't find any matching reactions for {} in message {}!",
				reaction.emoji.as_data(),
				msg.id
			)
		})?;

	send_to_reactboard(ctx, &matched, &msg, settings).await?;

	Ok(())
}

async fn send_to_reactboard(
	ctx: &Context,
	reaction: &MessageReaction,
	msg: &Message,
	settings: &Settings,
) -> Result<()> {
	if !settings.can_use_reaction(reaction) {
		info!("reaction {} can't be used!", reaction.reaction_type);
		return Ok(());
	}

	if reaction.count == settings.reactboard_requirement.unwrap_or(5) {
		let embed = utils::resolve_message_to_embed(ctx, msg).await;

		settings
			.reactboard_target
			.send_message(&ctx.http, |m| {
				m.allowed_mentions(|am| am.empty_parse())
					.content(format!(
						"{} **#{}**",
						reaction.reaction_type, reaction.count
					))
					.set_embed(embed)
			})
			.await?;
	} else {
		info!(
			"not putting message {} on reactboard, not enough reactions",
			msg.id
		)
	}

	Ok(())
}
