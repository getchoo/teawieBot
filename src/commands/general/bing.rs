use crate::{Context, Error};

use eyre::Result;

/// Make sure the wie is alive
#[poise::command(prefix_command)]
pub async fn bing(ctx: Context<'_>) -> Result<(), Error> {
	ctx.say("bong!").await?;
	Ok(())
}
