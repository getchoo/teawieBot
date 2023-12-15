use crate::Data;

use color_eyre::eyre::Report;
use poise::Command;

mod ask;
mod bing;
mod config;
mod convert;
mod random;
mod version;

pub fn to_comands() -> Vec<Command<Data, Report>> {
	vec![
		ask::ask(),
		bing::bing(),
		config::config(),
		convert::convert(),
		random::random(),
		version::version(),
	]
}
