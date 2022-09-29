# `[WIP]` telegram-transcriber-bot

Simple telegram bot capable of transcribing all messages containing (comprehensible) audio. For that purpose OpenAI's [whisper](https://github.com/openai/whisper) is used (for now, as a workaround it's invoked as a shell command and thus, the transcribee is unnecessarily downloaded to the hard drive).


## Usage
Reply with `/transcribe` to the message to be transcribed (on a server where the bot is present; alternatively you can forward the message to the bot and then `/transcribe` it) and wait.
