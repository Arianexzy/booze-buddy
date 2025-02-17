use crate::storage::{
    models::DrinkType,
    storage::drink_history::{add_drink_by, get_count_by},
};
use dioxus::prelude::*;

const DRINKS_CSS: Asset = asset!("/assets/styling/drinks.css");

#[derive(Debug)]
pub struct DrinkDisplay {
    pub label: &'static str,
    pub icon: &'static str,
    pub drink_type: DrinkType,
}

pub const DRINK_TYPES: [DrinkDisplay; 4] = [
    DrinkDisplay {
        label: "Beer",
        icon: "🍺",
        drink_type: DrinkType::Beer,
    },
    DrinkDisplay {
        label: "Wine",
        icon: "🍷",
        drink_type: DrinkType::Wine,
    },
    DrinkDisplay {
        label: "Mixed Drink",
        icon: "🍸",
        drink_type: DrinkType::MixedDrink,
    },
    DrinkDisplay {
        label: "Shot",
        icon: "🥃",
        drink_type: DrinkType::Shot,
    },
];

#[derive(PartialEq, Clone, Props)]
pub struct DrinksProps {
    on_drink_added: EventHandler<()>,
}

#[component]
pub fn Drinks(props: DrinksProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: DRINKS_CSS }

        div { class: "drinks-container",
            div { class: "drink-buttons",
                {
                    DRINK_TYPES
                        .iter()
                        .map(|drink| {
                            let mut drink_count = use_resource(move || async move {
                                get_count_by(drink.drink_type)
                            });
                            let count = match &*drink_count.read_unchecked() {
                                Some(count) => *count,
                                _ => 0,
                            };
                            rsx! {
                                button {
                                    class: "drink-button",
                                    key: "{drink.label}",
                                    ondoubleclick: move |_| {
                                        add_drink_by(drink.drink_type);
                                        drink_count.restart();
                                        props.on_drink_added.call(());
                                    },
                                    div { class: "drink-button-icon", "{drink.icon}" }
                                    span { class: "drink-button-label", "{drink.label}" }
                                    span { class: "drink-count", "{count}" }
                                }
                            }
                        })
                }
            }
        }
    }
}
