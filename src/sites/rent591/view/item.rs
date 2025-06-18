use scraper::Html;

use crate::sites::rent591::model::RentItem;

pub struct ItemView {
    pub document: Html,
}

impl ItemView {
    pub fn new(document: Html) -> Self {
        Self { document }
    }

    pub fn extract_rent_item(&self) -> RentItem {
        todo!();
    }
}

// Standard From trait - your brilliant design pattern!
impl From<Html> for ItemView {
    fn from(document: Html) -> Self {
        Self::new(document)
    }
}
