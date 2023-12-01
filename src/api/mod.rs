use once_cell::sync::Lazy;

pub mod guzzle;
pub mod shiggy;

pub const USER_AGENT: &str = "teawieBot/";

pub static REQWEST_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
	let version = option_env!("CARGO_PKG_VERSION").unwrap_or("development");

	reqwest::Client::builder()
		.user_agent(format!("{USER_AGENT}/{version}"))
		.build()
		.unwrap_or_default()
});
