use crate::utils::get_random_lore;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_: &[CommandDataOption]) -> String {
	get_random_lore()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command
		.name("random_lore")
		.description("get a random piece of teawie lore!")
}
