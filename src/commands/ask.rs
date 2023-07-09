use crate::utils;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_: &[CommandDataOption]) -> String {
	utils::get_random_response()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command
		.name("ask")
		.description("ask lord teawie a question and they shall respond")
		.create_option(|option| {
			option
				.name("question")
				.description("the question you want to ask teawie")
				.kind(CommandOptionType::String)
				.required(true)
		})
}
