use dioxus::prelude::*;

const DYNAMIC_BACKGROUND_CSS: Asset = asset!("assets/styling/dynamic_background.css");

#[derive(PartialEq, Clone, Props)]
pub struct DynamicBackgroundProps {
    total_drinks: i32,
}

#[component]
pub fn DynamicBackground(props: DynamicBackgroundProps) -> Element {
    let hue = (props.total_drinks * 20) % 360;
    let background_color = format!("hsl({}, 70%, 50%)", hue);
    
    rsx! {
        document::Link { rel: "stylesheet", href: DYNAMIC_BACKGROUND_CSS }
        
        div { 
            class: "dynamic-background", 
            style: format!("background-color: {}", background_color),
        }
    }
}
