use crate::{api, consts, utils, Context};

use eyre::Result;

#[allow(clippy::unused_async)]
#[poise::command(slash_command, subcommands("lore", "teawie", "shiggy"))]
pub async fn random(_ctx: Context<'_>) -> Result<()> {
	Ok(())
}

/// Get a random piece of teawie lore!
#[poise::command(prefix_command, slash_command)]
pub async fn lore(ctx: Context<'_>) -> Result<()> {
	let resp = utils::random_choice(consts::LORE)?;
	ctx.say(resp).await?;
	Ok(())
}

/// Get a random teawie
#[poise::command(prefix_command, slash_command)]
pub async fn teawie(ctx: Context<'_>) -> Result<()> {
	let url = api::guzzle::get_random_teawie().await?;
	utils::send_url_as_embed(ctx, url).await
}

/// Get a random shiggy
#[poise::command(prefix_command, slash_command)]
pub async fn shiggy(ctx: Context<'_>) -> Result<()> {
	let url = api::shiggy::get_random_shiggy().await?;
	utils::send_url_as_embed(ctx, url).await
}
