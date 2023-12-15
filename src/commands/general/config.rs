use std::str::FromStr;

use crate::{storage, Context};
use storage::{Settings, SettingsProperties};

use color_eyre::eyre::{eyre, Result};
use log::*;
use poise::serenity_prelude::{GuildChannel, ReactionType};

fn split_argument<T>(list: String) -> Vec<T>
where
	T: FromStr,
{
	list.split(',')
		.filter_map(|s| s.trim().parse().ok())
		.collect()
}

fn prop_to_val(setting: &SettingsProperties, settings: &Settings) -> String {
	match setting {
		SettingsProperties::GuildId => settings.guild_id.to_string(),
		SettingsProperties::PinBoardChannel => format!("{:#?}", settings.pinboard_channel),
		SettingsProperties::PinBoardWatch => format!("{:#?}", settings.pinboard_watch),
		SettingsProperties::PinBoardEnabled => settings.pinboard_enabled.to_string(),
		SettingsProperties::ReactBoardChannel => format!("{:#?}", settings.reactboard_channel),
		SettingsProperties::ReactBoardRequirement => {
			format!("{:?}", settings.reactboard_requirement)
		}
		SettingsProperties::ReactBoardReactions => format!("{:?}", settings.reactboard_reactions),
		SettingsProperties::ReactBoardEnabled => settings.reactboard_enabled.to_string(),
		SettingsProperties::OptionalCommandsEnabled => {
			settings.optional_commands_enabled.to_string()
		}
	}
}

#[poise::command(
	slash_command,
	prefix_command,
	subcommands("set", "get"),
	required_permissions = "MANAGE_GUILD",
	default_member_permissions = "MANAGE_GUILD"
)]
pub async fn config(_ctx: Context<'_>) -> Result<()> {
	Ok(())
}

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
	#[description = "Enables 'extra' commands like teawiespam and copypasta. Defaults to false."]
	optional_commands_enabled: Option<bool>,
) -> Result<()> {
	let storage = &ctx.data().storage;
	let gid = ctx.guild_id().unwrap_or_default();
	let mut settings = storage.get_guild_settings(&gid).await?;
	let previous_settings = settings.clone();

	if let Some(channel) = pinboard_channel {
		debug!("Setting pinboard_channel to {channel} for {gid}");
		settings.pinboard_channel = Some(channel.id);
	}

	if let Some(watch) = pinboard_watch {
		let channels = split_argument(watch);
		debug!("Setting pinboard_watch to {channels:#?} for {gid}");

		settings.pinboard_watch = Some(channels);
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

	if previous_settings != settings {
		debug!("Updating settings key for {gid}");
		storage.create_guild_settings(settings).await?;
		ctx.reply("Configuration updated!").await?;
	} else {
		debug!("Not updating settings key for {gid} since no changes were made");
		ctx.reply("No changes made, so i'm not updating anything")
			.await?;
	}

	Ok(())
}

#[poise::command(
	slash_command,
	prefix_command,
	ephemeral,
	guild_only,
	required_permissions = "MANAGE_GUILD"
)]
pub async fn get(
	ctx: Context<'_>,
	#[description = "The setting you want to get"] setting: SettingsProperties,
) -> Result<()> {
	let gid = &ctx
		.guild_id()
		.ok_or_else(|| eyre!("Failed to get GuildId from context!"))?;

	let settings = ctx.data().storage.get_guild_settings(gid).await?;
	let value = prop_to_val(&setting, &settings);

	ctx.send(|m| m.embed(|e| e.field(setting, value, false)))
		.await?;

	Ok(())
}
