use crate::{Context, Error};
use bottomify::bottom::{decode_string, encode_string};

fn decode_sync(s: &str) -> Result<String, bottomify::bottom::TranslationError> {
	decode_string(&s)
}

#[poise::command(slash_command, subcommands("encode", "decode"))]
pub async fn bottom(_ctx: Context<'_>) -> Result<(), Error> {
	Ok(())
}

/// teawie will translate to bottom 🥺
#[poise::command(slash_command)]
pub async fn encode(
	ctx: Context<'_>,
	#[description = "what teawie will translate into bottom"] message: String,
) -> Result<(), Error> {
	let encoded = encode_string(&message);
	ctx.say(encoded).await?;
	Ok(())
}

/// teawie will translate from bottom 🥸
#[poise::command(slash_command)]
pub async fn decode(
	ctx: Context<'_>,
	#[description = "what teawie will translate from bottom"] message: String,
) -> Result<(), Error> {
	let d = decode_sync(&message);
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
