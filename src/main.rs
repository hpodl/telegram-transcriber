use std::error::Error;
use teloxide::{prelude::*, utils::command::BotCommands};
use transcribe::*;

//TODO consider logging instead of stderr prints

mod transcribe;

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
    // string parameter specifies which model to use
    Transcribe(String),
    #[command(
        description = "placeholder for testing functionality.",
        parse_with = "split"
    )]
    Test(String),
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
        Command::Transcribe(model) => transcribe_handler(&bot, &message, &model).await?,
        Command::Test(param) => test_handler(&bot, &message, &param).await?,
    };

    Ok(())
}

/// Handles transcription requests
///
/// Request message must be a reply to audio to be transcribed
async fn transcribe_handler(
    bot: &AutoSend<Bot>,
    message: &Message,
    model: &str,
) -> Result<Message, teloxide::RequestError> {
    let model = if !is_valid_model(model) {
        if !model.is_empty() {
            bot.send_message(message.chat.id, "Invalid model specified, using 'small'")
                .reply_to_message_id(message.id)
                .await?;
        }
        "small"
    } else {
        model
    };

    Ok(match message.reply_to_message() {
        Some(replied) => match try_transcribe(bot, replied, model).await {
            Ok(transcribed) => {
                bot.send_message(message.chat.id, transcribed)
                    .reply_to_message_id(replied.id)
                    .await?
            }
            Err(e) => bot.send_message(message.chat.id, e).await?,
        },
        None => {
            bot.send_message(message.chat.id, "No message is being replied to.")
                .reply_to_message_id(message.id)
                .await?
        }
    })
}

async fn test_handler(
    bot: &AutoSend<Bot>,
    message: &Message,
    param: &str,
) -> Result<Message, teloxide::RequestError> {
    bot.send_message(message.chat.id, format!("Given: {}", param))
        .await
}
