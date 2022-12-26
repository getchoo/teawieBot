import discord
from discord.ext import commands

from moyai_bot.lib import get_random_response

intents = discord.Intents.default()
intents.message_content = True  # pylint: disable=assigning-non-slot
moyai = commands.Bot(command_prefix="m!", description="moyai", intents=intents)


@moyai.event
async def on_ready():
	print(f"logged in as {moyai.user}")


@moyai.event
async def on_message(message):
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
async def ask(ctx):
	await ctx.send(get_random_response(moyai))


@moyai.command()
async def moyaispam(ctx):
	msg = str()
	for _ in range(30):
		msg += str(discord.utils.get(moyai.emojis, name="moyai"))
	await ctx.send(msg)
