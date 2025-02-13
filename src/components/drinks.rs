use dioxus::prelude::*;

const DRINKS_CSS: Asset = asset!("/assets/styling/drinks.css");

#[derive(Debug)]
pub struct DrinkType {
    pub label: &'static str,
    pub emoji: &'static str,
}

pub const DRINK_TYPES: [DrinkType; 4] = [
    DrinkType {
        label: "Beer",
        emoji: "🍺",
    },
    DrinkType {
        label: "Wine",
        emoji: "🍷",
    },
    DrinkType {
        label: "Mixed Drink",
        emoji: "🍸",
    },
    DrinkType {
        label: "Shot",
        emoji: "🥃",
    },
];

fn handle_onclick(drink: &DrinkType) {
    println!("Clicked: {:?}", drink.label);
}

#[component]
pub fn Drinks() -> Element {
    let mut drink_selected = use_signal(|| "Beer");

    rsx! {
        document::Link { rel: "stylesheet", href: DRINKS_CSS }

        div { class: "drinks-container",
            div { class: "drink-buttons",
                {
                    DRINK_TYPES
                        .iter()
                        .map(|drink| {
                            let is_selected = drink_selected() == drink.label;
                            let button_class = if is_selected {
                                "drink-button selected"
                            } else {
                                "drink-button"
                            };
                            rsx! {
                                button {
                                    class: button_class,
                                    key: "{drink.label}",
                                    onclick: move |_| drink_selected.set(drink.label),
                                    div { class: "drink-button-icon", "{drink.emoji}" }
                                    span { class: "drink-button-label", "{drink.label}" }
                                }
                            }
                        })
                }
            }
            div { class: "counter-buttons",
                button { class: "increment-button", "+" }
                button { class: "decrement-button", "-" }
            }
        }
    }
}
