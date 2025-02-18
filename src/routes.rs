use crate::components::Navbar;
use crate::views::*;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    
    #[route("/")]
    TonightView {},
    
    #[route("/history")]
    DrinkingHistoryView {},
    
    #[route("/achievements")]
    AchievementsView {},
    
    #[route("/settings")]
    SettingsView {},
}
