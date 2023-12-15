use crate::api::REQWEST_CLIENT;

use color_eyre::eyre::{eyre, Result};
use log::debug;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GuzzleResponse {
	pub url: String,
}

const GUZZLE: &str = "https://api.mydadleft.me";
const RANDOM_TEAWIE: &str = "/random_teawie";

pub async fn get_random_teawie() -> Result<String> {
	let req = REQWEST_CLIENT
		.get(format!("{GUZZLE}{RANDOM_TEAWIE}"))
		.build()?;

	debug!("Making request to {}", req.url());
	let resp = REQWEST_CLIENT.execute(req).await?;
	let status = resp.status();

	if let StatusCode::OK = status {
		let data: GuzzleResponse = resp.json().await?;
		Ok(data.url)
	} else {
		Err(eyre!("Failed to get random Teawie with {status}"))
	}
}
