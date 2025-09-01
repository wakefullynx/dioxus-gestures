use dioxus::prelude::*;
use dioxus_gestures::{
    state::gestures::hover::Hover,
    use_gestures::{use_gestures, Gestures},
};

const UNHOVERED_TEXT: &str = "Hover over me!";
const HOVERED_TEXT: &str = "Hovering ...";

#[component]
pub fn HoverExample() -> Element {
    let mut text = use_signal(|| UNHOVERED_TEXT);

    let gestures = use_gestures(
        Gestures::default().hover(
            Hover::default()
                .on_start(move |_| text.set(HOVERED_TEXT))
                .on_end(move |_| text.set(UNHOVERED_TEXT))
                .on_cancel(move |_| text.set(UNHOVERED_TEXT)),
        ),
    );

    rsx! {
        div {
        class: "target hover",
        style: format!("
            user-select: none;
            touch-action: none;
            position: relative;
        "),
        ..gestures.event_handlers(),
            "{text}"
        }
    }
}
