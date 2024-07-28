use eyre::{bail, OptionExt, Result};
use serde::{Deserialize, Serialize};

// https://github.com/getchoo/teawieAPI
#[derive(Deserialize, Serialize)]
struct RandomTeawieResponse {
	url: Option<String>,
	error: Option<String>,
}

// TODO: read this from an env var
const TEAWIE: &str = "https://api.getchoo.com";
const RANDOM: &str = "/random_teawie";

pub async fn random<T>(http: &T) -> Result<String>
where
	T: super::Ext,
{
	let url = format!("{TEAWIE}{RANDOM}");
	let json: RandomTeawieResponse = http.get_json(&url).await?;

	if let Some(error) = json.error {
		bail!("TeawieAPI reported error: {error}");
	};

	json.url
		.ok_or_eyre("TeawieAPI didn't return an error or URL???")
}
