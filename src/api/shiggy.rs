use crate::api::REQWEST_CLIENT;
use crate::Error;
use reqwest::StatusCode;
use serde::Deserialize;

const URL: &str = "https://safebooru.donmai.us/posts/random.json?tags=kemomimi-chan_(naga_u)+naga_u&only=file_url";

#[derive(Deserialize)]
struct SafebooruResponse {
	file_url: String,
}

pub async fn get_random_shiggy() -> Result<String, Error> {
	let req = REQWEST_CLIENT.get(URL).build().unwrap();

	let resp = REQWEST_CLIENT.execute(req).await.unwrap();

	if let StatusCode::OK = resp.status() {
		match resp.json::<SafebooruResponse>().await {
			Ok(data) => Ok(data.file_url),
			Err(why) => Err(Box::new(why)),
		}
	} else {
		Err(resp.status().to_string().into())
	}
}
