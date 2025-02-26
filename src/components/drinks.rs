use crate::storage::{
    models::DrinkType,
    storage::drink_history::{add_drink_by, get_count_by},
};
use dioxus::prelude::*;

const DRINKS_CSS: Asset = asset!("/assets/styling/drinks.css");

#[derive(PartialEq, Clone, Props)]
pub struct DrinksProps {
    on_drink_added: EventHandler<()>,
    reset_drink_count: Signal<i32>,
}

#[component]
pub fn Drinks(props: DrinksProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: DRINKS_CSS }

        div { class: "drinks-container",
            div { class: "drink-buttons",
                {
                    DrinkType::ALL
                        .iter()
                        .map(|drink| {
                            let mut drink_count_resource = use_resource(move || async move {
                                props.reset_drink_count.read();
                                get_count_by(*drink).unwrap_or(0)
                            });
                            let drink_count = drink_count_resource().unwrap_or(0);
                            rsx! {
                                button {
                                    class: "drink-button",
                                    key: "{drink.display().label}",
                                    ondoubleclick: move |_| {
                                        add_drink_by(*drink).expect("Failed to add drink");
                                        drink_count_resource.restart();
                                        props.on_drink_added.call(());
                                    },
                                    div { class: "drink-button-icon", "{drink.display().icon}" }
                                    span { class: "drink-button-label", "{drink.display().label}" }
                                    if drink_count > 0 {
                                        span { class: "drink-count", "{drink_count}" }
                                    }
                                }
                            }
                        })
                }
            }
        }
    }
}
