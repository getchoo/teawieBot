use crate::Data;

use eyre::Report;
use poise::Command;

mod copypasta;
mod teawiespam;
mod uwurandom;

pub fn to_commands() -> Vec<Command<Data, Report>> {
	vec![
		copypasta::copypasta(),
		teawiespam::teawiespam(),
		uwurandom::uwurandom(),
	]
}
