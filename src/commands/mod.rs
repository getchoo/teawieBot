use crate::Data;

use eyre::Report;
use poise::Command;

mod general;
mod moderation;
mod optional;

pub fn global() -> Vec<Command<Data, Report>> {
	general::to_comands()
}

pub fn optional() -> Vec<Command<Data, Report>> {
	optional::to_commands()
}

pub fn moderation() -> Vec<Command<Data, Report>> {
	moderation::to_commands()
}
