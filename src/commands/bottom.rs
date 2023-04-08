use crate::utils::{bottom_decode, bottom_encode};
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption, CommandDataOptionValue,
};

pub async fn run(options: &[CommandDataOption]) -> String {
	let err = "failed to get nested option in";
	let mut ret = "did you forget to enter a message?".to_string();

	let data = options
		.get(0)
		.unwrap_or_else(|| panic!("{} {:?}", err, options));

	// get subcommand to decide whether to encode/decode
	let subcommand = data.name.as_str();

	// get message content
	let option = data
		.options
		.get(0)
		.unwrap_or_else(|| panic!("{} {:?}", err, data))
		.resolved
		.as_ref()
		.expect("failed to resolve string!"); // this is annoying

	if let CommandDataOptionValue::String(msg) = option {
		match subcommand {
			"encode" => {
				ret = bottom_encode(msg).await;
			}
			"decode" => {
				ret = bottom_decode(msg).await;
			}
			_ => {
				ret = "something went wrong :(".to_string();
			}
		};
	}

	ret
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command
		.name("bottom")
		.description("teawie will translate something to/from bottom for you 🥺")
		// nesting...so much nesting
		.create_option(|option| {
			option
				.name("encode")
				.description("teawie will encode a message in bottom for you 🥺")
				.kind(CommandOptionType::SubCommand)
				.create_sub_option(|suboption| {
					suboption
						.name("content")
						.description("what teawie will translate into bottom")
						.kind(CommandOptionType::String)
						.required(true)
				})
		})
		.create_option(|option| {
			option
				.name("decode")
				.description("teawie will decode a message in bottom for you 🥸")
				.kind(CommandOptionType::SubCommand)
				.create_sub_option(|suboption| {
					suboption
						.name("content")
						.description("what teawie will translate from bottom")
						.kind(CommandOptionType::String)
						.required(true)
				})
		})
}
