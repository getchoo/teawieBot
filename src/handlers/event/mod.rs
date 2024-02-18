use crate::Data;

use eyre::{Report, Result};
use log::info;
use poise::serenity_prelude as serenity;
use poise::FrameworkContext;
use serenity::FullEvent;

mod guild;
mod message;
mod pinboard;
mod reactboard;

pub async fn handle(
	ctx: &serenity::Context,
	event: &FullEvent,
	framework: FrameworkContext<'_, Data, Report>,
	data: &Data,
) -> Result<()> {
	match event {
		FullEvent::Ready { data_about_bot } => {
			info!("Logged in as {}!", data_about_bot.user.name);
		}

		FullEvent::Message { new_message } => {
			message::handle(ctx, framework, new_message, data).await?;
			pinboard::handle(ctx, new_message, data).await?;
		}

		FullEvent::ReactionAdd { add_reaction } => {
			reactboard::handle(ctx, add_reaction, data).await?;
		}

		FullEvent::GuildCreate { guild, is_new } => {
			guild::handle_create(guild, &is_new.unwrap_or_default(), data).await?;
		}

		FullEvent::GuildDelete {
			incomplete,
			full: _,
		} => guild::handle_delete(incomplete, data).await?,

		_ => {}
	}

	Ok(())
}
