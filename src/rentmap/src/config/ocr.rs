use clap::Args;
use serde::Deserialize;

#[derive(Debug, Deserialize, Args)]
#[command(next_help_heading = "OCR")]
pub struct OcrConfig {
    /// Language hints for text detection (e.g., zh-Hant, en, ja)
    ///
    /// See: https://cloud.google.com/vision/docs/languages
    #[arg(short, long, value_delimiter = ',', num_args = 1..)]
    pub languages: Option<Vec<String>>,
}
