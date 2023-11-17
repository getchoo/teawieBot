use crate::consts;
use crate::utils;
use crate::{Context, Error};

/// ask teawie a question!
#[poise::command(prefix_command, slash_command)]
pub async fn ask(
	ctx: Context<'_>,
	#[description = "the question you want to ask teawie"]
	#[rename = "question"]
	_question: String,
) -> Result<(), Error> {
	match utils::random_choice(consts::RESPONSES) {
		Ok(resp) => {
			ctx.say(resp).await?;
			Ok(())
		}
		Err(why) => {
			ctx.say("idk").await?;
			Err(why)
		}
	}
}
