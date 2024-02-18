#![warn(clippy::all, clippy::pedantic, clippy::perf)]
#![allow(clippy::missing_errors_doc, clippy::used_underscore_binding)]
#![forbid(unsafe_code)]

use std::sync::Arc;
use std::time::Duration;

use color_eyre::eyre::{eyre, Context as _, Report, Result};
use color_eyre::owo_colors::OwoColorize;
use log::{info, warn};
use poise::serenity_prelude as serenity;
use poise::{EditTracker, Framework, FrameworkOptions, PrefixFrameworkOptions};
use redis::ConnectionLike;
use storage::Storage;
use tokio::signal::ctrl_c;
use tokio::signal::unix::{signal, SignalKind};

mod api;
mod colors;
mod commands;
mod consts;
mod handlers;
mod storage;
mod utils;

type Context<'a> = poise::Context<'a, Data, Report>;

#[derive(Clone)]
pub struct Data {
	storage: Storage,
}

impl Data {
	pub fn new() -> Result<Self> {
		let redis_url = std::env::var("REDIS_URL")
			.wrap_err_with(|| "Couldn't find Redis URL in environment!")?;

		let storage = Storage::new(&redis_url)?;

		Ok(Self { storage })
	}
}

async fn setup(
	ctx: &serenity::Context,
	_ready: &serenity::Ready,
	framework: &Framework<Data, Report>,
) -> Result<Data> {
	let data = Data::new()?;
	let mut client = data.storage.client.clone();

	if !client.check_connection() {
		return Err(eyre!(
			"Couldn't connect to storage! Is your daemon running?"
		));
	}

	poise::builtins::register_globally(ctx, &framework.options().commands).await?;
	info!("Registered global commands!");

	// register "extra" commands in guilds that allow it
	let guilds = data.storage.get_opted_guilds().await?;

	for guild in guilds {
		poise::builtins::register_in_guild(ctx, &commands::optional(), guild).await?;

		info!("Registered guild commands to {}", guild);
	}

	Ok(data)
}

async fn handle_shutdown(shard_manager: Arc<serenity::ShardManager>, reason: &str) {
	warn!("{reason}! Shutting down bot...");
	shard_manager.shutdown_all().await;
	println!("{}", "Everything is shutdown. Goodbye!".green());
}

#[tokio::main]
async fn main() -> Result<()> {
	dotenvy::dotenv().ok();
	color_eyre::install()?;
	env_logger::init();

	let token = std::env::var("TOKEN").wrap_err_with(|| "Couldn't find token in environment!")?;

	let intents =
		serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

	let options = FrameworkOptions {
		commands: {
			let mut commands = commands::global();
			commands.append(&mut commands::moderation());
			commands
		},
		on_error: |error| Box::pin(handlers::handle_error(error)),

		command_check: Some(|ctx| {
			Box::pin(async move { Ok(ctx.author().id != ctx.framework().bot_id) })
		}),

		event_handler: |ctx, event, framework, data| {
			Box::pin(handlers::handle_event(ctx, event, framework, data))
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
		.setup(|ctx, ready, framework| Box::pin(setup(ctx, ready, framework)))
		.build();

	let mut client = serenity::ClientBuilder::new(token, intents)
		.framework(framework)
		.await?;

	let shard_manager = client.shard_manager.clone();
	let mut sigterm = signal(SignalKind::terminate())?;

	tokio::select! {
		result = client.start() => result.map_err(Report::from),
		_ = sigterm.recv() => {
			handle_shutdown(shard_manager, "Recieved SIGTERM").await;
			std::process::exit(0);
		},
		_ = ctrl_c() => {
			handle_shutdown(shard_manager, "Interrupted").await;
			std::process::exit(130);
		}
	}
}
