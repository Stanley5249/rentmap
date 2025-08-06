//! Serve command implementation - starts the Leptos web server

use axum::Router;
use clap::Parser;
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};
use miette::Result;
use tokio::net::TcpListener;
use tokio::signal::ctrl_c;
use tracing::{debug, info};

use super::error::ServerError;
use crate::config::google::GoogleConfig;
use crate::config::model::{Config, load_config};

/// Serve the web application
#[derive(Debug, Default, Parser)]
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

async fn serve(api_key: String) -> Result<(), ServerError> {
    // Get Leptos configuration - this reads from Cargo.toml metadata
    let args = get_configuration(None)?.leptos_options;
    let addr = args.site_addr;

    info!(addr = %format!("http://{}", addr), "starting leptos web server");

    let listener = TcpListener::bind(addr)
        .await
        .map_err(|source| ServerError::Bind { source, addr })?;

    // Generate the list of routes in the Leptos App
    let routes = generate_route_list(app::App);

    let app = Router::new()
        .leptos_routes(&args, routes, {
            let args = args.clone();
            let api_key = api_key.clone();
            move || app::shell(args.clone(), api_key.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(move |options| {
            app::shell(options, api_key.clone())
        }))
        .with_state(args);

    let signal = async {
        ctrl_c().await.expect("failed to listen for Ctrl+C");
        debug!("received shutdown signal");
    };

    axum::serve(listener, app)
        .with_graceful_shutdown(signal)
        .await
        .map_err(ServerError::Serve)?;

    info!("leptos server stopped");

    Ok(())
}

pub async fn run(args: Args) -> Result<()> {
    let args = match load_config() {
        Some(config) => merge_args(args, config),
        None => args,
    };
    debug!(?args);

    let api_key = args.google.get_api_key()?;

    Ok(serve(api_key).await?)
}
