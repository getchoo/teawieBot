# moyai bot

okay so like basically, it's just a discord bot named "Moyai" (so original!!)

## features / commands
(some are slash, some are not)

**m!ask** | Ask the bot a question with predefined answers 
	(this is also a slash command, use with /ask)

**/copypasta** | Gives out a random copypasta from the copypasta folder in *src\copypastas*
	current copypastas include:
		dvd
		egrill
		happymeal
		ismah (is my son a hacker)
		navyseal
		sus
		ticktock
		
**/random_teawie** | Sends out a random teawie, which is a soft cute character made by SympathyTea

## dev setup

```shell
python -m venv .env
.\.env\Scripts\activate.ps1 # for powershell (windows)
source .env/bin/activate # for linux

pip install flit
flit install --deps=develop --only-deps
pre-commit install
```
