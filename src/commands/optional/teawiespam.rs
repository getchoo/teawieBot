use crate::Context;

use color_eyre::eyre::Result;
use log::*;

/// teawie will spam you.
#[poise::command(slash_command, prefix_command)]
pub async fn teawiespam(ctx: Context<'_>) -> Result<()> {
	debug!("Running teawiespam command");

	let gid = ctx.guild_id().unwrap_or_default();
	let settings = ctx.data().storage.get_guild_settings(&gid).await?;

	if !settings.optional_commands_enabled {
		debug!("Not running teawiespam in {gid} since it's disabled");
		return Ok(());
	}

	let wies = "<:teawiesmile:1056438046440042546>".repeat(50);
	ctx.say(wies).await?;
	Ok(())
}
