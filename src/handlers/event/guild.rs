use color_eyre::eyre::Result;
use log::{debug, warn};
use poise::serenity_prelude::{Guild, UnavailableGuild};

use crate::{storage, Data};
use storage::Settings;

pub async fn handle_create(guild: &Guild, _is_new: &bool, data: &Data) -> Result<()> {
	let storage = &data.storage;

	if storage.guild_settings_exist(&guild.id).await? {
		debug!("Not recreating settings key for {}", guild.id);
		return Ok(());
	}

	let settings = Settings {
		guild_id: guild.id,
		optional_commands_enabled: false,
		..Default::default()
	};

	warn!("Creating new settings key for {}:\n{settings:#?}", guild.id);
	storage.create_guild_settings(settings).await?;

	Ok(())
}

pub async fn handle_delete(guild: &UnavailableGuild, data: &Data) -> Result<()> {
	data.storage.delete_guild_settings(&guild.id).await?;

	Ok(())
}
