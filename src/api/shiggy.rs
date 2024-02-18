use crate::api::REQWEST_CLIENT;

use eyre::{eyre, Result};
use log::debug;
use reqwest::StatusCode;
use serde::Deserialize;

const SHIGGY: &str = "https://safebooru.donmai.us";
const RANDOM_SHIGGY: &str = "/posts/random.json?tags=kemomimi-chan_(naga_u)+naga_u&only=file_url";

#[derive(Deserialize)]
struct SafebooruResponse {
	file_url: String,
}

#[allow(clippy::module_name_repetitions)]
pub async fn get_random_shiggy() -> Result<String> {
	let req = REQWEST_CLIENT
		.get(format!("{SHIGGY}{RANDOM_SHIGGY}"))
		.build()?;

	debug!("Making request to {}", req.url());
	let resp = REQWEST_CLIENT.execute(req).await?;
	let status = resp.status();

	if let StatusCode::OK = status {
		let data: SafebooruResponse = resp.json().await?;
		Ok(data.file_url)
	} else {
		Err(eyre!("Failed to get random shiggy with {status}"))
	}
}
