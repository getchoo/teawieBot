use once_cell::sync::Lazy;
use regex::Regex;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::application::command::Command;
use serenity::model::prelude::*;
use serenity::prelude::*;

mod api;
mod commands;
mod consts;
mod utils;

const TEAWIE_GUILD: GuildId = GuildId(1055663552679137310);
const BOT: UserId = UserId(1056467120986271764);

fn is_guild_allowed(gid: GuildId) -> bool {
    // Had to be global state because Serenity doesn't allow you to store
    // extra state in frameworks
    static ALLOWED_GUILDS: Lazy<Vec<GuildId>> = Lazy::new(|| {
    	std::env::var("ALLOWED_GUILDS")
    		.ok()
    		.and_then(|gs| {
    			gs.split(',')
    				.map(|g| u64::from_str_radix(g, 10).map(GuildId))
    				.collect::<Result<Vec<_>, _>>()
    				.ok()
    		})
    		.unwrap_or_else(|| vec![TEAWIE_GUILD, GuildId(1091969030694375444)])
    });

    ALLOWED_GUILDS.contains(&gid)
}


#[group]
#[commands(bing, ask, random_lore, random_teawie, teawiespam)]
struct General;

struct Handler {
	bot: UserId,
}

impl Handler {
	fn should_echo(&self, msg: &Message) -> bool {
		static MOYAI_REGEX: Lazy<Regex> =
			Lazy::new(|| Regex::new(r"^<a?:\w*moy?ai\w*:\d+>$").unwrap());

        // Don't echo to anything we posted ourselves, and don't echo at all unless on certain
        // servers
        if msg.author.id == self.bot || !is_guild_allowed(msg.guild_id.unwrap_or_default()) {
            return false;
        }

        let content = &msg.content;

        content == "🗿"
			|| consts::TEAMOJIS.contains(&content.as_str())
			|| MOYAI_REGEX.is_match(content)
			|| content
				.to_ascii_lowercase()
				.contains("twitter's recommendation algorithm")
	}
}

#[async_trait]
impl EventHandler for Handler {
	/*
	 * echo some messages when they're sent
	 */
	async fn message(&self, ctx: Context, msg: Message) {
		if self.should_echo(&msg) {
			let send = msg.reply(&ctx, &msg.content);
			if let Err(why) = send.await {
				println!("error when replying to {:?}: {:?}", msg.content, why);
			}
		}
	}

	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		if let Interaction::ApplicationCommand(command) = interaction {
			println!("Received command interaction: {command:#?}");
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
				println!("cannot respond to slash command: {why}");
			}
		}
	}

	async fn ready(&self, ctx: Context, ready: Ready) {
		println!("connected as {:?}", ready.user.name);

		let guild_commands =
			GuildId::set_application_commands(&TEAWIE_GUILD, &ctx.http, |commands| {
				commands.create_application_command(commands::copypasta::register)
			})
			.await;

		println!("registered guild commands: {guild_commands:#?}");

		let commands = Command::set_global_application_commands(&ctx.http, |commands| {
			commands
				.create_application_command(commands::ask::register)
				.create_application_command(commands::bottom::register)
				.create_application_command(commands::convert::register)
				.create_application_command(commands::random_lore::register)
				.create_application_command(commands::random_teawie::register)
		})
		.await;

		println!("registered global commands: {commands:#?}");
	}
}

#[tokio::main]
async fn main() {
	let framework = StandardFramework::new()
		.configure(|c| c.prefix("!"))
		.group(&GENERAL_GROUP);

	let token = std::env::var("TOKEN").expect("couldn't find token in environment.");

	let bot = std::env::var("BOT")
		.ok()
		.and_then(|b| u64::from_str_radix(&b, 10).map(UserId).ok())
		.unwrap_or(BOT);

	let intents = GatewayIntents::all();

	let handler = Handler { bot };

	let mut client = Client::builder(token, intents)
		.event_handler(handler)
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
	if !ALLOWED_GUILDS.contains(&msg.guild_id.unwrap_or_default()) {
		return Ok(());
	}

	let resp = "<:teawiesmile:1056438046440042546>".repeat(50);

	msg.channel_id
		.send_message(&ctx.http, |m| m.content(resp))
		.await?;

	Ok(())
}
