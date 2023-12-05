use std::fmt::{Debug, Display};

use color_eyre::eyre::Result;
use log::*;
use poise::serenity_prelude::GuildId;
use redis::{AsyncCommands, Client, FromRedisValue, ToRedisArgs};

pub mod reactboard;
pub mod settings;

pub use reactboard::*;
pub use settings::*;

#[derive(Clone, Debug)]
pub struct Storage {
	client: Client,
}

impl Storage {
	pub fn new(redis_url: &str) -> Result<Self> {
		let client = Client::open(redis_url)?;

		Ok(Self { client })
	}

	pub async fn get_key<T>(&self, key: &str) -> Result<T>
	where
		T: FromRedisValue,
	{
		debug!("Getting key {key}");

		let mut con = self.client.get_async_connection().await?;
		let res: T = con.get(key).await?;

		Ok(res)
	}

	pub async fn set_key<'a>(
		&self,
		key: &str,
		value: impl ToRedisArgs + Debug + Send + Sync + 'a,
	) -> Result<()> {
		debug!("Creating key {key}:\n{value:#?}");

		let mut con = self.client.get_async_connection().await?;
		con.set(key, value).await?;

		Ok(())
	}

	pub async fn key_exists(&self, key: &str) -> Result<bool> {
		debug!("Checking if key {key} exists");

		let mut con = self.client.get_async_connection().await?;
		let exists: u64 = con.exists(key).await?;

		Ok(exists > 0)
	}

	pub async fn delete_key(&self, key: &str) -> Result<()> {
		debug!("Deleting key {key}");

		let mut con = self.client.get_async_connection().await?;
		con.del(key).await?;

		Ok(())
	}

	pub async fn add_to_index<'a>(
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

	pub fn format_settings_key(subkey: impl Display) -> String {
		format!("{}:{subkey}", SETTINGS_KEY)
	}

	pub async fn create_settings_key(&self, settings: Settings) -> Result<()> {
		let key = Self::format_settings_key(settings.guild_id);

		self.set_key(&key, &settings).await?;
		self.add_to_index(SETTINGS_KEY, settings).await?;

		Ok(())
	}

	/// get guilds that have enabled optional commands
	pub async fn get_opted_guilds(&self) -> Result<Vec<GuildId>> {
		debug!("Fetching opted-in guilds");

		let guilds = self.get_all_guild_settings().await?;
		let opted: Vec<GuildId> = guilds
			.iter()
			.filter_map(|g| {
				if g.optional_commands_enabled {
					Some(g.guild_id)
				} else {
					None
				}
			})
			.collect();

		Ok(opted)
	}

	pub async fn get_all_guild_settings(&self) -> Result<Vec<Settings>> {
		debug!("Fetching all guild settings");

		let mut con = self.client.get_async_connection().await?;
		let key = Self::format_settings_key("index");

		let guilds: Vec<Settings> = con.smembers(key).await?;

		Ok(guilds)
	}

	pub async fn get_guild_settings(&self, guild_id: &GuildId) -> Result<Settings> {
		debug!("Fetching guild settings for {guild_id}");

		let key = Self::format_settings_key(guild_id);
		let settings: Settings = self.get_key(&key).await?;

		Ok(settings)
	}

	pub async fn create_reactboard_info_key(&self, reactboard: ReactBoardInfo) -> Result<()> {
		self.set_key(REACT_BOARD_KEY, reactboard).await?;
		Ok(())
	}

	pub async fn get_reactboard_info(&self) -> Result<ReactBoardInfo> {
		debug!("Fetching reactboard info");
		let reactboard: ReactBoardInfo = self.get_key(REACT_BOARD_KEY).await?;

		Ok(reactboard)
	}
}
