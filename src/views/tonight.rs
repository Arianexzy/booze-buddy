use crate::{
    components::{
        Drinks, DynamicBackground, EndNightButton, Stats, TonightAchievements, WittyMessage,
    },
    storage::storage::drink_history::{get_current_bac, get_total_drinks},
};
use dioxus::prelude::*;

#[component]
pub fn Tonight() -> Element {
    let mut total_drinks_resource =
        use_resource(move || async move { get_total_drinks().unwrap_or(0) });

    let mut bac_resource = 
        use_resource(move || async move { get_current_bac().unwrap_or(0.0) });

    let update_stats = move |_: ()| {
        total_drinks_resource.restart();
        bac_resource.restart();
    };

    rsx! {
        div { class: "tonight-view-container",
            DynamicBackground {}
            h1 { class: "view-header", "Booze Buddy" }
            WittyMessage { message: "Ready for poor decisions?" }
            Drinks { on_drink_added: update_stats }
            Stats { total_drinks_resource, bac_resource }
            TonightAchievements {}
            EndNightButton {}
        }
    }
}
