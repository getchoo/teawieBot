use crate::{utils, Data};

use eyre::{eyre, Context as _, OptionExt as _, Result};
use log::debug;
use poise::serenity_prelude::{
	ChannelId, Context, CreateAllowedMentions, CreateMessage, Message, MessageType, User,
};

pub async fn handle(ctx: &Context, message: &Message, data: &Data) -> Result<()> {
	if message.kind != MessageType::PinsAdd {
		return Ok(());
	}

	let gid = message.guild_id.unwrap_or_default();
	let settings = data.storage.get_guild_settings(&gid).await?;

	if !settings.pinboard_enabled {
		debug!("PinBoard is disabled in {gid}, ignoring");
		return Ok(());
	}

	let Some(target) = settings.pinboard_channel else {
		debug!("PinBoard is disabled in {gid}, ignoring");
		return Ok(());
	};

	if let Some(sources) = settings.pinboard_watch {
		if !sources.contains(&message.channel_id) {
			debug!(
				"{} not listed in PinBoard settings for {gid}, ignoring",
				message.channel_id
			);

			return Ok(());
		}
	}

	let reference_id = message
		.clone()
		.message_reference
		.ok_or_eyre("Couldn't get referenced message of pin!")?
		.message_id
		.ok_or_eyre("Couldn't get id of referenced message of pin!")?;

	let pins = message
		.channel_id
		.pins(ctx)
		.await
		.wrap_err("Couldn't get a list of pins!?")?;

	let pin = pins
		.iter()
		.find(|pin| pin.id == reference_id)
		.ok_or_else(|| eyre!("Couldn't find a pin for message {reference_id}!"))?;

	redirect(ctx, pin, &message.author, target).await?;
	pin.unpin(ctx).await?;

	Ok(())
}

async fn redirect(ctx: &Context, pin: &Message, pinner: &User, target: ChannelId) -> Result<()> {
	let embed = utils::resolve_message_to_embed(ctx, pin).await;
	let mentions = CreateAllowedMentions::new().empty_roles().empty_users();
	let message = CreateMessage::default()
		.allowed_mentions(mentions)
		.content(format!("📌'd by {pinner} in {}", pin.link()))
		.embed(embed);

	target
		.send_message(&ctx.http, message)
		.await
		.wrap_err("Couldn't redirect message")?;

	Ok(())
}
