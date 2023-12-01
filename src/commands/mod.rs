pub mod ask;
pub mod bing;
pub mod convert;
pub mod copypasta;
pub mod random;
pub mod teawiespam;
pub mod version;

use crate::Data;

use color_eyre::eyre::Report;
use poise::Command;

pub fn to_global_commands() -> Vec<Command<Data, Report>> {
	vec![
		ask::ask(),
		bing::bing(),
		convert::convert(),
		random::random(),
		copypasta::copypasta(),
		teawiespam::teawiespam(),
		version::version(),
	]
}

pub fn to_guild_commands() -> Vec<Command<Data, Report>> {
	vec![copypasta::copypasta(), teawiespam::teawiespam()]
}
