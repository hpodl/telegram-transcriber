use std::process::Command as OsCommand;
use teloxide::types::File as TelegramFile;
use teloxide::{net::Download, prelude::*};

use tokio::fs::{self, File};

/// Returns `Some(transcription)` of audio if `message` contains any
/// even if the transcription happens to be an empty string
///
/// Returns `None` if it doesn't
pub async fn transcribe_or_none(bot: &AutoSend<Bot>, message: &Message) -> Option<String> {
    let file_id = {
        if let Some(media) = message.voice() {
            &media.file_id
        } else if let Some(media) = message.audio() {
            &media.file_id
        } else if let Some(media) = message.video() {
            &media.file_id
        } else if let Some(media) = message.video_note() {
            &media.file_id
        } else {
            return None;
        }
    };

    download_and_transcribe(bot, file_id).await.ok()
}

// TODO: Reconsider file creations
/// Downloads and transcribes a message.
///
/// Returns transcription as a String
async fn download_and_transcribe(bot: &AutoSend<Bot>, file_id: &str) -> Result<String, String> {
    let TelegramFile { meta, file_path } = bot
        .get_file(file_id)
        .send()
        .await
        .map_err(|err| format!("Failed to download the file: {err}"))?;
    let local_filename = meta.file_unique_id.as_str();

    // Read transcription if already transcribed
    if let Ok(read) = fs::read_to_string(format!("{local_filename}.txt")).await {
        return Ok(read);
    }

    let mut local_file = File::create(local_filename)
        .await
        .map_err(|err| format!("Couldn't create the local file: {err}"))?;

    println!("Downloading {}B", meta.file_size);
    bot.download_file(&file_path, &mut local_file)
        .await
        .map_err(|err| format!("Downloading the file failed: {err}"))?;

    println!("Finished downloading");

    let transcribed = transcribe(local_filename).await;

    // errors if file was removed or permissions changed
    fs::remove_file(local_filename)
        .await
        .map_err(|err| format!("Failed to remove the local file: {err}"))?;
    println!("Transcription finished, file removed");

    transcribed
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
        Ok(result) => Ok(String::from_utf8_lossy(&result.stdout).to_string()),
        Err(e) => Err(format!("Failed to execute the transcriber: {e}")),
    }
}
