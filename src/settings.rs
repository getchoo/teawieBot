use color_eyre::eyre::{Context as _, Result};
use poise::serenity_prelude::{ChannelId, GuildId, ReactionType};
use redis::{AsyncCommands as _, Client};
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};

pub const ROOT_KEY: &str = "settings-v1";

#[derive(poise::ChoiceParameter)]
pub enum SettingsProperties {
	GuildId,
	PinBoardChannel,
	PinBoardWatch,
	ReactBoardChannel,
	ReactBoardRequirement,
	ReactBoardReactions,
	OptionalCommandsEnabled,
}

#[derive(Clone, Default, PartialEq, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
pub struct Settings {
	pub guild_id: GuildId,
	pub pinboard_channel: Option<ChannelId>,
	pub pinboard_watch: Option<Vec<ChannelId>>,
	pub reactboard_channel: Option<ChannelId>,
	pub reactboard_requirement: Option<u64>,
	pub reactboard_reactions: Option<Vec<ReactionType>>,
	pub optional_commands_enabled: bool,
}

impl Settings {
	pub async fn new_redis(redis: &Client, gid: &GuildId) -> Result<()> {
		let key = format!("{ROOT_KEY}:{gid}");
		let settings = Self {
			guild_id: *gid,
			optional_commands_enabled: false,
			..Default::default()
		};

		let mut con = redis.get_async_connection().await?;
		con.set(&key, settings)
			.await
			.wrap_err_with(|| format!("Couldn't set key {key} in Redis!"))?;

		Ok(())
	}

	pub async fn from_redis(redis: &Client, gid: &GuildId) -> Result<Self> {
		let key = format!("{ROOT_KEY}:{gid}");
		let mut con = redis.get_async_connection().await?;

		let settings: Settings = con
			.get(&key)
			.await
			.wrap_err_with(|| format!("Couldn't get {key} from Redis!"))?;

		Ok(settings)
	}

	pub async fn delete(&self, redis: &Client) -> Result<()> {
		let key = format!("{ROOT_KEY}:{}", self.guild_id);
		let mut con = redis.get_async_connection().await?;

		con.del(&key)
			.await
			.wrap_err_with(|| format!("Couldn't delete {key} from Redis!"))?;

		Ok(())
	}

	pub async fn save(&self, redis: &Client) -> Result<()> {
		let key = format!("{ROOT_KEY}:{}", self.guild_id);
		let mut con = redis.get_async_connection().await?;

		con.set(&key, self)
			.await
			.wrap_err_with(|| format!("Couldn't save {key} in Redis!"))?;
		Ok(())
	}

	pub fn can_use_reaction(&self, reaction: &ReactionType) -> bool {
		if let Some(reactions) = &self.reactboard_reactions {
			reactions.iter().any(|r| r == reaction)
		} else {
			false
		}
	}
}
