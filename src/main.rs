mod client;
mod commands;
mod consts;
mod events;
mod http;
mod storage;
mod utils;

use anyhow::Result;

use tokio::signal::ctrl_c;
#[cfg(target_family = "unix")]
use tokio::signal::unix::{signal, SignalKind};
#[cfg(target_family = "windows")]
use tokio::signal::windows::ctrl_close;

#[tokio::main]
async fn main() -> Result<()> {
	dotenvy::dotenv().ok();
	env_logger::init();

	let mut client = client::get().await?;

	let shard_manager = client.shard_manager.clone(); // We need this to shut down the bot
	#[cfg(target_family = "unix")]
	let mut sigterm = signal(SignalKind::terminate())?;
	#[cfg(target_family = "windows")]
	let mut sigterm = ctrl_close()?;

	tokio::select! {
		result = client.start() => result.map_err(anyhow::Error::from),
		_ = sigterm.recv() => {
			client::handle_shutdown(shard_manager, "Received SIGTERM").await;
			println!("Everything is shutdown. Goodbye!");
			std::process::exit(0);
		},
		_ = ctrl_c() => {
			client::handle_shutdown(shard_manager, "Interrupted").await;
			println!("Everything is shutdown. Goodbye!");
			std::process::exit(130);
		}
	}
}
