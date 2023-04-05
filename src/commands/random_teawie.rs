use crate::api::guzzle::get_random_teawie;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub async fn run(_: &[CommandDataOption]) -> String {
	get_random_teawie().await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command
		.name("random_teawie")
		.description("get a random teawie!")
}
