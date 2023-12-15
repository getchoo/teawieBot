use crate::Context;

use std::collections::HashMap;

use color_eyre::eyre::{eyre, Result};
use include_dir::{include_dir, Dir};
use log::debug;

const FILES: Dir = include_dir!("src/copypastas");

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, poise::ChoiceParameter)]
pub enum Copypastas {
	Astral,
	DVD,
	Egrill,
	HappyMeal,
	Sus,
	TickTock,
	Twitter,
}

impl Copypastas {
	fn as_str(&self) -> &str {
		match self {
			Copypastas::Astral => "astral",
			Copypastas::DVD => "dvd",
			Copypastas::Egrill => "egrill",
			Copypastas::HappyMeal => "happymeal",
			Copypastas::Sus => "sus",
			Copypastas::TickTock => "ticktock",
			Copypastas::Twitter => "twitter",
		}
	}
}

fn get_copypasta(name: &Copypastas) -> Result<String> {
	let mut files: HashMap<&str, &str> = HashMap::new();

	for file in FILES.files() {
		let name = file
			.path()
			.file_stem()
			.ok_or_else(|| eyre!("Couldn't get file stem from {file:#?}"))?
			.to_str()
			.ok_or_else(|| eyre!("Couldn't convert file stem to str!"))?;

		let contents = file
			.contents_utf8()
			.ok_or_else(|| eyre!("Couldnt get contents from copypasta!"))?;

		// refer to files by their name w/o extension
		files.insert(name, contents);
	}

	if files.contains_key(name.as_str()) {
		Ok(files[name.as_str()].to_string())
	} else {
		Err(eyre!("Couldnt find copypasta {name}!"))
	}
}

/// ask teawie to send funni copypasta
#[poise::command(slash_command)]
pub async fn copypasta(
	ctx: Context<'_>,
	#[description = "the copypasta you want to send"] copypasta: Copypastas,
) -> Result<()> {
	let gid = ctx.guild_id().unwrap_or_default();
	let settings = ctx.data().storage.get_guild_settings(&gid).await?;

	if !settings.optional_commands_enabled {
		debug!("Exited copypasta command in {gid} since it's disabled");
		return Ok(());
	}

	ctx.say(get_copypasta(&copypasta)?).await?;

	Ok(())
}
