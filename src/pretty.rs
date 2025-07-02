use std::time::Duration;

use colored::Colorize;
use comfy_table::{Cell, ContentArrangement, Table, presets};
use google_maps::prelude::{GeocodingResponse, LatLng};

use crate::apis::vision::model::OcrString;

/// Trait for types that can be pretty-printed to a String.
pub trait ToPrettyString {
    /// Renders the object to a String.
    fn to_pretty_string(&self) -> String;
}

impl ToPrettyString for Duration {
    fn to_pretty_string(&self) -> String {
        let s = self.as_secs_f64();
        if s < 1.0 {
            format!("{}ms", self.as_millis())
        } else {
            format!("{s:.1}s")
        }
    }
}

impl ToPrettyString for GeocodingResponse {
    fn to_pretty_string(&self) -> String {
        let mut table = Table::new();

        table
            .load_preset(comfy_table::presets::UTF8_FULL)
            .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("Formatted Address".bold().dimmed()),
                Cell::new("Latitude".bold().dimmed()),
                Cell::new("Longitude".bold().dimmed()),
            ]);

        for result in &self.results {
            let LatLng { lat, lng } = result.geometry.location;
            table.add_row(vec![
                Cell::new(result.formatted_address.white()),
                Cell::new(lat.to_string().bright_cyan()),
                Cell::new(lng.to_string().bright_cyan()),
            ]);
        }

        table.to_string()
    }
}

impl ToPrettyString for OcrString {
    fn to_pretty_string(&self) -> String {
        let mut table = Table::new();

        table
            .load_preset(presets::NOTHING)
            .set_content_arrangement(ContentArrangement::Disabled);

        for (i, line) in self.lines().enumerate() {
            table.add_row(vec![
                Cell::new((i + 1).to_string().bright_blue()),
                Cell::new(line),
            ]);
        }

        table.to_string()
    }
}
