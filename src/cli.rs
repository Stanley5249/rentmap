use clap::Parser;
use google_maps::prelude::*;

#[derive(Parser)]
#[command(name = "geocoding")]
#[command(about = "Geocode addresses using Google Maps API")]
pub struct Args {
    pub query: String,
    #[arg(long, env = "GOOGLE_MAPS_API_KEY")]
    pub api_key: Option<String>,
    #[arg(short, long)]
    pub language: Option<Language>,
    #[arg(short, long)]
    pub region: Option<Region>,
}
