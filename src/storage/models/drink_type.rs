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

#[derive(Debug)]
pub struct DrinkDisplay {
    pub label: &'static str,
    pub icon: &'static str,
}

impl DrinkType {
    pub fn display(self) -> DrinkDisplay {
        match self {
            DrinkType::Beer => DrinkDisplay {
                label: "Beer",
                icon: "🍺",
            },
            DrinkType::Wine => DrinkDisplay {
                label: "Wine",
                icon: "🍷",
            },
            DrinkType::MixedDrink => DrinkDisplay {
                label: "Mixed Drink",
                icon: "🍸",
            },
            DrinkType::Shot => DrinkDisplay {
                label: "Shot",
                icon: "🥃",
            },
            DrinkType::None => DrinkDisplay {
                label: "None",
                icon: "❓",
            },
        }
    }

    pub const ALL: [DrinkType; 4] = [
        DrinkType::Beer,
        DrinkType::Wine,
        DrinkType::MixedDrink,
        DrinkType::Shot,
    ];
}
