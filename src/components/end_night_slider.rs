use crate::storage::storage::drink_history::end_current_session;
use dioxus::prelude::*;

const END_NIGHT_SLIDER_CSS: Asset = asset!("/assets/styling/end_night_slider.css");

#[derive(PartialEq, Clone, Props)]
pub struct EndNightSliderProps {
    on_end_night: EventHandler<()>,
}

#[component]
pub fn EndNightSlider(props: EndNightSliderProps) -> Element {
    let mut value = use_signal(|| 0.0);

    let handle_input = move |event: FormEvent| {
        let incoming_value = event.value().parse::<f32>().unwrap_or(0.0);
        value.set(incoming_value);

        if value() >= 0.95 {
            props.on_end_night.call(());
            end_current_session().expect("Failed to end current session");
            value.set(0.0);
        }
    };

    let handle_touch_end = move |_: TouchEvent| {
        if value() < 0.95 {
            value.set(0.0);
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: END_NIGHT_SLIDER_CSS }
        div { class: "slider-container",
            span { class: "slider-text",
                if value() == 0.0 {
                    "Slide to end the night"
                } else {
                    "Ending night..."
                }
            }
            input {
                class: "slider-track",
                r#type: "range",
                value: "{value}",
                min: "0.0",
                max: "1.0",
                step: "0.01",
                // prevent_default: "oninput", // Let's remove this for now, see if default behavior is okay
                oninput: handle_input,
                ontouchend: handle_touch_end,
            }
        }
    }
}
