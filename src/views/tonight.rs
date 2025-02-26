use crate::{
    components::{
        Drinks, DynamicBackground, EndNightSlider, Stats, TonightAchievements, WittyMessage,
    },
    storage::{
        models::WittyMessageBank,
        storage::drink_history::{
            get_current_bac, get_newly_unlocked_achievements, get_total_drinks, has_active_session,
        },
    },
};
use dioxus::prelude::*;

#[component]
pub fn TonightView() -> Element {
    // --- HOOKS ---
    let mut total_drinks_resource =
        use_resource(move || async move { get_total_drinks().unwrap_or(0) });
    let mut bac_resource = use_resource(move || async move { get_current_bac().unwrap_or(0.0) });
    let mut reset_drink_count = use_signal(|| 0);
    let mut witty_message =
        use_signal(|| "Ready for poor decisions? Double click a drink!".to_string());
    let mut animation_trigger = use_signal(|| 0);
    let mut newly_unlocked_achievements =
        use_resource(move || async move { get_newly_unlocked_achievements().unwrap_or(vec![]) });

    // --- VIEW HANDLERS ---
    let update_view = move |_: ()| {
        refresh_stats(&mut total_drinks_resource, &mut bac_resource);
        witty_message.set(WittyMessageBank::get_random_active_session_message());
        trigger_witty_message_animation(&mut animation_trigger);
        newly_unlocked_achievements.restart();
    };
    let reset_view = move |_: ()| {
        refresh_stats(&mut total_drinks_resource, &mut bac_resource);
        trigger_drink_count_update(&mut reset_drink_count);
        witty_message.set(WittyMessageBank::get_random_end_session_message());
    };

    let has_active_session = has_active_session();

    rsx! {
        div { class: "tonight-view-container",
            if has_active_session {
                DynamicBackground { total_drinks: total_drinks_resource().unwrap_or(0) }
            }
            Stats { total_drinks_resource, bac_resource }
            WittyMessage {
                message: witty_message(),
                animation_trigger,
                key: witty_message(),
            }
            Drinks { on_drink_added: update_view, reset_drink_count }
            if has_active_session {
                TonightAchievements { newly_unlocked_achievements }
                EndNightSlider { on_end_night: reset_view }
            }
            if has_active_session {

            }
        }
    }
}

fn refresh_stats(total_drinks_resource: &mut Resource<i32>, bac_resource: &mut Resource<f32>) {
    total_drinks_resource.restart();
    bac_resource.restart();
}

fn trigger_witty_message_animation(animation_trigger: &mut Signal<i32>) {
    animation_trigger.set(animation_trigger() + 1);
}

fn trigger_drink_count_update(reset_drink_count: &mut Signal<i32>) {
    reset_drink_count.set(reset_drink_count() + 1);
}
