use crate::{client::Data, consts};

use eyre::{eyre, Result};
use log::{debug, warn};
use poise::serenity_prelude::{Context, Message};

pub async fn handle(ctx: &Context, msg: &Message, data: &Data) -> Result<()> {
	if should_echo(ctx, msg, data).await? {
		msg.reply(ctx, &msg.content).await?;
	}

	Ok(())
}

async fn should_echo(ctx: &Context, msg: &Message, data: &Data) -> Result<bool> {
	if (msg.author.bot && msg.webhook_id.is_none()) || msg.is_own(ctx) {
		debug!("Not repeating another bot");
		return Ok(false);
	}

	let gid = msg
		.guild_id
		.ok_or_else(|| eyre!("Couldn't get GuildId from {}!", msg.id))?;

	if let Some(storage) = &data.storage {
		let settings = storage.get_guild_settings(&gid).await?;

		if !settings.optional_commands_enabled {
			debug!("Not echoing in guild {gid}");
			return Ok(false);
		}
	} else {
		warn!("Ignoring restrictions on echoing messages; no storage backend is attached!");
	}

	let content = &msg.content;

	Ok(content == "🗿"
		|| content.to_ascii_lowercase() == "moyai"
		|| content
			.to_ascii_lowercase() == "twitter's recommendation algorithm")
}
