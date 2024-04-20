use crate::{Context, Error};

use eyre::Result;
use log::debug;
use rand::Rng;

/// Generate some amount of uwurandom
#[poise::command(slash_command)]
pub async fn uwurandom(
	ctx: Context<'_>,
	#[description = "The amount of uwurandom to generate"]
	#[min = 1]
	#[max = 2000]
	length: Option<u16>,
) -> Result<(), Error> {
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
		debug!("Ignoring restrictions on command; we're not in a guild");
	}

	let length = length.unwrap_or(rand::thread_rng().gen_range(1..50));

	let mut result = String::with_capacity(length as usize);
	// ThreadRng is not Send(obviously), and rustc is slightly too paranoid about rng spilling to await point
	// So calm it by constraining it to a block
	{
		let mut rng = rand::thread_rng();
		let mut state_machine = uwurandom_rs::StateMachine::new(&mut rng);
		for _ in 0..length {
			let generated;
			(state_machine, generated) = state_machine.generate(&mut rng);
			result.push(generated);
		}
	}
	ctx.say(result).await?;
	Ok(())
}
