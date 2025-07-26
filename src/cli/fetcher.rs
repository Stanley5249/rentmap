use clap::Args;

use crate::web::{BackendType, Fetcher};
use crate::workspace::Workspace;

#[derive(Debug, Args)]
#[command(next_help_heading = "Fetcher")]
pub struct FetcherArgs {
    /// Don't use cached pages, always fetch fresh
    #[arg(long = "no-cache", action = clap::ArgAction::SetFalse)]
    pub cache: bool,

    /// Don't comment out <script> and <link> tags
    #[arg(long = "no-clean", action = clap::ArgAction::SetFalse)]
    pub clean: bool,

    /// Web scraping backend to use
    #[arg(long = "backend", value_enum, default_value_t = Default::default())]
    pub backend: BackendType,
}

impl FetcherArgs {
    pub fn build(self, workspace: Workspace) -> Fetcher {
        let mut fetcher = Fetcher::new(workspace);
        fetcher.cache = self.cache;
        fetcher.clean = self.clean;
        fetcher = fetcher.with_backend(self.backend);
        fetcher
    }
}
