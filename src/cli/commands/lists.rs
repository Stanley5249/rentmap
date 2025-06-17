use crate::cli::fetcher::{FetcherArgs, setup_fetcher};
use crate::file::make_directory;
use crate::file::save_json;
use crate::sites::rent591::pipelines::scrape_rent_lists;
use clap::Parser;
use miette::Result;
use tracing::{debug, error};
use url::Url;

#[derive(Debug, Parser)]
pub struct Args {
    /// Target URL for rent.591.com.tw search results
    pub url: Url,

    /// Maximum pages to scrape
    #[arg(long, short)]
    pub limit: Option<u32>,

    /// Output JSON filename
    #[arg(long = "out-file", short = 'f', default_value = "rent591_lists.json")]
    pub out_file: String,

    #[clap(flatten)]
    pub fetcher: FetcherArgs,
}

#[tracing::instrument(skip_all)]
pub async fn run(args: Args) -> Result<()> {
    debug!(?args);

    make_directory(&args.fetcher.out_dir).inspect_err(|error| error!(%error))?;

    let fetcher = setup_fetcher(&args.fetcher);

    let rent_lists = scrape_rent_lists(&fetcher, args.url, args.limit).await;

    save_json(&rent_lists, &args.out_file, &args.fetcher.out_dir)
        .inspect_err(|error| error!(%error))?;

    Ok(())
}
