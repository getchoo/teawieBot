use crate::utils;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption, CommandDataOptionValue,
};

pub async fn run(options: &[CommandDataOption]) -> String {
	let err_msg = "expected a copyasta";
	let option = options
		.get(0)
		.expect(err_msg)
		.resolved
		.as_ref()
		.expect(err_msg);

	if let CommandDataOptionValue::String(msg) = option {
		return utils::bottom_decode(msg).await;
	}

	"did you forget to enter a message?".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command
		.name("bottom_decode")
		.description("teawie will translate something from bottom for you 🥺")
		.create_option(|option| {
			option
				.name("message")
				.description("what you want teawie to translate")
				.kind(CommandOptionType::String)
				.required(true)
		})
}
