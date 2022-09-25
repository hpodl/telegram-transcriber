use teloxide::{
    net::Download,
    prelude::*,
    types::{File as TelegramFile, MediaKind, MessageKind},
};
use tokio::fs::File;

const MAX_DUR: u32 = 60;

#[tokio::main]
async fn main() {
    println!("Starting up the bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        if let MessageKind::Common(kind) = message.kind {
            if let MediaKind::Voice(voice_kind) = kind.media_kind {
                let voice = voice_kind.voice;
                if voice.duration < MAX_DUR {
                    let TelegramFile { meta, file_path } =
                        bot.get_file(voice.file_id).send().await?;
                    let mut local_file = File::create(meta.file_unique_id.to_string()).await?;

                    println!("Downloading voice file...");
                    bot.download_file(&file_path, &mut local_file).await?;
                    println!("Finished downloading voice file...");
                }
            }
        }

        bot.send_dice(message.chat.id).await?;
        respond(())
    })
    .await;
}
