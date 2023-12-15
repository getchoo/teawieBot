use crate::Data;

use color_eyre::eyre::Report;
use poise::Command;

mod copypasta;
mod teawiespam;

pub fn to_commands() -> Vec<Command<Data, Report>> {
	vec![copypasta::copypasta(), teawiespam::teawiespam()]
}
