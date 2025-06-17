use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct OcrString(String);

impl OcrString {
    pub fn new(value: String) -> Self {
        Self(value)
    }
}

impl From<String> for OcrString {
    fn from(value: String) -> Self {
        OcrString(value)
    }
}

impl From<OcrString> for String {
    fn from(val: OcrString) -> Self {
        val.0
    }
}

impl Deref for OcrString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
