use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("no rental list found in workspace, please run `rentmap list` first")]
    #[diagnostic(
        code(rentmap::item::no_rent_list),
        help("run `rentmap list` to fetch rental list")
    )]
    NoRentList,
}

#[derive(Debug, Error, Diagnostic)]
pub enum ServerError {
    #[error("failed to bind to address {addr}")]
    #[diagnostic(
        code(rentmap::fetch::bind_error),
        help(
            "port {} may already be in use. Try using a different port or stop other services using this port.", addr.port()
        )
    )]
    Bind {
        #[source]
        source: std::io::Error,
        addr: std::net::SocketAddr,
    },

    #[error("server failed during operation")]
    #[diagnostic(
        code(rentmap::fetch::serve_error),
        help(
            "the HTTP server encountered an error while serving requests. This could be due to network issues or resource constraints."
        )
    )]
    Serve {
        #[source]
        source: std::io::Error,
    },
}
