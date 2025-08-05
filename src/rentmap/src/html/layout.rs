use ego_tree::NodeRef;
use scraper::node::Element;
use scraper::{ElementRef, Node};

pub enum Layout {
    Block,
    Inline,
    Br,
    Skip,
}

impl Layout {
    pub fn new(tag: &str) -> Layout {
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

pub fn extract_text_with_layout(element: ElementRef) -> String {
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

#[cfg(test)]
mod tests {
    use super::extract_text_with_layout;
    use scraper::Html;

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
