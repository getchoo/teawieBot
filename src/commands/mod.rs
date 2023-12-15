use crate::Data;

use color_eyre::eyre::Report;
use poise::Command;

mod general;
mod optional;

pub fn global() -> Vec<Command<Data, Report>> {
	general::to_comands()
}

pub fn optional() -> Vec<Command<Data, Report>> {
	optional::to_commands()
}
