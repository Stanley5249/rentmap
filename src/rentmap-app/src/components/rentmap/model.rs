#[derive(Debug, Clone, PartialEq)]
pub struct MarkerData {
    pub lat: f64,
    pub lng: f64,
    pub title: String,
}

impl MarkerData {
    pub fn new(lat: f64, lng: f64, title: impl Into<String>) -> Self {
        Self {
            lat,
            lng,
            title: title.into(),
        }
    }

    pub fn format_coordinates(&self, precision: usize) -> String {
        format!("{:.prec$}, {:.prec$}", self.lat, self.lng, prec = precision)
    }
}
