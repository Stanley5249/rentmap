use clap::Parser;
use serde::Deserialize;

#[derive(Debug, Deserialize, Parser)]
pub struct OcrConfig {
    /// Language hints (e.g., zh-Hant, en, ja)
    #[arg(short, long, value_delimiter = ',', num_args = 1..)]
    pub languages: Option<Vec<String>>,
}
