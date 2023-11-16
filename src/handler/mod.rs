use crate::{Data, Error};
use poise::serenity_prelude as serenity;
use poise::Event;

mod message;
pub mod pinboard;

pub async fn handle(
	ctx: &serenity::Context,
	event: &Event<'_>,
	_framework: poise::FrameworkContext<'_, Data, Error>,
	data: &Data,
) -> Result<(), Error> {
	match event {
		Event::Ready { data_about_bot } => {
			log::info!("logged in as {}", data_about_bot.user.name)
		}

		Event::Message { new_message } => {
			message::handle(ctx, event, _framework, data, new_message).await?;
		}

		Event::ChannelPinsUpdate { pin } => {
			let Some(pin_board) = &data.pin_board else {
				return Ok(());
			};

			pin_board.handle_pin(ctx, pin).await;
		}

		_ => {}
	}

	Ok(())
}
