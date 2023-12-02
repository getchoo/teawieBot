use crate::{Context, Settings};

use color_eyre::eyre::Result;
use log::*;

/// teawie will spam you.
#[poise::command(slash_command, prefix_command)]
pub async fn teawiespam(ctx: Context<'_>) -> Result<()> {
	let gid = ctx.guild_id().unwrap_or_default();
	let settings = Settings::from_redis(&ctx.data().redis, &gid).await?;

	if !settings.optional_commands_enabled {
		debug!("Not running teawiespam in {gid} since it's disabled");
		return Ok(());
	}

	let wies = "<:teawiesmile:1056438046440042546>".repeat(50);
	ctx.say(wies).await?;
	Ok(())
}
