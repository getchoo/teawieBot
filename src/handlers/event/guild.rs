use color_eyre::eyre::Result;
use log::*;
use poise::serenity_prelude::{Guild, UnavailableGuild};

use crate::{Data, Settings};

pub async fn handle_create(guild: &Guild, is_new: &bool, data: &Data) -> Result<()> {
	if !is_new && Settings::from_redis(&data.redis, &guild.id).await.is_ok() {
		debug!("Not recreating Redis key for {}", guild.id);
		return Ok(());
	}

	info!("Creating new Redis key for {}", guild.id);
	Settings::new_redis(&data.redis, &guild.id).await?;
	Ok(())
}

pub async fn handle_delete(guild: &UnavailableGuild, data: &Data) -> Result<()> {
	let redis = &data.redis;

	info!("Deleting redis key for {}", guild.id);
	let settings = Settings::from_redis(redis, &guild.id).await?;
	settings.delete(redis).await?;

	Ok(())
}
