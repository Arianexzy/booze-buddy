use dioxus::prelude::*;

const DYNAMIC_BACKGROUND_CSS: Asset = asset!("assets/styling/dynamic_background.css");

#[derive(PartialEq, Clone, Props)]
pub struct DynamicBackgroundProps {
    total_drinks: i32,
}

#[component]
pub fn DynamicBackground(props: DynamicBackgroundProps) -> Element {
    let (base_hue, gradient_range, l1, l2) = calculate_hue_progression(props.total_drinks);

    let gradient_style = format!(
        "--base-hue: {base_hue:.1};
            --hue-range: {gradient_range:.1};
            --lightness1: {l1}%;
            --lightness2: {l2}%;"
    );

    let tint_class = match props.total_drinks {
        9 => " dark-tint",
        10 => " darker-tint",
        n if n > 10 => " darkest-tint",
        _ => "",
    };

    rsx! {
        document::Link { rel: "stylesheet", href: DYNAMIC_BACKGROUND_CSS }
        div { class: "dynamic-background-container",
            div {
                class: "gradient-layer{tint_class}",
                style: "{gradient_style}",
            }
            div { class: "noise-layer" }
        }
    }
}

fn calculate_hue_progression(drinks: i32) -> (f32, f32, f32, f32) {
    let drinks = drinks.clamp(0, 8);
    let progression = (drinks as f32 / 8.0).clamp(0.0, 1.0);

    // Base color progression
    let base_hue = 270.0 * (1.0 - progression);
    let gradient_range = 55.0 * (1.0 - progression).powf(0.5);

    // Gentle darkening curve for final stage
    let lightness_factor = if drinks >= 8 {
        0.7 // Extra darkening for final stage
    } else {
        progression.powf(2.0).min(0.9)
    };

    let lightness1 = 35.0 - (15.0 * lightness_factor);
    let lightness2 = 45.0 - (10.0 * lightness_factor);

    (base_hue, gradient_range, lightness1, lightness2)
}
