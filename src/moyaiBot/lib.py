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
		"HOLY SHIT DID YOU JUST SAY THE WORD SUS???😳1?/1😱//1😳/1111!!!! Wait, you don't know what it is from?😳😳😳Let 👆give you a brief r/history. 📚📚📚👨‍🚀If you didn't r/knowyourshit, the r/term sus(suspicious) is a saying from the r/popular r/game r/AmongUs. Among us is so fun😔 👉👈, don't insult it, every youtuber and streamer says so!!!!!!!11 Corpses voice is so deep am i right or am i right😳😳????? I mean Mr beast and Dream play and pull big 🧠 1000000000000 iq moves in their videos..... YOU WERE THE IMPOSTER.... ඞ ඞ ඞ Get it because you don't know what sus means? r/stupidquestions r/youranidot r/stupidcuck. I CAnT BELEeVE YOUU dont KNoW WHT SUS MeaNS?/??!??!?!!🖕🖕🖕🖕🖕 Man why do i have to r/explain this to a r/idiot🤪🤪🤪📚📚📚... Sus is a GREAT WORD from a GREAT VIDEO GAME. in class, YOU CAN PLAY IT ON YOUR PHONE😜😜😜😜😜😜**??!?!?** such a masterpiece... FOR THE GREAT PRICE OF FREE!!!11!💰💰🤑🤑🤑🤑😜😜😜💰💰 It can also mean gay 😳😳😳😳",
	    str(discord.utils.get(moyai.emojis, name="moyai")),
	]
	return random.choice(responses)
