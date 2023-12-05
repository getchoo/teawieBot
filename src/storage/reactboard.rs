use poise::serenity_prelude::{ChannelId, MessageId};
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};

pub const REACT_BOARD_KEY: &str = "reactboard-v1";

#[derive(Clone, Debug, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
pub struct ReactBoardEntry {
	pub original_id: MessageId,
	pub reaction_count: u64,
	pub channel_id: ChannelId,
	pub message_id: MessageId,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
pub struct ReactBoardInfo {
	pub reactions: Vec<ReactBoardEntry>,
}
