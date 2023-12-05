use poise::serenity_prelude::{ChannelId, GuildId, ReactionType};
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};

pub const SETTINGS_KEY: &str = "settings-v1";

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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
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
	pub fn can_use_reaction(&self, reaction: &ReactionType) -> bool {
		if let Some(reactions) = &self.reactboard_reactions {
			reactions.iter().any(|r| r == reaction)
		} else {
			false
		}
	}
}
