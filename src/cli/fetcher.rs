use clap::Args;

use crate::file::Workspace;
use crate::web::Fetcher;

#[derive(Debug, Args)]
#[command(next_help_heading = "Fetcher")]
pub struct FetcherArgs {
    /// Don't use cached pages, always fetch fresh
    #[arg(long = "no-cache", action = clap::ArgAction::SetFalse)]
    pub cache: bool,

    /// Don't comment out <script> and <link> tags
    #[arg(long = "no-clean", action = clap::ArgAction::SetFalse)]
    pub clean: bool,
}

impl FetcherArgs {
    pub fn build(self, workspace: Workspace) -> Fetcher {
        let mut fetcher = Fetcher::new(workspace);

        if self.cache {
            fetcher = fetcher.with_cache();
        }

        if self.clean {
            fetcher = fetcher.with_clean();
        }

        fetcher
    }
}
