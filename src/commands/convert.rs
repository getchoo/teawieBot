use crate::{Context, Error};

#[poise::command(slash_command, subcommands("to_fahrenheit", "to_celsius"))]
pub async fn convert(_ctx: Context<'_>) -> Result<(), Error> {
	Ok(())
}

/// ask teawie to convert °F to °C
#[poise::command(slash_command)]
pub async fn to_celsius(
	ctx: Context<'_>,
	#[description = "what teawie will convert"] degrees_fahrenheit: f32,
) -> Result<(), Error> {
	let temp = (degrees_fahrenheit - 32.0) * (5.0 / 9.0);
	ctx.say(temp.to_string()).await?;
	Ok(())
}

/// ask teawie to convert °C to °F
#[poise::command(slash_command)]
pub async fn to_fahrenheit(
	ctx: Context<'_>,
	#[description = "what teawie will convert"] degrees_celsius: f32,
) -> Result<(), Error> {
	let temp = (degrees_celsius * (9.0 / 5.0)) + 32.0;
	ctx.say(temp.to_string()).await?;
	Ok(())
}
