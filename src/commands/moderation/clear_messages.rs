use crate::client::Context;

use anyhow::Result;
use log::debug;
use poise::serenity_prelude::GetMessages;

#[poise::command(
	slash_command,
	guild_only,
	ephemeral,
	required_permissions = "MANAGE_MESSAGES",
	default_member_permissions = "MANAGE_MESSAGES"
)]
pub async fn clear_messages(
	ctx: Context<'_>,
	#[description = "How many messages to delete"] num_messages: u8,
) -> Result<()> {
	ctx.defer_ephemeral().await?;

	let channel = ctx.channel_id();
	let to_get = GetMessages::new().limit(num_messages);
	let messages = channel.messages(ctx, to_get).await?;

	debug!("Clearing {num_messages} messages from channel {channel}!");
	channel.delete_messages(ctx, messages).await?;

	ctx.reply(format!("Deleted {num_messages} message(s)"))
		.await?;

	Ok(())
}
