use crate::scraping::fetcher::Fetcher;
use clap::Args;
use std::path::PathBuf;

#[derive(Debug, Args, Clone)]
pub struct FetcherArgs {
    /// Don't save HTML files
    #[arg(long = "no-html", action = clap::ArgAction::SetFalse, default_value_t = true)]
    pub html: bool,

    /// Don't comment out <script> and <link> tags
    #[arg(long = "no-clean", action = clap::ArgAction::SetFalse, default_value_t = true)]
    pub clean: bool,

    /// Directory for output files (used for HTML saving)
    #[arg(long = "out-dir", short = 'o', default_value = "output")]
    pub out_dir: PathBuf,
}

pub fn setup_fetcher(opts: &FetcherArgs) -> Fetcher {
    let mut fetcher = Fetcher::new();
    if opts.html {
        fetcher = fetcher.with_save(&opts.out_dir);
    }
    if opts.clean {
        fetcher = fetcher.with_clean();
    }
    fetcher
}
