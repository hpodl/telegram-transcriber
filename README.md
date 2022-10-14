# `[WIP]` telegram-transcriber-bot

Simple telegram bot capable of transcribing all messages containing (comprehensible) audio. For that purpose OpenAI's [Whisper](https://github.com/openai/whisper) is used.

Uses [Teloxide](https://github.com/teloxide/teloxide) to interact with Telegram API.


## Setup
Simply build the project using cargo.

Before running the bot, you need to provide your telegram API token as an environment variable `TELOXIDE_TOKEN`.

## Usage
Reply with `/transcribe` to the message to be transcribed (on a server where the bot is present; alternatively you can forward the message to the bot and then `/transcribe` it) and wait.


## Additional requirements
### Python
- [whisper](https://github.com/openai/whisper)
- [python-huggingface-hub](https://pypi.org/project/huggingface-hub/) (used to download pretrained whisper models, thus might be unnecessary if you have them already)

Both are specified in `requirements.txt` and can be installed with:

    pip install -r requirements.txt