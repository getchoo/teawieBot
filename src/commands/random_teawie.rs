use crate::api::guzzle::get_random_teawie;
use crate::{Context, Error};

/// get a random teawie
#[poise::command(prefix_command, slash_command)]
pub async fn random_teawie(ctx: Context<'_>) -> Result<(), Error> {
	match get_random_teawie().await {
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
