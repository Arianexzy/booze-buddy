use dioxus::prelude::*;

#[component]
pub fn History() -> Element {
    rsx! {
        h1 { class: "view-header", "Past Rockin Nights" },
    }
}
