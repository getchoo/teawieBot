import os

from .bot import moyai

TOKEN = os.getenv("TOKEN")


def main():
	moyai.run(TOKEN)
