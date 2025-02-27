use crate::storage::{models::{Achievement, AchievementRegistry}, storage::drink_history::get_unlocked_achievements};
use dioxus::prelude::*;
use lucide_dioxus::Trophy;

const TONIGHT_ACHIEVEMENTS_CSS: Asset = asset!("assets/styling/tonight_achievements.css");

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TonightAchievementsProps {
    newly_unlocked_achievements: Resource<Vec<u32>>,
}

#[component]
pub fn TonightAchievements(props: TonightAchievementsProps) -> Element {
    let registry = AchievementRegistry::global();
    let unlocked_achievement_ids = get_unlocked_achievements().unwrap_or(vec![]);
    let newly_unlocked_achievements_ids = match &*props.newly_unlocked_achievements.read_unchecked() {
        Some(achievements) => achievements.clone(),
        None => vec![],
    };

    rsx! {
        document::Link { rel: "stylesheet", href: TONIGHT_ACHIEVEMENTS_CSS }

        div { class: "tonight-achievements-container",
            div { class: "tonight-achievements-header",
                h2 { class: "tonight-achievements-title",
                    Trophy {}
                    "Tonight's Achievements"
                }
            }
            div { class: "tonight-achievements-list",
                {
                    unlocked_achievement_ids
                        .iter()
                        .map(|id| {
                            let achievement = registry.get_achievement_from(*id).unwrap();
                            let achievement_display = Achievement::display(achievement.tier);
                            let (tier_class, icon) = (
                                achievement_display.tier_label,
                                achievement_display.tier_icon,
                            );
                            let tier_class = match newly_unlocked_achievements_ids.contains(id) {
                                true => format!("{tier_class} new"),
                                false => format!("{tier_class}"),
                            };
                            rsx! {
                                div { class: "tonight-achievement-item achievement-{tier_class}",
                                    h3 { class: "tonight-achievement-title",
                                        "{achievement.title}"
                                        span { "{icon}" }
                                    }
                                    p { class: "tonight-achievement-description", "{achievement.description}" }
                                }
                            }
                        })
                }
            }
        }
    }
}
