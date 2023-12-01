use std::time::Duration;

use color_eyre::eyre::{eyre, Context as _, Report, Result};
use log::*;
use poise::{
	serenity_prelude as serenity, EditTracker, Framework, FrameworkOptions, PrefixFrameworkOptions,
};
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
	settings: Settings,
}

impl Data {
	pub fn new() -> Result<Self> {
		let settings =
			Settings::new().ok_or_else(|| eyre!("Couldn't create new settings object!"))?;

		Ok(Self { settings })
	}
}

#[tokio::main]
async fn main() -> Result<()> {
	color_eyre::install()?;
	env_logger::init();
	dotenvy::dotenv().ok();

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
				poise::builtins::register_globally(ctx, &framework.options().commands).await?;
				info!("Registered global commands!");

				poise::builtins::register_in_guild(
					ctx,
					&commands::to_guild_commands(),
					consts::TEAWIE_GUILD,
				)
				.await?;
				info!("Registered guild commands to {}", consts::TEAWIE_GUILD);

				let data = Data::new()?;
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
