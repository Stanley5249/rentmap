//! Fetch command implementation

use std::net::SocketAddr;

use axum::Router;
use axum::response::Html;
use axum::routing::get;
use clap::Parser;
use miette::Result;
use tokio::net::TcpListener;
use tokio::signal::ctrl_c;
use tracing::{debug, info};
use url::Url;

use super::error::ServerError;
use crate::web::FetcherArgs;
use crate::workspace::WorkspaceArgs;

const PREVIEW_PORT: u16 = 3000;

/// Download and clean HTML pages
#[derive(Debug, Parser)]
pub struct Args {
    /// Target URL to fetch and process
    pub url: Url,

    /// Preview the HTML page in a web browser
    #[arg(short, long)]
    pub preview: bool,

    #[clap(flatten)]
    pub workspace: WorkspaceArgs,

    #[command(flatten)]
    pub fetcher: FetcherArgs,
}

async fn start_preview_server<T>(html_content: String, addr: T) -> Result<()>
where
    T: Into<SocketAddr>,
{
    let addr: SocketAddr = addr.into();

    info!(addr = %format!("http://{addr}"), "start preview server");

    let listener = TcpListener::bind(addr)
        .await
        .map_err(|source| ServerError::Bind { source, addr })?;

    let app = Router::new().route("/", get(async || Html(html_content)));

    let signal = async {
        ctrl_c().await.expect("failed to listen for Ctrl+C");
        debug!("received shutdown signal");
    };

    axum::serve(listener, app)
        .with_graceful_shutdown(signal)
        .await
        .map_err(|source| ServerError::Serve { source })?;

    info!("preview server stopped");

    Ok(())
}

pub async fn run(args: Args) -> Result<()> {
    debug!(?args);

    let workspace = args.workspace.build().await?;

    let fetcher = args.fetcher.build(workspace).await?;

    let html = fetcher.try_fetch(&args.url).await?.html();

    fetcher.shutdown().await;

    if args.preview {
        start_preview_server(html, ([127, 0, 0, 1], PREVIEW_PORT)).await?
    }

    Ok(())
}
