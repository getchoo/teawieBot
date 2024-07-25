use crate::{client::Data, consts};

use eyre::Result;
use log::{debug, info};
use poise::serenity_prelude::{self as serenity, CreateBotAuthParameters};
use serenity::FullEvent;

mod guild;
mod message;
mod pinboard;
mod reactboard;

pub async fn handle(ctx: &serenity::Context, event: &FullEvent, data: &Data) -> Result<()> {
	match event {
		FullEvent::Ready { data_about_bot } => {
			info!("Logged in as {}!", data_about_bot.user.name);

			if let Ok(invite_link) = CreateBotAuthParameters::new().auto_client_id(ctx).await {
				let link = invite_link
					.scopes(consts::bot_scopes())
					.permissions(*consts::bot_permissions())
					.build();
				info!("Invite me to your server at {link}");
			} else {
				debug!("Not displaying invite_link since we couldn't find our client ID");
			}
		}

		FullEvent::Message { new_message } => {
			message::handle(ctx, new_message, data).await?;
			pinboard::handle(ctx, new_message, data).await?;
		}

		FullEvent::ReactionAdd { add_reaction } => {
			reactboard::handle(ctx, add_reaction, data).await?;
		}

		FullEvent::GuildCreate { guild, is_new: _ } => {
			guild::handle_create(guild, data).await?;
		}

		FullEvent::GuildDelete {
			incomplete,
			full: _,
		} => guild::handle_delete(incomplete, data).await?,

		_ => {}
	}

	Ok(())
}
