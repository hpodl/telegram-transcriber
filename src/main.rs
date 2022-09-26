use teloxide::{prelude::*, utils::command::BotCommands};

use std::error::Error;

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

#[inline]
async fn transcribe_handler(
    bot: &AutoSend<Bot>,
    message: &Message,
) -> Result<Message, teloxide::RequestError> {
    Ok(match message.reply_to_message() {
        Some(replied) => {
            bot.send_message(message.chat.id, "Hello o/")
                .reply_to_message_id(replied.id)
                .await?
        }
        None => {
            bot.send_message(message.chat.id, "No message is being replied to.")
                .reply_to_message_id(message.id)
                .await?
        }
    })
}
