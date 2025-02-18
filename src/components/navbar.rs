use crate::Route;
use dioxus::prelude::*;
use lucide_dioxus::{Beer, Calendar, Settings2, Trophy};

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        nav { class: "navbar",
            ul { class: "navbar-list",
                li { key: "tonight",
                    Link {
                        class: "navbar-link",
                        active_class: "active",
                        to: Route::TonightView {},
                        Beer {}
                        span { class: "navbar-label", "Tonight" }
                    }
                }
                li { key: "history",
                    Link {
                        class: "navbar-link",
                        active_class: "active",
                        to: Route::DrinkingHistoryView {},
                        Calendar {}
                        span { class: "navbar-label", "History" }
                    }
                }
                li { key: "achievements",
                    Link {
                        class: "navbar-link",
                        active_class: "active",
                        to: Route::AchievementsView {},
                        Trophy {}
                        span { class: "navbar-label", "Achievements" }
                    }
                }
                li { key: "settings",
                    Link {
                        class: "navbar-link",
                        active_class: "active",
                        to: Route::SettingsView {},
                        Settings2 {}
                        span { class: "navbar-label", "Settings" }
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}
