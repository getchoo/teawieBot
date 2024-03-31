use poise::serenity_prelude::{ChannelId, MessageId};
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
pub struct ReactBoardEntry {
	pub original_message_id: MessageId,
	pub reaction_count: u64,
	// we need these to update our message with new interactions
	pub posted_channel_id: ChannelId,
	pub posted_message_id: MessageId,
}
