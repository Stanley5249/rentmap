//! Serve command implementation - starts the Leptos web server

use axum::Router;
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};
use miette::Result;
use tokio::net::TcpListener;
use tokio::signal::ctrl_c;
use tracing::{debug, info};

use super::error::ServerError;

async fn serve() -> Result<(), ServerError> {
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
            move || app::shell(args.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(app::shell))
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

pub async fn run() -> Result<()> {
    Ok(serve().await?)
}
