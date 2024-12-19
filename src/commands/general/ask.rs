use crate::{client::Context, consts, utils};

use eyre::{Context as _, Result};

/// Ask a question!
#[poise::command(prefix_command, slash_command)]
#[allow(clippy::no_effect_underscore_binding)]
pub async fn ask(
	ctx: Context<'_>,
	#[rename = "question"]
	#[description = "The question you want to ask"]
	_question: String,
) -> Result<()> {
	let resp = utils::random_choice(consts::RESPONSES)
		.wrap_err("Couldn't choose from random responses!")?;

	ctx.say(resp).await?;
	Ok(())
}
