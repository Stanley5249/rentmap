use dioxus::prelude::*;

#[component]
pub fn MapIcon(#[props(optional)] class: String) -> Element {
    rsx! {
        svg { class, fill: "currentColor", view_box: "0 0 24 24",
            path { d: "M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5z" }
        }
    }
}

#[component]
pub fn SpinnerIcon(#[props(optional)] class: String) -> Element {
    rsx! {
        svg { class, fill: "none", view_box: "0 0 24 24",
            circle {
                class: "opacity-25",
                cx: "12",
                cy: "12",
                r: "10",
                stroke: "currentColor",
                stroke_width: "4",
            }
            path {
                class: "opacity-75",
                fill: "currentColor",
                d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z",
            }
        }
    }
}

#[component]
pub fn RefreshIcon(#[props(optional)] class: String) -> Element {
    rsx! {
        svg {
            class,
            fill: "none",
            stroke: "currentColor",
            view_box: "0 0 24 24",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                d: "M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15",
            }
        }
    }
}
