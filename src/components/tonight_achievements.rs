use crate::storage::{
    models::{Achievement, AchievementTier},
    storage::drink_history::get_achievements,
};
use dioxus::prelude::*;
use lucide_dioxus::Trophy;

const TONIGHT_ACHIEVEMENTS_CSS: Asset = asset!("assets/styling/tonight_achievements.css");

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TonightAchievementsProps {
    newly_unlocked_achievements: Resource<Vec<Achievement>>,
}

#[component]
pub fn TonightAchievements(props: TonightAchievementsProps) -> Element {
    let unlocked_achievements = match &*props.newly_unlocked_achievements.read_unchecked() {
        Some(achievements) => achievements.clone(),
        None => vec![],
    };

    let all_achievements = get_achievements().unwrap_or(vec![]);

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
                    all_achievements
                        .iter()
                        .map(|achievement| {
                            let (tier_class, icon) = match achievement.tier {
                                AchievementTier::Bronze => ("bronze", "🥉"),
                                AchievementTier::Silver => ("silver", "🥈"),
                                AchievementTier::Gold => ("gold", "🥇"),
                                AchievementTier::Platinum => ("platinum", "🏆"),
                            };
                            let tier_class = match unlocked_achievements.contains(&achievement) {
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
