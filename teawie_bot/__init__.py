import os

from .bot import bot

TOKEN = os.getenv("TOKEN")


def main():
	try:
		bot.run(TOKEN)
	except TypeError:
		print("invalid/no token!")
