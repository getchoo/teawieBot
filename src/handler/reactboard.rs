use crate::Error;
use crate::{settings::Settings, utils};
use log::*;
use poise::serenity_prelude::{Context, Message, MessageReaction, Reaction};

pub async fn handle(ctx: &Context, reaction: &Reaction, settings: &Settings) -> Result<(), Error> {
	let msg = match reaction.message(&ctx.http).await {
		Ok(msg) => msg,
		Err(why) => {
			warn!("couldn't get message of reaction! {}", why);
			return Err(Box::new(why));
		}
	};

	if let Some(matched) = msg
		.clone()
		.reactions
		.into_iter()
		.find(|r| r.reaction_type == reaction.emoji)
	{
		send_to_reactboard(ctx, &matched, &msg, settings).await?;
	} else {
		warn!(
			"couldn't find any matching reactions for {} in {}",
			reaction.emoji.as_data(),
			msg.id
		)
	}

	Ok(())
}

async fn send_to_reactboard(
	ctx: &Context,
	reaction: &MessageReaction,
	msg: &Message,
	settings: &Settings,
) -> Result<(), Error> {
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
