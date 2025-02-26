use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        input { r#type: "file" }
    }
}
