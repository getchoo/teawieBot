use once_cell::sync::Lazy;

pub mod guzzle;
pub mod shiggy;

pub const USER_AGENT: &str = "teawieBot/0.1.0";

pub static REQWEST_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
	reqwest::Client::builder()
		.user_agent(USER_AGENT)
		.build()
		.unwrap_or_default()
});
