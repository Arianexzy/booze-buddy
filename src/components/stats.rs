use dioxus::prelude::*;

const STATS_CSS: Asset = asset!("assets/styling/stats.css");

#[derive(PartialEq, Clone, Props)]
pub struct StatsProps {
    pub total_drinks_resource: Resource<i32>,
    pub bac_resource: Resource<f32>,
}

#[component]
pub fn Stats(props: StatsProps) -> Element {
    let total_drinks = match &*props.total_drinks_resource.read_unchecked() {
        Some(count) => *count,
        None => 0,
    };

    let bac_display = match &*props.bac_resource.read_unchecked() {
        Some(bac) => format!("{:.3}", bac),
        None => "N/A".to_string(),
    };

    rsx! {
        document::Link { rel: "stylesheet", href: STATS_CSS }
        div { class: "stats-container",
            div { class: "drink-stats",
                div { class: "total-drinks",
                    span { "Total Drinks: " }
                    span { "{total_drinks}" }
                }
                div { class: "bac",
                    span { "Estimated BAC: " }
                    span { "{bac_display}" }
                }
            }
        }
    }
}
