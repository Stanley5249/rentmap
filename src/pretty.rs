use crate::geocoding::GeocodingRequest;
use colored::Colorize;
use comfy_table::{Cell, Table};
use google_maps::prelude::{GeocodingResponse, LatLng};
use std::path::PathBuf;

/// Trait for types that can be pretty-printed to a String.
pub trait PrettyPrintable {
    /// Renders the object to a String.
    fn to_pretty_string(&self) -> String;
}

impl PrettyPrintable for GeocodingRequest {
    fn to_pretty_string(&self) -> String {
        let mut table = Table::new();
        table
            .load_preset(comfy_table::presets::NOTHING)
            .set_content_arrangement(comfy_table::ContentArrangement::Disabled);

        table.add_row(vec![
            Cell::new("Query".dimmed()),
            Cell::new(self.query.bright_cyan()),
        ]);

        table.add_row(vec![
            Cell::new("Language".dimmed()),
            Cell::new(match self.language {
                Some(lang) => lang.display().bright_cyan(),
                None => "default".dimmed(),
            }),
        ]);

        table.add_row(vec![
            Cell::new("Region".dimmed()),
            Cell::new(match self.region {
                Some(reg) => reg.display().bright_cyan(),
                None => "default".dimmed(),
            }),
        ]);

        format!("{}\n{}", "Request:".bold().underline(), table)
    }
}

impl PrettyPrintable for GeocodingResponse {
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

        let summary = match self.results.len() {
            0 => "No locations found.".red(), // unreachable
            1 => "Found 1 location.".bright_green(),
            n => format!("Found {} locations.", n).bright_green(),
        };

        format!("{}\n{}\n{}", "Response:".bold().underline(), table, summary)
    }
}

impl PrettyPrintable for Option<PathBuf> {
    fn to_pretty_string(&self) -> String {
        format!(
            "{}\n {}",
            "Config file:".bold().underline(),
            match self {
                Some(path) => path.display().to_string().bright_cyan(),
                None => "None".dimmed(),
            }
        )
    }
}
