use std::sync::LazyLock;

use scraper::node::Comment;
use scraper::{Html, Node, Selector};

use crate::define_selectors;

define_selectors! {
    DomSelectors,
    script: "link[as=\"script\"], script"
}

static DOM_SELECTORS: LazyLock<DomSelectors> = LazyLock::new(DomSelectors::new);

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
    let script_selector = &DOM_SELECTORS.script;
    hide_elements(document, script_selector);
}
