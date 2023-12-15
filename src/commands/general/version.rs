use crate::colors::Colors;
use crate::Context;

use color_eyre::eyre::Result;

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

	ctx.send(|c| {
		c.embed(|e| {
			e.title("Version Information")
				.description("powered by poise!")
				.fields(fields)
				.color(Colors::Blue)
		})
	})
	.await?;

	Ok(())
}
