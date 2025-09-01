use std::{cell::RefCell, rc::Rc};

use dioxus::{events::InteractionLocation, html::PointerData};

use crate::state::gestures::pointer::{IncrementalOffsetPointer, InitialPointer, OffsetPointer};

/// ```rust
/// use dioxus::{
///     html::geometry::{euclid::Point2D, ClientSpace},
///     prelude::*,
/// };
/// use dioxus_gestures::{
///     state::gestures::drag::Drag,
///     use_gestures::{use_gestures, Gestures},
/// };
///
/// const UNDRAGGED_TEXT: &str = "Drag me!";
/// const DRAGGED_TEXT: &str = "Dragging ...";
///
/// #[component]
/// pub fn DragExample() -> Element {
///     let mut position = use_signal(|| Point2D::<_, ClientSpace>::new(0.0, 0.0));
///     let mut text = use_signal(|| UNDRAGGED_TEXT);
///
///     let gestures = use_gestures(
///         Gestures::default()
///             .drag(
///                 Drag::default()
///                     .on_start(move |_| text.set(DRAGGED_TEXT))
///                     .on_update(move |data| {
///                         let movement = data.pointer.delta_movement();
///                         position.set(position() + movement);
///                     })
///                     .on_end(move |_| text.set(UNDRAGGED_TEXT))
///                     .on_cancel(move |_| text.set(UNDRAGGED_TEXT)),
///             )
///     );
///
///     rsx! {
///         div {
///         class: "target drag",
///         style: format!("
///             user-select: none;
///             touch-action: none;
///             position: relative;
///             left: {}px;
///             top: {}px;
///         ", position().x, position().y),
///         ..gestures.event_handlers(),
///             "{text}"
///         }
///     }
/// }
/// ```
#[derive(Clone)]
pub struct Drag {
    pub on_start: Option<Rc<RefCell<dyn FnMut(DragStartData)>>>,
    pub on_update: Option<Rc<RefCell<dyn FnMut(DragUpdateData)>>>,
    pub on_end: Option<Rc<RefCell<dyn FnMut(DragEndData)>>>,
    pub on_cancel: Option<Rc<RefCell<dyn FnMut(DragCancelData)>>>,
    pub has_started: Rc<dyn Fn([&PointerData; 2]) -> bool>,
}

impl Drag {
    pub fn on_start(mut self, handler: impl FnMut(DragStartData) + 'static) -> Self {
        self.on_start = Some(Rc::new(RefCell::new(handler)));
        self
    }

    pub fn on_update(mut self, handler: impl FnMut(DragUpdateData) + 'static) -> Self {
        self.on_update = Some(Rc::new(RefCell::new(handler)));
        self
    }

    pub fn on_end(mut self, handler: impl FnMut(DragEndData) + 'static) -> Self {
        self.on_end = Some(Rc::new(RefCell::new(handler)));
        self
    }

    pub fn on_cancel(mut self, handler: impl FnMut(DragCancelData) + 'static) -> Self {
        self.on_cancel = Some(Rc::new(RefCell::new(handler)));
        self
    }

    pub fn has_started(mut self, predicate: impl Fn([&PointerData; 2]) -> bool + 'static) -> Self {
        self.has_started = Rc::new(predicate);
        self
    }
}

pub struct DragStartData {
    pub pointer: InitialPointer,
}
pub struct DragUpdateData {
    pub pointer: IncrementalOffsetPointer,
}
pub struct DragEndData {
    pub pointer: OffsetPointer,
}
pub struct DragCancelData {
    pub pointer: OffsetPointer,
}

impl Default for Drag {
    fn default() -> Self {
        Self {
            on_start: Default::default(),
            on_update: Default::default(),
            on_end: Default::default(),
            on_cancel: Default::default(),
            has_started: Rc::new(|[initial, current]| {
                (current.client_coordinates() - initial.client_coordinates()).length() >= 5.0
            }),
        }
    }
}
