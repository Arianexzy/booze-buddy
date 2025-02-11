use crate::Route;
use dioxus::prelude::*;
use lucide_dioxus::{Beer, Calendar, Trophy};

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div { id: "navbar",
            Link {
                to: Route::Tonight {},
                Beer {},
                "Tonight",
            },
            Link { to: Route::History {},
                Calendar {},
                "History",

            },
            Link {
                to: Route::Achievements {},
                Trophy {},
                "Achievements"
            },
        }

        Outlet::<Route> {}
    }
}
