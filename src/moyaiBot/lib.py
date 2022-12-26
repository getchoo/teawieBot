import discord
import random


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
		"OH MY GOD ITS 3 IN THE MORNING AND IM IN MCDONALDS AND WE JUST FOUND OUT THAT WHEN U PULL UP IN MCDONALDS AT 3 AM YOU CAN BUY THE AMONG US HAPPY MEAL WITH A TOY IN IT WHICH IS EITHER THE IMPOSTOR OR THE CREWMATE AND IF YOU DONT KNOW WHAT AMONG US IS YOU MUST BE MUST REALLY BE LIVING UNDER A ROCK ITS AN AWESOME GAME WITH IMPOSTORS AND CREWMATES AND BASICALLY THE IMPOSTOR TRIES TO SABOTAGE THE WHOLE GAME AND THE CREWMATES NEED TO STOP HIM BUT APPARENTLY WHEN YOU PURCHASE THE AMONG US HAPPY MEAL SOMETHING SCARY HAPPENS",
        "Tick-tock \n Heavy like a Brinks truck \n Looking like I'm tip-top \n Shining like a wristwatch \n Time will grab your wrist \n Lock it down 'til the thing pop \n Can you stick around for a minute 'til the ring stop? \n Please, God",
	    str(discord.utils.get(moyai.emojis, name="moyai")),
	]
	return random.choice(responses)
