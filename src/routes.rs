use dioxus::prelude::*;
use crate::components::Navbar;
use crate::views::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    
    #[route("/")]
    Tonight {},
    
    #[route("/history")]
    History {},
    
    #[route("/achievements")]
    Achievements {},
}