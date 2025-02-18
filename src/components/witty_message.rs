use dioxus::prelude::*;

const WITTY_MESSAGE_CSS: Asset = asset!("assets/styling/witty_message.css");

#[derive(PartialEq, Props, Clone)]
pub struct WittyMessageProps {
    message: String,
}

#[component]
pub fn WittyMessage(props: WittyMessageProps) -> Element {    
    rsx! {
        document::Link { rel: "stylesheet", href: WITTY_MESSAGE_CSS }
        
        div {
            class: "witty-message-container",
            p { class: "witty-message", "{props.message}" },
        }
    }
}
