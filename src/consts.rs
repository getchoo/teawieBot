#![allow(clippy::unreadable_literal)]
use std::sync::OnceLock;

use poise::serenity_prelude::{Colour, Permissions, Scope};

pub fn bot_permissions() -> &'static Permissions {
	static BOT_PERMISSIONS: OnceLock<Permissions> = OnceLock::new();
	BOT_PERMISSIONS.get_or_init(|| {
		Permissions::MANAGE_ROLES
			| Permissions::MANAGE_CHANNELS
			| Permissions::KICK_MEMBERS
			| Permissions::BAN_MEMBERS
			| Permissions::MANAGE_NICKNAMES
			| Permissions::VIEW_CHANNEL
			| Permissions::MODERATE_MEMBERS
			| Permissions::SEND_MESSAGES
			| Permissions::CREATE_PUBLIC_THREADS
			| Permissions::CREATE_PRIVATE_THREADS
			| Permissions::SEND_MESSAGES_IN_THREADS
			| Permissions::MANAGE_MESSAGES
			| Permissions::MANAGE_THREADS
			| Permissions::EMBED_LINKS
			| Permissions::ATTACH_FILES
			| Permissions::READ_MESSAGE_HISTORY
			| Permissions::ADD_REACTIONS
	})
}

pub fn bot_scopes() -> &'static Vec<Scope> {
	static BOT_SCOPES: OnceLock<Vec<Scope>> = OnceLock::new();
	BOT_SCOPES.get_or_init(|| vec![Scope::Bot, Scope::ApplicationsCommands])
}

pub const RESPONSES: [&str; 5] = [
	"soon",
	"maybe",
	"perhaps",
	"elaborate",
	"Twitter's Recommendation Algorithm",
];

pub enum Colors {
	Blue,
	Orange,
	Red,
}

impl From<Colors> for Colour {
	fn from(val: Colors) -> Self {
		match val {
			Colors::Blue => Colour::from(0x88C7FD),
			Colors::Orange => Colour::from(0xFFB34A),
			Colors::Red => Colour::from(0xFF5E4A),
		}
	}
}
