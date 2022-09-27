use teloxide::types::{self, MediaKind, MessageKind};
use teloxide::{prelude::*, utils::command::BotCommands};

use std::error::Error;

mod transcribe;
use transcribe::*;

#[tokio::main]
async fn main() {
    let bot = Bot::from_env().auto_send();

    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "ping.")]
    Ping,
    #[command(description = "transcribe voice message being replied to.")]
    Transcribe,
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => {
            bot.send_message(message.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Ping => bot.send_message(message.chat.id, "Pong.").await?,
        Command::Transcribe => transcribe_handler(&bot, &message).await?,
    };

    Ok(())
}

async fn transcribe_handler(
    bot: &AutoSend<Bot>,
    message: &Message,
) -> Result<Message, teloxide::RequestError> {
    Ok(match message.reply_to_message() {
        Some(replied) => {
            if let Some(voice) = get_voice_from_message(replied) {
                let transcribed = download_and_transcribe(bot, voice).await.unwrap();
                bot.send_message(message.chat.id, transcribed)
                    .reply_to_message_id(replied.id)
                    .await?
            } else {
                bot.send_message(message.chat.id, "Not a voice message").await?
            }
        }
        None => {
            bot.send_message(message.chat.id, "No message is being replied to.")
                .reply_to_message_id(message.id)
                .await?
        }
    })
}

#[inline]
fn get_voice_from_message(message: &Message) -> Option<&types::Voice> {
    if let MessageKind::Common(kind) = &message.kind {
        if let MediaKind::Voice(voice_kind) = &kind.media_kind {
            return Some(&voice_kind.voice);
        }
    }
    None
}
