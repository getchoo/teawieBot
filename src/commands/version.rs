use crate::{Context, Error};

/// get version info
#[poise::command(slash_command)]
pub async fn version(ctx: Context<'_>) -> Result<(), Error> {
	let sha = option_env!("GIT_SHA").unwrap_or("main");

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
		("Revision:", &revision_url, false),
		("User Agent:", crate::api::USER_AGENT, false),
	];

	ctx.send(|c| {
		c.embed(|e| {
			e.title("Vesion Information")
				.description("powered by poise!")
				.fields(fields)
				.color((136, 199, 253))
		})
	})
	.await?;

	Ok(())
}
