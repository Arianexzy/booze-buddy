use dioxus::prelude::*;
use crate::components::drinks::DRINK_TYPES;

const STATS_CSS: Asset = asset!("assets/styling/stats.css");

#[component]
pub fn Stats() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: STATS_CSS }
        
        div { class: "stats-container",
            div { class: "drink-counts-container",
                {
                    DRINK_TYPES.iter().map(|drink|{
                        rsx! {
                            div { class: "drink-count", key: "{drink.label}",
                                div { class: "drink-type",
                                    span { class: "drink-count-icon", "{drink.emoji}" }
                                    span { class: "drink-count-label","{drink.label}" }
                                }
                                span { class: "drink-count-value", "Drink Count" }
                            }
                        }
                    })
                }
            },
            div { class: "drink-stats",
                div { class: "total-drinks",
                    span { "Total Drinks: " },
                    span { "10"}
                }
                div { class: "bac",
                    span { "Estimated BAC: " },
                    span { "0.08" }
                }
            }
        }
    }
}
