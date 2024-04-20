use crate::{api, consts, utils, Context, Error};

#[poise::command(slash_command, subcommands("lore", "teawie", "shiggy"))]
#[allow(clippy::unused_async)]
pub async fn random(_: Context<'_>) -> Result<(), Error> {
	Ok(())
}

/// Get a random piece of teawie lore!
#[poise::command(prefix_command, slash_command)]
pub async fn lore(ctx: Context<'_>) -> Result<(), Error> {
	let resp = utils::random_choice(consts::LORE)?;
	ctx.say(resp).await?;

	Ok(())
}

/// Get a random teawie
#[poise::command(prefix_command, slash_command)]
pub async fn teawie(ctx: Context<'_>) -> Result<(), Error> {
	let url = api::guzzle::random_teawie().await?;
	utils::send_url_as_embed(ctx, url).await?;

	Ok(())
}

/// Get a random shiggy
#[poise::command(prefix_command, slash_command)]
pub async fn shiggy(ctx: Context<'_>) -> Result<(), Error> {
	let url = api::shiggy::random_shiggy().await?;
	utils::send_url_as_embed(ctx, url).await?;

	Ok(())
}
