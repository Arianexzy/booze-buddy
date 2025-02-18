use crate::components::SettingsForm;
use dioxus::prelude::*;

#[component]
pub fn SettingsView() -> Element {
    rsx! {
        SettingsForm {}
    }
}
