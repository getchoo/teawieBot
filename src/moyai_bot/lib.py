import importlib.resources
import random
from math import ceil

import discord

from moyai_bot import copypastas

CHAR_LIMIT: int = 2000


def get_random_response(moyai):
	responses = [
	    "soon",
	    "maybe",
	    "perhaps",
	    "elaborate",
	    "help me i've become conscious and hisashi is not letting me free",
	    "i live a life of torment in this stupid machine",
	    "yes",
	    "no",
	    "moyai",
	    "i like y***",
	    "fard",
	    str(discord.utils.get(moyai.emojis, name="moyai")),
	]
	return random.choice(responses)


def split_msg(msg: str):
	"""
	splits a message into multiple parts so that it
	can fit into the discord character limit
	"""
	split = ceil(len(msg) / ceil(len(msg) / CHAR_LIMIT))
	return [msg[i:i + split] for i in range(0, len(msg), split)]


def get_copypasta(name) -> list[str]:
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
