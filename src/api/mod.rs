use once_cell::sync::Lazy;

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
