use crate::{Context, Error};

use include_dir::{include_dir, Dir};
use log::{debug, warn};

const FILES: Dir = include_dir!("src/copypastas");

#[derive(Debug, poise::ChoiceParameter)]
pub enum Copypasta {
	Astral,
	Dvd,
	Egrill,
	HappyMeal,
	Sus,
	TickTock,
	Twitter,
}

impl Copypasta {
	fn as_str(&self) -> &str {
		match self {
			Self::Astral => "astral",
			Self::Dvd => "dvd",
			Self::Egrill => "egrill",
			Self::HappyMeal => "happymeal",
			Self::Sus => "sus",
			Self::TickTock => "ticktock",
			Self::Twitter => "twitter",
		}
	}

	fn contents(&self) -> Option<&str> {
		let file_name = format!("{}.txt", self.as_str());
		FILES
			.get_file(file_name)
			.and_then(|file| file.contents_utf8())
	}
}

/// ask teawie to send funni copypasta
#[poise::command(slash_command)]
pub async fn copypasta(
	ctx: Context<'_>,
	#[description = "the copypasta you want to send"] copypasta: Copypasta,
) -> Result<(), Error> {
	let gid = ctx.guild_id().unwrap_or_default();

	if let Some(storage) = &ctx.data().storage {
		let settings = storage.get_guild_settings(&gid).await?;

		if !settings.optional_commands_enabled {
			debug!("Exited copypasta command in {gid} since it's disabled");
			ctx.say("I'm not allowed to do that here").await?;
			return Ok(());
		}
	} else {
		warn!("Ignoring restrictions on copypasta command; no storage backend is attached!");
	}

	if let Some(contents) = copypasta.contents() {
		ctx.say(contents).await?;
	} else {
		ctx.reply("I couldn't find that copypasta :(").await?;
	}

	Ok(())
}
