use super::DrinkingSession;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DrinkingHistory {
    pub sessions: Vec<DrinkingSession>,
    pub current_session_index: Option<usize>,
}

impl DrinkingHistory {
    pub fn ensure_active_session(&mut self) -> &mut DrinkingSession {
        if self.current_session().is_none() {
            self.start_new_session();
        }
        self.current_session().unwrap()
    }

    pub fn current_session(&mut self) -> Option<&mut DrinkingSession> {
        self.current_session_index
            .and_then(|index| self.sessions.get_mut(index))
    }

    fn start_new_session(&mut self) {
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

    pub fn past_sessions(&self) -> Vec<DrinkingSession> {
        self.sessions
            .iter()
            .filter(|session| !session.is_active)
            .cloned()
            .collect()
    }
}
