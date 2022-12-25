import os
from .moyaiBot import moyai

TOKEN = os.getenv("TOKEN")


def main():
	moyai.run(TOKEN)
