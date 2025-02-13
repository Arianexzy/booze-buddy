use dioxus::prelude::*;
use lucide_dioxus::Trophy;

const TONIGHT_ACHIEVEMENTS: Asset = asset!("assets/styling/tonight_achievements.css");

#[derive(Clone, PartialEq)]
pub struct Achievement {
    text: String,
    achievement: String,
    tier: AchievementTier,
}

#[derive(Clone, PartialEq)]
pub enum AchievementTier {
    Bronze,
    Silver,
    Gold,
    Platinum,
}

#[component]
pub fn TonightAchievements() -> Element {
    
    // let achievements = vec![
    //     Achievement {
    //         text: "Break the seal".to_string(),
    //         achievement: "First drink of the night".to_string(),
    //         tier: AchievementTier::Bronze,
    //     },
    //     Achievement {
    //         text: "Break the seal".to_string(),
    //         achievement: "First drink of the night".to_string(),
    //         tier: AchievementTier::Silver,
    //     },
    //     // Achievement {
    //     //     text: "Break the seal!".to_string(),
    //     //     achievement: "First drink of the night".to_string(),
    //     //     tier: AchievementTier::Gold,
    //     // },
    //     // Achievement {
    //     //     text: "Break the seal!".to_string(),
    //     //     achievement: "First drink of the night".to_string(),
    //     //     tier: AchievementTier::Platinum,
    //     // },
    // ];
    
    rsx! {
        document::Link { rel: "stylesheet", href: TONIGHT_ACHIEVEMENTS }
        
        div { class: "tonight-achievements-container",
            div { class: "tonight-achievements-header",
                h2 { class: "tonight-achievements-title",
                    Trophy {},
                    "Tonight's Achievements" 
                }
            }
            div { class: "tonight-achievements-list",
                // {
                //     achievements
                //         .iter()
                //         .map(|achievement| {
                //             let tier_class = match achievement.tier {
                //                 AchievementTier::Bronze => "achievement-bronze",
                //                 AchievementTier::Silver => "achievement-silver",
                //                 AchievementTier::Gold => "achievement-gold",
                //                 AchievementTier::Platinum => "achievement-platinum",
                //             };
                //             rsx! {
                //                 div {
                //                     class: "achievement-item {tier_class}",
                //                     key: "{achievement.text}",
                //                     "{achievement.text}"
                //                 }
                //             }
                //         })
                // }
            }
        }
    }
}
