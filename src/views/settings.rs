use crate::components::SettingsForm;
use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    rsx! {
        SettingsForm {}
    }
}
