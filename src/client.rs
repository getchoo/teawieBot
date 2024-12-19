use crate::{commands, events, http, storage::Storage};

use std::{sync::Arc, time::Duration};

use anyhow::{Context as _, Result};
use log::{info, warn};
use poise::{
	serenity_prelude::{self as serenity},
	EditTracker, Framework, FrameworkOptions, PrefixFrameworkOptions,
};

pub type Error = anyhow::Error;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Clone, Debug, Default)]
pub struct Data {
	pub http_client: http::Client,
	pub storage: Option<Storage>,
}

async fn setup(ctx: &serenity::Context) -> Result<Data> {
	let storage = Storage::from_env().ok();

	poise::builtins::register_globally(ctx, &commands::all()).await?;
	info!("Registered global commands!");

	let http_client = <http::Client as http::Ext>::default();
	let data = Data {
		http_client,
		storage,
	};

	Ok(data)
}

pub async fn handle_shutdown(shard_manager: Arc<serenity::ShardManager>, reason: &str) {
	warn!("{reason}! Shutting down bot...");
	shard_manager.shutdown_all().await;
	println!("Everything is shutdown. Goodbye!");
}

pub async fn get() -> Result<serenity::Client> {
	let token = std::env::var("TOKEN").context("Couldn't find bot token in environment!")?;

	let intents =
		serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

	let options = FrameworkOptions {
		commands: commands::all(),
		on_error: |error| Box::pin(events::error::handle(error)),

		command_check: Some(|ctx| {
			Box::pin(async move { Ok(ctx.author().id != ctx.framework().bot_id) })
		}),

		event_handler: |ctx, event, _framework, data| Box::pin(events::handle(ctx, event, data)),

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

	let client = serenity::ClientBuilder::new(token, intents)
		.framework(framework)
		.await?;

	Ok(client)
}
