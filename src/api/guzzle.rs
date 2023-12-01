use crate::api::REQWEST_CLIENT;

use color_eyre::eyre::{eyre, Result};
use log::*;
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

	info!("making request to {}", req.url());
	let resp = REQWEST_CLIENT.execute(req).await?;
	let status = resp.status();

	if let StatusCode::OK = status {
		let data = resp.json::<GuzzleResponse>().await?;
		Ok(data.url)
	} else {
		error!(
			"couldn't fetch random teawie from {}! {}",
			resp.url(),
			status
		);

		Err(eyre!("failed to get random teawie with {status}"))
	}
}
