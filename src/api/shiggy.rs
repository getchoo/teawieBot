use crate::api::REQWEST_CLIENT;
use crate::Error;

use log::*;
use reqwest::StatusCode;
use serde::Deserialize;

const SHIGGY: &str = "https://safebooru.donmai.us";

#[derive(Deserialize)]
struct SafebooruResponse {
	file_url: String,
}

pub async fn get_random_shiggy() -> Result<String, Error> {
	let endpoint = "/posts/random.json?tags=kemomimi-chan_(naga_u)+naga_u&only=file_url";

	let req = REQWEST_CLIENT
		.get(format!("{SHIGGY}{endpoint}"))
		.build()
		.unwrap();

	info!("making request to {}", req.url());
	let resp = REQWEST_CLIENT.execute(req).await.unwrap();
	let status = resp.status();

	if let StatusCode::OK = status {
		match resp.json::<SafebooruResponse>().await {
			Ok(data) => Ok(data.file_url),
			Err(why) => {
				if let Some(url) = why.url() {
					error!("failed to make a request to {}! {}", url, why)
				} else {
					error!("couldn't even figure out the url! {}", why)
				};

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
