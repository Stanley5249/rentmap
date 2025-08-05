//! Serve command implementation - starts the Leptos web server

use axum::Router;
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};
use miette::Result;
use tokio::signal::ctrl_c;
use tracing::{debug, info};

use super::error::ServerError;

pub async fn run() -> Result<()> {
    // Get Leptos configuration - this reads from Cargo.toml metadata
    let args = get_configuration(None)
        .map_err(|source| ServerError::LeptosConfig { source })?
        .leptos_options;

    info!(addr = %format!("http://{}", &args.site_addr), "starting leptos web server");

    // Generate the list of routes in the Leptos App
    let routes = generate_route_list(app::App);

    let app = Router::new()
        .leptos_routes(&args, routes, {
            let args = args.clone();
            move || app::shell(args.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(app::shell))
        .with_state(args.clone());

    // Set up graceful shutdown
    let signal = async {
        ctrl_c().await.expect("failed to listen for Ctrl+C");
        debug!("received shutdown signal");
    };

    let listener = tokio::net::TcpListener::bind(&args.site_addr)
        .await
        .map_err(|source| ServerError::Bind {
            source,
            addr: args.site_addr,
        })?;

    // Start the server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(signal)
        .await
        .map_err(|source| ServerError::Serve { source })?;

    info!("leptos server stopped");

    Ok(())
}
