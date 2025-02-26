use dioxus::prelude::*;

use crate::storage::storage::drink_history::has_active_session;

const WITTY_MESSAGE_CSS: Asset = asset!("assets/styling/witty_message.css");

#[derive(PartialEq, Props, Clone)]
pub struct WittyMessageProps {
    message: String,
    animation_trigger: Signal<i32>,
}

#[component]
pub fn WittyMessage(props: WittyMessageProps) -> Element {
    // @dev I could not get the animation to rerun on each update.
    // I tried everything I could.  The only thing that worked
    // was creating new class and animation names and rotating
    // thru them with the modulo operator

    let rotation_counter = *&*props.animation_trigger.read_unchecked();
    let dynamic_class = format!("witty-message dynamic-{}", rotation_counter % 3);

    let container_class = match has_active_session() {
        true => "witty-message-container",
        false => "witty-message-container pulse",
    };

    rsx! {
        document::Link { rel: "stylesheet", href: WITTY_MESSAGE_CSS }
        div { class: "{container_class}",
            p { class: "{dynamic_class}", "{props.message}" }
        }
    }
}
