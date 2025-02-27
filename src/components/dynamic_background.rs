use dioxus::prelude::*;

const DYNAMIC_BACKGROUND_CSS: Asset = asset!("assets/styling/dynamic_background.css");

#[derive(PartialEq, Clone, Props)]
pub struct DynamicBackgroundProps {
    total_drinks: i32,
}

#[component]
pub fn DynamicBackground(props: DynamicBackgroundProps) -> Element {
    let (base_hue, glow_intensity) = calculate_cyberpunk_mood(props.total_drinks);

    let style = format!(
        "--base-hue: {base_hue:.1}; --glow-intensity: {glow_intensity:.3};"
    );

    rsx! {
        document::Link { rel: "stylesheet", href: DYNAMIC_BACKGROUND_CSS }
        div { class: "dynamic-background-container",
            div {
                class: "gradient-layer",
                style: "{style}",
            }
            div { class: "grid-layer" }
            div { class: "glow-layer" }
        }
    }
}

fn calculate_cyberpunk_mood(drinks: i32) -> (f32, f32) {
    // Cap at 10 drinks for full effect
    let drinks = drinks.clamp(0, 10);
    let progression = (drinks as f32 / 10.0).clamp(0.0, 1.0);
    
    // Start with cyan/teal (180) and shift gradually toward red (0) with more drinks
    // But keep the shift VERY subtle - only 40 degrees in hue
    let base_hue = 180.0 - (40.0 * progression);
    
    // Glow intensity increases very subtly with drink count
    // Starts at 0 and maxes at 1.0 for full effect
    let glow_intensity = progression.powf(1.5);
    
    (base_hue, glow_intensity)
}