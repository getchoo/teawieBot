use crate::{Data, Error};
use poise::serenity_prelude as serenity;
use poise::Event;

mod message;
pub mod pinboard;
mod reactboard;

pub async fn handle(
	ctx: &serenity::Context,
	event: &Event<'_>,
	framework: poise::FrameworkContext<'_, Data, Error>,
	data: &Data,
) -> Result<(), Error> {
	match event {
		Event::Ready { data_about_bot } => {
			log::info!("logged in as {}", data_about_bot.user.name)
		}

		Event::Message { new_message } => {
			message::handle(ctx, event, framework, data, new_message).await?
		}

		Event::ChannelPinsUpdate { pin } => {
			if let Some(settings) = &data.settings {
				pinboard::handle(ctx, pin, settings).await
			}
		}

		Event::ReactionAdd { add_reaction } => {
			if let Some(settings) = &data.settings {
				reactboard::handle(ctx, add_reaction, settings).await?
			}
		}

		_ => {}
	}

	Ok(())
}
