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
            if let Some(node) = self.tree.get(id) {
                // Get child IDs in single traversal
                let child_ids: Vec<_> = node.descendants().skip(1).map(|n| n.id()).collect();

                // Replace element with comment
                if let Some(mut node) = self.tree.get_mut(id) {
                    *node.value() = Node::Comment(Comment {
                        comment: html.into(),
                    });
                }

                // Convert child nodes to empty comments
                for child_id in child_ids {
                    if let Some(mut child_node) = self.tree.get_mut(child_id) {
                        child_node.detach();
                    }
                }
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

#[cfg(test)]
mod tests {
    use super::*;
    use scraper::{Html, Selector};

    #[track_caller]
    fn hide_elements_test(original: &str, expected: &str, selector: &str) {
        let mut html = Html::parse_document(original);
        html.hide_elements(&Selector::parse(selector).unwrap());
        assert_eq!(html.html(), expected);
    }

    #[track_caller]
    fn hide_scripts_test(original: &str, expected: &str) {
        let mut html = Html::parse_document(original);
        html.hide_scripts();
        assert_eq!(html.html(), expected);
    }

    #[test]
    fn hide_elements_no_matches() {
        let original = r#"<html><head></head><body><p>Text</p></body></html>"#;
        hide_elements_test(original, original, "div");
    }

    #[test]
    fn hide_elements_single() {
        hide_elements_test(
            r#"<html><head></head><body><div>content</div><p>keep</p></body></html>"#,
            r#"<html><head></head><body><!--<div>content</div>--><p>keep</p></body></html>"#,
            "div",
        );
    }

    #[test]
    fn hide_elements_nested() {
        hide_elements_test(
            r#"<html><head></head><body><div><span>inner</span><p>nested</p></div></body></html>"#,
            r#"<html><head></head><body><!--<div><span>inner</span><p>nested</p></div>--></body></html>"#,
            "div",
        );
    }

    #[test]
    fn hide_elements_multiple() {
        hide_elements_test(
            r#"<html><head></head><body><div>first</div><p>keep</p><div>second</div></body></html>"#,
            r#"<html><head></head><body><!--<div>first</div>--><p>keep</p><!--<div>second</div>--></body></html>"#,
            "div",
        );
    }

    #[test]
    fn hide_elements_complex_selector() {
        hide_elements_test(
            r#"<html><head></head><body><div class="hide">hide me</div><div class="keep">keep me</div></body></html>"#,
            r#"<html><head></head><body><!--<div class="hide">hide me</div>--><div class="keep">keep me</div></body></html>"#,
            "div.hide",
        );
    }

    #[test]
    fn hide_scripts_no_scripts() {
        let original =
            r#"<html><head><title>Test</title></head><body><div>Content</div></body></html>"#;
        hide_scripts_test(original, original);
    }

    #[test]
    fn hide_scripts_inline_and_external() {
        hide_scripts_test(
            r#"<html><head><script>console.log('test');</script><script src="app.js"></script></head><body>Content</body></html>"#,
            r#"<html><head><!--<script>console.log('test');</script>--><!--<script src="app.js"></script>--></head><body>Content</body></html>"#,
        );
    }

    #[test]
    fn hide_scripts_link_preload() {
        hide_scripts_test(
            r#"<html><head><link as="script" href="module.js" rel="preload"></head></html>"#,
            r#"<html><head><!--<link as="script" href="module.js" rel="preload">--></head><body></body></html>"#,
        );
    }

    #[test]
    fn hide_scripts_preserves_styles() {
        hide_scripts_test(
            r#"<html><head><link rel="stylesheet" href="style.css"><link as="script" href="app.js"></head></html>"#,
            r#"<html><head><link href="style.css" rel="stylesheet"><!--<link as="script" href="app.js">--></head><body></body></html>"#,
        );
    }
}
