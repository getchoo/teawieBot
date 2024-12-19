use crate::client::{Data, Error};

mod general;
mod moderation;
mod optional;

type Command = poise::Command<Data, Error>;

#[macro_export]
macro_rules! cmd {
	($module: ident, $name: ident) => {
		$module::$name::$name()
	};

	($module: ident, $name: ident, $func: ident) => {
		$module::$name::$func()
	};
}

pub fn all() -> Vec<Command> {
	let mut all_commands = global();
	all_commands.append(&mut optional());
	all_commands
}

pub fn global() -> Vec<Command> {
	vec![
		cmd!(general, ask),
		cmd!(general, bing),
		cmd!(general, config),
		cmd!(general, convert),
		cmd!(general, emoji),
		cmd!(general, pfp),
		cmd!(general, random),
		cmd!(general, version),
		cmd!(moderation, clear_messages),
	]
}

pub fn optional() -> Vec<Command> {
	vec![cmd!(optional, uwurandom)]
}
