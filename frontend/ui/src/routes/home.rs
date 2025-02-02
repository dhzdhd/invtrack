use dioxus::prelude::*;
use crate::Hero;

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
    }
}
