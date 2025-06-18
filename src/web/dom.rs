use scraper::node::Comment;
use scraper::{Html, Node, Selector};

use crate::selectors;

selectors! {
    SCRIPT_SELECTOR: "link[as=\"script\"], script",
}

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

pub fn clean_html(document: &mut Html) {
    let script_selector = &*SCRIPT_SELECTOR;
    hide_elements(document, script_selector);
}
