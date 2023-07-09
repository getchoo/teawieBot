use crate::utils;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption, CommandDataOptionValue,
};

pub async fn run(options: &[CommandDataOption]) -> String {
	let err = "couldn't get convert subcommand!";
	let data = options
		.get(0)
		.unwrap_or_else(|| panic!("{} {:?}", err, options));
	let subcommand = data.name.as_str();
	// get message content
	let option = data
		.options
		.get(0)
		.unwrap_or_else(|| panic!("{} {:?}", err, data))
		.resolved
		.as_ref()
		.expect("failed to resolve string!");

	let temp = if let &CommandDataOptionValue::Number(number) = option {
		match subcommand {
			"fahrenheit" => Some(utils::celsius_to_fahrenheit(number)),
			"celsius" => Some(utils::fahrenheit_to_celsius(number)),
			_ => None,
		}
	} else {
		None
	};

	if let Some(temp) = temp {
		format!("{temp:.2}")
	} else {
		"couldn't figure it out oops".to_owned()
	}
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command
		.name("convertto")
		.description("ask teawie to convert something for you")
		.create_option(|option| {
			option
				.name("fahrenheit")
				.description("ask teawie to convert celsius to fahrenheit")
				.kind(CommandOptionType::SubCommand)
				.create_sub_option(|suboption| {
					suboption
						.name("degrees_celsius")
						.description("what teawie will convert")
						.kind(CommandOptionType::Number)
						.required(true)
				})
		})
		.create_option(|option| {
			option
				.name("celsius")
				.description("ask teawie to convert fahrenheit to celsius")
				.kind(CommandOptionType::SubCommand)
				.create_sub_option(|suboption| {
					suboption
						.name("degrees_fahrenheit")
						.description("what teawie will convert")
						.kind(CommandOptionType::Number)
						.required(true)
				})
		})
}
