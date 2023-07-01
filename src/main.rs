use lazy_static::lazy_static;
use regex::Regex;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::channel::Message;
use serenity::model::id::GuildId;
use serenity::model::prelude::Ready;
use serenity::prelude::*;
use std::{env, vec};

mod api;
mod commands;
mod consts;
mod utils;

const TEAWIE_GUILD: GuildId = GuildId(1055663552679137310);
const BOT: u64 = 1056467120986271764;

#[group]
#[commands(bing, ask, random_lore, random_teawie, teawiespam)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	/*
	 * echo some messages when they're sent
	 */
	async fn message(&self, ctx: Context, msg: Message) {
		let author = msg.author.id.as_u64();

		if author == &BOT || msg.guild_id.unwrap_or_else(|| GuildId::from(0)) != TEAWIE_GUILD {
			return;
		}

		let mut echo_msgs = vec!["🗿", "Twitter's Recommendation Algorithm"];

		for emoji in consts::TEAMOJIS {
			// i was also lazy here
			echo_msgs.push(emoji);
		}

		let mut should_echo = echo_msgs.contains(&msg.content.as_str());

		if !should_echo {
			lazy_static! {
				static ref EMOJI_RE: Regex = Regex::new(r"^<a?:(\w+):\d+>$").unwrap();
			}
			if let Some(cap) = EMOJI_RE.captures(msg.content.as_str()) {
				if let Some(emoji_name) = cap.get(1) {
					let emoji_name = emoji_name.as_str();
					should_echo = emoji_name.contains("moai") || emoji_name.contains("moyai");
				}
			}
		}

		if should_echo {
			let send = msg.reply(&ctx, msg.content.as_str());
			if let Err(why) = send.await {
				println!("error when replying to {:?}: {:?}", msg.content, why);
			}
		}
	}

	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		if let Interaction::ApplicationCommand(command) = interaction {
			println!("Received command interaction: {:#?}", command);
			let content = match command.data.name.as_str() {
				"ask" => commands::ask::run(&command.data.options).await,
				"bottom" => commands::bottom::run(&command.data.options).await,
				"convertto" => commands::convert::run(&command.data.options).await,
				"copypasta" => {
					commands::copypasta::run(&command.data.options, command.channel_id, &ctx.http)
						.await
				}
				"random_lore" => commands::random_lore::run(&command.data.options).await,
				"random_teawie" => commands::random_teawie::run(&command.data.options).await,
				_ => "not implemented :(".to_string(),
			};

			if let Err(why) = command
				.create_interaction_response(&ctx.http, |response| {
					response
						.kind(InteractionResponseType::ChannelMessageWithSource)
						.interaction_response_data(|message| message.content(content))
				})
				.await
			{
				println!("cannot respond to slash command: {}", why);
			}
		}
	}

	async fn ready(&self, ctx: Context, ready: Ready) {
		println!("connected as {:?}", ready.user.name);

		let guild_commands =
			GuildId::set_application_commands(&TEAWIE_GUILD, &ctx.http, |commands| {
				commands
					.create_application_command(|command| commands::copypasta::register(command))
			})
			.await;

		println!("registered guild commands: {:#?}", guild_commands);

		let commands = Command::set_global_application_commands(&ctx.http, |commands| {
			commands
				.create_application_command(|command| commands::ask::register(command))
				.create_application_command(|command| commands::bottom::register(command))
				.create_application_command(|command| commands::convert::register(command))
				.create_application_command(|command| commands::random_lore::register(command))
				.create_application_command(|command| commands::random_teawie::register(command))
		})
		.await;

		println!("registered global commands: {:#?}", commands);
	}
}

#[tokio::main]
async fn main() {
	let framework = StandardFramework::new()
		.configure(|c| c.prefix("!"))
		.group(&GENERAL_GROUP);

	let token = env::var("TOKEN").expect("couldn't find token in environment.");

	let intents = GatewayIntents::all();
	let mut client = Client::builder(token, intents)
		.event_handler(Handler)
		.framework(framework)
		.await
		.expect("error creating client");

	if let Err(why) = client.start().await {
		println!("an error occurred: {:?}", why);
	}
}

#[command]
async fn bing(ctx: &Context, msg: &Message) -> CommandResult {
	msg.channel_id
		.send_message(&ctx.http, |m| m.content("bong"))
		.await?;

	Ok(())
}

#[command]
async fn ask(ctx: &Context, msg: &Message) -> CommandResult {
	let resp = utils::get_random_response().await;
	msg.channel_id
		.send_message(&ctx.http, |m| m.content(resp))
		.await?;

	Ok(())
}

#[command]
async fn random_lore(ctx: &Context, msg: &Message) -> CommandResult {
	let resp = utils::get_random_lore().await;
	msg.channel_id
		.send_message(&ctx.http, |m| m.content(resp))
		.await?;

	Ok(())
}

#[command]
async fn random_teawie(ctx: &Context, msg: &Message) -> CommandResult {
	let resp = api::guzzle::get_random_teawie().await;
	msg.channel_id
		.send_message(&ctx.http, |m| m.content(resp))
		.await?;

	Ok(())
}

#[command]
async fn teawiespam(ctx: &Context, msg: &Message) -> CommandResult {
	if msg.guild_id.unwrap_or_else(|| GuildId::from(0)) != TEAWIE_GUILD {
		return Ok(());
	}

	let mut resp = String::new();

	for _ in 0..50 {
		resp += "<:teawiesmile:1056438046440042546>";
	}

	msg.channel_id
		.send_message(&ctx.http, |m| m.content(resp))
		.await?;

	Ok(())
}
