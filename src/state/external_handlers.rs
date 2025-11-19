use dioxus::{core::Event, html::PointerData, prelude::EventHandler};

use crate::state::events::PointerEventReceiver;

#[derive(Clone, Default)]
pub struct ExternalHandlers {
    pub on_pointer_over: Option<EventHandler<Event<PointerData>>>,
    pub on_pointer_enter: Option<EventHandler<Event<PointerData>>>,
    pub on_pointer_down: Option<EventHandler<Event<PointerData>>>,
    pub on_pointer_move: Option<EventHandler<Event<PointerData>>>,
    pub on_pointer_up: Option<EventHandler<Event<PointerData>>>,
    pub on_pointer_cancel: Option<EventHandler<Event<PointerData>>>,
    pub on_pointer_out: Option<EventHandler<Event<PointerData>>>,
    pub on_pointer_leave: Option<EventHandler<Event<PointerData>>>,
}

impl ExternalHandlers {
    pub fn stop_propagation() -> Self {
        Self {
            on_pointer_over: Some(EventHandler::new(|e: Event<PointerData>| {
                e.stop_propagation()
            })),
            on_pointer_enter: Some(EventHandler::new(|e: Event<PointerData>| {
                e.stop_propagation()
            })),
            on_pointer_down: Some(EventHandler::new(|e: Event<PointerData>| {
                e.stop_propagation()
            })),
            on_pointer_move: Some(EventHandler::new(|e: Event<PointerData>| {
                e.stop_propagation()
            })),
            on_pointer_up: Some(EventHandler::new(|e: Event<PointerData>| {
                e.stop_propagation()
            })),
            on_pointer_cancel: Some(EventHandler::new(|e: Event<PointerData>| {
                e.stop_propagation()
            })),
            on_pointer_out: Some(EventHandler::new(|e: Event<PointerData>| {
                e.stop_propagation()
            })),
            on_pointer_leave: Some(EventHandler::new(|e: Event<PointerData>| {
                e.stop_propagation()
            })),
        }
    }

    pub fn on_pointer_over(mut self, handler: impl FnMut(Event<PointerData>) + 'static) -> Self {
        self.on_pointer_over = Some(EventHandler::new(handler));
        self
    }

    pub fn on_pointer_enter(mut self, handler: impl FnMut(Event<PointerData>) + 'static) -> Self {
        self.on_pointer_enter = Some(EventHandler::new(handler));
        self
    }

    pub fn on_pointer_down(mut self, handler: impl FnMut(Event<PointerData>) + 'static) -> Self {
        self.on_pointer_down = Some(EventHandler::new(handler));
        self
    }

    pub fn on_pointer_move(mut self, handler: impl FnMut(Event<PointerData>) + 'static) -> Self {
        self.on_pointer_move = Some(EventHandler::new(handler));
        self
    }

    pub fn on_pointer_up(mut self, handler: impl FnMut(Event<PointerData>) + 'static) -> Self {
        self.on_pointer_up = Some(EventHandler::new(handler));
        self
    }

    pub fn on_pointer_cancel(mut self, handler: impl FnMut(Event<PointerData>) + 'static) -> Self {
        self.on_pointer_cancel = Some(EventHandler::new(handler));
        self
    }

    pub fn on_pointer_out(mut self, handler: impl FnMut(Event<PointerData>) + 'static) -> Self {
        self.on_pointer_out = Some(EventHandler::new(handler));
        self
    }

    pub fn on_pointer_leave(mut self, handler: impl FnMut(Event<PointerData>) + 'static) -> Self {
        self.on_pointer_leave = Some(EventHandler::new(handler));
        self
    }
}

impl PointerEventReceiver<Event<PointerData>> for ExternalHandlers {
    fn pointer_over(&mut self, event: Event<PointerData>) {
        self.on_pointer_over.inspect(|handler| handler(event));
    }

    fn pointer_enter(&mut self, event: Event<PointerData>) {
        self.on_pointer_enter.inspect(|handler| handler(event));
    }

    fn pointer_down(&mut self, event: Event<PointerData>) {
        self.on_pointer_down.inspect(|handler| handler(event));
    }

    fn pointer_move(&mut self, event: Event<PointerData>) {
        self.on_pointer_move.inspect(|handler| handler(event));
    }

    fn pointer_up(&mut self, event: Event<PointerData>) {
        self.on_pointer_up.inspect(|handler| handler(event));
    }

    fn pointer_cancel(&mut self, event: Event<PointerData>) {
        self.on_pointer_cancel.inspect(|handler| handler(event));
    }

    fn pointer_out(&mut self, event: Event<PointerData>) {
        self.on_pointer_out.inspect(|handler| handler(event));
    }

    fn pointer_leave(&mut self, event: Event<PointerData>) {
        self.on_pointer_leave.inspect(|handler| handler(event));
    }
}
