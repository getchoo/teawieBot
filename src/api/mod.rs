use eyre::Result;
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;

pub mod guzzle;
pub mod shiggy;

pub static USER_AGENT: Lazy<String> = Lazy::new(|| {
	let version = option_env!("CARGO_PKG_VERSION").unwrap_or("development");

	format!("teawieBot/{version}")
});

pub static REQWEST_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
	reqwest::Client::builder()
		.user_agent(USER_AGENT.to_string())
		.build()
		.unwrap_or_default()
});

async fn get_json<T: DeserializeOwned>(url: &str) -> Result<T> {
	let resp = REQWEST_CLIENT.get(url).send().await?;
	resp.error_for_status_ref()?;
	let json = resp.json().await?;

	Ok(json)
}
