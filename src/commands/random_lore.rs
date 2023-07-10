use crate::{consts, utils, Context, Error};

/// get a random piece of teawie lore!
#[poise::command(prefix_command, slash_command)]
pub async fn random_lore(ctx: Context<'_>) -> Result<(), Error> {
	match utils::random_choice(consts::LORE) {
		Ok(resp) => {
			ctx.say(resp).await?;
			Ok(())
		}
		Err(why) => {
			ctx.say("i can't think of any right now :(").await?;
			Err(why)
		}
	}
}
