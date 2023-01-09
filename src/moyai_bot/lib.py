import importlib.resources
import random

import discord

from moyai_bot import copypastas


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


def get_copypasta(name):
	try:
		res = importlib.resources.read_text(copypastas, name + ".txt")
		if res != "":
			return res
	except OSError:
		pass
	return f"couldn't send copypasta: {name} :("
