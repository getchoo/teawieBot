use color_eyre::eyre::Result;
use log::*;
use poise::serenity_prelude::{Guild, UnavailableGuild};

use crate::{storage, Data};
use storage::settings::Settings;
use storage::Storage;

pub async fn handle_create(guild: &Guild, _is_new: &bool, data: &Data) -> Result<()> {
	let storage = &data.storage;
	let key = Storage::format_settings_key(guild.id);

	if storage.key_exists(&key).await? {
		debug!("Not recreating settings key for {}", guild.id);
		return Ok(());
	}

	let settings = Settings {
		guild_id: guild.id,
		optional_commands_enabled: false,
		..Default::default()
	};

	warn!("Creating new settings key {key}:\n{settings:#?}");
	storage.create_settings_key(settings).await?;

	Ok(())
}

pub async fn handle_delete(guild: &UnavailableGuild, data: &Data) -> Result<()> {
	let key = Storage::format_settings_key(guild.id);
	data.storage.delete_key(&key).await?;
	Ok(())
}
