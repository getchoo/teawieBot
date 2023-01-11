# moyai bot

okay so like basically, it's just a discord bot named "Moyai" (so original!!)

## features / commands
(some are slash, some are not)

**m!ask** | ask the bot a question with predefined answers 
	(this is also a slash command, use with /ask)

**m!moyaispam** | what do you think it does. It spams the moyai emoji.

**/copypasta** | gives out a random copypasta from the copypasta folder in *src\moyai_bot\copypastas*

**/random_teawie** | sends out a random teawie, which is a soft cute character made by SympathyTea


## dev setup

```shell
python -m venv .env
.\.env\Scripts\activate.ps1 # for powershell (windows)
source .env/bin/activate # for linux

pip install flit
flit install --deps=develop --only-deps
pre-commit install
```
