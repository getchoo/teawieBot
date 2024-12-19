use std::{env, fmt::Debug, str::FromStr};

use anyhow::Result;
use log::debug;
use poise::serenity_prelude::{GuildId, MessageId};
use redis::{AsyncCommands, Client, ConnectionLike, RedisError};

pub mod reactboard;
pub mod settings;

use reactboard::ReactBoardEntry;
use settings::Settings;

pub const REACTBOARD_KEY: &str = "reactboard-v2";
pub const SETTINGS_KEY: &str = "settings-v1";

#[derive(Clone, Debug)]
pub struct Storage {
	client: Client,
}

impl Storage {
	pub fn new(client: Client) -> Self {
		Self { client }
	}

	pub fn from_env() -> Result<Self> {
		let redis_url = env::var("REDIS_URL")?;

		Ok(Self::from_str(&redis_url)?)
	}

	pub fn is_connected(&mut self) -> bool {
		self.client.check_connection()
	}

	pub async fn create_guild_settings(&self, settings: Settings) -> Result<()> {
		let guild_key = format!("{SETTINGS_KEY}:{}", settings.guild_id);

		let mut con = self.client.get_multiplexed_async_connection().await?;
		redis::pipe()
			.set(&guild_key, &settings)
			.sadd(SETTINGS_KEY, u64::from(settings.guild_id))
			.exec_async(&mut con)
			.await?;

		Ok(())
	}

	pub async fn get_guild_settings(&self, guild_id: &GuildId) -> Result<Settings> {
		debug!("Fetching guild settings for {guild_id}");
		let guild_key = format!("{SETTINGS_KEY}:{guild_id}");

		let mut con = self.client.get_multiplexed_async_connection().await?;
		let settings: Settings = con.get(&guild_key).await?;

		Ok(settings)
	}

	pub async fn delete_guild_settings(&self, guild_id: &GuildId) -> Result<()> {
		debug!("Deleting guild settings for {guild_id}");
		let guild_key = format!("{SETTINGS_KEY}:{guild_id}");

		let mut con = self.client.get_multiplexed_async_connection().await?;
		redis::pipe()
			.del(&guild_key)
			.srem(SETTINGS_KEY, u64::from(*guild_id))
			.exec_async(&mut con)
			.await?;

		Ok(())
	}

	pub async fn guild_settings_exist(&self, guild_id: &GuildId) -> Result<bool> {
		debug!("Checking if guild settings for {guild_id} exist");
		let guild_key = format!("{SETTINGS_KEY}:{guild_id}");

		let mut con = self.client.get_multiplexed_async_connection().await?;
		let exists = con.exists(&guild_key).await?;

		Ok(exists)
	}

	pub async fn get_all_guild_settings(&self) -> Result<Vec<Settings>> {
		debug!("Fetching all guild settings");

		let mut con = self.client.get_multiplexed_async_connection().await?;
		let found: Vec<u64> = con.smembers(SETTINGS_KEY).await?;

		let mut guilds = vec![];
		for key in found {
			let settings = self.get_guild_settings(&key.into()).await?;
			guilds.push(settings);
		}

		Ok(guilds)
	}

	/// get guilds that have enabled optional commands
	pub async fn get_opted_guilds(&self) -> Result<Vec<GuildId>> {
		debug!("Fetching opted-in guilds");

		let guilds = self.get_all_guild_settings().await?;
		let opted: Vec<GuildId> = guilds
			.iter()
			.filter_map(|g| g.optional_commands_enabled.then_some(g.guild_id))
			.collect();

		Ok(opted)
	}

	// reactboard

	pub async fn create_reactboard_entry(
		&self,
		guild_id: &GuildId,
		entry: ReactBoardEntry,
	) -> Result<()> {
		debug!(
			"Creating reactboard entry for {} in {guild_id}",
			&entry.original_message_id
		);
		let entry_key = format!("{REACTBOARD_KEY}:{guild_id}:{}", entry.original_message_id);

		let mut con = self.client.get_multiplexed_async_connection().await?;
		// https://github.com/redis-rs/redis-rs/issues/1228
		con.set_ex::<_, _, ()>(&entry_key, &entry, 30 * 24 * 60 * 60)
			.await?; // 30 days

		Ok(())
	}

	pub async fn get_reactboard_entry(
		&self,
		guild_id: &GuildId,
		message_id: &MessageId,
	) -> Result<ReactBoardEntry> {
		debug!("Fetching reactboard entry {message_id} in {guild_id}");
		let entry_key = format!("{REACTBOARD_KEY}:{guild_id}:{message_id}");

		let mut con = self.client.get_multiplexed_async_connection().await?;
		let entry: ReactBoardEntry = con.get(&entry_key).await?;

		Ok(entry)
	}

	pub async fn reactboard_entry_exists(
		&self,
		guild_id: &GuildId,
		message_id: &MessageId,
	) -> Result<bool> {
		debug!("Checking if reactboard entry {message_id} exists in {guild_id}");
		let entry_key = format!("{REACTBOARD_KEY}:{guild_id}:{message_id}");

		let mut con = self.client.get_multiplexed_async_connection().await?;
		let exists = con.exists(&entry_key).await?;

		Ok(exists)
	}
}

impl FromStr for Storage {
	type Err = RedisError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let client = Client::open(s)?;
		Ok(Self::new(client))
	}
}
