use crate::storage::{
    models::{Gender, User},
    storage::settings::{get_user, save_user_settings},
};
use dioxus::prelude::*;

#[component]
pub fn SettingsView() -> Element {
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
        h1 { class: "view-header", "Settings" }

        p { class: "info", "Questions below are to estimate blood alcohol content (BAC). It's just a rough estimate for entertainment purposes."}

        form { class: "settings-form", onsubmit: update_user,

            div { class: "form-group",
                label { "How much meat on your bones? (lbs)" }
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
                label { "What are you spiking your gender gin with?" }
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
                    option { value: "Male", "Dude Juice" }
                    option { value: "Female", "Lady Liquor" }
                    option { value: "Other", "Mysterious Moonshine" }
                }
            }

            button { class: "form-submit", r#type: "submit", "Save Settings" }
        }

        // Replace your existing p tag with class "bac-info" with this:
        div { class: "bac-info",
            h3 { "How We Calculate Your BAC" }

            p { "Your Blood Alcohol Content (BAC) is estimated using a scientific formula that considers your weight, gender, and drinking timeline." }

            h4 { "The Science Behind It" }
            p { "Each drink adds approximately 0.6 fluid ounces of pure alcohol to your system. This alcohol is distributed throughout your body's water content, which varies based on your weight and biological factors." }

            p { "For each drink you log, we use the Widmark Formula tocalculate:" }

            pre { "Drink BAC = (0.6 oz alcohol × 5.14) ÷ (your weight × gender constant)" }

            p { "The gender constants account for differences in body water content:" }
            ul {
                li { "Dude Juice (Male): 0.68" }
                li { "Lady Liquor (Female): 0.55" }
                li { "Mysterious Moonshine (Other): 0.615" }
            }

            h4 { "Sobering Up" }
            p { "Your body metabolizes alcohol at a fairly constant rate of about 0.015% BAC per hour. We subtract this from your calculated BAC based on when you had each drink:" }

            pre { "Actual BAC = Drink BAC - (0.015 × hours since drink)" }

            p { "Your current BAC is the sum of all your drinks' individual contributions, minus what your body has metabolized." }

            h4 { "Keep in Mind" }
            p { "This is just an estimate for entertainment purposes. Many factors can affect your actual BAC, including:" }
            ul {
                li { "Food consumption" }
                li { "Medications" }
                li { "Individual metabolism variations" }
                li { "Health conditions" }
            }

            p { class: "disclaimer", "Never rely on this app to determine if you're safe to drive or operate machinery. Always plan for a safe ride home when drinking." }
        }
    }
}

fn load_existing_user(weight: &mut Signal<String>, gender: &mut Signal<Gender>) {
    let user = get_user();
    weight.set(user.weight.to_string());
    gender.set(user.gender);
}
