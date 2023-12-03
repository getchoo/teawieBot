use std::time::Duration;

use color_eyre::eyre::{eyre, Context as _, Report, Result};
use log::*;
use poise::{
	serenity_prelude as serenity, EditTracker, Framework, FrameworkOptions, PrefixFrameworkOptions,
};
use redis::AsyncCommands;
use settings::Settings;

mod api;
mod colors;
mod commands;
mod consts;
mod handlers;
mod settings;
mod utils;

type Context<'a> = poise::Context<'a, Data, Report>;

#[derive(Clone)]
pub struct Data {
	redis: redis::Client,
}

impl Data {
	pub fn new() -> Result<Self> {
		let redis_url = std::env::var("REDIS_URL")
			.wrap_err_with(|| eyre!("Couldn't find Redis URL in environment!"))?;

		let redis = redis::Client::open(redis_url)?;

		Ok(Self { redis })
	}
}

#[tokio::main]
async fn main() -> Result<()> {
	dotenvy::dotenv().ok();
	color_eyre::install()?;
	env_logger::init();

	let token =
		std::env::var("TOKEN").wrap_err_with(|| eyre!("Couldn't find token in environment!"))?;

	let intents =
		serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

	let options = FrameworkOptions {
		commands: commands::to_global_commands(),
		on_error: |error| Box::pin(handlers::handle_error(error)),
		command_check: Some(|ctx| {
			Box::pin(async move { Ok(ctx.author().id != ctx.framework().bot_id) })
		}),
		event_handler: |ctx, event, framework, data| {
			Box::pin(handlers::handle_event(ctx, event, framework, data))
		},
		prefix_options: PrefixFrameworkOptions {
			prefix: Some("!".into()),
			edit_tracker: Some(EditTracker::for_timespan(Duration::from_secs(3600))),
			..Default::default()
		},
		..Default::default()
	};

	let framework = Framework::builder()
		.token(token)
		.intents(intents)
		.options(options)
		.setup(|ctx, _ready, framework| {
			Box::pin(async move {
				let data = Data::new()?;

				poise::builtins::register_globally(ctx, &framework.options().commands).await?;
				info!("Registered global commands!");

				// register "extra" commands in guilds that allow it
				let mut con = data.redis.get_async_connection().await?;

				info!("Fetching all guild settings from Redis...this might take a while");
				let guilds: Vec<String> = con.keys(format!("{}:*", settings::ROOT_KEY)).await?;

				for guild in guilds {
					let settings: Settings = con.get(guild).await?;

					if settings.optional_commands_enabled {
						poise::builtins::register_in_guild(
							ctx,
							&commands::to_guild_commands(),
							settings.guild_id,
						)
						.await?;
						info!("Registered guild commands to {}", settings.guild_id);
					} else {
						debug!("Not registering guild commands to {} since optional_commands_enabled is False", settings.guild_id);
					}
				}

				Ok(data)
			})
		});

	tokio::select! {
		result = framework.run() => { result.map_err(Report::from) },
		_ = tokio::signal::ctrl_c() => {
			info!("Interrupted! Exiting...");
			std::process::exit(130);
		}
	}
}
