use crate::api::REQWEST_CLIENT;
use crate::Error;

use log::*;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GuzzleResponse {
	pub url: String,
}

const GUZZLE: &str = "https://api.mydadleft.me";

pub async fn get_random_teawie() -> Result<String, Error> {
	let endpoint = "/get_random_teawie";

	let req = REQWEST_CLIENT
		.get(format!("{GUZZLE}{endpoint}"))
		.build()
		.unwrap();

	info!("making request to {}", req.url());
	let resp = REQWEST_CLIENT.execute(req).await.unwrap();
	let status = resp.status();

	if let StatusCode::OK = status {
		match resp.json::<GuzzleResponse>().await {
			Ok(data) => Ok(data.url),
			Err(why) => {
				if let Some(url) = why.url() {
					error!("error parsing json from {}! {}", url, why)
				} else {
					error!("couldn't even get the url! {}", why);
				}

				Err(Box::new(why))
			}
		}
	} else {
		error!(
			"couldn't fetch random teawie from {}! {}",
			resp.url(),
			status
		);

		Err(status.to_string().into())
	}
}
