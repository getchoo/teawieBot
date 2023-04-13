use crate::utils;
use serenity::builder::CreateApplicationCommand;
use serenity::http::client::Http;
use serenity::model::id::ChannelId;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption, CommandDataOptionValue,
};
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
		let replies = utils::get_copypasta(copypasta).await;

		if replies.len() > 1 {
			for reply in replies {
				let resp = channel_id.send_message(&http, |m| m.content(reply)).await;

				match resp {
					Ok(_) => continue,
					Err(why) => {
						println!("couldn't send message: {:?}", why);
						"something went wrong!";
					}
				}
			}
			return "here's your copypasta:".to_string(); // yes this causes the
			                                 // application to not respond.
			                                 // no i don't care.
		}
		return replies[0].to_string();
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
				.add_string_choice("astral", "astral")
				.add_string_choice("dvd", "dvd")
				.add_string_choice("egrill", "egrill")
				.add_string_choice("happymeal", "happymeal")
				.add_string_choice("ismah", "ismah")
				.add_string_choice("sus", "sus")
				.add_string_choice("ticktock", "ticktock")
				.add_string_choice("twitter", "twitter")
		})
}
