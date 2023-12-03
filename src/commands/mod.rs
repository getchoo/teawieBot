use crate::Data;

use color_eyre::eyre::Report;
use poise::Command;

mod general;
mod moderation;
mod optional;

pub fn to_global_commands() -> Vec<Command<Data, Report>> {
	vec![
		general::ask(),
		general::bing(),
		general::convert(),
		general::random(),
		general::version(),
		moderation::config(),
		moderation::ban_user(),
		moderation::kick_user(),
	]
}

pub fn to_guild_commands() -> Vec<Command<Data, Report>> {
	vec![optional::copypasta(), optional::teawiespam()]
}
