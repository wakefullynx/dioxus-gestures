use paste;
use std::{cell::RefCell, rc::Rc};

use dioxus::{
    html::{PlatformEventData},
    prelude::{use_hook, Attribute},
    core::{ListenerCallback},
};

use crate::state::{
    events::PointerEventReceiver,
    gestures::{drag::Drag, hover::Hover, pinch::Pinch},
};
use crate::state::{external_handlers::ExternalHandlers, state::UseGesturesState};

#[derive(Clone)]
pub struct UseGestures {
    state: Rc<RefCell<UseGesturesState>>,
}

impl From<Gestures> for UseGestures {
    fn from(value: Gestures) -> Self {
        let Gestures {
            external_handlers,
            hover,
            drag,
            pinch,
        } = value;
        Self {
            state: Rc::new(RefCell::new(UseGesturesState::new(
                external_handlers,
                hover,
                drag,
                pinch,
            ))),
        }
    }
}


impl UseGestures {
    pub fn event_handlers(self) -> Vec<Attribute> {

        macro_rules! pointer_event_handler {
            ($attribute_name: ident, $function_name: ident) => {{
                let pointer_ref = Rc::clone(&self.state);
                dioxus_core::Attribute::new(
                    paste::paste! { stringify!([<$attribute_name:camel:lower>])},
                    dioxus_core::AttributeValue::Listener(
                        ListenerCallback::new(
                        move |e: dioxus_core::Event<PlatformEventData>| {
                            let _ = pointer_ref.try_borrow_mut().map(|mut s| s.$function_name(e.map(|data| data.into())));
                        }).erase()
                    ),
                    None,
                    false,
                )
            }};
        }

        vec![
            pointer_event_handler!(on_pointer_over, pointer_over),
            pointer_event_handler!(on_pointer_enter, pointer_enter),
            pointer_event_handler!(on_pointer_down, pointer_down),
            pointer_event_handler!(on_pointer_move, pointer_move),
            pointer_event_handler!(on_pointer_up, pointer_up),
            pointer_event_handler!(on_pointer_cancel, pointer_cancel),
            pointer_event_handler!(on_pointer_out, pointer_out),
            pointer_event_handler!(on_pointer_leave, pointer_leave),
        ]
    }
}

pub fn use_gestures<'a>(props: Gestures) -> UseGestures {
    use_hook(|| UseGestures::from(props))
}

#[derive(Default)]
pub struct Gestures {
    pub external_handlers: ExternalHandlers,
    pub hover: Hover,
    pub drag: Drag,
    pub pinch: Pinch,
}

impl Gestures {
    pub fn external_handlers(mut self, external_handlers: ExternalHandlers) -> Self {
        self.external_handlers = external_handlers;
        self
    }

    pub fn hover(mut self, hover: Hover) -> Self {
        self.hover = hover;
        self
    }

    pub fn drag(mut self, drag: Drag) -> Self {
        self.drag = drag;
        self
    }

    pub fn pinch(mut self, pinch: Pinch) -> Self {
        self.pinch = pinch;
        self
    }
}
