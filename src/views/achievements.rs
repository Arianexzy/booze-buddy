use dioxus::prelude::*;

#[component]
pub fn Achievements() -> Element {
    rsx! {
        h1 { class: "view-header", "Achievements" }
    }
}
