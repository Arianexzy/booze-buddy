use dioxus::prelude::*;

use crate::storage::models::{AchievementRegistry, AchievementTier};

const ACHIEVEMENTS_VIEW_CSS: Asset = asset!("assets/styling/achievements_view.css");

#[component]
pub fn AchievementsView() -> Element {
    let registry = AchievementRegistry::global();

    let tiers = [
        ("Bronze Tier", AchievementTier::Bronze),
        ("Silver Tier", AchievementTier::Silver),
        ("Gold Tier", AchievementTier::Gold),
        ("Platinum Tier", AchievementTier::Platinum),
    ];

    rsx! {
        document::Link { rel: "stylesheet", href: ACHIEVEMENTS_VIEW_CSS }
        h1 { class: "view-header", "Achievements" }
        div {
            {
                tiers
                    .iter()
                    .map(|(label, tier)| {
                        let achievements: Vec<_> = registry
                            .achievements
                            .iter()
                            .filter(|a| a.tier == *tier)
                            .collect();
                        rsx! {
                            div {
                                h2 { class: "view-subheader", "{label}" }
                                {
                                    achievements
                                        .iter()
                                        .map(|achievement| {
                                            rsx! {
                                                div { class: "achievement",
                                                    h3 { class: "title", "{achievement.title}" }
                                                    p { class: "description", "{achievement.description}" }
                                                }
                                            }
                                        })
                                }
                            }
                        }
                    })
            }
        }
    }
}
