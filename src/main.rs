use std::time::Duration;
use std::{env, error};

use crate::commands::*;
use crate::consts::*;
use crate::pinboard::PinBoard;
use log::*;
use poise::serenity_prelude as serentiy;
use poise::serenity_prelude::*;

mod api;
mod commands;
mod consts;
mod handler;
mod pinboard;
mod utils;

type Error = Box<dyn error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Clone)]
pub struct Data {
	bot: serentiy::UserId,
	pin_board: Option<PinBoard>,
}

impl Default for Data {
	fn default() -> Self {
		Self::new()
	}
}

impl Data {
	pub fn new() -> Self {
		let bot = utils::parse_snowflake_from_env("BOT", UserId).unwrap_or(consts::BOT);
		let pin_board = PinBoard::new();

		Self { bot, pin_board }
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
	dotenvy::dotenv().unwrap();

	let guild_commands = vec![copypasta::copypasta(), teawiespam::teawiespam()];

	let options = poise::FrameworkOptions {
		commands: vec![
			ask::ask(),
			bing::bing(),
			bottom::bottom(),
			convert::convert(),
			random_lore::random_lore(),
			random_shiggy::random_shiggy(),
			random_teawie::random_teawie(),
			copypasta::copypasta(),
			teawiespam::teawiespam(),
			version::version(),
		],
		event_handler: |ctx, event, _, data| {
			Box::pin(async move {
				// yes this is dumb. no i don't care.
				let handler = handler::Handler::new(data.clone());
				event.clone().dispatch(ctx.clone(), &handler).await;
				Ok(())
			})
		},
		prefix_options: poise::PrefixFrameworkOptions {
			prefix: Some("!".into()),
			edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
			..Default::default()
		},
		on_error: |error| Box::pin(on_error(error)),
		command_check: Some(|ctx| {
			Box::pin(async move {
				Ok(ctx.author().id != ctx.framework().bot_id && ctx.author().id != consts::BOT)
			})
		}),
		..Default::default()
	};

	let framework = poise::Framework::builder()
		.options(options)
		.token(env::var("TOKEN").expect("couldn't find token in environment."))
		.intents(serentiy::GatewayIntents::all())
		.setup(|ctx, _ready, framework| {
			Box::pin(async move {
				info!("logged in as {}", _ready.user.name);

				poise::builtins::register_globally(ctx, &framework.options().commands).await?;
				info!("registered global commands!");
				poise::builtins::register_in_guild(ctx, &guild_commands, TEAWIE_GUILD).await?;
				info!("registered guild commands!");

				Ok(Data::new())
			})
		});

	framework.run().await.unwrap()
}
