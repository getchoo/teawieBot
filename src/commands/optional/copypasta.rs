use crate::client::Context;

use eyre::Result;
use include_dir::{include_dir, Dir};
use log::debug;

const COPYPASTAS: Dir = include_dir!("src/copypastas");

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

impl ToString for Copypasta {
	fn to_string(&self) -> String {
		let str = match self {
			Self::Astral => "astral",
			Self::Dvd => "dvd",
			Self::Egrill => "egrill",
			Self::HappyMeal => "happymeal",
			Self::Sus => "sus",
			Self::TickTock => "ticktock",
			Self::Twitter => "twitter",
		};
		str.to_string()
	}
}

impl Copypasta {
	fn contents(&self) -> Option<&str> {
		let file_name = format!("{}.txt", self.to_string());
		COPYPASTAS
			.get_file(file_name)
			.and_then(|file| file.contents_utf8())
	}
}

/// ask teawie to send funni copypasta
#[poise::command(slash_command)]
pub async fn copypasta(
	ctx: Context<'_>,
	#[description = "the copypasta you want to send"] copypasta: Copypasta,
) -> Result<()> {
	if let Some(guild_id) = ctx.guild_id() {
		if let Some(storage) = &ctx.data().storage {
			let settings = storage.get_guild_settings(&guild_id).await?;

			if !settings.optional_commands_enabled {
				debug!("Not running command in {guild_id} since it's disabled");
				ctx.reply("I'm not allowed to do that here").await?;

				return Ok(());
			}
		} else {
			debug!("Ignoring restrictions on command; no storage backend is attached!");
		}
	} else {
		debug!("Ignoring restrictions on command; we're not in a guild");
	}

	if let Some(contents) = copypasta.contents() {
		ctx.say(contents).await?;
	} else {
		ctx.reply("I couldn't find that copypasta :(").await?;
	}

	Ok(())
}
