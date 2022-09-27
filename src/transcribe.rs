use std::process::Command as OsCommand;
use teloxide::types::{self, File as TelegramFile};
use teloxide::{net::Download, prelude::*};

use tokio::fs::{read_to_string, File};

pub async fn download_and_transcribe(
    bot: &AutoSend<Bot>,
    voice: &types::Voice,
) -> Result<String, String> {
    let TelegramFile { meta, file_path } = bot.get_file(&voice.file_id).send().await.unwrap();
    let local_filename = meta.file_unique_id.as_str();

    if let Ok(_) = File::open(format!("{local_filename}.vtt")).await {
        return Ok(read_to_string(format!("{local_filename}.vtt"))
            .await
            .unwrap());
    }

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
