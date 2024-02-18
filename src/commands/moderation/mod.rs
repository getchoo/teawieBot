use crate::Data;

use color_eyre::eyre::Report;
use poise::Command;

mod clear;

pub fn to_commands() -> Vec<Command<Data, Report>> {
	vec![clear::clear_messages()]
}
