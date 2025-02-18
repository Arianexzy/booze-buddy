use dioxus::prelude::*;

#[component]
pub fn AchievementsView() -> Element {
    rsx! {
        h1 { class: "view-header", "Achievements" }
    }
}
