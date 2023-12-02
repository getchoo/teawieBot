use crate::{consts, Data, Settings};

use color_eyre::eyre::{eyre, Report, Result};
use log::*;
use poise::serenity_prelude::{Context, Message};
use poise::FrameworkContext;

pub async fn handle(
	ctx: &Context,
	framework: FrameworkContext<'_, Data, Report>,
	msg: &Message,
	data: &Data,
) -> Result<()> {
	if should_echo(framework, msg, data).await? {
		msg.reply(ctx, &msg.content).await?;
	}

	Ok(())
}

async fn should_echo(
	_framework: FrameworkContext<'_, Data, Report>,
	msg: &Message,
	data: &Data,
) -> Result<bool> {
	if msg.author.bot && msg.webhook_id.is_none() {
		debug!("Not repeating another bot");
		return Ok(false);
	}

	let gid = msg
		.guild_id
		.ok_or_else(|| eyre!("Couldn't get GuildId from {}!", msg.id))?;
	let settings = Settings::from_redis(&data.redis, &gid).await?;

	if !settings.optional_commands_enabled {
		debug!("Not echoing in guild {gid}");
		return Ok(false);
	}

	let content = &msg.content;

	Ok(content == "🗿"
		|| consts::TEAMOJIS.contains(&content.as_str())
		|| content.to_ascii_lowercase() == "moyai"
		|| content
			.to_ascii_lowercase()
			.contains("twitter's recommendation algorithm"))
}
