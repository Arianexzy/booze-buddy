use super::constants::{DRINK_ALCOHOL_CONTENT, METABOLISM_RATE};
use super::user::{Gender, User};
use super::{DrinkEvent, DrinkType};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DrinkingSession {
    pub events: Vec<DrinkEvent>,
    pub is_active: bool,
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

    pub fn calculate_bac(&self, user: &User) -> f32 {
        let now = Local::now();

        let gender_constant = match user.gender {
            Gender::Male => 0.68,
            Gender::Female => 0.55,
            Gender::Other => 0.615,
        };

        self.events.iter().fold(0.0, |acc, event| {
            let hours_since_drink =
                now.signed_duration_since(event.timestamp).num_minutes() as f32 / 60.0;

            let drink_bac = (DRINK_ALCOHOL_CONTENT * 5.14 / (user.weight * gender_constant))
                - (METABOLISM_RATE * hours_since_drink);

            acc + drink_bac.max(0.0) // prevent negative BAC values
        })
    }
}
