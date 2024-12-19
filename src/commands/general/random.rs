use crate::{client::Context, consts, http, utils};

use eyre::Result;

#[poise::command(slash_command, subcommands("teawie", "shiggy"))]
#[allow(clippy::unused_async)]
pub async fn random(_: Context<'_>) -> Result<()> {
	Ok(())
}

/// Get a random teawie
#[poise::command(prefix_command, slash_command)]
pub async fn teawie(ctx: Context<'_>) -> Result<()> {
	let url = http::teawie::random(&ctx.data().http_client).await?;
	utils::send_url_as_embed(ctx, url).await?;

	Ok(())
}

/// Get a random shiggy
#[poise::command(prefix_command, slash_command)]
pub async fn shiggy(ctx: Context<'_>) -> Result<()> {
	let url = http::shiggy::random(&ctx.data().http_client).await?;
	utils::send_url_as_embed(ctx, url).await?;

	Ok(())
}
