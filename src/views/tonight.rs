use crate::{
    components::{
        Drinks, DynamicBackground, EndNightButton, Stats, TonightAchievements, WittyMessage,
    },
    storage::storage::{has_active_session, start_new_session},
};
use dioxus::prelude::*;

#[component]
pub fn Tonight() -> Element {
    use_effect(move || {
        if !has_active_session() {
            start_new_session();
        }
    });

    rsx! {
        div { class: "tonight-view-container",
            DynamicBackground {}
            h1 { class: "view-header", "Booze Buddy" }
            WittyMessage { message: "Ready for poor decisions?" }
            Drinks {}
            Stats {}
            TonightAchievements {}
            EndNightButton {}
        }
    }
}
