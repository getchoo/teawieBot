use crate::api::REQWEST_CLIENT;
use reqwest::StatusCode;
use serde::Deserialize;

const URL: &str = "https://safebooru.donmai.us/posts/random.json?tags=kemomimi-chan_(naga_u)+naga_u&only=file_url";
const ERROR_MSG: &str = "couldn't get a shiggy";

#[derive(Deserialize)]
struct SafebooruResponse {
	file_url: String,
}

pub async fn get_random_shiggy() -> String {
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
		other => format!("{} ({:?})", ERROR_MSG, other),
	}
}
