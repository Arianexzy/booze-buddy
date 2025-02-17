use dioxus::prelude::*;

const STATS_CSS: Asset = asset!("assets/styling/stats.css");

#[component]
pub fn Stats() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: STATS_CSS }
        div { class: "stats-container",
            div { class: "drink-stats",
                div { class: "total-drinks",
                    span { "Total Drinks: " }
                    span { "10" }
                }
                div { class: "bac",
                    span { "Estimated BAC: " }
                    span { "0.08" }
                }
            }
        }
    }
}
