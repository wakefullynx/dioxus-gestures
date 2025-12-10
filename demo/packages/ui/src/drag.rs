use dioxus::{
    html::geometry::{euclid::Point2D, ClientSpace},
    prelude::*,
};
use dioxus_gestures::{
    state::gestures::drag::Drag,
    use_gestures::{use_gestures, Gestures},
};

const UNDRAGGED_TEXT: &str = "Drag me!";
const DRAGGED_TEXT: &str = "Dragging ...";

#[component]
pub fn DragExample() -> Element {
    let mut position = use_signal(|| Point2D::<_, ClientSpace>::new(0.0, 0.0));
    let mut text = use_signal(|| UNDRAGGED_TEXT);

    let gestures = use_gestures(
        Gestures::default().drag(
            Drag::default()
                .on_start(move |_| text.set(DRAGGED_TEXT))
                .on_update(move |data| {
                    let movement = data.pointer.delta_movement();
                    position.set(position() + movement);
                })
                .on_end(move |_| text.set(UNDRAGGED_TEXT))
                .on_cancel(move |_| text.set(UNDRAGGED_TEXT)),
        ),
    );

    rsx! {
        div {
        class: "target drag",
        style: format!("
            user-select: none;
            touch-action: none;
            position: relative;
            left: {}px;
            top: {}px;
        ", position().x, position().y),
        ..gestures.event_handlers(),
            "{text}"
        }
    }
}
