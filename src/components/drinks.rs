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
        div { class: "grid grid-cols-3 sm:grid-cols-5 gap-2 px-4 my-8", 
            {  
                DRINKS
                    .iter()
                    .map(|drink| {
                        rsx! {
                            button {
                                class: "p-3 rounded-xl flex flex-col items-center gap-2 transition-all duration-200 ease-in-out bg-zinc-800 text-rose-100",
                                key: "{drink.label}",
                                onclick: move |_| handle_onclick(drink),
                                div { class: "text-3xl transition-transform", "{drink.emoji}" }
                                span { class: "text-xs sm:text-sm font-medium", "{drink.label}" }
                            }
                        }
                    })
            }
        }
    }
}
