use crate::{
	client::{Context, Error},
	consts, utils,
};

use eyre::Context as _;

/// Ask teawie a question!
#[poise::command(prefix_command, slash_command)]
#[allow(clippy::no_effect_underscore_binding)]
pub async fn ask(
	ctx: Context<'_>,
	#[rename = "question"]
	#[description = "The question you want to ask teawie"]
	_question: String,
) -> Result<(), Error> {
	let resp = utils::random_choice(consts::RESPONSES)
		.wrap_err("Couldn't choose from random responses!")?;

	ctx.say(resp).await?;
	Ok(())
}
