use std::ops::Deref;

pub struct OcrString(String);

impl From<String> for OcrString {
    fn from(value: String) -> Self {
        OcrString(value)
    }
}

impl Deref for OcrString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
