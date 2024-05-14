use eyre::Result;
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RandomTeawieResponse {
	url: String,
}

// TODO: read this from an env var
const GUZZLE: &str = "https://api.getchoo.com";
const RANDOM_TEAWIE: &str = "/random_teawie";

pub async fn random_teawie() -> Result<String> {
	let url = format!("{GUZZLE}{RANDOM_TEAWIE}");
	debug!("Making request to {url}");
	let json: RandomTeawieResponse = super::get_json(&url).await?;

	Ok(json.url)
}
