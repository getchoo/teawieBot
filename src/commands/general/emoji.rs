use crate::{consts::Colors, Context, Error};

use poise::{
	serenity_prelude::{CreateEmbed, Emoji},
	CreateReply,
};

/// Get the URL for an emoji
#[poise::command(slash_command)]
pub async fn emoji(ctx: Context<'_>, emoji: Emoji) -> Result<(), Error> {
	let url = emoji.url();
	let embed = CreateEmbed::new()
		.title(emoji.name)
		.color(Colors::Blue)
		.image(&url)
		.url(&url);
	let message = CreateReply::default().embed(embed);

	ctx.send(message).await?;

	Ok(())
}
