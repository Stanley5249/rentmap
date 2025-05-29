use crate::error::Error;

use google_maps::prelude::*;
use tokio::runtime;

/// Represents a geocoding request with all required parameters
pub struct GeocodingRequest {
    pub query: String,
    pub api_key: String,
    pub language: Option<Language>,
    pub region: Option<Region>,
}

pub fn run_geocoding(request: GeocodingRequest) -> Result<GeocodingResponse, Error> {
    let client = Client::try_new(request.api_key)?;

    let mut geocoding_builder = client.geocoding().with_address(&request.query);

    if let Some(language) = request.language {
        geocoding_builder = geocoding_builder.with_language(language);
    }

    if let Some(region) = request.region {
        geocoding_builder = geocoding_builder.with_region(region);
    }
    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    let response = rt.block_on(geocoding_builder.execute())?;

    Ok(response)
}
