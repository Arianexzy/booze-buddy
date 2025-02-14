use dioxus::prelude::*;
use crate::storage::{storage, models::DrinkType};

const DRINKS_CSS: Asset = asset!("/assets/styling/drinks.css");

#[derive(Debug)]
pub struct DrinkDisplay {
    pub label: &'static str,
    pub emoji: &'static str,
    pub drink_type: DrinkType,
}

pub const DRINK_TYPES: [DrinkDisplay; 4] = [
    DrinkDisplay {
        label: "Beer",
        emoji: "🍺",
        drink_type: DrinkType::Beer,
    },
    DrinkDisplay {
        label: "Wine",
        emoji: "🍷",
        drink_type: DrinkType::Wine,
    },
    DrinkDisplay {
        label: "Mixed Drink",
        emoji: "🍸",
        drink_type: DrinkType::MixedDrink,
    },
    DrinkDisplay {
        label: "Shot",
        emoji: "🥃",
        drink_type: DrinkType::Shot,
    },
];

#[component]
pub fn Drinks() -> Element {
    let mut drink_selected = use_signal(|| DrinkType::Beer);
    let mut drink_count = use_signal(|| storage::get_count_by(drink_selected()));

    rsx! {
        document::Link { rel: "stylesheet", href: DRINKS_CSS }

        div { class: "drinks-container",
            div { class: "drink-buttons",
                { DRINK_TYPES
                    .iter()
                    .map(|drink| {
                        let is_selected = drink_selected() == drink.drink_type;
                        rsx! {
                            button {
                                class: if is_selected {"drink-button selected"} else {"drink-button"},
                                key: "{drink.label}",
                                onclick: move |_| {
                                    drink_selected.set(drink.drink_type);
                                    drink_count.set(storage::get_count_by(drink.drink_type));
                                },
                                div { class: "drink-button-icon", "{drink.emoji}" }
                                span { class: "drink-button-label", "{drink.label}" }
                            }
                        }
                    })
                }
            }
            div { class: "counter-buttons",
                button { 
                    class: "increment-button", 
                    onclick: move |_| {
                        storage::add_drink_by(drink_selected());
                        drink_count.set(storage::get_count_by(drink_selected()));
                    },
                    "+",
                    "{drink_count()}",
                }
                button { class: "decrement-button", "-" }
            }
        }
    }
}
