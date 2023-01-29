import discord
from discord import app_commands
from discord.ext import commands

from teawie_bot import utils
from teawie_bot.apis import guzzle

SERVER_ID = discord.Object(id=1055663552679137310)
intents = discord.Intents.default()
intents.message_content = True  # pylint: disable=assigning-non-slot
bot = commands.Bot(command_prefix="t!",
                   description="teawie time",
                   intents=intents)


@bot.event
async def on_ready():
	print(f"logged in as {bot.user}")
	await bot.tree.sync(guild=SERVER_ID)
	bot.teawies = utils.Teawies(bot)
	print("ready!")


@bot.event
async def on_message(message: discord.Message):
	if message.author == bot.user:
		return

	echo_messages = [
	    "🗿",
	]
	echo_messages = echo_messages + bot.teawies.emojis
	try:
		index = echo_messages.index(message.content.lower())
		await message.channel.send(echo_messages[index])
	except ValueError:
		pass

	await bot.process_commands(message)


@bot.command()
async def ask(ctx: commands.Context):
	await ctx.send(utils.get_random_response(bot))


@bot.tree.command(
    name="ask",
    description="ask lord teawie a question and they shall respond",
    guild=SERVER_ID)
async def ask_slash_command(interaction: discord.Interaction):
	msg = utils.get_random_response(bot)
	await interaction.response.send_message(msg)


@bot.command()
async def teawiespam(ctx: commands.Context):
	emoji = str(discord.utils.get(bot.emojis, name="teawiesmile"))
	msg = str()
	for _ in range(50):
		msg += emoji

	await ctx.send(msg)


@bot.tree.command(name="copypasta",
                  description="send funni copypasta",
                  guild=SERVER_ID)
@app_commands.choices(choices=[
    app_commands.Choice(name="happymeal", value="happymeal"),
    app_commands.Choice(name="ismah", value="ismah"),
    app_commands.Choice(name="sus", value="sus"),
    app_commands.Choice(name="ticktock", value="ticktock"),
    app_commands.Choice(name="amongus_sus", value="amongus_sus"),
    app_commands.Choice(name="egrill", value="egrill"),
    app_commands.Choice(name="dvd", value="dvd"),
])
async def copypasta(interaction: discord.Interaction,
                    choices: app_commands.Choice[str]):
	msgs = utils.get_copypasta(choices.value)
	for i, msg in enumerate(msgs):
		if i == 0:
			await interaction.response.send_message(msg)
		else:
			await interaction.channel.send(msg)


@bot.tree.command(name="random_teawie",
                  description="get a random teawie!",
                  guild=SERVER_ID)
async def random_teawie(interaction: discord.Interaction):
	msg = guzzle.get_random_teawie()
	await interaction.response.send_message(msg)
