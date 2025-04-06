use dioxus::prelude::*;

#[component]
pub fn Header(children: Element) -> Element {
    rsx! {
        div { id: "navbar", class: "fixed top-0 h-20 bg-yellow-500", {children} }
    }
}
