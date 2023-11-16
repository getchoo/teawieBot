pub mod ask;
pub mod bing;
pub mod convert;
pub mod copypasta;
pub mod random_lore;
pub mod random_shiggy;
pub mod random_teawie;
pub mod teawiespam;
pub mod version;

use crate::{Data, Error};
use poise::Command;

pub fn to_global_commands() -> Vec<Command<Data, Error>> {
	vec![
		ask::ask(),
		bing::bing(),
		convert::convert(),
		random_lore::random_lore(),
		random_shiggy::random_shiggy(),
		random_teawie::random_teawie(),
		copypasta::copypasta(),
		teawiespam::teawiespam(),
		version::version(),
	]
}

pub fn to_guild_commands() -> Vec<Command<Data, Error>> {
	vec![copypasta::copypasta(), teawiespam::teawiespam()]
}
