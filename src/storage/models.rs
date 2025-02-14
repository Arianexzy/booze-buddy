use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum DrinkType {
    Beer,
    Wine,
    Shot,
    MixedDrink,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DrinkEvent {
    pub drink_type: DrinkType,
    pub timestamp: DateTime<Local>,
}

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
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DrinkingHistory {
    pub sessions: Vec<DrinkingSession>,
    pub current_session_index: Option<usize>,
}

impl DrinkingHistory {
    pub fn current_session(&mut self) -> Option<&mut DrinkingSession> {
        self.current_session_index.and_then(|index| self.sessions.get_mut(index))
    }
    
    pub fn start_new_session(&mut self) {
        let mut session = DrinkingSession::default();
        session.start();
        self.sessions.push(session);
        self.current_session_index = Some(self.sessions.len() - 1);
    }
    
    pub fn end_current_session(&mut self) {
        if let Some(index) = self.current_session_index {
            if let Some(session) = self.sessions.get_mut(index) {
                session.end();
            }
            self.current_session_index = None;
        }
    }

    pub fn past_sessions(&self) -> Vec<&DrinkingSession> {
        self.sessions
            .iter()
            .filter(|session| !session.is_active)
            .collect()
    }
}
