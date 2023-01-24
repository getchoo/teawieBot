import importlib.resources
import random
from math import ceil

from discord.ext import commands
import discord

from teawie_bot import copypastas

CHAR_LIMIT: int = 2000


class Teawies:
	"""
    wrapper class around list[discord.Emoji]
    """

	def __init__(self, bot: commands.Bot):
		names = [
		    "teawiecry", "teawiederp", "teawiedizzy",
		    "teawienerdcroppedanddownsized", "teawieneutral", "teawiepet",
		    "teawiepetfast", "teawiepop", "teawiesmile", "teawiesmug",
		    "teawiestarstruck", "tei", "wavy", "wie", "wie~1",
		    "manythoughtsheadfull"
		]

		self.emojis: list[str] = []
		for name in names:
			emoji = discord.utils.get(bot.emojis, name=name)
			if emoji:
				self.emojis.append(str(emoji))

	def random(self) -> str:
		return random.choice(self.emojis)


def get_random_response(bot: commands.Bot, teawies: Teawies) -> str:
	responses = [
	    "soon", "maybe", "perhaps", "elaborate",
	    str(discord.utils.get(bot.emojis, name="moyai")),
	]
	responses = responses + teawies.emojis
	return random.choice(responses)


def split_msg(msg: str) -> list[str]:
	"""
    splits a message into multiple parts so that it
    can fit into the discord character limit
    """
	split = ceil(len(msg) / ceil(len(msg) / CHAR_LIMIT))
	return [msg[i:i + split] for i in range(0, len(msg), split)]


def get_copypasta(name: str) -> list[str]:
	try:
		res = importlib.resources.read_text(copypastas, name + ".txt")
	except OSError:
		return ["something went wrong :("]

	if res == "":
		return [f"couldn't send copypasta: {name} :("]

	if len(res) >= CHAR_LIMIT:
		res = split_msg(res)
	else:
		res = [res]

	return res
