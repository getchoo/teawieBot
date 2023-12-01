use crate::Context;

use color_eyre::eyre::Result;
use log::*;

/// teawie will spam you.
#[poise::command(slash_command, prefix_command)]
pub async fn teawiespam(ctx: Context<'_>) -> Result<()> {
	let gid = ctx.guild_id().unwrap_or_default();

	if !ctx.data().settings.is_guild_allowed(gid) {
		info!("not running teawiespam command in {gid}");
		return Ok(());
	}

	let wies = "<:teawiesmile:1056438046440042546>".repeat(50);
	ctx.say(wies).await?;
	Ok(())
}
