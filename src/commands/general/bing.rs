use crate::Context;

use color_eyre::eyre::Result;

/// Make sure the wie is alive
#[poise::command(prefix_command)]
pub async fn bing(ctx: Context<'_>) -> Result<()> {
	ctx.say("bong!").await?;
	Ok(())
}
