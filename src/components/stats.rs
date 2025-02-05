use dioxus::prelude::*;

#[component]
pub fn Stats() -> Element {
    rsx! {
        div {
            div {
                span { "Total Drinks: " }
                        // span { "{total_drinks}"}
            }
            div {
                span { "Estimated BAC: " }
                span { " " }
            }
        }
    }
}
