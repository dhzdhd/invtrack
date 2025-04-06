use crate::{Header, Route};
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/output.css");

#[component]
pub fn Layout() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Header {
            button {}
        }
        Outlet::<Route> {} // <slot>
    }
}
