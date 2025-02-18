use crate::storage::{
    models::{Gender, User},
    storage::settings::{get_user, save_user_settings},
};
use dioxus::prelude::*;

const SETTINGS_CSS: Asset = asset!("/assets/styling/settings.css");

#[component]
pub fn SettingsForm() -> Element {
    let mut weight = use_signal(|| String::new());
    let mut gender = use_signal(|| Gender::Other);

    use_effect(move || load_existing_user(&mut weight, &mut gender));

    let update_user = move |event: Event<FormData>| {
        event.prevent_default();
        if let Ok(weight_value) = weight().parse::<f32>() {
            let user = User {
                weight: weight_value,
                gender: gender(),
            };
            save_user_settings(user);
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: SETTINGS_CSS }

        form { class: "settings-form", onsubmit: update_user,

            div { class: "form-group",
                label { "Weight (lbs)" }
                input {
                    class: "form-input",
                    r#type: "number",
                    value: "{weight}",
                    oninput: move |event| weight.set(event.value().clone()),
                    step: "0.1",
                    min: "50",
                    max: "500",
                }
            }

            div { class: "form-group",
                label { "Gender" }
                select {
                    class: "form-select",
                    value: "{gender():?}",
                    onchange: move |event| {
                        gender
                            .set(
                                match event.value().as_str() {
                                    "Male" => Gender::Male,
                                    "Female" => Gender::Female,
                                    _ => Gender::Other,
                                },
                            )
                    },
                    option { value: "Male", "Male" }
                    option { value: "Female", "Female" }
                    option { value: "Other", "Other" }
                }
            }

            button { class: "form-submit", r#type: "submit", "Save Settings" }
        }
    }
}

fn load_existing_user(weight: &mut Signal<String>, gender: &mut Signal<Gender>) {
    let user = get_user();
    weight.set(user.weight.to_string());
    gender.set(user.gender);
}
