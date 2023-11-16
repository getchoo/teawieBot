pub mod ask;
pub mod bing;
pub mod convert;
pub mod copypasta;
pub mod random;
pub mod teawiespam;
pub mod version;

use crate::{Data, Error};
use poise::Command;

pub fn to_global_commands() -> Vec<Command<Data, Error>> {
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

pub fn to_guild_commands() -> Vec<Command<Data, Error>> {
	vec![copypasta::copypasta(), teawiespam::teawiespam()]
}
