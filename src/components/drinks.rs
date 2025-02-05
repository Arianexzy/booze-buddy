use dioxus::prelude::*;

#[derive(Debug)]
struct DrinkType {
    label: &'static str,
    emoji: &'static str,
}

const DRINKS: [DrinkType; 4] = [
    DrinkType {
        label: "Beer",
        emoji: "🍺",
    },
    DrinkType {
        label: "Wine",
        emoji: "🍷",
    },
    DrinkType {
        label: "Cocktail",
        emoji: "🍸",
    },
    DrinkType {
        label: "Shot",
        emoji: "🥃",
    },
];

fn handle_onclick(drink: &DrinkType) {
    println!("Clicked: {:?}", drink);
}

#[component]
pub fn DrinkTypes() -> Element {
    rsx! {
        div { class: "",
            {
                DRINKS
                    .iter()
                    .map(|drink| {
                        rsx! {
                            button {
                                class: "",
                                key: "{drink.label}",
                                onclick: move |_| handle_onclick(drink),
                                div { class: "", "{drink.emoji}" }
                                span { class: "", "{drink.label}" }
                            }
                        }
                    })
            }
        }
    }
}
