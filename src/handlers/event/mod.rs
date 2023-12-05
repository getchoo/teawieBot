use crate::storage::{ReactBoardInfo, REACT_BOARD_KEY};
use crate::Data;

use color_eyre::eyre::{Report, Result};
use log::*;
use poise::serenity_prelude as serenity;
use poise::{Event, FrameworkContext};

mod guild;
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
			info!("Logged in as {}!", data_about_bot.user.name);

			// make sure react board is setup
			let storage = &data.storage;
			if !storage.key_exists(REACT_BOARD_KEY).await? {
				warn!("Creating new ReactBoardInfo key {REACT_BOARD_KEY}");

				storage
					.create_reactboard_info_key(ReactBoardInfo::default())
					.await?;
			}
		}

		Event::Message { new_message } => {
			message::handle(ctx, framework, new_message, data).await?
		}

		Event::ChannelPinsUpdate { pin } => pinboard::handle(ctx, pin, data).await?,

		Event::ReactionAdd { add_reaction } => reactboard::handle(ctx, add_reaction, data).await?,

		Event::GuildCreate { guild, is_new } => guild::handle_create(guild, is_new, data).await?,
		Event::GuildDelete {
			incomplete,
			full: _,
		} => guild::handle_delete(incomplete, data).await?,

		_ => {}
	}

	Ok(())
}