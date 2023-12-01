use crate::settings::Settings;
use crate::{consts, Data};

use color_eyre::eyre::{Report, Result};
use log::*;
use poise::serenity_prelude::{Context, Message};
use poise::FrameworkContext;

pub async fn handle(
	ctx: &Context,
	framework: FrameworkContext<'_, Data, Report>,
	msg: &Message,
	settings: &Settings,
) -> Result<()> {
	if should_echo(framework, msg, settings) {
		msg.reply(ctx, &msg.content).await?;
	}

	Ok(())
}

fn should_echo(
	framework: FrameworkContext<'_, Data, Report>,
	msg: &Message,
	settings: &Settings,
) -> bool {
	let gid = msg.guild_id.unwrap_or_default();
	if msg.author.id == framework.bot_id || !settings.is_guild_allowed(gid) {
		info!("not running copypasta command in {gid}");
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
