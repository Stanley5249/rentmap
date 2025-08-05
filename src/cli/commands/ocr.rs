use std::path::PathBuf;

use clap::Parser;
use miette::Result;
use tracing::debug;

use crate::apis::vision::Client;
use crate::config::google::GoogleConfig;
use crate::config::model::{Config, load_config};
use crate::config::ocr::OcrConfig;
use crate::file::load_image;
use crate::pretty::pretty_print_ocr_text;

/// Extract text from images using Google Vision API
#[derive(Debug, Parser)]
#[command(name = "ocr")]
pub struct Args {
    /// Image file path
    pub path: PathBuf,

    #[clap(flatten)]
    pub google: GoogleConfig,

    #[clap(flatten)]
    pub ocr: OcrConfig,
}

fn merge_args(mut args: Args, config: Config) -> Args {
    if let Some(google_config) = config.google {
        args.google.api_key = args.google.api_key.or(google_config.api_key);
    }

    if let Some(ocr) = config.ocr {
        args.ocr.languages_hints = args.ocr.languages_hints.or(ocr.languages_hints);
    }

    args
}

pub async fn run(args: Args) -> Result<()> {
    let args = match load_config() {
        Some(config) => merge_args(args, config),
        None => args,
    };
    debug!(?args);

    let image_bytes = load_image(&args.path)?;

    let api_key = args.google.get_api_key()?;

    let mut client = Client::from_api_key(api_key).await?;
    client.language_hints = args.ocr.languages_hints;

    let text = client.text_detection_single(image_bytes).await?;

    pretty_print_ocr_text(&text);

    Ok(())
}
