use crate::Context;

use color_eyre::eyre::Result;
use log::debug;

/// teawie will spam you.
#[poise::command(slash_command)]
pub async fn teawiespam(ctx: Context<'_>) -> Result<()> {
	let gid = ctx.guild_id().unwrap_or_default();
	let settings = ctx.data().storage.get_guild_settings(&gid).await?;

	if !settings.optional_commands_enabled {
		debug!("Not running teawiespam in {gid} since it's disabled");
		ctx.say("I'm not allowed to do that here").await?;
		return Ok(());
	}

	let wies = "<:teawiesmile:1056438046440042546>".repeat(50);
	ctx.say(wies).await?;
	Ok(())
}
