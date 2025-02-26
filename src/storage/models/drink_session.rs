use super::{
    Achievement, AchievementRegistry, DRINK_ALCOHOL_CONTENT, DrinkEvent, DrinkType, Gender,
    METABOLISM_RATE, User,
};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DrinkingSession {
    pub events: Vec<DrinkEvent>,
    pub achievements: Vec<Achievement>,
    pub is_active: bool,
    pub current_bac: f32,
    pub max_bac: f32,
    pub start_time: Option<DateTime<Local>>,
    pub end_time: Option<DateTime<Local>>,
}

impl DrinkingSession {
    pub fn start(&mut self) {
        self.is_active = true;
        self.start_time = Some(Local::now());
    }

    pub fn end(&mut self) {
        self.is_active = false;
        self.end_time = Some(Local::now());
    }

    pub fn count_by(&self, drink_type: DrinkType) -> i32 {
        self.events
            .iter()
            .filter(|event| event.drink_type == drink_type)
            .count() as i32
    }

    pub fn add_drink_by(&mut self, drink_type: DrinkType) {
        self.events.push(DrinkEvent {
            drink_type,
            timestamp: Local::now(),
        })
    }

    pub fn remove_last(&mut self, drink_type: DrinkType) {
        if let Some(index) = self
            .events
            .iter()
            .rposition(|event| event.drink_type == drink_type)
        {
            self.events.remove(index);
        }
    }

    pub fn total_drinks(&self) -> i32 {
        self.events.len() as i32
    }

    pub fn calculate_bac(&mut self, user: &User) -> f32 {
        let now = Local::now();

        let gender_constant = match user.gender {
            Gender::Male => 0.68,
            Gender::Female => 0.55,
            Gender::Other => 0.615,
        };

        let calculated_bac = self.events.iter().fold(0.0, |acc, event| {
            let hours_since_drink =
                now.signed_duration_since(event.timestamp).num_minutes() as f32 / 60.0;

            let drink_bac = (DRINK_ALCOHOL_CONTENT * 5.14 / (user.weight * gender_constant))
                - (METABOLISM_RATE * hours_since_drink);

            acc + drink_bac.max(0.0) // prevent negative BAC values
        });

        self.current_bac = calculated_bac;
        self.max_bac = calculated_bac.max(self.max_bac);

        calculated_bac
    }

    pub fn check_achievements(&mut self, user: &User) -> Vec<Achievement> {
        let mut newly_unlocked = Vec::new();
        let registry = AchievementRegistry::global();
        for achievement in &registry.achievements {
            if !self.achievements.contains(&achievement) && achievement.is_achieved(self, user) {
                self.achievements.push(achievement.clone());
                newly_unlocked.push(achievement.clone());
            }
        }
        newly_unlocked
    }
}
