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

    let (bac_display, bac_class) = match &*props.bac_resource.read_unchecked() {
        Some(bac) => {
            let class = match *bac {
                b if b < 0.02 => "bac-sober",
                b if b < 0.04 => "bac-safe",
                b if b < 0.08 => "bac-buzzed",
                b if b < 0.15 => "bac-tipsy",
                b if b < 0.24 => "bac-drunk",
                b if b < 0.35 => "bac-very-drunk",
                b if b < 0.50 => "bac-danger",
                _ => "bac-extreme",
            };
            (format!("{:.3}", bac), class)
        }
        None => ("N/A".to_string(), "bac-sober"),
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
                    span {
                        class: "bac-value {bac_class}",
                        "{bac_display}"
                    }
                }
            }
        }
    }
}
