use std::path::PathBuf;

use clap::Parser;
use colored::Colorize;
use miette::Result;
use tracing::debug;

use crate::apis::vision::client::Client;
use crate::apis::vision::model::OcrString;
use crate::config::google::GoogleConfig;
use crate::config::model::{Config, load_config};
use crate::config::ocr::OcrConfig;
use crate::file::load_image;
use crate::pretty::ToPrettyString;

/// Extract text from images using Google Vision API
#[derive(Debug, Parser)]
#[command(name = "ocr")]
pub struct Args {
    /// Image file path
    pub path: PathBuf,

    #[clap(flatten)]
    pub google: GoogleConfig,

    #[clap(flatten)]
    pub config: OcrConfig,
}

fn merge_args(mut args: Args, config: Config) -> Args {
    if let Some(google_config) = config.google {
        args.google.api_key = args.google.api_key.or(google_config.api_key);
    }

    if let Some(ocr) = config.ocr {
        args.config.languages = args.config.languages.or(ocr.languages);
    }

    args
}

fn format_ocr_result(detected_text: &OcrString) -> String {
    if detected_text.is_empty() {
        "No text detected".red().to_string()
    } else {
        let title = "Detected Text:".bold().underline();
        let table = detected_text.to_pretty_string();
        format!("{title}\n{table}")
    }
}

#[tracing::instrument(skip_all)]
pub async fn run(args: Args) -> Result<()> {
    let args = match load_config() {
        Some(config) => merge_args(args, config),
        None => args,
    };
    debug!(?args);

    let image_bytes = load_image(&args.path)?;

    let api_key = args.google.get_api_key()?;

    let vision_client = Client::new(api_key).await?;

    let detected_text = vision_client
        .text_detection_single(image_bytes, args.config.languages)
        .await?;

    println!("\n{}", format_ocr_result(&detected_text));

    Ok(())
}
