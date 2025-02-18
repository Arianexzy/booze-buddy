use dioxus::prelude::*;

#[component]
pub fn DrinkingHistoryView() -> Element {
    rsx! {
        h1 { class: "view-header", "Past Rockin Nights" }
    }
}
