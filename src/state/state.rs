use dioxus::{document, html::PointerData, prelude::Event};
use nanoid::nanoid;

use crate::state::{
    events::PointerEventReceiver,
    external_handlers::ExternalHandlers,
    gestures::{
        down_pointer::DownPointerGestureState,
        drag::Drag,
        hover::{Hover, HoverGestureState},
        pinch::Pinch,
    },
};

#[derive(Clone)]
pub struct UseGesturesState {
    pub target_unique_id: String,
    external: ExternalHandlers,
    hover: HoverGestureState,
    down_pointer: DownPointerGestureState,
}

impl UseGesturesState {
    pub fn new(external: ExternalHandlers, hover: Hover, drag: Drag, pinch: Pinch) -> Self {
        let target_unique_id = nanoid!();
        Self {
            target_unique_id: target_unique_id.clone(),
            external,
            hover: HoverGestureState::new(hover),
            down_pointer: DownPointerGestureState::new(drag, pinch),
        }
    }
}

impl UseGesturesState {
    fn set_pointer_capture(&self, pointer_id: i32) {
        let target_unique_id = &self.target_unique_id;
        document::eval(&format!(
            r#"document.querySelector("*[data-gestures-id='{target_unique_id}']").setPointerCapture({:?})"#,
            pointer_id
        ));
    }
}

impl UseGesturesState {
    fn release_pointer_capture(&self, pointer_id: i32) {
        let target_unique_id = &self.target_unique_id;
        document::eval(&format!(
            r#"document.querySelector("*[data-gestures-id='{target_unique_id}']").releasePointerCapture("{:?}")"#,
            pointer_id
        ));
    }
}

impl PointerEventReceiver<Event<PointerData>> for UseGesturesState {
    fn pointer_over(&mut self, event: Event<PointerData>) {
        self.hover.pointer_over(&event);
        self.down_pointer.pointer_over(&event);
        self.external.pointer_over(event);
    }

    fn pointer_enter(&mut self, event: Event<PointerData>) {
        self.hover.pointer_enter(&event);
        self.down_pointer.pointer_enter(&event);
        self.external.pointer_enter(event);
    }

    fn pointer_down(&mut self, event: Event<PointerData>) {
        self.set_pointer_capture(event.pointer_id());

        self.hover.pointer_down(&event);
        self.down_pointer.pointer_down(&event);
        self.external.pointer_down(event);
    }

    fn pointer_move(&mut self, event: Event<PointerData>) {
        self.hover.pointer_move(&event);
        self.down_pointer.pointer_move(&event);
        self.external.pointer_move(event);
    }

    fn pointer_up(&mut self, event: Event<PointerData>) {
        self.release_pointer_capture(event.pointer_id());

        self.hover.pointer_up(&event);
        self.down_pointer.pointer_up(&event);
        self.external.pointer_up(event);
    }

    fn pointer_cancel(&mut self, event: Event<PointerData>) {
        self.release_pointer_capture(event.pointer_id());

        self.hover.pointer_cancel(&event);
        self.down_pointer.pointer_cancel(&event);
        self.external.pointer_cancel(event);
    }

    fn pointer_out(&mut self, event: Event<PointerData>) {
        self.hover.pointer_out(&event);
        self.down_pointer.pointer_out(&event);
        self.external.pointer_out(event);
    }

    fn pointer_leave(&mut self, event: Event<PointerData>) {
        self.hover.pointer_leave(&event);
        self.down_pointer.pointer_leave(&event);
        self.external.pointer_leave(event);
    }
}
