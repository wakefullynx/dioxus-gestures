use crate::drag::DragExample;
use crate::hover::HoverExample;
use crate::pinch::PinchExample;
use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/styling/main.css");

#[component]
pub fn Example() -> Element {
    rsx! {
        dioxus::document::Link { rel: "stylesheet", href: CSS }

        div {
            class: "examples",
            HoverExample {}
            DragExample {}
            PinchExample {}
        }
    }
}
