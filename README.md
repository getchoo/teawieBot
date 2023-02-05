# teawie bot

okay so like basically, it's just a discord bot named "teawie" (so cool!!)

## features / commands

(some are slash, some are not)

**m!ask** | ask the bot a question with predefined answers
(this is also a slash command, use with /ask)

**m!teawiespam** | spams :teawiesmile:

**/copypasta** | sends a random copypasta from src\teawie_bot\copypastas

**/random_teawie** | sends out a random teawie, which is a soft cute character made by SympathyTea

## dev setup

```shell
python -m venv .env
.\.env\Scripts\activate.ps1 # for powershell (windows)
source .env/bin/activate # for linux

pip install flit
flit install --only-deps
pre-commit install
```
