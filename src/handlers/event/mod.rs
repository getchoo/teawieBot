use crate::Data;

use color_eyre::eyre::{Report, Result};
use poise::serenity_prelude as serenity;
use poise::{Event, FrameworkContext};

mod message;
mod pinboard;
mod reactboard;

pub async fn handle(
	ctx: &serenity::Context,
	event: &Event<'_>,
	framework: FrameworkContext<'_, Data, Report>,
	data: &Data,
) -> Result<()> {
	match event {
		Event::Ready { data_about_bot } => {
			log::info!("logged in as {}", data_about_bot.user.name)
		}

		Event::Message { new_message } => {
			message::handle(ctx, framework, new_message, &data.settings).await?
		}

		Event::ChannelPinsUpdate { pin } => pinboard::handle(ctx, pin, &data.settings).await,

		Event::ReactionAdd { add_reaction } => {
			reactboard::handle(ctx, add_reaction, &data.settings).await?
		}

		_ => {}
	}

	Ok(())
}
