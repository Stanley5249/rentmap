use std::str::FromStr;
use std::sync::LazyLock;

use ego_tree::NodeRef;
use scraper::node::{Comment, Element};
use scraper::{ElementRef, Html, Node, Selector};
use url::Url;

#[macro_export]
macro_rules! define_selectors {
    ($struct_name:ident, $($field:ident: $selector:literal),* $(,)?) => {
        struct $struct_name {
            $(pub $field: ::scraper::Selector,)*
        }
        impl $struct_name {
            pub fn new() -> Self {
                Self {
                    $($field: ::scraper::Selector::parse($selector).unwrap(),)*
                }
            }
        }
    };
}

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

enum Layout {
    Block,
    Inline,
    Br,
    Skip,
}

impl Layout {
    fn new(tag: &str) -> Layout {
        match tag {
            "br" => Layout::Br,

            "address" | "article" | "aside" | "blockquote" | "details" | "dialog" | "dd"
            | "div" | "dl" | "dt" | "fieldset" | "figcaption" | "figure" | "footer" | "form"
            | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "header" | "hgroup" | "hr" | "li"
            | "main" | "nav" | "ol" | "p" | "pre" | "section" | "table" | "ul" => Layout::Block,

            "a" | "abbr" | "b" | "bdi" | "bdo" | "cite" | "code" | "data" | "dfn" | "em" | "i"
            | "kbd" | "mark" | "q" | "s" | "samp" | "small" | "span" | "strong" | "sub" | "sup"
            | "time" | "u" | "var" => Layout::Inline,

            "script" | "style" | "meta" | "link" | "title" | "head" => Layout::Skip,

            _ => Layout::Inline,
        }
    }
}

enum Edge<'a> {
    Enter(NodeRef<'a, Node>),
    Exit(&'a Element),
}

fn extract_text_with_layout(element: ElementRef) -> String {
    let mut result = String::new();
    let mut tree: Vec<_> = element.children().map(Edge::Enter).rev().collect();

    while let Some(edge) = tree.pop() {
        match edge {
            Edge::Enter(current) => match current.value() {
                Node::Text(text) => result.push_str(text),

                Node::Element(element) => {
                    match Layout::new(element.name()) {
                        Layout::Block => {
                            new_line_soft(&mut result);
                        }
                        Layout::Inline => {}
                        Layout::Br => {
                            new_line_hard(&mut result);
                            continue;
                        }
                        Layout::Skip => continue,
                    }

                    tree.push(Edge::Exit(element));
                    tree.extend(current.children().map(Edge::Enter).rev());
                }

                _ => {}
            },
            Edge::Exit(element) => match Layout::new(element.name()) {
                Layout::Block => new_line_soft(&mut result),
                Layout::Inline => {}
                Layout::Br | Layout::Skip => unreachable!(),
            },
        }
    }

    if result.ends_with('\n') {
        result.pop();
    }

    result
}

fn new_line_hard(result: &mut String) {
    result.push('\n');
}

fn new_line_soft(result: &mut String) {
    if !result.is_empty() && !result.ends_with('\n') {
        result.push('\n');
    }
}

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

#[cfg(test)]
mod tests {
    use scraper::Html;

    use super::*;

    #[track_caller]
    fn assert_extract_text(html: &str, expected: &str) {
        let document = Html::parse_fragment(html);
        let root = document.root_element();
        let result = extract_text_with_layout(root);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_skip_elements() {
        assert_extract_text("<script>alert('test')</script>", "");
        assert_extract_text("<style>body{color:red}</style>", "");
        assert_extract_text("<meta charset='utf-8'>", "");
        assert_extract_text("<title>Page Title</title>", "");
        assert_extract_text("<head><title>Test</title></head>", "");
        assert_extract_text("<link rel='stylesheet' href='style.css'>", "");
    }

    #[test]
    fn test_block_elements() {
        assert_extract_text("<div>text</div>", "text");
        assert_extract_text("<p>paragraph</p>", "paragraph");
        assert_extract_text("<h1>heading</h1>", "heading");
        assert_extract_text("<div>first</div><div>second</div>", "first\nsecond");
    }

    #[test]
    fn test_inline_elements() {
        assert_extract_text("<span>text</span>", "text");
        assert_extract_text("<strong>bold</strong>", "bold");
        assert_extract_text("<em>italic</em>", "italic");
        assert_extract_text("<span>first</span><span>second</span>", "firstsecond");
    }

    #[test]
    fn test_br_elements() {
        assert_extract_text("<br>", "");
        assert_extract_text("text<br>more", "text\nmore");
        assert_extract_text("line1<br><br>line2", "line1\n\nline2");
    }

    #[test]
    fn test_whitespace_preservation() {
        assert_extract_text("hello world", "hello world");
        assert_extract_text("hello   world", "hello   world");
        assert_extract_text("hello\tworld", "hello\tworld");
    }

    #[test]
    fn test_mixed_elements() {
        assert_extract_text(
            "<div>Block <span>inline</span> text</div>",
            "Block inline text",
        );
        assert_extract_text(
            "<p>Para</p><script>skip</script><p>Graph</p>",
            "Para\nGraph",
        );
        assert_extract_text("<div>Text<br>New line</div>", "Text\nNew line");
    }
}
