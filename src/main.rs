use clap::Parser;
use geocoding::{
    Args, Error, PrettyPrintable, find_config_file, resolve_geocoding_request,
    run_geocoding,
};
use google_maps::prelude::*;

fn main() -> miette::Result<()> {
    let args = Args::parse();

    let config_path = find_config_file();
    println!("{}\n", config_path.to_pretty_string());

    let request = resolve_geocoding_request(args, config_path)?;
    println!("{}\n", request.to_pretty_string());

    let response = match run_geocoding(request) {
        Ok(response) => response,
        // GoogleMapsError did not make diagnostic transparent
        Err(Error::GoogleMaps(GoogleMapsError::Geocoding(e))) => return Err(e.into()),
        Err(e) => return Err(e.into()),
    };
    println!("{}", response.to_pretty_string());

    Ok(())
}
