use crate::components::{
    Drinks, DynamicBackground, SliderButton, Stats, TonightAchievements, WittyMessage,
};
use dioxus::prelude::*;

#[component]
pub fn Tonight() -> Element {
    rsx! {
        div { class: "tonight-view-container",
            DynamicBackground {},
            h1 { class: "view-header", "Booze Buddy" },
            // h2 { class: "app-subtitle" },
            WittyMessage {},
            Drinks {},
            Stats {},
            TonightAchievements {},
            SliderButton {},
        }
    }
}
