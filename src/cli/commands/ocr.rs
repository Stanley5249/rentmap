use crate::apis::vision::client::Client;
use crate::cli::error::ApiKeyMissingError;
use crate::config::model::load_config;
use crate::file::load_image;
use crate::pretty::ToPrettyString;
use clap::Parser;
use miette::Result;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "ocr")]
#[command(about = "Perform OCR (text detection) on an image using Google Vision API")]
pub struct Args {
    /// Image file path
    path: PathBuf,

    /// Language hints (e.g., zh-Hant, en, ja)
    #[arg(short, long, value_delimiter = ',', num_args = 1..)]
    languages: Option<Vec<String>>,
}

pub async fn run(args: Args) -> Result<()> {
    let config = load_config();

    let image_bytes = load_image(&args.path)?;

    let api_key = config.and_then(|c| c.api_key).ok_or(ApiKeyMissingError)?;

    let vision_client = Client::new(api_key).await?;

    let detected_text = vision_client
        .text_detection_single(image_bytes, args.languages)
        .await?;

    println!("{}", detected_text.to_pretty_string());

    Ok(())
}
