use crate::{Context, Error};
use bottomify::bottom::{decode_string, encode_string};

#[poise::command(
	slash_command,
	subcommands("to_fahrenheit", "to_celsius", "to_bottom", "from_bottom")
)]
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

/// teawie will translate to bottom 🥺
#[poise::command(slash_command)]
pub async fn to_bottom(
	ctx: Context<'_>,
	#[description = "what teawie will translate into bottom"] message: String,
) -> Result<(), Error> {
	let encoded = encode_string(&message);
	ctx.say(encoded).await?;
	Ok(())
}

/// teawie will translate from bottom 🥸
#[poise::command(slash_command)]
pub async fn from_bottom(
	ctx: Context<'_>,
	#[description = "what teawie will translate from bottom"] message: String,
) -> Result<(), Error> {
	let d = decode_string(&message);
	match d {
		Ok(decoded) => {
			ctx.say(decoded).await?;
			Ok(())
		}
		Err(why) => {
			ctx.say("couldn't decode that for you, i'm sowwy!! :((".to_string())
				.await?;
			Err(Box::new(why))
		}
	}
}
