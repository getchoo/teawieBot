use crate::client::Context;
use crate::storage::settings::{Properties, Settings};

use std::str::FromStr;

use anyhow::{Context as _, Result};
use log::debug;
use poise::serenity_prelude::{CreateEmbed, GuildChannel, ReactionType};
use poise::{ChoiceParameter, CreateReply};

fn split_argument<T>(list: &str) -> Vec<T>
where
	T: FromStr,
{
	list.split(',')
		.filter_map(|s| s.trim().parse().ok())
		.collect()
}

fn prop_to_val(setting: &Properties, settings: &Settings) -> String {
	match setting {
		Properties::GuildId => settings.guild_id.to_string(),
		Properties::PinBoardChannel => format!("{:#?}", settings.pinboard_channel),
		Properties::PinBoardWatch => format!("{:#?}", settings.pinboard_watch),
		Properties::PinBoardEnabled => settings.pinboard_enabled.to_string(),
		Properties::ReactBoardChannel => format!("{:#?}", settings.reactboard_channel),
		Properties::ReactBoardRequirement => {
			format!("{:?}", settings.reactboard_requirement)
		}
		Properties::ReactBoardReactions => format!("{:?}", settings.reactboard_reactions),
		Properties::ReactBoardEnabled => settings.reactboard_enabled.to_string(),
		Properties::OptionalCommandsEnabled => settings.optional_commands_enabled.to_string(),
	}
}

#[allow(clippy::unused_async)]
#[poise::command(
	slash_command,
	prefix_command,
	subcommands("set", "get"),
	required_permissions = "MANAGE_GUILD",
	default_member_permissions = "MANAGE_GUILD"
)]
pub async fn config(_: Context<'_>) -> Result<()> {
	Ok(())
}

// Set a configuration option
#[allow(clippy::too_many_arguments)]
#[poise::command(
	slash_command,
	prefix_command,
	ephemeral,
	guild_only,
	required_permissions = "MANAGE_GUILD"
)]
pub async fn set(
	ctx: Context<'_>,
	#[channel_types("Text")]
	#[description = "Where to redirect pins from channels. If empty (the default), the PinBoard is disabled."]
	pinboard_channel: Option<GuildChannel>,
	#[description = "Comma separated list of channels PinBoard redirects. If empty, this will be all channels"]
	pinboard_watch: Option<String>,
	#[description = "Toggle PinBoard"] pinboard_enabled: Option<bool>,
	#[channel_types("Text")]
	#[description = "Where to post messages that made it to the ReactBoard. If left empty, ReactBoard is disabled."]
	reactboard_channel: Option<GuildChannel>,
	#[description = "Comma separated list of emojis that will count towards ReactBoard. If empty, ReactBoard is disabled."]
	reactboard_reaction: Option<String>,
	#[description = "Minimum number of reactions a message needs to make it to the ReactBoard (defaults to 5)"]
	reactboard_requirement: Option<u64>,
	#[description = "Toggle ReactBoard"] reactboard_enabled: Option<bool>,
	#[description = "Enables 'extra' commands like uwurandom. Defaults to false."]
	optional_commands_enabled: Option<bool>,
) -> Result<()> {
	if let Some(storage) = &ctx.data().storage {
		let gid = ctx.guild_id().unwrap_or_default();
		let mut settings = storage.get_guild_settings(&gid).await?;
		let previous_settings = settings.clone();

		if let Some(channel) = pinboard_channel {
			debug!("Setting pinboard_channel to {channel} for {gid}");
			settings.pinboard_channel = Some(channel.id);
		}

		if let Some(watch) = pinboard_watch {
			let channels = split_argument(&watch);
			settings.pinboard_watch = (!channels.is_empty()).then_some(channels);
		}

		if let Some(enabled) = pinboard_enabled {
			debug!("Setting pinboard_enabled to {enabled} for {gid}");
			settings.pinboard_enabled = enabled;
		}

		if let Some(channel) = reactboard_channel {
			debug!("Setting reactboard_channel to {channel} for {gid}");
			settings.reactboard_channel = Some(channel.id);
		}

		if let Some(requirement) = reactboard_requirement {
			debug!("Setting reactboard_requirement to {requirement} for {gid}");
			settings.reactboard_requirement = Some(requirement);
		}

		if let Some(reaction) = reactboard_reaction {
			let emojis: Vec<ReactionType> =
				reaction.split(',').filter_map(|r| r.parse().ok()).collect();
			debug!("Setting reactboard_reactions to {emojis:#?} for {gid}");

			settings.reactboard_reactions = Some(emojis);
		}

		if let Some(enabled) = reactboard_enabled {
			debug!("Setting reactboard_enabled to {enabled} for {gid}");
			settings.reactboard_enabled = enabled;
		}

		if let Some(enabled) = optional_commands_enabled {
			debug!("Setting optional_commands_enabled to {enabled} for {}", gid);
			settings.optional_commands_enabled = enabled;
		}

		if previous_settings == settings {
			debug!("Not updating settings key for {gid} since no changes were made");
			ctx.reply("No changes made, so i'm not updating anything")
				.await?;
		} else {
			debug!("Updating settings key for {gid}");
			storage.create_guild_settings(settings).await?;
			ctx.reply("Configuration updated!").await?;
		}
	} else {
		ctx.reply("I have no storage backend right now, so I can't set settings :(")
			.await?;
	}

	Ok(())
}

// Get a configuration option
#[poise::command(
	slash_command,
	prefix_command,
	ephemeral,
	guild_only,
	required_permissions = "MANAGE_GUILD"
)]
pub async fn get(
	ctx: Context<'_>,
	#[description = "The setting you want to get"] setting: Properties,
) -> Result<()> {
	let gid = &ctx
		.guild_id()
		.context("Failed to get GuildId from context!")?;

	if let Some(storage) = &ctx.data().storage {
		let settings = storage.get_guild_settings(gid).await?;
		let value = prop_to_val(&setting, &settings);

		let embed = CreateEmbed::new().field(setting.name(), value, false);
		let message = CreateReply::default().embed(embed);
		ctx.send(message).await?;
	} else {
		ctx.reply("I have no storage backend right now, so I can't fetch settings :(")
			.await?;
	}

	Ok(())
}
