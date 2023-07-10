use crate::utils;
use crate::{consts, Data};
use log::*;

use poise::serenity_prelude::Message;

mod events;

pub struct Handler {
	data: Data,
}

impl Handler {
	pub fn new(data: Data) -> Self {
		Self { data }
	}

	fn should_echo(&self, msg: &Message) -> bool {
		let gid = msg.guild_id.unwrap_or_default();
		if msg.author.id == self.data.bot || !utils::is_guild_allowed(gid) {
			info!("not running copypasta command in {gid}");
			return false;
		}

		let content = &msg.content;

		content == "🗿"
			|| consts::TEAMOJIS.contains(&content.as_str())
			|| content.to_ascii_lowercase() == "moyai"
			|| content
				.to_ascii_lowercase()
				.contains("twitter's recommendation algorithm")
	}
}
