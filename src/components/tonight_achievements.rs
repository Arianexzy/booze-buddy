use dioxus::prelude::*;
use lucide_dioxus::Trophy;
use crate::storage::models::{Achievement, AchievementTier};

const TONIGHT_ACHIEVEMENTS_CSS: Asset = asset!("assets/styling/tonight_achievements.css");

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TonightAchievementsProps {
    achievements: Resource<Vec<Achievement>>,
}

#[component]
pub fn TonightAchievements(props: TonightAchievementsProps) -> Element {
    let unlocked_achievements = match &*props.achievements.read_unchecked() {
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
            div { class: "tonight-achievements-list" ,
                {
                    unlocked_achievements.iter().map(|achievement| {
                        let tier_class = match achievement.tier {
                            AchievementTier::Bronze => "bronze",
                            AchievementTier::Silver => "silver",
                            AchievementTier::Gold => "gold",
                            AchievementTier::Platinum => "platinum",
                        };
                        rsx!{
                            div { 
                                    class: "tonight-achievement-item achievement-{tier_class}", 
                                    h3 { class: "tonight-achievement-title", "{achievement.title}" }
                                    p { class: "tonight-achievement-description", "{achievement.description}" }
                                }
                        }
                    })
                }
            }
        }
    }
}
