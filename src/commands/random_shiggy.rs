use crate::api::shiggy::get_random_shiggy;
use crate::{Context, Error};

/// get a random shiggy
#[poise::command(prefix_command, slash_command)]
pub async fn random_shiggy(ctx: Context<'_>) -> Result<(), Error> {
	match get_random_shiggy().await {
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
