//! Serve helper endpoints for the web UI (e.g., Google Maps key injection)

use clap::Parser;
use miette::Result;

use crate::config::error::NoApiKey;
use crate::config::google::GoogleConfig;
use crate::config::model::{Config, load_config};

/// Simple server exposing config-derived scripts for the web UI
#[derive(Debug, Parser)]
pub struct Args {
    #[clap(flatten)]
    pub google: GoogleConfig,
}

fn merge_args(mut args: Args, config: Config) -> Args {
    if let Some(google_config) = config.google {
        args.google.api_key = args.google.api_key.or(google_config.api_key);
    }
    args
}

pub fn get_args() -> Result<Args, NoApiKey> {
    let args = Args::parse();

    let args = match load_config() {
        Some(config) => merge_args(args, config),
        None => args,
    };

    args.google.validate()?;

    Ok(args)
}
