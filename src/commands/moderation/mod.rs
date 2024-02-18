use crate::Data;

use eyre::Report;
use poise::Command;

mod clear;

pub fn to_commands() -> Vec<Command<Data, Report>> {
	vec![clear::clear_messages()]
}
