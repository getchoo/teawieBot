use crate::utils;
use crate::{Context, Error};
use include_dir::{include_dir, Dir};
use log::*;
use std::collections::HashMap;

const FILES: Dir = include_dir!("src/copypastas");

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, poise::ChoiceParameter)]
pub enum Copypastas {
	Astral,
	DVD,
	Egrill,
	HappyMeal,
	//Ismah,
	Sus,
	TickTock,
	Twitter,
    WSYI,
}

impl Copypastas {
	fn as_str(&self) -> &str {
		match self {
			Copypastas::Astral => "astral",
			Copypastas::DVD => "dvd",
			Copypastas::Egrill => "egrill",
			Copypastas::HappyMeal => "happymeal",
			//Copypastas::Ismah => "ismah",
			Copypastas::Sus => "sus",
			Copypastas::TickTock => "ticktock",
			Copypastas::Twitter => "twitter",
			Copypastas::WYSI => "WYSI",
		}
	}
}

fn get_copypasta(name: Copypastas) -> String {
	let mut files: HashMap<&str, &str> = HashMap::new();

	for file in FILES.files() {
		let name = file.path().file_stem().unwrap().to_str().unwrap();

		let contents = file.contents_utf8().unwrap();

		// refer to files by their name w/o extension
		files.insert(name, contents);
	}

	if files.contains_key(name.as_str()) {
		files[name.as_str()].to_string()
	} else {
		format!("i don't have a copypasta named {name} :(")
	}
}

/// ask teawie to send funni copypasta
#[poise::command(slash_command)]
pub async fn copypasta(
	ctx: Context<'_>,
	#[description = "the copypasta you want to send"] copypasta: Copypastas,
) -> Result<(), Error> {
	let gid = ctx.guild_id().unwrap_or_default();
	if !utils::is_guild_allowed(gid) {
		info!("not running copypasta command in {gid}");
		return Ok(());
	}

	ctx.say(get_copypasta(copypasta)).await?;

	Ok(())
}
