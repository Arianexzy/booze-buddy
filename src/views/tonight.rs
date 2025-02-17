use crate::{
    components::{
        Drinks, DynamicBackground, EndNightButton, Stats, TonightAchievements, WittyMessage,
    },
    storage::storage::drink_history::{get_total_drinks, has_active_session, start_new_session},
};
use dioxus::prelude::*;

#[component]
pub fn Tonight() -> Element {
    use_effect(move || {
        start_or_continue_drinking_session();
    });

    let mut total_drinks_resource = use_resource(move || async move { get_total_drinks() });

    let update_total_drink_count = move |_: ()| {
        total_drinks_resource.restart();
    };

    rsx! {
        div { class: "tonight-view-container",
            DynamicBackground {}
            h1 { class: "view-header", "Booze Buddy" }
            WittyMessage { message: "Ready for poor decisions?" }
            Drinks { on_drink_added: update_total_drink_count }
            Stats { total_drinks_resource }
            TonightAchievements {}
            EndNightButton {}
        }
    }
}

fn start_or_continue_drinking_session() {
    if !has_active_session() {
        start_new_session();
    }
}
