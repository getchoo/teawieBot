use crate::colors::Colors;
use crate::Context;

use color_eyre::eyre::{eyre, Result};
use log::*;
use poise::serenity_prelude::{CreateEmbed, User};

fn create_moderation_embed(
	title: String,
	user: &User,
	delete_messages_days: Option<u8>,
	reason: String,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
	let reason = if reason.is_empty() {
		"n/a".to_string()
	} else {
		reason
	};

	let fields = [
		("User", format!("{} ({})", user.name, user.id), false),
		("Reason", reason, false),
		(
			"Deleted messages",
			format!("Last {} days", delete_messages_days.unwrap_or(0)),
			false,
		),
	];

	|e: &mut CreateEmbed| e.title(title).fields(fields).color(Colors::Red)
}

/// ban a user
#[poise::command(
	slash_command,
	prefix_command,
	required_permissions = "BAN_MEMBERS",
	default_member_permissions = "BAN_MEMBERS"
)]
pub async fn ban_user(
	ctx: Context<'_>,
	user: User,
	delete_messages_days: Option<u8>,
	reason: Option<String>,
) -> Result<()> {
	let days = delete_messages_days.unwrap_or(1);
	let guild = ctx
		.guild()
		.ok_or_else(|| eyre!("Couldn't get guild from message; Unable to ban!"))?;

	let reason = reason.unwrap_or_default();

	debug!("Banning user {} with reason {reason}", user.id);
	guild.ban_with_reason(ctx, &user, days, &reason).await?;

	let embed = create_moderation_embed("User banned!".to_string(), &user, Some(days), reason);

	ctx.send(|m| m.embed(embed)).await?;

	Ok(())
}

/// kick a user
#[poise::command(
	slash_command,
	prefix_command,
	required_permissions = "KICK_MEMBERS",
	default_member_permissions = "KICK_MEMBERS"
)]
pub async fn kick_user(ctx: Context<'_>, user: User, reason: Option<String>) -> Result<()> {
	let guild = ctx
		.guild()
		.ok_or_else(|| eyre!("Couldn't get guild from message; Unable to ban!"))?;

	let reason = reason.unwrap_or_default();

	debug!("Kicking user {} for reason {reason}", user.id);
	guild.kick_with_reason(ctx, &user, &reason).await?;

	let embed = create_moderation_embed("User kicked!".to_string(), &user, None, reason);
	ctx.send(|m| m.embed(embed)).await?;

	Ok(())
}
