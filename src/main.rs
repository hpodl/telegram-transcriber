use std::process::Command;
use std::str;
use teloxide::{
    net::Download,
    prelude::*,
    types::{self, File as TelegramFile, MediaKind, MessageKind},
};
use tokio::fs::File;

const MAX_DUR: u32 = 75;

#[tokio::main]
async fn main() {
    println!("Starting up the bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        if let Some(voice) = get_voice_from_message(&message) {
            if voice.duration < MAX_DUR {
                let TelegramFile { meta, file_path } = bot.get_file(&voice.file_id).send().await?;
                let local_filename = meta.file_unique_id.as_str();
                let mut local_file = File::create(local_filename).await?;

                println!("Downloading voice file...");
                bot.download_file(&file_path, &mut local_file).await?;
                println!("Finished downloading voice file...");

                bot.send_message(
                    message.chat.id,
                    transcribe(local_filename)
                        .await
                        .unwrap_or("Failed to transcribe the message".to_string()),
                )
                .reply_to_message_id(message.id)
                .await?;
            }
        } else if Some("ping") == message.text() {
            bot.send_message(
                message.chat.id,
                format!("Pong"),
            )
            .reply_to_message_id(message.id)
            .await
            .unwrap();
        }

        respond(())
    })
    .await;
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

async fn transcribe(path: &str) -> Result<String, ()> {
    if cfg!(target_os = "windows") {
        panic!()
    }

    eprintln!("Starting transcription of {}.", path);
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "whisper --model small --language pl --task transcribe {path}"
        ))
        .output();

    match output {
        Ok(result) => {
            let transcribed = str::from_utf8(&result.stdout).unwrap().to_string();
            eprintln!("{}", str::from_utf8(&result.stderr).unwrap());

            Ok(transcribed)
        }
        Err(e) => {
            eprintln!("Failed transcribing {}: {}", path, e);
            Err(())
        }
    }
}
