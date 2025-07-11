use clap::Args;

use crate::file::Workspace;
use crate::web::Fetcher;

#[derive(Debug, Args)]
#[command(next_help_heading = "Fetcher")]
pub struct FetcherArgs {
    /// Don't save HTML files
    #[arg(long = "no-html", action = clap::ArgAction::SetFalse, default_value_t = true)]
    pub html: bool,

    /// Don't comment out <script> and <link> tags
    #[arg(long = "no-clean", action = clap::ArgAction::SetFalse, default_value_t = true)]
    pub clean: bool,
}

pub fn setup_fetcher(args: &FetcherArgs, workspace: Workspace) -> Fetcher {
    let mut fetcher = Fetcher::new();
    if args.html {
        fetcher = fetcher.with_workspace(workspace);
    }
    if args.clean {
        fetcher = fetcher.with_clean();
    }
    fetcher
}
