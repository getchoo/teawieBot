use crate::client::Context;

use bottomify::bottom;
use eyre::Result;
use poise::serenity_prelude::constants::MESSAGE_CODE_LIMIT;

#[allow(clippy::unused_async)]
#[poise::command(
	slash_command,
	subcommands("to_fahrenheit", "to_celsius", "to_bottom", "from_bottom")
)]
pub async fn convert(_: Context<'_>) -> Result<()> {
	Ok(())
}

/// Convert °F to °C
#[poise::command(slash_command)]
pub async fn to_celsius(
	ctx: Context<'_>,
	#[description = "What to convert"] degrees_fahrenheit: f32,
) -> Result<()> {
	let temp = (degrees_fahrenheit - 32.0) * (5.0 / 9.0);
	ctx.say(temp.to_string()).await?;
	Ok(())
}

/// Convert °C to °F
#[poise::command(slash_command)]
pub async fn to_fahrenheit(
	ctx: Context<'_>,
	#[description = "What to convert"] degrees_celsius: f32,
) -> Result<()> {
	let temp = (degrees_celsius * (9.0 / 5.0)) + 32.0;
	ctx.say(temp.to_string()).await?;
	Ok(())
}

/// Translate to bottom 🥺
#[poise::command(slash_command)]
pub async fn to_bottom(
	ctx: Context<'_>,
	#[description = "What to translate into bottom"] message: String,
) -> Result<()> {
	let encoded = bottom::encode_string(&message);
	ctx.say(encoded).await?;
	Ok(())
}

/// Translate from bottom 🥸
#[poise::command(slash_command)]
pub async fn from_bottom(
	ctx: Context<'_>,
	#[description = "What to translate from bottom"] message: String,
) -> Result<()> {
	let resp: String;

	if let Ok(decoded) = bottom::decode_string(&message.clone()) {
		resp = if decoded.len() > MESSAGE_CODE_LIMIT {
			"The translation is too long to send, sorry :(".to_string()
		} else {
			decoded
		}
	} else {
		resp = "Couldn't translate that for you :(".to_string();
	}

	ctx.say(resp).await?;

	Ok(())
}
