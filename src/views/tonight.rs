use crate::components::{DrinkTypes, Stats};
use dioxus::prelude::*;

#[component]
pub fn Tonight() -> Element {
    rsx! {
        h1 { class: "app-title", "Booze Buddy" }
        DrinkTypes {}
        Stats {}
    }
}
