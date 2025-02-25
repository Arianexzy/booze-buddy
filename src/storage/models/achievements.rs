use crate::storage::models::{DrinkingSession, DrinkType, User, Gender};
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Achievement {
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
            AchievementCondition::MinBAC(bac) => session.calculate_bac(user) >= *bac,
            AchievementCondition::DrinkTypeCount(drink_type, count) => session.count_by(*drink_type) >= *count,
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
}

pub struct AchievementRegistry {
    pub achievements: Vec<Achievement>,
}

impl AchievementRegistry {
    pub fn global() -> &'static AchievementRegistry {
        static REGISTRY: OnceLock<AchievementRegistry> = OnceLock::new();
        REGISTRY.get_or_init(|| {
            let mut registry = AchievementRegistry::new();
            registry.register_achievements();
            registry
        })
    }
    
    pub fn new() -> Self {
        Self { achievements: Vec::new() }
    }
    
    pub fn register_achievements(&mut self) {
        // Bronze
        self.achievements.push(Achievement {
            title: "Break The Seal".to_string(),
            description: "Have your first drink of the night".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::TotalDrinks(1)],
        });
        self.achievements.push(Achievement {
            title: "Wet Your Whistle".to_string(),
            description: "Have your second drink to really get things going.".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::TotalDrinks(2)],
        });
        self.achievements.push(Achievement {
            title: "Tipsy Turvy".to_string(),
            description: "Down three drinks and you're off to a shaky start.".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::TotalDrinks(3)],
        });
        self.achievements.push(Achievement {
            title: "One Pump Chump".to_string(),
            description: "Down a shot in the first hour of starting.".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![
                AchievementCondition::DrinkTypeCount(DrinkType::Shot, 1),
                AchievementCondition::DrinksWithinDuration(1, Duration::minutes(30)),
            ],
        });
        self.achievements.push(Achievement {
            title: "Wine Whiner".to_string(),
            description: "Have a glass of wine and complain about life.".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::DrinkTypeCount(DrinkType::Wine, 1)],
        });
        self.achievements.push(Achievement {
            title: "Beer Goggles On".to_string(),
            description: "Reach a BAC of 0.04% - things are starting to look better!".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::MinBAC(0.04)],
        });
        self.achievements.push(Achievement {
            title: "Shot Caller".to_string(),
            description: "Take your first shot of the night".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::DrinkTypeCount(DrinkType::Shot, 1)],
        });
        self.achievements.push(Achievement {
            title: "That's What She Said".to_string(),
            description: "Down 2 drinks within 30 minutes".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::DrinksWithinDuration(2, Duration::minutes(30))],
        });
        self.achievements.push(Achievement {
            title: "Buzz Lightyear".to_string(),
            description: "Reach a BAC of 0.06% - to intoxication and beyond!".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::MinBAC(0.06)],
        });
        self.achievements.push(Achievement {
            title: "Slowpoke Sipper".to_string(),
            description: "Take over 30 minutes to finish your first drink—pathetic!".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::Custom(|session, _| {
                if session.events.len() != 1 { return false; }
                let first_drink = session.events.first().unwrap();
                let time_since = chrono::Local::now() - first_drink.timestamp;
                time_since > Duration::minutes(30)
            })],
        });
        self.achievements.push(Achievement {
            title: "Lightweight Limp".to_string(),
            description: "Feel buzzed (BAC > 0.03) under 170 lbs—weak sauce!".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::Custom(|session, user| {
                session.calculate_bac(user) > 0.03 && user.weight < 170.0
            })],
        });
        
        // Silver
        self.achievements.push(Achievement {
            title: "Two Pump Chump".to_string(),
            description: "Down two shots".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::DrinkTypeCount(DrinkType::Shot, 2)],
        });
        self.achievements.push(Achievement {
            title: "Breaking Bad".to_string(),
            description: "Crush four drinks in a night".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::TotalDrinks(4)],
        });
        self.achievements.push(Achievement {
            title: "Wine a bit, you'll feel better".to_string(),
            description: "Throw down three glasses of wine".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::DrinkTypeCount(DrinkType::Wine, 3)],
        });
        self.achievements.push(Achievement {
            title: "Beer Belly".to_string(),
            description: "Hammer five beers".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::DrinkTypeCount(DrinkType::Beer, 5)],
        });
        self.achievements.push(Achievement {
            title: "Pants Optional".to_string(),
            description: "Reach a BAC of 0.08%".to_string(),
            tier: AchievementTier::Silver,
            conditions: vec![AchievementCondition::MinBAC(0.08)],
        });
        self.achievements.push(Achievement {
            title: "Mixed Up Like Milkshake".to_string(),
            description: "Have two mixed drinks in one night".to_string(),
            tier: AchievementTier::Silver,
            conditions: vec![AchievementCondition::DrinkTypeCount(DrinkType::MixedDrink, 2)],
        });
        self.achievements.push(Achievement {
                title: "Slippery When Wet".to_string(),
                description: "Hit a BAC of 0.10 – time to get a little sloppy!".to_string(),
                tier: AchievementTier::Silver,
                conditions: vec![AchievementCondition::MinBAC(0.10)],
        });
        self.achievements.push(Achievement {
            title: "Gender Bender".to_string(),
            description: "Men over 200 lbs or women under 130 lbs hit BAC 0.08—flip the script!".to_string(),
            tier: AchievementTier::Silver,
            conditions: vec![AchievementCondition::Custom(|session, user| {
                let bac = session.calculate_bac(user);
                match user.gender {
                    Gender::Male => bac >= 0.08 && user.weight > 200.0,
                    Gender::Female => bac >= 0.08 && user.weight < 130.0,
                    _ => false, // Other gender doesn’t qualify
                }
            })],
        });
        
        // Gold
        self.achievements.push(Achievement {
            title: "Break Dance".to_string(),
            description: "Crush six drinks in a night".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::TotalDrinks(6)],
        });
        self.achievements.push(Achievement {
            title: "Juicy Booty Shorts".to_string(),
            description: "Slam eight beers in a night".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::DrinkTypeCount(DrinkType::Beer, 8)],
        });
        self.achievements.push(Achievement {
            title: "Juggalo 4 Life".to_string(),
            description: "Hit a BAC of 0.16 – you insane clown possy".to_string(),
            tier: AchievementTier::Silver,
            conditions: vec![AchievementCondition::MinBAC(0.16)],
        });
        self.achievements.push(Achievement {
            title: "Whiskey Dick".to_string(),
            description: "Down three shots".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::DrinkTypeCount(DrinkType::Shot, 3)],
        });
        self.achievements.push(Achievement {
            title: "Carpet Muncher".to_string(),
            description: "Five glasses of wine - your face is gonna hit the floor".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::DrinkTypeCount(DrinkType::Wine, 5)],
        });
        self.achievements.push(Achievement {
            title: "Cocktail Cocktease".to_string(),
            description: "Have 2 mixed drinks in 30 minutes—slow down, tiger.".to_string(),
            tier: AchievementTier::Silver,
            conditions: vec![
                AchievementCondition::DrinkTypeCount(DrinkType::MixedDrink, 2),
                AchievementCondition::DrinksWithinDuration(2, Duration::minutes(30)),
            ],
        });
        self.achievements.push(Achievement {
            title: "Staggered Stumbler".to_string(),
            description: "Space 3 drinks exactly 10-15 minutes apart—drunk clockwork!".to_string(),
            tier: AchievementTier::Silver,
            conditions: vec![AchievementCondition::Custom(|session, _| {
                if session.events.len() < 3 { return false; }
                let windows = session.events.windows(2);
                windows.enumerate().all(|(i, window)| {
                    if i >= 2 { return true; } // Only check first two gaps
                    let gap = window[1].timestamp - window[0].timestamp;
                    gap >= Duration::minutes(10) && gap <= Duration::minutes(15)
                })
            })],
        });
        self.achievements.push(Achievement {
            title: "BAC-kward Ass".to_string(),
            description: "BAC drops below 0.05 after peaking above 0.10—sober up, dumbas".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![AchievementCondition::Custom(|session, user| {
                let current_bac = session.calculate_bac(user);
                let peak_bac = session.events.iter().fold(0.0, |acc: f32, event| {
                    let partial_session = DrinkingSession {
                        events: session.events[..session.events.iter().position(|e| e == event).unwrap() + 1].to_vec(),
                        ..session.clone()
                    };
                    let bac = partial_session.calculate_bac(user);
                    acc.max(bac)
                });
                current_bac < 0.05 && peak_bac > 0.10
            })],
        });
        self.achievements.push(Achievement {
            title: "Rapid Pounding".to_string(),
            description: "Have 3 drinks in less than 30 minutes".to_string(),
            tier: AchievementTier::Silver,
            conditions: vec![
                AchievementCondition::TotalDrinks(3),
                AchievementCondition::DrinksWithinDuration(3, Duration::minutes(30)),
            ],
        });
        self.achievements.push(Achievement {
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
            title: "Swallow Don't Spit".to_string(),
            description: "Down five shots".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![
                AchievementCondition::DrinkTypeCount(DrinkType::Shot, 5),
            ],
        });
        self.achievements.push(Achievement {
            title: "Double Penetration".to_string(),
            description: "Finish two different drink types simultaneously (within 20 minutes of each other)".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![
                AchievementCondition::Custom(|session, _user| {
                    let mut events = session.events.clone();
                    events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
                    
                    for i in 1..events.len() {
                        if events[i].drink_type != events[i-1].drink_type {
                            let duration = events[i].timestamp - events[i-1].timestamp;
                            if duration <= Duration::minutes(20) {
                                return true;
                            }
                        }
                    }
                    false
                }),
            ],
        });
        self.achievements.push(Achievement {
            title: "Five Knuckle Shuffle".to_string(),
            description: "Down 5 drinks in 1hour — hand’s getting busy!".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![AchievementCondition::DrinksWithinDuration(5, Duration::minutes(60))],
        });
        self.achievements.push(Achievement {
            title: "Tequila Titty-Twister".to_string(),
            description: "4 shots in 1 hour — get limber!".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![
                AchievementCondition::DrinkTypeCount(DrinkType::Shot, 4),
                AchievementCondition::DrinksWithinDuration(4, Duration::minutes(60)),
            ],
        });
        self.achievements.push(Achievement {
            title: "Walk of Shame Preparation".to_string(),
            description: "Mixing three different drink types in one hour".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::Custom(|session, _user| {
                let last_hour = chrono::Local::now() - chrono::Duration::hours(1);
                let recent_events = session.events.iter()
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
            title: "Hangover From Hell".to_string(),
            description: "BAC over 0.12 with three different drinks and more than 7 total drinks".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![AchievementCondition::Custom(|session, user| {
                // Mix of high BAC, lots of different drink types, and late night
                let unique_drinks = session.events.iter()
                    .map(|e| e.drink_type)
                    .collect::<std::collections::HashSet<_>>()
                    .len();
                session.calculate_bac(user) >= 0.12 && 
                unique_drinks >= 3 && 
                session.total_drinks() >= 7            
            })],
        });
        self.achievements.push(Achievement {
            title: "Gang Bang".to_string(),
            description: "Have 4 different drink types in less than 2 hours".to_string(),
            tier: AchievementTier::Platinum,
            conditions: vec![
                AchievementCondition::DrinkTypeCount(DrinkType::Beer, 1),
                AchievementCondition::DrinkTypeCount(DrinkType::Wine, 1),
                AchievementCondition::DrinkTypeCount(DrinkType::Shot, 1),
                AchievementCondition::DrinkTypeCount(DrinkType::MixedDrink, 1),
                AchievementCondition::DrinksWithinDuration(4, Duration::hours(2)),
            ],
        });
        self.achievements.push(Achievement {
            title: "Not Your First Rodeo".to_string(),
            description: "Start strong with 3 drinks in the first 20 minutes".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![
                AchievementCondition::Custom(|session, _user| {
                    if let Some(start_time) = session.start_time {
                        let early_drinks = session.events.iter()
                            .filter(|e| e.timestamp - start_time <= Duration::minutes(20))
                            .count();
                        return early_drinks >= 3;
                    }
                    false
                }),
            ],
        });
        
        // Platinum
        self.achievements.push(Achievement {
            title: "Broke Ass Bitch".to_string(),
            description: "Crush 10 drinks in a night".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::TotalDrinks(10)],
        });
        self.achievements.push(Achievement {
            title: "Liquid Shits".to_string(),
            description: "Slam twelve beers in a night".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::DrinkTypeCount(DrinkType::Beer, 12)],
        });
        self.achievements.push(Achievement {
            title: "Walk of Shame".to_string(),
            description: "Hit a BAC of 0.24 – hope you're not wearing heels".to_string(),
            tier: AchievementTier::Silver,
            conditions: vec![AchievementCondition::MinBAC(0.24)],
        });
        self.achievements.push(Achievement {
            title: "Tequila Mockingbird".to_string(),
            description: "Down five shots".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::DrinkTypeCount(DrinkType::Shot, 5)],
        });
        self.achievements.push(Achievement {
            title: "Bottom of the Bottle".to_string(),
            description: "Hit seven glasses of wine".to_string(),
            tier: AchievementTier::Bronze,
            conditions: vec![AchievementCondition::DrinkTypeCount(DrinkType::Wine, 7)],
        });
        self.achievements.push(Achievement {
            title: "Don't Text your Ex".to_string(),
            description: "Hit a BAC of 0.30 – they don't deserve you".to_string(),
            tier: AchievementTier::Silver,
            conditions: vec![AchievementCondition::MinBAC(0.30)],
        });
        self.achievements.push(Achievement {
            title: "Porcelain Throne Awaits".to_string(),
            description: "Hit a BAC of 0.35 – they don't deserve you".to_string(),
            tier: AchievementTier::Silver,
            conditions: vec![AchievementCondition::MinBAC(0.35)],
        });
        self.achievements.push(Achievement {
            title: "Long Live the Liver".to_string(),
            description: "Hit a BAC of 0.40 – Hail thy Liver for thy Liver is supreme".to_string(),
            tier: AchievementTier::Silver,
            conditions: vec![AchievementCondition::MinBAC(0.40)],
        });
        self.achievements.push(Achievement {
            title: "Tomorrow is Cancelled".to_string(),
            description: "More than 12 drinks and at least one of each drink type".to_string(),
            tier: AchievementTier::Platinum,
            conditions: vec![AchievementCondition::Custom(|session, user| {
                let bac = session.calculate_bac(user);
                let high_drink_count = session.total_drinks() >= 12;
                let all_types_mixed = {
                    let types = session.events.iter()
                        .map(|e| e.drink_type)
                        .collect::<std::collections::HashSet<_>>();
                    types.contains(&DrinkType::Beer) && 
                    types.contains(&DrinkType::Wine) && 
                    types.contains(&DrinkType::Shot) && 
                    types.contains(&DrinkType::MixedDrink)
                };
                
                bac >= 0.18 && high_drink_count && all_types_mixed
            })],
        });
        self.achievements.push(Achievement {
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
            title: "Deep Throat Champion".to_string(),
            description: "Down 5 drinks within 30 minutes - your gag reflex is impressive".to_string(),
            tier: AchievementTier::Platinum,
            conditions: vec![
                AchievementCondition::TotalDrinks(5),
                AchievementCondition::DrinksWithinDuration(5, Duration::minutes(30)),
            ],
        });
        self.achievements.push(Achievement {
            title: "Unholy Debauchery".to_string(),
            description: "15 drinks and a BAC above 0.25 – you're a goddamn legend.".to_string(),
            tier: AchievementTier::Platinum,
            conditions: vec![
                AchievementCondition::TotalDrinks(15),
                AchievementCondition::MinBAC(0.25),
            ],
        });
        self.achievements.push(Achievement {
            title: "Booze Hound".to_string(),
            description: "15 drinks and a BAC of 0.30 – you fiesty beast.".to_string(),
            tier: AchievementTier::Platinum,
            conditions: vec![
                AchievementCondition::TotalDrinks(15),
                AchievementCondition::MinBAC(0.30),
            ],
        });
        self.achievements.push(Achievement {
            title: "Sloppy Speedster".to_string(),
            description: "8 drinks in 45 minutes—puke and rally, champ!".to_string(),
            tier: AchievementTier::Platinum,
            conditions: vec![AchievementCondition::DrinksWithinDuration(8, Duration::minutes(45))],
        });
        self.achievements.push(Achievement {
            title: "Shotgun Shitshow".to_string(),
            description: "Alternate beer and shots 4 times—sloshed perfection!".to_string(),
            tier: AchievementTier::Gold,
            conditions: vec![AchievementCondition::Custom(|session, _| {
                if session.events.len() < 4 { return false; }
                let sequence = &session.events[..4];
                sequence.iter().enumerate().all(|(i, event)| {
                    if i % 2 == 0 { event.drink_type == DrinkType::Beer }
                    else { event.drink_type == DrinkType::Shot }
                })
            })],
        });
        self.achievements.push(Achievement {
            title: "Drunk Unicorn".to_string(),
            description: "BAC > 0.15 with exactly 7 drinks, no repeats—mythical madness!".to_string(),
            tier: AchievementTier::Platinum,
            conditions: vec![AchievementCondition::Custom(|session, user| {
                let bac = session.calculate_bac(user);
                let types: std::collections::HashSet<DrinkType> = session.events.iter()
                    .map(|e| e.drink_type)
                    .filter(|&dt| dt != DrinkType::None)
                    .collect();
                bac > 0.15 && session.events.len() == 7 && types.len() == session.events.len()
            })],
        });
        self.achievements.push(Achievement {
            title: "Cocktail Clusterfuck".to_string(),
            description: "5 mixed drinks, BAC over 0.12, under 45 mins—total trainwreck!".to_string(),
            tier: AchievementTier::Platinum,
            conditions: vec![AchievementCondition::Custom(|session, user| {
                let bac = session.calculate_bac(user);
                let mixed_count = session.count_by(DrinkType::MixedDrink);
                let recent_five = session.events.iter().rev()
                    .filter(|e| e.drink_type == DrinkType::MixedDrink)
                    .take(5)
                    .collect::<Vec<_>>();
                if recent_five.len() < 5 { return false; }
                let time_span = recent_five.first().unwrap().timestamp - recent_five.last().unwrap().timestamp;
                mixed_count >= 5 && bac > 0.12 && time_span < Duration::minutes(45)
            })],
        });
    }   
}
