use crate::colors::Colors;
use crate::Data;

use eyre::Report;
use log::error;
use poise::serenity_prelude::{CreateEmbed, Timestamp};
use poise::{CreateReply, FrameworkError};

pub async fn handle(error: poise::FrameworkError<'_, Data, Report>) {
	match error {
		FrameworkError::Setup {
			error, framework, ..
		} => {
			error!("Error setting up client! Bailing out");
			framework.shard_manager().shutdown_all().await;

			panic!("{error}")
		}

		FrameworkError::Command { error, ctx, .. } => {
			error!("Error in command {}:\n{error}", ctx.command().name);

			let embed = CreateEmbed::default()
				.title("Something went wrong!")
				.description("oopsie")
				.timestamp(Timestamp::now())
				.color(Colors::Orange);

			let reply = CreateReply::default().embed(embed);

			ctx.send(reply).await.ok();
		}

		FrameworkError::EventHandler {
			error,
			ctx: _,
			event,
			framework: _,
			..
		} => {
			error!(
				"Error while handling event {}:\n{error}",
				event.snake_case_name()
			);
		}

		error => {
			if let Err(e) = poise::builtins::on_error(error).await {
				error!("Unhandled error occured:\n{e:#?}");
			}
		}
	}
}
