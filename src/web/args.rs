use clap::Args;

use super::{BackendType, Fetcher, SpiderChromeArgs, SpiderChromeBackend, WebError};
use crate::web::Backend;
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

    #[command(flatten)]
    pub spider_chrome: SpiderChromeArgs,
}

impl FetcherArgs {
    pub async fn build(self, workspace: Workspace) -> Result<Fetcher, WebError> {
        let backend: Backend = match self.backend {
            BackendType::SpiderChrome => SpiderChromeBackend::new(self.spider_chrome.try_into()?)
                .await?
                .into(),
        };

        let mut fetcher = Fetcher::new(workspace, backend);
        fetcher.cache = self.cache;
        fetcher.clean = self.clean;

        Ok(fetcher)
    }
}
