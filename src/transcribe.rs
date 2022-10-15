use std::collections::HashSet;
use std::env;

use teloxide::prelude::*;
use teloxide::types::File as TelegramFile;

use pyo3::prelude::*;
use pyo3::types::PyTuple;

use lazy_static::lazy_static;

use crate::logger::Logger as _;
// type alias for an object implementing crate::logger:Logger trait
use crate::Logger;

lazy_static! {
    static ref AVAILABLE_MODELS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("tiny");
        set.insert("base");
        set.insert("small");
        set.insert("medium");
        set.insert("large");
        set
    };
    static ref TOKEN: String = env::var("TELOXIDE_TOKEN").unwrap();
}

// TODO Refector out the need to pass model parameter through multiple function calls
// TODO Refactor out the workaround of using whisper as a shell command

/// Returns `Ok(transcription)` of audio if `message` contains any
/// even if the transcription happens to be an empty string
///
/// Else returns `Err(error_messsage)`
pub async fn try_transcribe(
    bot: &AutoSend<Bot>,
    message: &Message,
    model: &str,
) -> Result<String, String> {
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
            return Err("Doesn't contain audio media.".to_string());
        }
    };

    transcribe(bot, file_id, model).await
}

/// Downloads and transcribes a message.
///
/// Returns transcription as a String
async fn transcribe(bot: &AutoSend<Bot>, file_id: &str, model: &str) -> Result<String, String> {
    let TelegramFile { file_path, .. } = bot
        .get_file(file_id)
        .send()
        .await
        .map_err(|err| format!("Failed to download the file: {err}"))?;

    let dl_link = &format!(
        "https://api.telegram.org/file/bot{}/{}",
        TOKEN.as_str(),
        file_path
    );

    Logger::log(&format!("Transcribing audio from {}.", dl_link));
    transcribe_file(dl_link, model).await.map(|val| {
        Logger::log(&format!("Finished transcribing {}.", dl_link));
        val
    })
}

async fn transcribe_file(path: &str, model: &str) -> Result<String, String> {
    python_transcribe(path, model)
        .await
        .map_err(|e| format!("{}", e))
}

async fn python_transcribe(path: &str, model: &str) -> Result<String, PyErr> {
    Python::with_gil(|py| -> PyResult<String> {
        let transcribe: Py<PyAny> = PyModule::from_code(
            py,
            "def transcribe(model, path):
                import whisper
                model = whisper.load_model(model)
                return model.transcribe(path)[\"text\"]",
            "",
            "",
        )?
        .getattr("transcribe")?
        .into();

        let args = PyTuple::new(py, &[model, path]);
        Ok(transcribe.call1(py, args)?.to_string())
    })
}

pub fn is_valid_model(model: &str) -> bool {
    AVAILABLE_MODELS.contains(model)
}
