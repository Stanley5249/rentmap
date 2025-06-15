use crate::cli::fetcher::{FetcherArgs, setup_fetcher};
use crate::file::make_directory;
use crate::file::save_json;
use crate::sites::rent591::pipelines::scrape_rent_lists;
use clap::Parser;
use miette::Result;
use tracing::{error, info};
use url::Url;

#[derive(Debug, Parser)]
pub struct Args {
    /// URL of rent591 search results page
    pub url: Url,

    /// JSON output filename
    #[arg(long = "out-file", short = 'f', default_value = "rent591_lists.json")]
    pub out_file: String,

    /// Maximum pages to scrape
    #[arg(long, short)]
    pub limit: Option<u32>,

    #[clap(flatten)]
    pub fetcher: FetcherArgs,
}

pub async fn run(args: Args) -> Result<()> {
    info!("starting rent-lists CLI tool");

    make_directory(&args.fetcher.out_dir).inspect_err(|error| error!(%error))?;

    let fetcher = setup_fetcher(&args.fetcher);

    let rent_lists = scrape_rent_lists(&fetcher, args.url.clone(), args.limit).await;

    save_json(&rent_lists, &args.out_file, &args.fetcher.out_dir)
        .inspect_err(|error| error!(%error))?;

    info!("rent-lists completed successfully");

    Ok(())
}
