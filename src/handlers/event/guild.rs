use eyre::Result;
use log::{debug, warn};
use poise::serenity_prelude::{Guild, UnavailableGuild};

use crate::{storage, Data};
use storage::settings::Settings;

pub async fn handle_create(guild: &Guild, data: &Data) -> Result<()> {
	if let Some(storage) = &data.storage {
		if storage.guild_settings_exist(&guild.id).await? {
			debug!("Not recreating settings key for {}", guild.id);
			return Ok(());
		}

		let settings = Settings {
			guild_id: guild.id,
			..Default::default()
		};

		warn!("Creating new settings key for {}:\n{settings:#?}", guild.id);
		storage.create_guild_settings(settings).await?;
	} else {
		warn!("Can't create guild settings; no storage backend found!");
	}

	Ok(())
}

pub async fn handle_delete(guild: &UnavailableGuild, data: &Data) -> Result<()> {
	if let Some(storage) = &data.storage {
		storage.delete_guild_settings(&guild.id).await?;
	} else {
		warn!("Can't delete guild settings; no storage backend found!");
	}

	Ok(())
}
