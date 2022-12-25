import discord
from discord.ext import commands
from .lib import get_random_response

intents = discord.Intents.default()
intents.message_content = True
moyai = commands.Bot(command_prefix="m!", description="moyai", intents=intents)


@moyai.event
async def on_ready(self):
	print(f"logged in as {self.user}")


@moyai.event
async def on_message(self, message):
	if message.author == self.user or not message.channel == "moyai-testing":
		return

	echo_messages = ["moyai", discord.utils.get(moyai.emojis, name="moyai")]
	try:
		index = echo_messages.index(message.content.toLower())
		await message.channel.send(echo_messages[index])
	except ValueError:
		return


@moyai.command()
async def ask(ctx):
	await ctx.send(get_random_response(moyai))


@moyai.command()
async def moyaispam(ctx):
	msg = str()
	for i in range(30):
		msg += str(discord.utils.get(moyai.emojis, name="moyai"))
	await ctx.send(msg)
