// Core library modules
pub mod config;
pub mod error;
pub mod geocoding;

// Display functionality
pub mod pretty;

// CLI-specific modules (only public for the binary)
pub mod cli;

// Core library public API
pub use config::{Config, find_config_file};
pub use error::Error;
pub use geocoding::{GeocodingRequest, run_geocoding};

// Display public API
pub use pretty::PrettyPrintable;

// CLI-specific exports (for the binary)
pub use cli::{Args, run_cli};
