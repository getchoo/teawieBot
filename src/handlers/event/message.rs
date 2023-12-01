use crate::Settings;
use crate::{consts, Data};

use color_eyre::eyre::{Report, Result};
use log::info;
use poise::serenity_prelude::{Context, Message};
use poise::FrameworkContext;

pub async fn handle(
	ctx: &Context,
	framework: FrameworkContext<'_, Data, Report>,
	msg: &Message,
	data: &Data,
) -> Result<()> {
	if should_echo(framework, msg, &data.settings) {
		msg.reply(ctx, &msg.content).await?;
	}

	Ok(())
}

fn should_echo(
	_framework: FrameworkContext<'_, Data, Report>,
	msg: &Message,
	settings: &Settings,
) -> bool {
	let gid = msg.guild_id.unwrap_or_default();
	if msg.author.bot && msg.webhook_id.is_none() {
		info!("I don't like repeating myself...");
		return false;
	}

	if !settings.is_guild_allowed(gid) {
		info!("Not echoing in guild {gid}");
		return false;
	}

	let content = &msg.content;

	content == "🗿"
		|| consts::TEAMOJIS.contains(&content.as_str())
		|| content.to_ascii_lowercase() == "moyai"
		|| content
			.to_ascii_lowercase()
			.contains("twitter's recommendation algorithm")
}
