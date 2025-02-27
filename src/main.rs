use dioxus::prelude::*;

mod components;
mod routes;
mod storage;
mod views;

use routes::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const SETTINGS_CSS: Asset = asset!("/assets/styling/settings.css");
const HISTORY_VIEW_CSS: Asset = asset!("/assets/styling/history_view.css");
const ACHIEVEMENTS_VIEW_CSS: Asset = asset!("assets/styling/achievements_view.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover",
        }
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: SETTINGS_CSS }
        document::Link { rel: "stylesheet", href: HISTORY_VIEW_CSS }
        document::Link { rel: "stylesheet", href: ACHIEVEMENTS_VIEW_CSS }

        Router::<Route> {}
    }
}
