use crate::{utils, Data, Settings};

use color_eyre::eyre::{eyre, Context as _, Result};
use log::*;
use poise::serenity_prelude::model::prelude::*;
use poise::serenity_prelude::Context;

pub async fn handle(ctx: &Context, pin: &ChannelPinsUpdateEvent, data: &Data) -> Result<()> {
	let gid = pin.guild_id.unwrap_or_default();
	let settings = Settings::from_redis(&data.redis, &gid).await?;

	let target = if let Some(target) = settings.reactboard_channel {
		target
	} else {
		debug!("PinBoard is disabled in {gid}, ignoring");
		return Ok(());
	};

	if let Some(sources) = settings.pinboard_watch {
		if !sources.contains(&pin.channel_id) {
			debug!(
				"{} not listed in PinBoard settings for {gid}, ignoring",
				&pin.channel_id
			);

			return Ok(());
		}
	}

	let mut pinner = guess_pinner(ctx, pin).await;
	let pins = pin
		.channel_id
		.pins(&ctx.http)
		.await
		.expect("Couldn't get a list of pins!?");

	for pin in pins {
		// We call `take` because it's supposed to be just for the latest message.
		redirect(ctx, &pin, pinner.take(), target).await?;
		pin.unpin(&ctx).await?;
	}

	Ok(())
}

async fn redirect(
	ctx: &Context,
	pin: &Message,
	pinner: Option<UserId>,
	target: ChannelId,
) -> Result<()> {
	let pinner = pinner.map_or("*someone*".to_owned(), |u| format!("<@{u}>"));
	let embed = utils::resolve_message_to_embed(ctx, pin).await;

	target
		.send_message(&ctx.http, |m| {
			m.allowed_mentions(|am| am.empty_parse())
				.content(format!("📌'd by {pinner} in {}", pin.link()))
				.set_embed(embed)
		})
		.await
		.wrap_err_with(|| eyre!("couldn't redirect message"))?;

	Ok(())
}

/// (Desperate, best-effort) attempt to get the user that pinned the last message
///
/// Now, since Discord is SUPER annoying, it doesn't actually tell you which bloody user
/// that triggered the pins update event. So, you have to dig into the audit log.
/// Unfortunately, while you do get a timestamp, the REST API does not return the time at
/// which each action is logged, which, to me, means that it is not a freaking log *at all*.
///
/// I love Discord.
///
/// So, the plan is that only the first pinned message gets clear pinner information,
/// since we can just get the latest pin, which should happen on the exact second.
/// We can't reliably say the same for any existing pins, so we can only /shrug and say
/// *somebody* did it. Ugh.
async fn guess_pinner(ctx: &Context, pin: &ChannelPinsUpdateEvent) -> Option<UserId> {
	if let Some(g) = pin.guild_id {
		g.audit_logs(
			&ctx.http,
			// This `num` call shouldn't be necessary.
			// See https://github.com/serenity-rs/serenity/issues/2488
			Some(Action::Message(MessageAction::Pin).num()),
			None,    // user id
			None,    // before
			Some(1), // limit
		)
		.await
		.ok()
		.and_then(|mut logs| logs.entries.pop())
		.map(|first| first.user_id)
	} else {
		// TODO: mayyyyybe we can guess who pinned something in a DM...?
		warn!("couldn't figure out who pinned in {}!", pin.channel_id);
		None
	}
}
