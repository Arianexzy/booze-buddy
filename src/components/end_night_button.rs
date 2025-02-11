use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

const END_NIGHT_BUTTON_CSS: Asset = asset!("/assets/styling/end_night_button.css");

#[component]
pub fn EndNightButton() -> Element {
    let mut progress = use_motion(0.0f32);
    let is_dragging = use_signal(|| false);
    let start_x = use_signal(|| 0.0f32);
    let slider_width = use_signal(|| 0.0f32);

    let handle_start = move |event: MouseEvent| {
        is_dragging.set(true);
        start_x.set(event.client_x() as f32);
    };

    let handle_move = move |event: MouseEvent| {
        if !is_dragging() {
            return;
        }

        let delta = event.client_x() as f32 - start_x();
        let new_progress = (delta / slider_width()).clamp(0.0, 1.0) * 100.0;
        progress.set_value(new_progress);
    };

    let handle_end = move |_| {
        if progress.get_value() >= 100.0 {
            // Trigger completion action
        } else {
            progress.animate_to(
                0.0,
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_millis(500),
                    easing: easer::functions::Sine::ease_out,
                })),
            );
        }
        is_dragging.set(false);
    };

    rsx! {
        document::Link { rel: "stylesheet", href: END_NIGHT_BUTTON_CSS }
        
        div { class: "end-night-container",
            div { class: "end-night-card",
                div { class: "counter-text", "{progress.get_value().round() as i32}%" }

                div {
                    class: "slider-container",
                    onmounted: move |cx| {
                        let rect = cx.get_client_rect();
                        slider_width.set(rect.width() as f32);
                    },
                    
                    div { class: "slider-track" }
                    div {
                        class: "slider-progress",
                        style: "--progress-width: {progress.get_value()}%"
                    }
                    div {
                        class: "slider-thumb",
                        style: "left: {progress.get_value()}%",
                        onmousedown: handle_start,
                        ontouchstart: |e| {
                            is_dragging.set(true);
                            start_x.set(e.touches().get(0).client_x() as f32);
                        },
                        ontouchmove: |e| {
                            if is_dragging() {
                                let delta = e.touches().get(0).client_x() as f32 - start_x();
                                let new_progress = (delta / slider_width()).clamp(0.0, 1.0) * 100.0;
                                progress.set_value(new_progress);
                            }
                        },
                        ontouchend: handle_end,
                        onmouseup: handle_end,
                    }
                }
            }
        }
    }
}