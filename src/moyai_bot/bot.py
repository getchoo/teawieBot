import discord
from discord import app_commands
from discord.ext import commands

from moyai_bot.lib import get_copypasta, get_random_response

SERVER_ID = discord.Object(id=1055663552679137310)
intents = discord.Intents.default()
intents.message_content = True  # pylint: disable=assigning-non-slot
moyai = commands.Bot(command_prefix="m!", description="moyai", intents=intents)


@moyai.event
async def on_ready():
	print(f"logged in as {moyai.user}")
	await moyai.tree.sync(guild=SERVER_ID)
	print("ready!")


@moyai.event
async def on_message(message: discord.Message):
	if message.author == moyai.user:
		return

	echo_messages = [
	    "moyai", str(discord.utils.get(moyai.emojis, name="moyai"))
	]
	try:
		index = echo_messages.index(message.content.lower())
		await message.channel.send(echo_messages[index])
	except ValueError:
		pass

	await moyai.process_commands(message)


@moyai.command()
async def ask(ctx: commands.Context):
	await ctx.send(get_random_response(moyai))


@moyai.command()
async def moyaispam(ctx: commands.Context):
	msg = str()
	for _ in range(30):
		msg += str(discord.utils.get(moyai.emojis, name="moyai"))
	await ctx.send(msg)


@moyai.tree.command(name="copypasta",
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
	msgs = get_copypasta(choices.value)
	for i, msg in enumerate(msgs):
		if i == 0:
			await interaction.response.send_message(msg)
		else:
			await interaction.channel.send(msg)
