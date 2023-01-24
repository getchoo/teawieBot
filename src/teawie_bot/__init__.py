import os

from .bot import bot

TOKEN = os.getenv("TOKEN")


def main():
	bot.run(TOKEN)
