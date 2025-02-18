use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub weight: f32,
    pub gender: Gender,
}

impl Default for User {
    fn default() -> Self {
        User {
            weight: 150.0,
            gender: Gender::Other,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AppSettings {
    pub user: Option<User>,
}
