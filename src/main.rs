use std::{sync::Arc, time::Duration};

use eyre::{Context as _, Report, Result};
use log::{info, trace, warn};
use poise::{
	serenity_prelude::{self as serenity},
	EditTracker, Framework, FrameworkOptions, PrefixFrameworkOptions,
};
use tokio::signal::ctrl_c;
#[cfg(target_family = "unix")]
use tokio::signal::unix::{signal, SignalKind};
#[cfg(target_family = "windows")]
use tokio::signal::windows::ctrl_close;

mod api;
mod commands;
mod consts;
mod handlers;
mod storage;
mod utils;

use storage::Storage;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Clone, Debug, Default)]
pub struct Data {
	storage: Option<Storage>,
}

async fn setup(ctx: &serenity::Context) -> Result<Data, Error> {
	let storage = Storage::from_env().ok();

	if let Some(storage) = storage.as_ref() {
		if !storage.clone().is_connected() {
			return Err(
				"You specified a storage backend but there's no connection! Is it running?".into(),
			);
		}
		trace!("Storage backend connected!");

		poise::builtins::register_globally(ctx, &commands::to_vec_global()).await?;
		info!("Registered global commands!");

		// register "extra" commands in guilds that allow it
		let guilds = storage.get_opted_guilds().await?;

		for guild in guilds {
			poise::builtins::register_in_guild(ctx, &commands::to_vec_optional(), guild).await?;

			info!("Registered guild commands to {}", guild);
		}
	} else {
		warn!("No storage backend was specified. Features requiring storage will be disabled");
		warn!("Registering optional commands globally since there's no storage backend");
		poise::builtins::register_globally(ctx, &commands::to_vec()).await?;
	}

	let data = Data { storage };

	Ok(data)
}

async fn handle_shutdown(shard_manager: Arc<serenity::ShardManager>, reason: &str) {
	warn!("{reason}! Shutting down bot...");
	shard_manager.shutdown_all().await;
	println!("Everything is shutdown. Goodbye!");
}

#[tokio::main]
async fn main() -> Result<()> {
	dotenvy::dotenv().ok();
	color_eyre::install()?;
	env_logger::init();

	let token = std::env::var("TOKEN").wrap_err("Couldn't find bot token in environment!")?;

	let intents =
		serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

	let options = FrameworkOptions {
		commands: commands::to_vec(),
		on_error: |error| Box::pin(handlers::error::handle(error)),

		command_check: Some(|ctx| {
			Box::pin(async move { Ok(ctx.author().id != ctx.framework().bot_id) })
		}),

		event_handler: |ctx, event, _framework, data| {
			Box::pin(handlers::event::handle(ctx, event, data))
		},

		prefix_options: PrefixFrameworkOptions {
			prefix: Some("!".into()),
			edit_tracker: Some(Arc::new(EditTracker::for_timespan(Duration::from_secs(
				3600,
			)))),
			..Default::default()
		},

		..Default::default()
	};

	let framework = Framework::builder()
		.options(options)
		.setup(|ctx, _ready, _framework| Box::pin(setup(ctx)))
		.build();

	let mut client = serenity::ClientBuilder::new(token, intents)
		.framework(framework)
		.await?;

	let shard_manager = client.shard_manager.clone();
	#[cfg(target_family = "unix")]
	let mut sigterm = signal(SignalKind::terminate())?;
	#[cfg(target_family = "windows")]
	let mut sigterm = ctrl_close()?;

	tokio::select! {
		result = client.start() => result.map_err(Report::from),
		_ = sigterm.recv() => {
			handle_shutdown(shard_manager, "Received SIGTERM").await;
			std::process::exit(0);
		},
		_ = ctrl_c() => {
			handle_shutdown(shard_manager, "Interrupted").await;
			std::process::exit(130);
		}
	}
}
