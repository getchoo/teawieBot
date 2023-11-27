use crate::utils;

use log::*;
use poise::serenity_prelude::model::prelude::*;
use poise::serenity_prelude::{Context, CreateEmbed};

#[derive(Clone)]
pub struct PinBoard {
	sources: Option<Vec<ChannelId>>,
	target: ChannelId,
}
impl PinBoard {
	pub fn new() -> Option<Self> {
		let Some(target) = utils::parse_snowflake_from_env("PIN_BOARD_TARGET", ChannelId) else {
			return None;
		};
		let sources = utils::parse_snowflakes_from_env("PIN_BOARD_SOURCES", ChannelId);

		Some(Self { sources, target })
	}

	pub async fn handle_pin(&self, ctx: &Context, pin: &ChannelPinsUpdateEvent) {
		if let Some(sources) = &self.sources {
			if !sources.contains(&pin.channel_id) {
				warn!("can't access source of pin!");
				return;
			}
		}

		let mut pinner = guess_pinner(ctx, pin).await;
		let pins = pin
			.channel_id
			.pins(&ctx.http)
			.await
			.expect("couldn't get a list of pins!?");

		for pin in pins {
			// We call `take` because it's supposed to be just for the latest message.
			self.redirect(ctx, &pin, pinner.take()).await;
			pin.unpin(&ctx).await.expect("couldn't unpin message");
		}
	}

	async fn redirect(&self, ctx: &Context, pin: &Message, pinner: Option<UserId>) {
		let pinner = pinner.map_or("*someone*".to_owned(), |u| format!("<@{u}>"));

		let truncation_point = utils::floor_char_boundary(&pin.content, 700);
		let truncated_content = if pin.content.len() <= truncation_point {
			pin.content.to_string()
		} else {
			format!("{}...", &pin.content[..truncation_point])
		};
		let color = pin
			.member(ctx)
			.await
			.ok()
			.and_then(|m| m.highest_role_info(&ctx.cache))
			.and_then(|(role, _)| role.to_role_cached(&ctx.cache))
			.map(|role| role.colour);

		let attached_image = pin
			.attachments
			.iter()
			.filter(|a| {
				a.content_type
					.as_ref()
					.filter(|ct| ct.contains("image/"))
					.is_some()
			})
			.map(|a| &a.url)
			.next();

		let attachments_len = pin.attachments.len();

		self.target
			.send_message(&ctx.http, |m| {
				m.allowed_mentions(|am| am.empty_parse())
					.content(format!("📌'd by {pinner} in {}", pin.link()))
					.add_embed(|embed| {
						// only use the first embed if it's in the message, since more could be a little spammy
						if let Some(pinned_embed) = pin.embeds.first() {
							embed.clone_from(&CreateEmbed::from(pinned_embed.clone()))
						}

						embed.author(|author| {
							author.name(&pin.author.name).icon_url(pin.author.face())
						});

						if let Some(color) = color {
							embed.color(color);
						}

						if let Some(attachment) = attached_image {
							embed.image(attachment);
						}

						if attachments_len > 1 {
							embed.footer(|footer| {
								// yes it will say '1 attachments' no i do not care
								footer.text(format!("{} attachments", attachments_len))
							});
						}

						embed.description(truncated_content)
					})
			})
			.await
			.expect("couldn't redirect message");
	}
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
