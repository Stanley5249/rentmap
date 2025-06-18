use url::Url;

#[derive(Debug, Clone)]
pub struct Page {
    pub url_final: Url,
    pub html: String,
}

impl From<&spider::page::Page> for Page {
    fn from(page: &spider::page::Page) -> Self {
        Page {
            url_final: page.get_url_final().parse().unwrap(),
            html: page.get_html(),
        }
    }
}
