use eyre::Result;
use log::debug;
use serde::Deserialize;

const SHIGGY: &str = "https://safebooru.donmai.us";
const RANDOM_SHIGGY: &str = "/posts/random.json?tags=kemomimi-chan_(naga_u)+naga_u&only=file_url";

#[derive(Deserialize)]
struct SafebooruResponse {
	file_url: String,
}

#[allow(clippy::module_name_repetitions)]
pub async fn random_shiggy() -> Result<String> {
	let url = format!("{SHIGGY}{RANDOM_SHIGGY}");
	debug!("Making request to {url}");

	let resp: SafebooruResponse = super::get_json(&url).await?;
	Ok(resp.file_url)
}
