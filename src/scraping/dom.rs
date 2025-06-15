use scraper::node::Comment;
use scraper::{Html, Node, Selector};
use std::ops::Deref;
use std::sync::LazyLock;

static TITLE_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse("title").unwrap());

static CLEANUP_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("link[as=\"script\"], script").unwrap());

fn hide_elements(document: &mut Html, selector: &Selector) {
    let elements: Vec<_> = document
        .select(selector)
        .map(|element| (element.id(), element.html()))
        .collect();

    for (id, html) in elements {
        if let Some(mut node) = document.tree.get_mut(id) {
            *node.value() = Node::Comment(Comment {
                comment: html.into(),
            });
        }
    }
}

pub fn extract_title(document: &Html) -> Option<String> {
    document.select(TITLE_SELECTOR.deref()).find_map(|e| {
        let title = e.text().collect::<String>().trim().to_string();
        (!title.is_empty()).then_some(title)
    })
}

pub fn clean_html(document: &mut Html) {
    hide_elements(document, CLEANUP_SELECTOR.deref());
}
