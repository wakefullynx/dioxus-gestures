use dioxus::{html::geometry::euclid::Angle, prelude::*};
use dioxus_gestures::{
    state::gestures::pinch::Pinch,
    use_gestures::{use_gestures, Gestures},
};

const UNDRAGGED_TEXT: &str = "Pinch me!";
const DRAGGED_TEXT: &str = "Pinching ...";

#[component]
pub fn PinchExample() -> Element {
    let mut rotation = use_signal(|| Angle::<f64>::default());
    let mut scale = use_signal(|| 1.0);
    let mut text = use_signal(|| UNDRAGGED_TEXT);

    let gestures = use_gestures(
        Gestures::default().pinch(
            Pinch::default()
                .on_start(move |_| text.set(DRAGGED_TEXT))
                .on_update(move |data| {
                    scale.set(scale() * data.delta_distance_scale());
                    rotation.set(rotation() + data.delta_angle());
                })
                .on_end(move |_| text.set(UNDRAGGED_TEXT))
                .on_cancel(move |_| text.set(UNDRAGGED_TEXT)),
        ),
    );

    rsx! {
        div {
        class: "target pinch",
        style: format!("
            user-select: none;
            touch-action: none;
            position: relative;
            transform: scale({}) rotate({}rad);
        ", scale(), rotation().get()),
        ..gestures.event_handlers(),
            "{text}"
        }
    }
}
