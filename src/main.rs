use std::{error, time};

use handler::pinboard::PinBoard;
use log::*;
use poise::serenity_prelude as serentiy;

mod api;
mod commands;
mod consts;
mod handler;
mod utils;

type Error = Box<dyn error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Clone)]
pub struct Data {
	pin_board: Option<PinBoard>,
}

impl Data {
	pub fn new() -> Self {
		let pin_board = PinBoard::new();

		Self { pin_board }
	}
}

impl Default for Data {
	fn default() -> Self {
		Self::new()
	}
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
	match error {
		poise::FrameworkError::Setup { error, .. } => panic!("failed to start bot: {error:?}"),
		poise::FrameworkError::Command { error, ctx } => {
			error!("error in command {}: {:?}", ctx.command().name, error);
		}
		error => {
			if let Err(e) = poise::builtins::on_error(error).await {
				error!("error while handling an error: {}", e);
			}
		}
	}
}

#[tokio::main]
async fn main() {
	env_logger::init();
	dotenvy::dotenv().ok();

	let options = poise::FrameworkOptions {
		commands: commands::to_global_commands(),
		event_handler: |ctx, event, framework, data| {
			Box::pin(handler::handle(ctx, event, framework, data))
		},
		prefix_options: poise::PrefixFrameworkOptions {
			prefix: Some("!".into()),
			edit_tracker: Some(poise::EditTracker::for_timespan(time::Duration::from_secs(
				3600,
			))),
			..Default::default()
		},
		on_error: |error| Box::pin(on_error(error)),
		command_check: Some(|ctx| {
			Box::pin(async move { Ok(ctx.author().id != ctx.framework().bot_id) })
		}),
		..Default::default()
	};

	let framework = poise::Framework::builder()
		.options(options)
		.token(std::env::var("TOKEN").expect("couldn't find token in environment."))
		.intents(
			serentiy::GatewayIntents::non_privileged() | serentiy::GatewayIntents::MESSAGE_CONTENT,
		)
		.setup(|ctx, _ready, framework| {
			Box::pin(async move {
				poise::builtins::register_globally(ctx, &framework.options().commands).await?;
				info!("registered global commands!");

				poise::builtins::register_in_guild(
					ctx,
					&commands::to_guild_commands(),
					consts::TEAWIE_GUILD,
				)
				.await?;
				info!("registered guild commands!");

				Ok(Data::new())
			})
		});

	framework.run().await.unwrap()
}
