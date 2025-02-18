use crate::{
    components::{
        Drinks, DynamicBackground, EndNightButton, Stats, TonightAchievements, WittyMessage, WittyMessageBank
    },
    storage::storage::drink_history::{get_current_bac, get_total_drinks},    
};
use dioxus::prelude::*;

#[component]
pub fn TonightView() -> Element {
    let mut total_drinks_resource =
        use_resource(move || async move { get_total_drinks().unwrap_or(0) });

    let mut bac_resource = use_resource(move || async move { get_current_bac().unwrap_or(0.0) });

    let mut witty_message = use_signal(|| "Ready for poor decisions?".to_string());
    
    let update_view = move |_: ()| {
        total_drinks_resource.restart();
        bac_resource.restart();
        witty_message.set(WittyMessageBank::get_random_message());
    };

    rsx! {
        div { class: "tonight-view-container",
            DynamicBackground {}
            h1 { class: "view-header", "Booze Buddy" }
            WittyMessage { 
                message: witty_message() ,
                key: witty_message(),
            }
            Drinks { on_drink_added: update_view }
            Stats { total_drinks_resource, bac_resource }
            TonightAchievements {}
            EndNightButton {}
        }
    }
}
