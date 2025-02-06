use dioxus::prelude::*;

#[component]
pub fn Stats() -> Element {
    rsx! {
        div {
            // Div for the Drink counts
            div {
                // map out each drink for their own div
                div {
                    // div for each drink name
                    div {
                        span { "Drink name" }
                    }
                    span { "Drink Count" }
                }
            }
            // Div for the total count and BAC
            div {
                div {
                    span { "Total Drinks: " }
                                // span { "{total_drinks}"}
                }
                div {
                    span { "Estimated BAC: " }
                                // span { "{bac}" }
                }
            }
        }
    }
}
