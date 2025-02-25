use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Hash, Eq)]
pub enum DrinkType {
    Beer,
    Wine,
    Shot,
    MixedDrink,
    None,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DrinkEvent {
    pub drink_type: DrinkType,
    pub timestamp: DateTime<Local>,
}
