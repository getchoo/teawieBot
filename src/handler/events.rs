use crate::handler::Handler;
use log::*;
use poise::async_trait;
use poise::serenity_prelude::{ChannelPinsUpdateEvent, Context, EventHandler, Message};

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, msg: Message) {
		if self.should_echo(&msg) {
			let send = msg.reply(&ctx, &msg.content);
			if let Err(why) = send.await {
				error!("error when replying to {:?}: {:?}", msg.content, why);
			}
		}
	}

	async fn channel_pins_update(&self, ctx: Context, pin: ChannelPinsUpdateEvent) {
		let Some(pin_board) = &self.data.pin_board else {
			return;
		};

		pin_board.handle_pin(&ctx, &pin).await;
	}
}
