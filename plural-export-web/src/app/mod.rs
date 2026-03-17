mod home;

use dioxus::prelude::*;

use self::home::Home;
use crate::hooks::ThemeProvider;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");


#[derive(Debug, Clone, Routable, PartialEq, Eq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home
}


#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Meta { name: "darkreader-lock" }
        document::Script { {include_str!("../../assets/theme.js")} }
        ThemeProvider { Router::<Route> {} }
    }
}
