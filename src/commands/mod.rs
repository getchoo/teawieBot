use crate::{Data, Error};

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

pub fn to_vec() -> Vec<Command> {
	vec![
		cmd!(general, ask),
		cmd!(general, bing),
		cmd!(general, config),
		cmd!(general, convert),
		cmd!(general, random),
		cmd!(general, version),
		cmd!(moderation, clear_messages),
		cmd!(optional, copypasta),
		cmd!(optional, teawiespam),
		cmd!(optional, uwurandom),
	]
}

pub fn to_vec_global() -> Vec<Command> {
	vec![
		cmd!(general, ask),
		cmd!(general, bing),
		cmd!(general, config),
		cmd!(general, convert),
		cmd!(general, random),
		cmd!(general, version),
		cmd!(moderation, clear_messages),
	]
}

pub fn to_vec_optional() -> Vec<Command> {
	vec![
		cmd!(optional, copypasta),
		cmd!(optional, teawiespam),
		cmd!(optional, uwurandom),
	]
}
