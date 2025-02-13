use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;
use std::time::Duration;

const END_NIGHT_BUTTON_CSS: Asset = asset!("/assets/styling/end_night_button.css");

#[component]
pub fn EndNightButton() -> Element {
    let mut progress = use_motion(0.0f32);
    let mut is_holding = use_signal(|| false);
    let animation_duration = Duration::from_millis(2500);

    // Convert progress to CSS custom property
    let progress_style = format!("--progress: {};", progress.get_value());

    let mut handle_start_progress = move || {
        is_holding.set(true);
        progress.animate_to(
            100.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: animation_duration,
                easing: easer::functions::Sine::ease_in_out,
            })),
        );
    };

    let mut handle_stop_progress = move || {
        is_holding.set(false);
        progress.stop();

        if progress.get_value() < 100.0 {
            progress.animate_to(
                0.0,
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_millis(500),
                    easing: easer::functions::Sine::ease_out,
                })),
            );
        }
    };

    let start_progress_mouse = move |_: Event<MouseData>| handle_start_progress();
    let stop_progress_mouse = move |_: Event<MouseData>| handle_stop_progress();

    let start_progress_touch = move |_: Event<TouchData>| handle_start_progress();
    let stop_progress_touch = move |_: Event<TouchData>| handle_stop_progress();

    rsx! {
        document::Link { rel: "stylesheet", href: END_NIGHT_BUTTON_CSS }
        div { class: "end-night-container",
            button {
                class: "end-night-button",
                style: "{progress_style}",
                onmousedown: start_progress_mouse,
                onmouseup: stop_progress_mouse,
                onmouseleave: stop_progress_mouse,
                ontouchstart: start_progress_touch,
                ontouchend: stop_progress_touch,
                if !*is_holding.read() {
                    "Hold to call it a night!"
                }
            }
        }
    }
}
