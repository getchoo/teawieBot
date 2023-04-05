use crate::utils;
use serenity::builder::CreateApplicationCommand;
use serenity::http::client::Http;
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption, CommandDataOptionValue,
};
use serenity::prelude::SerenityError;
use std::sync::Arc;

pub async fn run(options: &[CommandDataOption], channel_id: ChannelId, http: &Arc<Http>) -> String {
	let err_msg = "expected a copyasta";
	let option = options
		.get(0)
		.expect(err_msg)
		.resolved
		.as_ref()
		.expect(err_msg);

	if let CommandDataOptionValue::String(copypasta) = option {
		let replies = &utils::get_copypasta(copypasta).await;
		let len = replies.len() - 1;

		// send messages separately if we have > 1
		for (i, reply) in replies.iter().enumerate() {
			let resp: Result<Message, SerenityError>;

			if i < len {
				resp = channel_id.send_message(&http, |m| m.content(reply)).await;

				match resp {
					Ok(_) => continue,
					Err(why) => {
						println!("couldn't send message: {:?}", why);
					}
				}
			}
		}

		return replies[len].to_string();
	}

	"couldn't find a copypasta".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command
		.name("copypasta")
		.description("send funni copypasta")
		.create_option(|option| {
			option
				.name("copypasta")
				.description("the copypasta you want to send")
				.kind(CommandOptionType::String)
				.required(true)
				// .add_string_choice("ismah", "ismah") // renable this later
				.add_string_choice("happymeal", "happymeal")
				.add_string_choice("sus", "sus")
				.add_string_choice("ticktock", "ticktock")
				.add_string_choice("egrill", "egrill")
				.add_string_choice("dvd", "dvd")
				.add_string_choice("twitter", "twitter")
		})
}
