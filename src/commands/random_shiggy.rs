use crate::api::shiggy::get_random_shiggy;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::CommandDataOption;

pub async fn run(_: &[CommandDataOption]) -> String {
	get_random_shiggy().await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command
		.name("random_shiggy")
		.description("get a random shiggy!")
}
