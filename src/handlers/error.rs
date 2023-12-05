use crate::colors::Colors;
use crate::Data;

use color_eyre::eyre::Report;
use log::*;
use poise::serenity_prelude::Timestamp;
use poise::FrameworkError;

pub async fn handle(error: poise::FrameworkError<'_, Data, Report>) {
	match error {
		FrameworkError::Setup {
			error, framework, ..
		} => {
			error!("Error setting up client! Bailing out");
			framework.shard_manager().lock().await.shutdown_all().await;

			panic!("{error}")
		}

		FrameworkError::Command { error, ctx } => {
			error!("Error in command {}:\n{error:?}", ctx.command().name);
			ctx.send(|c| {
				c.embed(|e| {
					e.title("Something went wrong!")
						.description("oopsie")
						.timestamp(Timestamp::now())
						.color(Colors::Orange)
				})
			})
			.await
			.ok();
		}

		FrameworkError::EventHandler {
			error,
			ctx: _,
			event,
			framework: _,
		} => {
			error!("Error while handling event {}:\n{error:?}", event.name());
		}

		error => {
			if let Err(e) = poise::builtins::on_error(error).await {
				error!("Unhandled error occured:\n{e:#?}");
			}
		}
	}
}
