use dioxus::prelude::*;

const WITTY_MESSAGE_CSS: Asset = asset!("assets/styling/witty_message.css");

#[derive(PartialEq, Props, Clone)]
pub struct WittyMessageProps {
    message: String,
    animation_trigger: Signal<i32>,
}

#[component]
pub fn WittyMessage(props: WittyMessageProps) -> Element {
    // @dev I could not get the animation rerun on each update
    // I tried everything I could.  The only thing that worked
    // was creating new class names and animation names and rotating
    // thru them with the modulo operator

    let rotation_counter = *&*props.animation_trigger.read_unchecked();
    let dynamic_class = format!("witty-message dynamic-{}", rotation_counter % 3);

    rsx! {
        document::Link { rel: "stylesheet", href: WITTY_MESSAGE_CSS }
        div { class: "witty-message-container",
            p { class: "{dynamic_class}", "{props.message}" }
        }
    }
}
