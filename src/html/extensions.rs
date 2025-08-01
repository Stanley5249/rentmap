use scraper::node::Comment;
use scraper::{ElementRef, Html, Node, Selector};
use std::str::FromStr;
use std::sync::LazyLock;
use url::Url;

use crate::define_selectors;
use crate::html::layout::extract_text_with_layout;
use crate::html::text::TextExt;

define_selectors! {
    DomSelectors,
    script: "link[as=\"script\"], script"
}

static SELECTORS: LazyLock<DomSelectors> = LazyLock::new(DomSelectors::new);

pub trait HtmlExt {
    fn hide_elements(&mut self, selector: &Selector);

    fn hide_scripts(&mut self) {
        self.hide_elements(&SELECTORS.script);
    }
}

impl HtmlExt for Html {
    fn hide_elements(&mut self, selector: &Selector) {
        let elements: Vec<_> = self
            .select(selector)
            .map(|element| (element.id(), element.html()))
            .collect();

        for (id, html) in elements {
            if let Some(mut node) = self.tree.get_mut(id) {
                *node.value() = Node::Comment(Comment {
                    comment: html.into(),
                });
            }
        }
    }
}

pub trait ElementExt {
    fn select_text_concat(&self, selector: &Selector) -> impl Iterator<Item = String>;

    fn select_text_join(&self, selector: &Selector, sep: &str) -> impl Iterator<Item = String>;

    fn select_url(&self, selector: &Selector, attr: &str) -> impl Iterator<Item = Url>;

    fn select_from_str<T: FromStr>(&self, selector: &Selector) -> impl Iterator<Item = T>;

    fn select_content(&self, selector: &Selector) -> String;
}

impl ElementExt for ElementRef<'_> {
    fn select_text_concat(&self, selector: &Selector) -> impl Iterator<Item = String> {
        self.select(selector)
            .map(|e| e.text().trimmed_concat())
            .filter(|s| !s.is_empty())
    }

    fn select_text_join(&self, selector: &Selector, sep: &str) -> impl Iterator<Item = String> {
        self.select(selector)
            .map(|e| e.text().trimmed_join(sep))
            .filter(|s| !s.is_empty())
    }

    fn select_url(&self, selector: &Selector, attr: &str) -> impl Iterator<Item = Url> {
        self.select(selector)
            .filter_map(|e| e.attr(attr))
            .filter_map(|s| Url::parse(s).ok())
    }

    fn select_from_str<T>(&self, selector: &Selector) -> impl Iterator<Item = T>
    where
        T: FromStr,
    {
        self.select(selector)
            .filter_map(|e| e.text().trimmed_concat().parse::<T>().ok())
    }

    fn select_content(&self, selector: &Selector) -> String {
        self.select(selector)
            .map(|e| extract_text_with_layout(e))
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}
