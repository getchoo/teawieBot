use std::sync::OnceLock;

use eyre::Result;
use reqwest::Client;
use serde::de::DeserializeOwned;

pub mod guzzle;
pub mod shiggy;

pub fn client() -> &'static Client {
	static USER_AGENT: OnceLock<String> = OnceLock::new();
	static CLIENT: OnceLock<Client> = OnceLock::new();

	let user_agent = USER_AGENT.get_or_init(|| {
		let version = option_env!("CARGO_PKG_VERSION").unwrap_or("development");

		format!("teawieBot/{version}")
	});

	CLIENT.get_or_init(|| Client::builder().user_agent(user_agent).build().unwrap())
}

async fn get_json<T: DeserializeOwned>(url: &str) -> Result<T> {
	let resp = client().get(url).send().await?;
	resp.error_for_status_ref()?;
	let json = resp.json().await?;

	Ok(json)
}
