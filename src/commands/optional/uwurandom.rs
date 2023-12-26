use crate::Context;

use color_eyre::eyre::Result;
use log::debug;

/// Generate some amount of uwurandom
#[poise::command(slash_command)]
pub async fn uwurandom(
	ctx: Context<'_>,
	#[description = "The amount of uwurandom to generate"]
	#[min = 1]
	#[max = 2000]
	length: u16,
) -> Result<()> {
	let gid = ctx.guild_id().unwrap_or_default();
	let settings = ctx.data().storage.get_guild_settings(&gid).await?;

	if !settings.optional_commands_enabled {
		debug!("Not running uwurandom in {gid} since it's disabled");
		ctx.say("I'm not allowed to do that here").await?;
		return Ok(());
	}

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
