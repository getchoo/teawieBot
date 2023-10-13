use once_cell::sync::Lazy;

pub mod guzzle;
pub mod shiggy;

pub static REQWEST_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
	reqwest::Client::builder()
		.user_agent("teawieBot/0.1.0")
		.build()
		.unwrap_or_default()
});
