use crate::{consts, utils, Data, Error};
use log::*;
use poise::serenity_prelude as serenity;
use poise::{Event, FrameworkContext};

fn should_echo(framework: FrameworkContext<'_, Data, Error>, msg: &serenity::Message) -> bool {
	let gid = msg.guild_id.unwrap_or_default();
	if msg.author.id == framework.bot_id || !utils::is_guild_allowed(gid) {
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

pub async fn handle(
	ctx: &serenity::Context,
	_event: &Event<'_>,
	framework: FrameworkContext<'_, Data, Error>,
	_data: &Data,
	msg: &serenity::Message,
) -> Result<(), Error> {
	if should_echo(framework, msg) {
		msg.reply(ctx, &msg.content).await?;
	}

	Ok(())
}
