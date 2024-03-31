use crate::{Context, Error};

use log::{debug, warn};

/// teawie will spam you.
#[poise::command(slash_command)]
pub async fn teawiespam(ctx: Context<'_>) -> Result<(), Error> {
	let gid = ctx.guild_id().unwrap_or_default();

	if let Some(storage) = &ctx.data().storage {
		let settings = storage.get_guild_settings(&gid).await?;

		if !settings.optional_commands_enabled {
			debug!("Not running teawiespam in {gid} since it's disabled");
			ctx.say("I'm not allowed to do that here").await?;
			return Ok(());
		}
	} else {
		warn!("Ignoring restrictions on copypasta command; no storage backend is attached!");
	}

	let wies = "<:teawiesmile:1056438046440042546>".repeat(50);
	ctx.say(wies).await?;
	Ok(())
}
