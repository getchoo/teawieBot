use crate::{api, consts, utils, Context, Error};

#[poise::command(slash_command, subcommands("lore", "teawie", "shiggy"))]
pub async fn random(_ctx: Context<'_>) -> Result<(), Error> {
	Ok(())
}

/// get a random piece of teawie lore!
#[poise::command(prefix_command, slash_command)]
pub async fn lore(ctx: Context<'_>) -> Result<(), Error> {
	match utils::random_choice(consts::LORE) {
		Ok(resp) => {
			ctx.say(resp).await?;
			Ok(())
		}
		Err(why) => {
			ctx.say("i can't think of any right now :(").await?;
			Err(why)
		}
	}
}

/// get a random teawie
#[poise::command(prefix_command, slash_command)]
pub async fn teawie(ctx: Context<'_>) -> Result<(), Error> {
	match api::guzzle::get_random_teawie().await {
		Ok(resp) => {
			ctx.say(resp).await?;
			Ok(())
		}
		Err(why) => {
			ctx.say("i'm too lazy to send a selfie").await?;
			Err(why)
		}
	}
}

/// get a random shiggy
#[poise::command(prefix_command, slash_command)]
pub async fn shiggy(ctx: Context<'_>) -> Result<(), Error> {
	match api::shiggy::get_random_shiggy().await {
		Ok(resp) => {
			ctx.say(resp).await?;
			Ok(())
		}
		Err(why) => {
			ctx.say("i can't get a shiggy right now :(").await?;
			Err(why)
		}
	}
}
