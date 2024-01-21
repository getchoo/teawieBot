use crate::colors::Colors;
use crate::Context;

use color_eyre::eyre::Result;
use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;

/// Get version info
#[poise::command(slash_command)]
pub async fn version(ctx: Context<'_>) -> Result<()> {
	let sha = option_env!("GIT_SHA").unwrap_or("main");
	let target = option_env!("TARGET").unwrap_or("Unknown");

	let revision_url = format!(
		"[{}]({}/tree/{})",
		sha,
		option_env!("CARGO_PKG_REPOSITORY").unwrap_or("https://github.com/getchoo/teawieBot"),
		sha,
	);

	let fields = [
		(
			"Version:",
			option_env!("CARGO_PKG_VERSION").unwrap_or("not found"),
			false,
		),
		("Target:", target, false),
		("Revision:", &revision_url, false),
		("User Agent:", &crate::api::USER_AGENT, false),
	];

	let embed = CreateEmbed::new()
		.title("Version Information")
		.description("now with over 9000 more builders!")
		.fields(fields)
		.color(Colors::Blue);
	let message = CreateReply::default().embed(embed);

	ctx.send(message).await?;

	Ok(())
}
