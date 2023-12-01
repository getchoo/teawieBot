use crate::colors::Colors;
use crate::Data;

use color_eyre::eyre::Report;
use log::*;
use poise::serenity_prelude::Timestamp;
use poise::FrameworkError;

pub async fn handle(error: poise::FrameworkError<'_, Data, Report>) {
	match error {
		FrameworkError::Setup { error, .. } => error!("error setting up client! {error:#?}"),

		FrameworkError::Command { error, ctx } => {
			error!("error in command {}:\n{error:?}", ctx.command().name);
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
			event: _,
			framework: _,
		} => {
			error!("error while handling event:\n{error:#?}");
		}

		error => {
			if let Err(e) = poise::builtins::on_error(error).await {
				error!("error while handling an error: {}", e);
			}
		}
	}
}
