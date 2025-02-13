use crate::Route;
use dioxus::prelude::*;
use lucide_dioxus::{Beer, Calendar, Trophy};

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    rsx! {  
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        nav { class: "navbar",
            ul { class: "navbar-list",
                li { 
                    key: "tonight",
                    Link {
                        class: "navbar-link",
                        active_class: "active",
                        to: Route::Tonight {},
                        Beer {},
                        span { class: "navbar-label", "Tonight" }
                    }
                },
                li { 
                    key: "history",
                    Link {
                        class: "navbar-link",
                        active_class: "active",
                        to: Route::History {},
                        Calendar {},
                        span { class: "navbar-label", "History" }
                    }
                },
                li { 
                    key: "achievements",
                    Link {
                        class: "navbar-link",
                        active_class: "active",
                        to: Route::Achievements {},
                        Trophy {},
                        span { class: "navbar-label", "Achievements" }
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}
