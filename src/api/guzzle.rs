use crate::api::REQWEST_CLIENT;
use crate::Error;

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GuzzleResponse {
	pub url: String,
}

const GUZZLE: &str = "https://api.mydadleft.me";

pub async fn get_random_teawie() -> Result<String, Error> {
	let endpoint = "/get_random_teawie";

	let req = REQWEST_CLIENT
		.get(format!("{GUZZLE}{endpoint}"))
		.build()
		.unwrap();

	let resp = REQWEST_CLIENT.execute(req).await.unwrap();

	if let StatusCode::OK = resp.status() {
		match resp.json::<GuzzleResponse>().await {
			Ok(data) => Ok(data.url),
			Err(why) => Err(Box::new(why)),
		}
	} else {
		Err(resp.status().to_string().into())
	}
}
