use crate::client::Context;

use anyhow::Result;

/// Make sure the wie is alive
#[poise::command(prefix_command)]
pub async fn bing(ctx: Context<'_>) -> Result<()> {
	ctx.say("bong!").await?;
	Ok(())
}
