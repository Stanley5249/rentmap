pub trait TextExt<'a>: Iterator<Item = &'a str>
where
    Self: Sized,
{
    fn trimmed(self) -> impl Iterator<Item = &'a str> {
        self.map(|s: &'a str| s.trim())
    }

    fn non_empty(self) -> impl Iterator<Item = &'a str> {
        self.filter(|s| !s.is_empty())
    }

    fn map_to_string(self) -> impl Iterator<Item = String> {
        self.map(|s| s.to_string())
    }

    fn trimmed_concat(self) -> String {
        self.trimmed().collect::<String>()
    }

    fn trimmed_join(self, sep: &str) -> String {
        self.trimmed().non_empty().collect::<Vec<_>>().join(sep)
    }
}

impl<'a, I> TextExt<'a> for I where I: Iterator<Item = &'a str> {}
