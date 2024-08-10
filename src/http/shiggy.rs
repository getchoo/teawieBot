use eyre::Result;
use serde::Deserialize;

const SHIGGY: &str = "https://safebooru.donmai.us";
const RANDOM: &str = "/posts/random.json?tags=kemomimi-chan_(naga_u)+naga_u&only=file_url";

#[derive(Deserialize)]
struct SafebooruResponse {
	file_url: String,
}

pub async fn random<T>(http: &T) -> Result<String>
where
	T: super::Ext,
{
	let url = format!("{SHIGGY}{RANDOM}");
	let resp: SafebooruResponse = http.get_json(&url).await?;

	Ok(resp.file_url)
}
