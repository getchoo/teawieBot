use crate::Context;

use bottomify::bottom;
use color_eyre::eyre::Result;

#[poise::command(
	slash_command,
	subcommands("to_fahrenheit", "to_celsius", "to_bottom", "from_bottom")
)]
pub async fn convert(_ctx: Context<'_>) -> Result<()> {
	Ok(())
}

/// ask teawie to convert °F to °C
#[poise::command(slash_command)]
pub async fn to_celsius(
	ctx: Context<'_>,
	#[description = "what teawie will convert"] degrees_fahrenheit: f32,
) -> Result<()> {
	let temp = (degrees_fahrenheit - 32.0) * (5.0 / 9.0);
	ctx.say(temp.to_string()).await?;
	Ok(())
}

/// ask teawie to convert °C to °F
#[poise::command(slash_command)]
pub async fn to_fahrenheit(
	ctx: Context<'_>,
	#[description = "what teawie will convert"] degrees_celsius: f32,
) -> Result<()> {
	let temp = (degrees_celsius * (9.0 / 5.0)) + 32.0;
	ctx.say(temp.to_string()).await?;
	Ok(())
}

/// teawie will translate to bottom 🥺
#[poise::command(slash_command)]
pub async fn to_bottom(
	ctx: Context<'_>,
	#[description = "what teawie will translate into bottom"] message: String,
) -> Result<()> {
	let encoded = bottom::encode_string(&message);
	ctx.say(encoded).await?;
	Ok(())
}

/// teawie will translate from bottom 🥸
#[poise::command(slash_command)]
pub async fn from_bottom(
	ctx: Context<'_>,
	#[description = "what teawie will translate from bottom"] message: String,
) -> Result<()> {
	let decoded = bottom::decode_string(&message)?;
	ctx.say(decoded).await?;
	Ok(())
}
