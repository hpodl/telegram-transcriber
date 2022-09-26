use std::process::Command as OsCommand;
use teloxide::types::{self, File as TelegramFile, MediaKind, MessageKind};
use teloxide::{net::Download, prelude::*, utils::command::BotCommands};

use std::error::Error;
use tokio::fs::File;

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
    #[command(description = "transcribe voice message being replied to")]
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
                bot.send_message(message.chat.id, "").await?
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

async fn download_and_transcribe(
    bot: &AutoSend<Bot>,
    voice: &types::Voice,
) -> Result<String, String> {
    let TelegramFile { meta, file_path } = bot.get_file(&voice.file_id).send().await.unwrap();
    let local_filename = meta.file_unique_id.as_str();
    let mut local_file = File::create(local_filename).await.unwrap();

    println!("Downloading voice file...");
    bot.download_file(&file_path, &mut local_file)
        .await
        .unwrap();
    println!("Finished downloading voice file...");

    transcribe(local_filename).await
}

async fn transcribe(path: &str) -> Result<String, String> {
    if cfg!(target_os = "windows") {
        panic!()
    }

    eprintln!("Starting transcription of {}.", path);
    let output = OsCommand::new("sh")
        .arg("-c")
        .arg(format!(
            "whisper --model small --language pl --task transcribe {path}"
        ))
        .output();

    match output {
        Ok(result) => {
            let transcribed = std::str::from_utf8(&result.stdout).unwrap().to_string();
            eprintln!("{}", std::str::from_utf8(&result.stderr).unwrap());

            Ok(transcribed)
        }
        Err(e) => {
            eprintln!("Failed transcribing {}: {}", path, e);
            Err("Couldn't transcribe the message.".to_string())
        }
    }
}
