use crate::settings::{Settings, SettingsProperties};
use crate::Context;

use color_eyre::eyre::{eyre, Context as _, ContextCompat, Result};
use log::*;
use poise::serenity_prelude::{GuildChannel, ReactionType};

#[poise::command(
	slash_command,
	subcommands("set", "get"),
	default_member_permissions = "MANAGE_GUILD"
)]
pub async fn config(_ctx: Context<'_>) -> Result<()> {
	Ok(())
}

#[poise::command(slash_command, ephemeral, guild_only)]
pub async fn set(
	ctx: Context<'_>,
	#[channel_types("Text")]
	#[description = "Where to redirect pins from channels. If empty (the default), the PinBoard is disabled."]
	pinboard_channel: Option<GuildChannel>,
	#[channel_types("Text")]
	#[description = "A channel that PinBoard will redirect pins from. This will be all channels if empty."]
	pinboard_watch: Option<GuildChannel>,
	#[channel_types("Text")]
	#[description = "Where to post messages that made it to the ReactBoard. If left empty, ReactBoard is disabled."]
	reactboard_channel: Option<GuildChannel>,
	#[description = "An emoji that will get messages on the ReactBoard. If empty, ReactBoard is disabled."]
	reactboard_reaction: Option<String>,
	#[description = "Minimum number of reactions a message needs to make it to the ReactBoard (defaults to 5)"]
	reactboard_requirement: Option<u64>,
	#[description = "Enables 'extra' commands like teawiespam and copypasta. Defaults to false."]
	optional_commands_enabled: Option<bool>,
) -> Result<()> {
	let redis = &ctx.data().redis;
	let gid = ctx.guild_id().unwrap_or_default();
	let mut settings = Settings::from_redis(redis, &gid).await?;
	let previous_settings = settings.clone();

	if let Some(channel) = pinboard_channel {
		settings.pinboard_channel = Some(channel.id);
	}

	if let Some(watch) = pinboard_watch {
		if let Some(mut prev) = settings.pinboard_watch {
			prev.push(watch.id);
			settings.pinboard_watch = Some(prev);
		} else {
			let new = Vec::from([watch.id]);
			debug!("Setting pinboard_watch to {new:#?} for {} in Redis", gid);

			settings.pinboard_watch = Some(new);
		}
	}

	if let Some(channel) = reactboard_channel {
		debug!(
			"Setting reactboard_channel to {channel} for {} in Redis",
			gid
		);

		settings.reactboard_channel = Some(channel.id);
	}

	if let Some(requirement) = reactboard_requirement {
		debug!(
			"Setting reactboard_requirement to {requirement} for {} in Redis",
			gid
		);

		settings.reactboard_requirement = Some(requirement);
	}

	if let Some(reaction) = reactboard_reaction {
		let emoji = reaction
			.parse::<ReactionType>()
			.wrap_err_with(|| format!("Couldn't parse {reaction} as string!"))?;

		if let Some(mut prev) = settings.reactboard_reactions {
			prev.push(emoji);
			settings.reactboard_reactions = Some(prev);
		} else {
			let new = Vec::from([emoji]);
			debug!("Setting pinboard_watch to {new:#?} for {} in Redis", gid);

			settings.reactboard_reactions = Some(new);
		}
	}

	if let Some(enabled) = optional_commands_enabled {
		debug!(
			"Setting optional_commands_enabled to {enabled} for {} in Redis",
			gid
		);

		settings.optional_commands_enabled = enabled;
	}

	if previous_settings != settings {
		settings.save(redis).await?;
		ctx.reply("Configuration updated!").await?;
	} else {
		ctx.reply("No changes made, so i'm not updating anything")
			.await?;
	}

	Ok(())
}

#[poise::command(slash_command, ephemeral, guild_only)]
pub async fn get(
	ctx: Context<'_>,
	#[description = "The setting you want to get"] setting: SettingsProperties,
) -> Result<()> {
	let gid = &ctx
		.guild_id()
		.wrap_err_with(|| eyre!("Failed to get GuildId from context!"))?;

	let settings = Settings::from_redis(&ctx.data().redis, gid).await?;

	let value = match setting {
		SettingsProperties::GuildId => settings.guild_id.to_string(),
		SettingsProperties::PinBoardChannel => format!("{:#?}", settings.pinboard_channel),
		SettingsProperties::PinBoardWatch => format!("{:#?}", settings.pinboard_watch),
		SettingsProperties::ReactBoardChannel => format!("{:#?}", settings.reactboard_channel),
		SettingsProperties::ReactBoardRequirement => {
			format!("{:?}", settings.reactboard_requirement)
		}
		SettingsProperties::ReactBoardReactions => format!("{:?}", settings.reactboard_reactions),
		SettingsProperties::OptionalCommandsEnabled => {
			settings.optional_commands_enabled.to_string()
		}
	};

	ctx.send(|m| m.embed(|e| e.field(setting, value, false)))
		.await?;

	Ok(())
}
