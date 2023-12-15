use crate::Data;

use color_eyre::eyre::Report;
use poise::Command;

mod general;
mod optional;

pub fn to_global_commands() -> Vec<Command<Data, Report>> {
	general::to_comands()
}

pub fn to_optional_commands() -> Vec<Command<Data, Report>> {
	optional::to_commands()
}
