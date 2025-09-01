use std::{cell::RefCell, rc::Rc};

use dioxus::{html::PointerData, prelude::Event};

use crate::state::{
    events::PointerEventReceiver,
    gestures::pointer::{IncrementalOffsetPointer, InitialPointer, OffsetPointer, PointerId},
};

#[derive(Clone)]
pub struct HoverGestureState {
    hover: Hover,
    pointers: Vec<HoverState>,
}

#[derive(Clone)]
pub struct HoverState {
    pointer: HoverPointerState,
}

#[derive(Clone)]
pub struct HoverPointerState {
    id: PointerId,
    initial_state: Rc<PointerData>,
    previous_state: Rc<PointerData>,
}

/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_gestures::{
///     state::gestures::hover::Hover,
///     use_gestures::{use_gestures, Gestures},
/// };
///
/// const UNHOVERED_TEXT: &str = "Hover over me!";
/// const HOVERED_TEXT: &str = "Hovering ...";
///
/// #[component]
/// pub fn HoverExample() -> Element {
///     let mut text = use_signal(|| UNHOVERED_TEXT);
///
///     let gestures = use_gestures(
///         Gestures::default().hover(
///             Hover::default()
///                 .on_start(move |_| text.set(HOVERED_TEXT))
///                 .on_end(move |_| text.set(UNHOVERED_TEXT))
///                 .on_cancel(move |_| text.set(UNHOVERED_TEXT)),
///         ),
///     );
///
///     rsx! {
///         div {
///         class: "target hover",
///         style: format!("
///             user-select: none;
///             touch-action: none;
///             position: relative;
///         "),
///         ..gestures.event_handlers(),
///             "{text}"
///         }
///     }
/// }
/// ```

#[derive(Default, Clone)]
pub struct Hover {
    pub on_start: Option<Rc<RefCell<dyn FnMut(())>>>,
    pub on_end: Option<Rc<RefCell<dyn FnMut(())>>>,
    pub on_cancel: Option<Rc<RefCell<dyn FnMut(())>>>,
    pub on_pointer_appear: Option<Rc<RefCell<dyn FnMut(HoverPointerAppearData)>>>,
    pub on_pointer_update: Option<Rc<RefCell<dyn FnMut(HoverPointerUpdateData)>>>,
    pub on_pointer_disappear: Option<Rc<RefCell<dyn FnMut(HoverPointerDisappearData)>>>,
    pub on_pointer_cancel: Option<Rc<RefCell<dyn FnMut(HoverPointerCancelData)>>>,
}

impl Hover {
    pub fn on_start(mut self, handler: impl FnMut(()) + 'static) -> Self {
        self.on_start = Some(Rc::new(RefCell::new(handler)));
        self
    }

    pub fn on_end(mut self, handler: impl FnMut(()) + 'static) -> Self {
        self.on_end = Some(Rc::new(RefCell::new(handler)));
        self
    }

    pub fn on_cancel(mut self, handler: impl FnMut(()) + 'static) -> Self {
        self.on_cancel = Some(Rc::new(RefCell::new(handler)));
        self
    }

    pub fn on_pointer_appear(
        mut self,
        handler: impl FnMut(HoverPointerAppearData) + 'static,
    ) -> Self {
        self.on_pointer_appear = Some(Rc::new(RefCell::new(handler)));
        self
    }

    pub fn on_pointer_update(
        mut self,
        handler: impl FnMut(HoverPointerUpdateData) + 'static,
    ) -> Self {
        self.on_pointer_update = Some(Rc::new(RefCell::new(handler)));
        self
    }

    pub fn on_pointer_disappear(
        mut self,
        handler: impl FnMut(HoverPointerDisappearData) + 'static,
    ) -> Self {
        self.on_pointer_disappear = Some(Rc::new(RefCell::new(handler)));
        self
    }

    pub fn on_pointer_cancel(
        mut self,
        handler: impl FnMut(HoverPointerCancelData) + 'static,
    ) -> Self {
        self.on_pointer_cancel = Some(Rc::new(RefCell::new(handler)));
        self
    }
}

pub struct HoverPointerAppearData {
    pub pointer: InitialPointer,
}

pub struct HoverPointerUpdateData {
    pub pointer: IncrementalOffsetPointer,
}

pub struct HoverPointerDisappearData {
    pub pointer: OffsetPointer,
}

pub struct HoverPointerCancelData {
    pub pointer: OffsetPointer,
}

impl HoverGestureState {
    pub fn new(hover: Hover) -> Self {
        Self {
            hover,
            pointers: Vec::new(),
        }
    }

    fn add_hover_event(&mut self, pointer_data: Rc<PointerData>) {
        let is_first = self.pointers.is_empty();
        self.pointers.push(HoverState {
            pointer: HoverPointerState {
                id: PointerId::from(pointer_data.pointer_id()),
                initial_state: Rc::clone(&pointer_data),
                previous_state: Rc::clone(&pointer_data),
            },
        });

        if is_first {
            if let Some(handler) = &self.hover.on_start {
                handler.borrow_mut()(());
            }
        }

        if let Some(handler) = &self.hover.on_pointer_appear {
            handler.borrow_mut()(HoverPointerAppearData {
                pointer: InitialPointer { data: pointer_data },
            });
        }
    }

    fn update_known_hover_event(&mut self, index: usize, pointer_data: Rc<PointerData>) {
        let initial_data = Rc::clone(&self.pointers[index].pointer.initial_state);
        let preceding_data = Rc::clone(&self.pointers[index].pointer.previous_state);
        self.pointers[index].pointer.previous_state = Rc::clone(&pointer_data);
        if let Some(handler) = &self.hover.on_pointer_update {
            handler.borrow_mut()(HoverPointerUpdateData {
                pointer: IncrementalOffsetPointer {
                    initial_data,
                    preceding_data,
                    current_data: pointer_data,
                },
            });
        }
    }

    fn remove_known_hover_event(&mut self, index: usize, pointer_data: Rc<PointerData>) {
        let hover = self.pointers.remove(index);
        let initial_data = hover.pointer.initial_state;
        if let Some(handler) = &self.hover.on_pointer_disappear {
            handler.borrow_mut()(HoverPointerDisappearData {
                pointer: OffsetPointer {
                    initial_data,
                    final_data: pointer_data,
                },
            });
        }

        if self.pointers.is_empty() {
            if let Some(handler) = &self.hover.on_end {
                handler.borrow_mut()(());
            }
        }
    }

    fn cancel_known_hover_event(&mut self, index: usize, pointer_data: Rc<PointerData>) {
        let hover = self.pointers.remove(index);
        let initial_data = hover.pointer.initial_state;
        if let Some(handler) = &self.hover.on_pointer_cancel {
            handler.borrow_mut()(HoverPointerCancelData {
                pointer: OffsetPointer {
                    initial_data,
                    final_data: pointer_data,
                },
            });
        }

        if self.pointers.is_empty() {
            if let Some(handler) = &self.hover.on_cancel {
                handler.borrow_mut()(())
            }
        }
    }

    fn add_or_update(&mut self, event: &Event<PointerData>) {
        let pointer_data = event.data();
        let pointer_id = pointer_data.pointer_id();
        let associated_hover_event = self
            .pointers
            .iter()
            .position(|p| p.pointer.id.is_equal_i32(pointer_id));
        match associated_hover_event {
            Some(position) => {
                self.update_known_hover_event(position, pointer_data);
            }
            None => {
                self.add_hover_event(pointer_data);
            }
        };
    }

    fn remove(&mut self, event: &Event<PointerData>) {
        let pointer_data = event.data();
        let pointer_id = pointer_data.pointer_id();
        let associated_hover_event = self
            .pointers
            .iter()
            .position(|p| p.pointer.id.is_equal_i32(pointer_id));
        match associated_hover_event {
            Some(position) => {
                self.remove_known_hover_event(position, pointer_data);
            }
            None => {}
        };
    }

    fn cancel(&mut self, event: &Event<PointerData>) {
        let pointer_data = event.data();
        let pointer_id = pointer_data.pointer_id();
        let associated_hover_event = self
            .pointers
            .iter()
            .position(|p| p.pointer.id.is_equal_i32(pointer_id));
        match associated_hover_event {
            Some(position) => {
                self.cancel_known_hover_event(position, pointer_data);
            }
            None => {}
        };
    }
}

impl PointerEventReceiver<&Event<PointerData>> for HoverGestureState {
    fn pointer_over(&mut self, _: &Event<PointerData>) {}

    fn pointer_enter(&mut self, event: &Event<PointerData>) {
        self.add_or_update(event);
    }

    fn pointer_down(&mut self, event: &Event<PointerData>) {
        self.add_or_update(event);
    }

    fn pointer_move(&mut self, event: &Event<PointerData>) {
        self.add_or_update(event);
    }

    fn pointer_up(&mut self, event: &Event<PointerData>) {
        self.add_or_update(event);
    }

    fn pointer_cancel(&mut self, event: &Event<PointerData>) {
        self.cancel(event);
    }

    fn pointer_out(&mut self, _: &Event<PointerData>) {}

    fn pointer_leave(&mut self, event: &Event<PointerData>) {
        self.remove(event);
    }
}
