use crate::storage::{models::{AchievementRegistry, AchievementTier}, storage::drink_history::get_past_sessions};
use dioxus::prelude::*;

#[component]
pub fn AchievementsView() -> Element {
    let registry = AchievementRegistry::global();
    let mut all_unlocked_ids: Vec<u32> = Vec::new();

    let past_sessions = get_past_sessions()?;
    for session in past_sessions {
        let unlocked_ids = session.get_unlocked_achievement_ids();
        unlocked_ids.iter().for_each(|id| {
            all_unlocked_ids.push(*id);
        });
    }

    let tiers = [
        ("Bronze Tier", AchievementTier::Bronze),
        ("Silver Tier", AchievementTier::Silver),
        ("Gold Tier", AchievementTier::Gold),
        ("Platinum Tier", AchievementTier::Platinum),
    ];

    rsx! {
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
                                            let count = all_unlocked_ids.iter().filter(|id| **id == achievement.id).count();
                                            all_unlocked_ids.retain(|id| *id != achievement.id);
                                            rsx! {
                                                div { class: "achievement",
                                                    h3 { class: "title", "{achievement.title}" }
                                                    p { class: "description", "{achievement.description}" }
                                                    if count > 0 {
                                                        span { class: "count", "{count}x" }
                                                    }
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
