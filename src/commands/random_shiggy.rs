use crate::api::guzzle::REQWEST_CLIENT;
use reqwest::StatusCode;
use serde::Deserialize;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::CommandDataOption;

const URL: &str = "https://safebooru.donmai.us/posts/random.json?tags=kemomimi-chan_(naga_u)+naga_u&only=file_url";
const ERROR_MSG: &str = "couldn't get a shiggy";

#[derive(Deserialize)]
struct SafebooruResponse {
	file_url: String,
}

pub async fn run(_: &[CommandDataOption]) -> String {
	let resp = match REQWEST_CLIENT
		.execute(REQWEST_CLIENT.get(URL).build().unwrap())
		.await
	{
		Ok(r) => r,
		Err(e) => return format!("{} ({:?})", ERROR_MSG, e),
	};

	match resp.status() {
		StatusCode::OK => match resp.json::<SafebooruResponse>().await {
			Ok(sr) => sr.file_url,
			Err(e) => format!("{} ({:?})", ERROR_MSG, e),
		},
		other => {
			println!("{}", resp.text().await.unwrap());
			format!("{} ({:?})", ERROR_MSG, other)
		}
	}
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command
		.name("random_shiggy")
		.description("get a random shiggy!")
}
