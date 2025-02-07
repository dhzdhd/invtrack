use dioxus::prelude::*;
use crate::{Navbar, Route};

const MAIN_CSS: Asset = asset!("/assets/output.css");

#[component]
pub fn Layout() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Navbar { }
        Outlet::<Route> {}  // <slot>
    }
}
