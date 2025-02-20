use crate::{
    components::{
        Drinks, DynamicBackground, EndNightButton, Stats, TonightAchievements, WittyMessage,
        WittyMessageBank,
    },
    storage::storage::drink_history::{get_current_bac, get_total_drinks, has_active_session},
};
use dioxus::prelude::*;

#[component]
pub fn TonightView() -> Element {
    let mut total_drinks_resource =
        use_resource(move || async move { get_total_drinks().unwrap_or(0) });
    let mut bac_resource = use_resource(move || async move { get_current_bac().unwrap_or(0.0) });
    let mut reset_drink_count = use_signal(|| 0);

    let mut witty_message =
        use_signal(|| "Ready for poor decisions? Double click a drink!".to_string());
    let mut animation_trigger = use_signal(|| 0);

    let update_view = move |_: ()| {
        refresh_stats(&mut total_drinks_resource, &mut bac_resource);
        witty_message.set(WittyMessageBank::get_random_active_session_message());
        animation_trigger.set(animation_trigger() + 1);
    };

    let reset_view = move |_: ()| {
        refresh_stats(&mut total_drinks_resource, &mut bac_resource);
        reset_drink_count.set(reset_drink_count() + 1); // force update
        witty_message.set(WittyMessageBank::get_random_end_session_message());
    };

    rsx! {
        div { class: "tonight-view-container",
            if has_active_session() {
                DynamicBackground { total_drinks: total_drinks_resource().unwrap_or(0) }
            }
            Stats { total_drinks_resource, bac_resource }
            WittyMessage {
                message: witty_message(),
                animation_trigger,
                key: witty_message(),
            }
            Drinks { on_drink_added: update_view, reset_drink_count }
            TonightAchievements {}
            EndNightButton { on_end_night: reset_view }
        }
    }
}

fn refresh_stats(total_drinks_resource: &mut Resource<i32>, bac_resource: &mut Resource<f32>) {
    total_drinks_resource.restart();
    bac_resource.restart();
}
