use crate::storage::models::{DrinkType, DrinkingSession, Gender, User};
use chrono::Duration;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum AchievementTier {
    Bronze,
    Silver,
    Gold,
    Platinum,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AchievementCondition {
    TotalDrinks(i32),
    MinBAC(f32),
    DrinkTypeCount(DrinkType, i32),
    DrinksWithinDuration(i32, Duration),
    Custom(fn(&DrinkingSession, &User) -> bool),
}

#[derive(Debug)]
pub struct AchievementDisplay {
    pub tier_label: &'static str,
    pub tier_icon: &'static str,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Achievement {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub tier: AchievementTier,
    #[serde(skip)]
    pub conditions: Vec<AchievementCondition>,
}

impl Achievement {
    pub fn is_achieved(&self, session: &DrinkingSession, user: &User) -> bool {
        self.conditions.iter().all(|cond| match cond {
            AchievementCondition::TotalDrinks(drinks) => session.total_drinks() >= *drinks,
            AchievementCondition::MinBAC(bac) => session.current_bac >= *bac,
            AchievementCondition::DrinkTypeCount(drink_type, count) => {
                session.count_by(*drink_type) >= *count
            }
            AchievementCondition::DrinksWithinDuration(count, duration) => {
                session.events.windows(*count as usize).any(|window| {
                    let first = window.first().unwrap();
                    let last = window.last().unwrap();
                    last.timestamp - first.timestamp <= *duration
                })
            }
            AchievementCondition::Custom(func) => func(session, user),
        })
    }

    pub fn display(tier: AchievementTier) -> AchievementDisplay {
        match tier {
            AchievementTier::Bronze => AchievementDisplay {
                tier_label: "bronze",
                tier_icon: "🥉",
            },
            AchievementTier::Silver => AchievementDisplay {
                tier_label: "silver",
                tier_icon: "🥈",
            },
            AchievementTier::Gold => AchievementDisplay {
                tier_label: "gold",
                tier_icon: "🥇",
            },
            AchievementTier::Platinum => AchievementDisplay {
                tier_label: "platinum",
                tier_icon: "🏆",
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AchievementRegistry {
    pub achievements: Vec<Achievement>,
}

impl AchievementRegistry {
    pub fn global() -> &'static AchievementRegistry {
        static REGISTRY: OnceLock<AchievementRegistry> = OnceLock::new();
        REGISTRY.get_or_init(|| AchievementRegistry::new() )
    }

    pub fn new() -> Self {
        let mut registry = AchievementRegistry {
            achievements: Vec::new(),
        };
        registry.register_achievements();
        registry
    }
    
    pub fn get_achievement_from(&self, id: u32) -> Option<&Achievement> {
        self.achievements.iter().find(|achievement| achievement.id == id)
    }

    pub fn register_achievements(&mut self) {
        // Bronze
        self.achievements.push(Achievement {
            id: 3,
            title: "Wet Your Whistle".to_string(),
            description: "Down three drinks and you're off to a shaky start.".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::TotalDrinks(3)],
        });
        self.achievements.push(Achievement {
            id: 4,
            title: "One Pump Chump".to_string(),
            description: "Down a shot in the first hour of starting.".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![
                AchievementCondition::DrinkTypeCount(DrinkType::Shot, 1),
                AchievementCondition::DrinksWithinDuration(1, Duration::minutes(30)),
            ],
        });
        self.achievements.push(Achievement {
            id: 5,
            title: "Wine Whiner".to_string(),
            description: "Have a glass of wine and complain about life.".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::DrinkTypeCount(DrinkType::Wine, 1)],
        });
        self.achievements.push(Achievement {
            id: 6,
            title: "Beer Goggles On".to_string(),
            description: "Reach a BAC of 0.05% - things are starting to look better!".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::MinBAC(0.05)],
        });
        self.achievements.push(Achievement {
            id: 7,
            title: "Shot Caller".to_string(),
            description: "Take your first shot of the night".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::DrinkTypeCount(DrinkType::Shot, 1)],
        });
        self.achievements.push(Achievement {
            id: 8,
            title: "Thirsty Beaver".to_string(),
            description: "Down 2 drinks within 30 minutes".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::DrinksWithinDuration(
                2,
                Duration::minutes(30),
            )],
        });
        self.achievements.push(Achievement {
            id: 10,
            title: "Slowpoke Sipper".to_string(),
            description: "Take over 30 minutes to finish your first drink—pathetic!".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::Custom(|session, _| {
                if session.events.len() != 1 {
                    return false;
                }
                let first_drink = session.events.first().unwrap();
                let time_since = chrono::Local::now() - first_drink.timestamp;
                time_since > Duration::minutes(30)
            })],
        });
        self.achievements.push(Achievement {
            id: 11,
            title: "Lightweight Limp".to_string(),
            description: "Feel buzzed (BAC > 0.03) under 170 lbs—weak sauce!".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::Custom(|session, user| {
                session.current_bac > 0.03 && user.weight < 170.0
            })],
        });

        // Silver
        self.achievements.push(Achievement {
            id: 13,
            title: "Breaking Bad".to_string(),
            description: "Crush four drinks in a night".to_string(),
            tier: AchievementTier::Silver,
            conditions: vec![AchievementCondition::TotalDrinks(4)],
        });
        self.achievements.push(Achievement {
            id: 14,
            title: "Wine a bit, you'll feel better".to_string(),
            description: "Throw down three glasses of wine".to_string(),
            tier: AchievementTier::Silver,
            conditions: vec![AchievementCondition::DrinkTypeCount(DrinkType::Wine, 3)],
        });
        self.achievements.push(Achievement {
            id: 15,
            title: "Beer Belly".to_string(),
            description: "Hammer five beers".to_string(),
            tier: AchievementTier::Silver,
            conditions: vec![AchievementCondition::DrinkTypeCount(DrinkType::Beer, 5)],
        });
        self.achievements.push(Achievement {
            id: 16,
            title: "Buzzed Lightyear".to_string(),
            description: "Reach a BAC of 0.08%".to_string(),
            tier: AchievementTier::Silver,
            conditions: vec![AchievementCondition::MinBAC(0.08)],
        });
        self.achievements.push(Achievement {
            id: 17,
            title: "Mixed Up Like A Milkshake".to_string(),
            description: "Have two mixed drinks in one night".to_string(),
            tier: AchievementTier::Silver,
            conditions: vec![AchievementCondition::DrinkTypeCount(
                DrinkType::MixedDrink,
                2,
            )],
        });
        self.achievements.push(Achievement {
            id: 18,
            title: "Slippery When Wet".to_string(),
            description: "Hit a BAC of 0.12 – time to get a little sloppy!".to_string(),
            tier: AchievementTier::Silver,
            conditions: vec![AchievementCondition::MinBAC(0.12)],
        });
        self.achievements.push(Achievement {
            id: 19,
            title: "Gender Bender".to_string(),
            description: "Men over 200 lbs or women under 130 lbs hit BAC 0.08—flip the script!"
                .to_string(),
            tier: AchievementTier::Silver,
            conditions: vec![AchievementCondition::Custom(|session, user| {
                let bac = session.current_bac;
                match user.gender {
                    Gender::Male => bac >= 0.08 && user.weight > 200.0,
                    Gender::Female => bac >= 0.08 && user.weight < 130.0,
                    _ => false, // Other gender doesn’t qualify
                }
            })],
        });

        // Gold
        self.achievements.push(Achievement {
            id: 20,
            title: "Sexy Six Shooter".to_string(),
            description: "Crush six drinks in a night".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![AchievementCondition::TotalDrinks(6)],
        });
        self.achievements.push(Achievement {
            id: 21,
            title: "Juicy Booty Shorts".to_string(),
            description: "Slam eight beers in a night - time to stare awkwardly at a booty"
                .to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![AchievementCondition::DrinkTypeCount(DrinkType::Beer, 8)],
        });
        self.achievements.push(Achievement {
            id: 22,
            title: "Juggalo 4 Life".to_string(),
            description: "Hit a BAC of 0.16 – you're no insane clown pussy".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![AchievementCondition::MinBAC(0.16)],
        });
        self.achievements.push(Achievement {
            id: 23,
            title: "Whiskey Dick".to_string(),
            description: "Down three shots".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![AchievementCondition::DrinkTypeCount(DrinkType::Shot, 3)],
        });
        self.achievements.push(Achievement {
            id: 24,
            title: "Carpet Muncher".to_string(),
            description: "Five glasses of wine - your face is gonna hit the floor".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![AchievementCondition::DrinkTypeCount(DrinkType::Wine, 5)],
        });
        self.achievements.push(Achievement {
            id: 25,
            title: "Cocktail Cocktease".to_string(),
            description: "Have 2 mixed drinks in 30 minutes—slow down, tiger.".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![
                AchievementCondition::DrinkTypeCount(DrinkType::MixedDrink, 2),
                AchievementCondition::DrinksWithinDuration(2, Duration::minutes(30)),
            ],
        });
        self.achievements.push(Achievement {
            id: 26,
            title: "Staggered Stumbler".to_string(),
            description: "Space 3 drinks exactly 15-20 minutes apart—drunk clockwork!".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![AchievementCondition::Custom(|session, _| {
                if session.events.len() < 3 {
                    return false;
                }
                let windows = session.events.windows(2);
                windows.enumerate().all(|(i, window)| {
                    if i >= 2 {
                        return true;
                    } // Only check first two gaps
                    let gap = window[1].timestamp - window[0].timestamp;
                    gap >= Duration::minutes(15) && gap <= Duration::minutes(20)
                })
            })],
        });
        self.achievements.push(Achievement {
            id: 27,
            title: "BAC-kward Ass".to_string(),
            description: "BAC drops below 0.05 after peaking above 0.10".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![AchievementCondition::Custom(|session, _| {
                let current_bac = session.current_bac;
                let peak_bac = session.events.iter().fold(0.0, |acc: f32, event| {
                    let partial_session = DrinkingSession {
                        events: session.events
                            [..session.events.iter().position(|e| e == event).unwrap() + 1]
                            .to_vec(),
                        ..session.clone()
                    };
                    let bac = partial_session.current_bac;
                    acc.max(bac)
                });
                current_bac < 0.05 && peak_bac > 0.10
            })],
        });
        self.achievements.push(Achievement {
            id: 28,
            title: "Rapid Pounding".to_string(),
            description: "Have 3 drinks in less than 30 minutes".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![
                AchievementCondition::TotalDrinks(3),
                AchievementCondition::DrinksWithinDuration(3, Duration::minutes(30)),
            ],
        });
        self.achievements.push(Achievement {
            id: 29,
            title: "Boozy Bukkake".to_string(),
            description: "Have one of each drink type".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![
                AchievementCondition::DrinkTypeCount(DrinkType::Beer, 1),
                AchievementCondition::DrinkTypeCount(DrinkType::Wine, 1),
                AchievementCondition::DrinkTypeCount(DrinkType::Shot, 1),
                AchievementCondition::DrinkTypeCount(DrinkType::MixedDrink, 1),
            ],
        });
        self.achievements.push(Achievement {
            id: 30,
            title: "Swallow Don't Spit".to_string(),
            description: "Down five shots".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![AchievementCondition::DrinkTypeCount(DrinkType::Shot, 5)],
        });
        self.achievements.push(Achievement {
            id: 31,
            title: "Double Penetration".to_string(),
            description: "Finish two different drinks within 20 minutes".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![AchievementCondition::Custom(|session, _user| {
                let mut events = session.events.clone();
                events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

                for i in 1..events.len() {
                    if events[i].drink_type != events[i - 1].drink_type {
                        let duration = events[i].timestamp - events[i - 1].timestamp;
                        if duration <= Duration::minutes(20) {
                            return true;
                        }
                    }
                }
                false
            })],
        });
        self.achievements.push(Achievement {
            id: 32,
            title: "Five Knuckle Shuffle".to_string(),
            description: "Down 5 drinks in 1hour — hand’s getting busy!".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![AchievementCondition::DrinksWithinDuration(
                5,
                Duration::minutes(60),
            )],
        });
        self.achievements.push(Achievement {
            id: 33,
            title: "Tequila Titty-Twister".to_string(),
            description: "4 shots in 1 hour — get limber!".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![
                AchievementCondition::DrinkTypeCount(DrinkType::Shot, 4),
                AchievementCondition::DrinksWithinDuration(4, Duration::minutes(60)),
            ],
        });
        self.achievements.push(Achievement {
            id: 34,
            title: "Walk of Shame Preparation".to_string(),
            description: "Mixing three different drink types in one hour".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![AchievementCondition::Custom(|session, _user| {
                let last_hour = chrono::Local::now() - chrono::Duration::hours(1);
                let recent_events = session
                    .events
                    .iter()
                    .filter(|e| e.timestamp > last_hour)
                    .collect::<Vec<_>>();

                let mut drink_types = std::collections::HashSet::new();
                for event in recent_events {
                    drink_types.insert(event.drink_type);
                }
                drink_types.len() >= 3
            })],
        });
        self.achievements.push(Achievement {
            id: 35,
            title: "Pants Optional".to_string(),
            description: "BAC over 0.12, three different drinks and more than 7 total".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![AchievementCondition::Custom(|session, _| {
                // Mix of high BAC, lots of different drink types, and late night
                let unique_drinks = session
                    .events
                    .iter()
                    .map(|e| e.drink_type)
                    .collect::<std::collections::HashSet<_>>()
                    .len();
                session.current_bac >= 0.12 && unique_drinks >= 3 && session.total_drinks() >= 7
            })],
        });
        self.achievements.push(Achievement {
            id: 36,
            title: "Gang Bang".to_string(),
            description: "Have 4 different drink types in less than 2 hours".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![
                AchievementCondition::DrinkTypeCount(DrinkType::Beer, 1),
                AchievementCondition::DrinkTypeCount(DrinkType::Wine, 1),
                AchievementCondition::DrinkTypeCount(DrinkType::Shot, 1),
                AchievementCondition::DrinkTypeCount(DrinkType::MixedDrink, 1),
                AchievementCondition::DrinksWithinDuration(4, Duration::hours(2)),
            ],
        });
        self.achievements.push(Achievement {
            id: 37,
            title: "Not Your First Rodeo".to_string(),
            description: "Start strong with 3 drinks in the first 25 minutes".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![AchievementCondition::Custom(|session, _user| {
                if let Some(start_time) = session.start_time {
                    let early_drinks = session
                        .events
                        .iter()
                        .filter(|e| e.timestamp - start_time <= Duration::minutes(25))
                        .count();
                    return early_drinks >= 3;
                }
                false
            })],
        });

        // Platinum
        self.achievements.push(Achievement {
            id: 38,
            title: "Liquid Diet".to_string(),
            description: "Crush 10 drinks in a night".to_string(),
            tier: AchievementTier::Platinum,
            conditions: vec![AchievementCondition::TotalDrinks(10)],
        });
        self.achievements.push(Achievement {
            id: 39,
            title: "Liquid Shits".to_string(),
            description: "Slam twelve beers in a night".to_string(),
            tier: AchievementTier::Platinum,
            conditions: vec![AchievementCondition::DrinkTypeCount(DrinkType::Beer, 12)],
        });
        self.achievements.push(Achievement {
            id: 40,
            title: "Walk of Shame".to_string(),
            description: "Hit a BAC of 0.22 – hope you're not wearing heels".to_string(),
            tier: AchievementTier::Silver,
            conditions: vec![AchievementCondition::MinBAC(0.22)],
        });
        self.achievements.push(Achievement {
            id: 41,
            title: "Tequila Mockingbird".to_string(),
            description: "Sixth shot of the night, baby".to_string(),
            tier: AchievementTier::Platinum,
            conditions: vec![AchievementCondition::DrinkTypeCount(DrinkType::Shot, 6)],
        });
        self.achievements.push(Achievement {
            id: 42,
            title: "Bottom of the Bottle".to_string(),
            description: "Hit seven glasses of wine".to_string(),
            tier: AchievementTier::Platinum,
            conditions: vec![AchievementCondition::DrinkTypeCount(DrinkType::Wine, 7)],
        });
        self.achievements.push(Achievement {
            id: 45,
            title: "Tomorrow is Cancelled".to_string(),
            description: "More than 12 drinks and at least one of each drink type".to_string(),
            tier: AchievementTier::Platinum,
            conditions: vec![AchievementCondition::Custom(|session, _| {
                let bac = session.current_bac;
                let high_drink_count = session.total_drinks() >= 12;
                let all_types_mixed = {
                    let types = session
                        .events
                        .iter()
                        .map(|e| e.drink_type)
                        .collect::<std::collections::HashSet<_>>();
                    types.contains(&DrinkType::Beer)
                        && types.contains(&DrinkType::Wine)
                        && types.contains(&DrinkType::Shot)
                        && types.contains(&DrinkType::MixedDrink)
                };

                bac >= 0.18 && high_drink_count && all_types_mixed
            })],
        });
        self.achievements.push(Achievement {
            id: 46,
            title: "Alcohol Slut".to_string(),
            description: "Have at least 2 of each type of drink in a single night".to_string(),
            tier: AchievementTier::Platinum,
            conditions: vec![
                AchievementCondition::DrinkTypeCount(DrinkType::Beer, 2),
                AchievementCondition::DrinkTypeCount(DrinkType::Wine, 2),
                AchievementCondition::DrinkTypeCount(DrinkType::Shot, 2),
                AchievementCondition::DrinkTypeCount(DrinkType::MixedDrink, 2),
            ],
        });
        self.achievements.push(Achievement {
            id: 47,
            title: "Deep Throat Champion".to_string(),
            description: "5 drinks in 30 minutes - your gag reflex is hot".to_string(),
            tier: AchievementTier::Platinum,
            conditions: vec![
                AchievementCondition::TotalDrinks(5),
                AchievementCondition::DrinksWithinDuration(5, Duration::minutes(30)),
            ],
        });
        self.achievements.push(Achievement {
            id: 48,
            title: "Unholy Debauchery".to_string(),
            description: "15 drinks and a BAC above 0.27 – you're a goddamn legend.".to_string(),
            tier: AchievementTier::Platinum,
            conditions: vec![
                AchievementCondition::TotalDrinks(15),
                AchievementCondition::MinBAC(0.27),
            ],
        });
        self.achievements.push(Achievement {
            id: 49,
            title: "Booze Hound".to_string(),
            description: "15 drinks and a BAC of 0.30 – you fiesty beast.".to_string(),
            tier: AchievementTier::Platinum,
            conditions: vec![
                AchievementCondition::TotalDrinks(15),
                AchievementCondition::MinBAC(0.30),
            ],
        });
        self.achievements.push(Achievement {
            id: 50,
            title: "Sloppy Speedster".to_string(),
            description: "8 drinks in 45 minutes—puke and rally, champ!".to_string(),
            tier: AchievementTier::Platinum,
            conditions: vec![AchievementCondition::DrinksWithinDuration(
                8,
                Duration::minutes(45),
            )],
        });
        self.achievements.push(Achievement {
            id: 51,
            title: "Shotgun Shitshow".to_string(),
            description: "Alternate beer and shots 4 times—sloshed perfection!".to_string(),
            tier: AchievementTier::Platinum,
            conditions: vec![AchievementCondition::Custom(|session, _| {
                if session.events.len() < 4 {
                    return false;
                }
                let sequence = &session.events[..4];
                sequence.iter().enumerate().all(|(i, event)| {
                    if i % 2 == 0 {
                        event.drink_type == DrinkType::Beer
                    } else {
                        event.drink_type == DrinkType::Shot
                    }
                })
            })],
        });
        self.achievements.push(Achievement {
            id: 52,
            title: "Drunk Unicorn".to_string(),
            description: "BAC > 0.15 with exactly 7 drinks, no repeats—mythical madness!"
                .to_string(),
            tier: AchievementTier::Platinum,
            conditions: vec![AchievementCondition::Custom(|session, _| {
                let bac = session.current_bac;
                let types: std::collections::HashSet<DrinkType> = session
                    .events
                    .iter()
                    .map(|e| e.drink_type)
                    .filter(|&dt| dt != DrinkType::None)
                    .collect();
                bac > 0.15 && session.events.len() == 7 && types.len() == session.events.len()
            })],
        });
        self.achievements.push(Achievement {
            id: 53,
            title: "Cocktail Clusterfuck".to_string(),
            description: "5 mixed drinks, BAC over 0.12, under 45 mins—total trainwreck!"
                .to_string(),
            tier: AchievementTier::Platinum,
            conditions: vec![AchievementCondition::Custom(|session, _| {
                let bac = session.current_bac;
                let mixed_count = session.count_by(DrinkType::MixedDrink);
                let recent_five = session
                    .events
                    .iter()
                    .rev()
                    .filter(|e| e.drink_type == DrinkType::MixedDrink)
                    .take(5)
                    .collect::<Vec<_>>();
                if recent_five.len() < 5 {
                    return false;
                }
                let time_span =
                    recent_five.first().unwrap().timestamp - recent_five.last().unwrap().timestamp;
                mixed_count >= 5 && bac > 0.12 && time_span < Duration::minutes(45)
            })],
        });
        self.achievements.push(Achievement {
            id: 54,
            title: "Tom Hanks".to_string(),
            description: "Hit a BAC of 0.35 – you are the captain now ".to_string(),
            tier: AchievementTier::Platinum,
            conditions: vec![AchievementCondition::MinBAC(0.35)],
        });
        self.achievements.push(Achievement {
            id: 55,
            title: "Donald Trump".to_string(),
            description: "Hit a BAC of 0.40 – you are where you shouldn't be ".to_string(),
            tier: AchievementTier::Platinum,
            conditions: vec![AchievementCondition::MinBAC(0.40)],
        });
    }
}
