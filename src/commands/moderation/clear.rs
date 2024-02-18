use crate::Context;

use eyre::{Context as _, Result};
use log::debug;
use poise::serenity_prelude::futures::{StreamExt, TryStreamExt};

#[poise::command(
	slash_command,
	ephemeral,
	required_permissions = "MANAGE_MESSAGES",
	default_member_permissions = "MANAGE_MESSAGES"
)]
pub async fn clear_messages(
	ctx: Context<'_>,
	#[description = "How many messages to delete"] num_messages: usize,
) -> Result<()> {
	ctx.defer_ephemeral().await?;

	let channel = ctx.channel_id();
	let messages = channel
		.messages_iter(ctx)
		.take(num_messages)
		.try_fold(Vec::new(), |mut acc, msg| async move {
			acc.push(msg);
			Ok(acc)
		})
		.await
		.wrap_err_with(|| {
			format!("Couldn't collect {num_messages} messages from channel {channel}")
		})?;

	debug!("Clearing {num_messages} messages from channel {channel}!");
	channel.delete_messages(ctx, messages).await?;

	ctx.reply(format!("Deleted {num_messages} message(s)"))
		.await?;

	Ok(())
}
