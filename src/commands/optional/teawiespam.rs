use crate::client::Context;

use eyre::Result;
use log::debug;

/// teawie will spam you.
#[poise::command(slash_command)]
pub async fn teawiespam(ctx: Context<'_>) -> Result<()> {
	if let Some(guild_id) = ctx.guild_id() {
		if let Some(storage) = &ctx.data().storage {
			let settings = storage.get_guild_settings(&guild_id).await?;

			if !settings.optional_commands_enabled {
				debug!("Not running command in {guild_id} since it's disabled");
				ctx.say("I'm not allowed to do that here").await?;

				return Ok(());
			}
		} else {
			debug!("Ignoring restrictions on command; no storage backend is attached!");
		}
	} else {
		debug!("Ignoring restrictions on command; we're not in a guild.");
	}

	let wies = "<:teawiesmile:1056438046440042546>".repeat(50);
	ctx.say(wies).await?;

	Ok(())
}
