use crate::{client::Context, http, utils};

use anyhow::Result;
use rand::Rng;

#[poise::command(slash_command, subcommands("teawie", "shiggy", "uwu"))]
#[allow(clippy::unused_async)]
pub async fn random(_: Context<'_>) -> Result<()> {
	Ok(())
}

/// Get a random teawie
#[poise::command(prefix_command, slash_command)]
pub async fn teawie(ctx: Context<'_>) -> Result<()> {
	let url = http::teawie::random(&ctx.data().http_client).await?;
	utils::send_url_as_embed(ctx, url).await?;

	Ok(())
}

/// Get a random shiggy
#[poise::command(prefix_command, slash_command)]
pub async fn shiggy(ctx: Context<'_>) -> Result<()> {
	let url = http::shiggy::random(&ctx.data().http_client).await?;
	utils::send_url_as_embed(ctx, url).await?;

	Ok(())
}

/// Some uwu
#[poise::command(prefix_command, slash_command)]
pub async fn uwu(
	ctx: Context<'_>,
	#[description = "The amount of uwurandom to generate"]
	#[min = 1]
	#[max = 100]
	length: Option<u16>,
) -> Result<()> {
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
