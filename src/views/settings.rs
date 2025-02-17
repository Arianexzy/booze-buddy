use dioxus::prelude::*;
use crate::components::SettingsForm;

#[component]
pub fn Settings() -> Element {
    rsx! {
        SettingsForm {}
    }
}
