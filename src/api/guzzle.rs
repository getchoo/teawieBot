use once_cell::sync::Lazy;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GuzzleResponse {
	pub url: String,
}

const GUZZLE: &str = "https://api.mydadleft.me";

pub static REQWEST_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
	reqwest::Client::builder()
		.user_agent("teawieBot/0.1.0")
		.build()
		.unwrap_or_default()
});

pub async fn get_random_teawie() -> String {
	let endpoint = "get_random_teawie";
	let req = REQWEST_CLIENT
		.get(format!("{GUZZLE}/{endpoint}"))
		.build()
		.unwrap();
	let resp = REQWEST_CLIENT.execute(req).await.unwrap(); // why did i have to own
													   // this constant? i have
													   // no idea!
	let err_msg = "couldn't get a teawie";

	match resp.status() {
		StatusCode::OK => match resp.json::<GuzzleResponse>().await {
			Ok(data) => data.url,
			Err(why) => format!("{} ({:?})", err_msg, why),
		},
		other => format!("{} ({:?})", err_msg, other),
	}
}
