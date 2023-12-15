use std::fmt::Debug;

use color_eyre::eyre::Result;
use log::*;
use poise::serenity_prelude::{GuildId, MessageId};
use redis::{AsyncCommands, Client, FromRedisValue, ToRedisArgs};

mod reactboard;
mod settings;
// these are purposefully private. see the comment below
use reactboard::REACTBOARD_KEY;
use settings::SETTINGS_KEY;

pub use reactboard::ReactBoardEntry;
pub use settings::{Settings, SettingsProperties};

#[derive(Clone, Debug)]
pub struct Storage {
	pub client: Client,
}

impl Storage {
	pub fn new(redis_url: &str) -> Result<Self> {
		let client = Client::open(redis_url)?;

		Ok(Self { client })
	}

	/*
	  these are mainly light abstractions to avoid the `let mut con`
	  boilerplate, as well as not require the caller to format the
	  strings for keys
	*/

	async fn get_key<T>(&self, key: &str) -> Result<T>
	where
		T: FromRedisValue,
	{
		debug!("Getting key {key}");

		let mut con = self.client.get_async_connection().await?;
		let res: T = con.get(key).await?;

		Ok(res)
	}

	async fn set_key<'a>(
		&self,
		key: &str,
		value: impl ToRedisArgs + Debug + Send + Sync + 'a,
	) -> Result<()> {
		debug!("Creating key {key}:\n{value:#?}");

		let mut con = self.client.get_async_connection().await?;
		con.set(key, value).await?;

		Ok(())
	}

	async fn key_exists(&self, key: &str) -> Result<bool> {
		debug!("Checking if key {key} exists");

		let mut con = self.client.get_async_connection().await?;
		let exists: u64 = con.exists(key).await?;

		Ok(exists > 0)
	}

	async fn delete_key(&self, key: &str) -> Result<()> {
		debug!("Deleting key {key}");

		let mut con = self.client.get_async_connection().await?;
		con.del(key).await?;

		Ok(())
	}

	async fn expire_key(&self, key: &str, expire_seconds: usize) -> Result<()> {
		debug!("Expiring key {key} in {expire_seconds}");

		let mut con = self.client.get_async_connection().await?;
		con.expire(key, expire_seconds).await?;

		Ok(())
	}

	async fn add_to_index<'a>(
		&self,
		key: &str,
		member: impl ToRedisArgs + Send + Sync + 'a,
	) -> Result<()> {
		let index = format!("{key}:index");
		debug!("Appending index {index}");

		let mut con = self.client.get_async_connection().await?;
		con.sadd(index, member).await?;

		Ok(())
	}

	async fn get_from_index<T>(&self, key: &str) -> Result<Vec<T>>
	where
		T: FromRedisValue,
	{
		let index = format!("{key}:index");
		debug!("Fetching index {index}");

		let mut con = self.client.get_async_connection().await?;
		let mems = con.smembers(key).await?;

		Ok(mems)
	}

	// guild settings

	pub async fn create_guild_settings(&self, settings: Settings) -> Result<()> {
		let key = format!("{SETTINGS_KEY}:{}", settings.guild_id);

		self.set_key(&key, &settings).await?;
		// adding to index since we need to look all of these up sometimes
		self.add_to_index(SETTINGS_KEY, settings.guild_id.as_u64())
			.await?;

		Ok(())
	}

	pub async fn get_guild_settings(&self, guild_id: &GuildId) -> Result<Settings> {
		debug!("Fetching guild settings for {guild_id}");

		let key = format!("{SETTINGS_KEY}:{guild_id}");
		let settings: Settings = self.get_key(&key).await?;

		Ok(settings)
	}

	pub async fn delete_guild_settings(&self, guild_id: &GuildId) -> Result<()> {
		let key = format!("{SETTINGS_KEY}:{guild_id}");
		self.delete_key(&key).await?;

		Ok(())
	}

	pub async fn guild_settings_exist(&self, guild_id: &GuildId) -> Result<bool> {
		let key = format!("{SETTINGS_KEY}:{guild_id}");
		self.key_exists(&key).await
	}

	pub async fn get_all_guild_settings(&self) -> Result<Vec<Settings>> {
		debug!("Fetching all guild settings");

		let guild_ids: Vec<u64> = self.get_from_index(SETTINGS_KEY).await?;
		let mut guilds = vec![];
		for id in guild_ids {
			let settings = self.get_guild_settings(&GuildId::from(id)).await?;
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
		let key = format!("{REACTBOARD_KEY}:{guild_id}:{}", entry.original_message_id);

		self.set_key(&key, &entry).await?;
		self.expire_key(&key, 30 * 24 * 60 * 60).await?; // 30 days

		Ok(())
	}

	pub async fn get_reactboard_entry(
		&self,
		guild_id: &GuildId,
		message_id: &MessageId,
	) -> Result<ReactBoardEntry> {
		debug!("Fetching reactboard entry in {guild_id}");

		let key = format!("{REACTBOARD_KEY}:{guild_id}:{message_id}");
		let entry: ReactBoardEntry = self.get_key(&key).await?;

		Ok(entry)
	}

	pub async fn reactboard_entry_exists(
		&self,
		guild_id: &GuildId,
		message_id: &MessageId,
	) -> Result<bool> {
		let key = format!("{REACTBOARD_KEY}:{guild_id}:{message_id}");
		self.key_exists(&key).await
	}
}
