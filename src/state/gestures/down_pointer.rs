use std::rc::Rc;

use dioxus::core::Event;
use dioxus::html::PointerData;

use crate::state::{
    events::PointerEventReceiver,
    gestures::{
        drag::{Drag, DragCancelData, DragEndData, DragStartData, DragUpdateData},
        pinch::{
            Pinch, PinchCancelData, PinchEndData, PinchStartData, PinchUpdateData,
            PinchUpdatedPointer,
        },
        pointer::{IncrementalOffsetPointer, InitialPointer, OffsetPointer, PointerId},
    },
};

#[derive(Clone)]
pub struct DownPointerGestureState {
    drag: Drag,
    pinch: Pinch,
    pointers: Vec<DownPointerState>,
    gesture_state: GestureState,
}

#[derive(Clone, Default)]
enum GestureState {
    #[default]
    Pending,
    Started,
}

#[derive(Clone)]
struct DownPointerState {
    id: PointerId,
    initial: Rc<PointerData>,
    current: Rc<PointerData>,
}

impl DownPointerGestureState {
    pub fn new(drag: Drag, pinch: Pinch) -> Self {
        Self {
            drag,
            pinch,
            pointers: Vec::new(),
            gesture_state: GestureState::default(),
        }
    }

    fn add_down_pointer_event(&mut self, pointer_data: Rc<PointerData>) {
        self.pointers.push(DownPointerState {
            id: PointerId::from(pointer_data.pointer_id()),
            initial: Rc::clone(&pointer_data),
            current: Rc::clone(&pointer_data),
        });

        match self.pointers.len() {
            1 => {
                self.gesture_state = match (self.drag.has_started)([&pointer_data, &pointer_data]) {
                    false => GestureState::Pending,
                    true => GestureState::Started,
                };

                match self.gesture_state {
                    GestureState::Started => {
                        if let Some(handler) = &self.drag.on_start {
                            handler.borrow_mut()(DragStartData {
                                pointer: InitialPointer { data: pointer_data },
                            })
                        }
                    }
                    _ => (),
                }
            }
            2 => {
                if let Some(handler) = &self.drag.on_end {
                    let existing_pointer = &self.pointers[0];
                    handler.borrow_mut()(DragEndData {
                        pointer: OffsetPointer {
                            initial_data: Rc::clone(&existing_pointer.initial),
                            final_data: Rc::clone(&pointer_data),
                        },
                    });
                }

                self.pointers[0].initial = Rc::clone(&self.pointers[0].current);

                self.gesture_state = match (self.pinch.has_started)(
                    [&self.pointers[0].initial, &self.pointers[0].initial],
                    [&pointer_data, &pointer_data],
                ) {
                    false => GestureState::Pending,
                    true => GestureState::Started,
                };

                match self.gesture_state {
                    GestureState::Started => {
                        if let Some(handler) = &self.pinch.on_start {
                            handler.borrow_mut()(PinchStartData {
                                pointers: [
                                    InitialPointer {
                                        data: Rc::clone(&self.pointers[0].initial),
                                    },
                                    InitialPointer { data: pointer_data },
                                ],
                            });
                        }
                    }
                    _ => (),
                }
            }
            3 => {
                if let Some(handler) = &self.pinch.on_end {
                    let [first, second]: &[DownPointerState; 2] =
                        self.pointers[0..=1].try_into().unwrap();
                    handler.borrow_mut()(PinchEndData {
                        pointers: [
                            {
                                OffsetPointer {
                                    initial_data: Rc::clone(&first.initial),
                                    final_data: Rc::clone(&first.current),
                                }
                            },
                            {
                                OffsetPointer {
                                    initial_data: Rc::clone(&second.initial),
                                    final_data: Rc::clone(&second.current),
                                }
                            },
                        ],
                    });
                }
            }
            _ => {}
        }
    }

    fn update_known_down_pointer_event(&mut self, index: usize, pointer_data: Rc<PointerData>) {
        let initial_data = Rc::clone(&self.pointers[index].initial);
        let preceding_data = Rc::clone(&self.pointers[index].current);
        self.pointers[index].current = Rc::clone(&pointer_data);

        match self.pointers.len() {
            1 => {
                match self.gesture_state {
                    GestureState::Pending => {
                        match (self.drag.has_started)([&initial_data, &pointer_data]) {
                            false => (),
                            true => {
                                self.gesture_state = GestureState::Started;
                                if let Some(handler) = &self.drag.on_start {
                                    handler.borrow_mut()(DragStartData {
                                        pointer: InitialPointer {
                                            data: Rc::clone(&initial_data),
                                        },
                                    });
                                }
                            }
                        }
                    }
                    GestureState::Started => match &self.drag.on_update {
                        Some(handler) => {
                            (handler.borrow_mut())(DragUpdateData {
                                pointer: IncrementalOffsetPointer {
                                    initial_data,
                                    preceding_data,
                                    current_data: Rc::clone(&pointer_data),
                                },
                            });
                        }
                        None => (),
                    },
                };
            }
            2 => {
                let [first, second]: &[DownPointerState; 2] =
                    self.pointers[0..=1].try_into().unwrap();
                match self.gesture_state {
                    GestureState::Pending => match (self.pinch.has_started)(
                        [&first.initial, &first.current],
                        [&second.initial, &second.current],
                    ) {
                        false => (),
                        true => {
                            self.gesture_state = GestureState::Started;
                            if let Some(handler) = &self.pinch.on_start {
                                handler.borrow_mut()(PinchStartData {
                                    pointers: [
                                        InitialPointer {
                                            data: Rc::clone(&first.initial),
                                        },
                                        InitialPointer {
                                            data: Rc::clone(&second.initial),
                                        },
                                    ],
                                });
                            }
                        }
                    },
                    GestureState::Started => {
                        if let Some(handler) = &self.pinch.on_update {
                            let (pointers, updated_pointer) = if index == 0 {
                                (
                                    [
                                        IncrementalOffsetPointer {
                                            initial_data: Rc::clone(&initial_data),
                                            preceding_data,
                                            current_data: pointer_data,
                                        },
                                        IncrementalOffsetPointer {
                                            initial_data: Rc::clone(&second.initial),
                                            preceding_data: Rc::clone(&second.current),
                                            current_data: Rc::clone(&second.current),
                                        },
                                    ],
                                    PinchUpdatedPointer::First,
                                )
                            } else {
                                (
                                    [
                                        IncrementalOffsetPointer {
                                            initial_data: Rc::clone(&first.initial),
                                            preceding_data: Rc::clone(&first.current),
                                            current_data: Rc::clone(&first.current),
                                        },
                                        IncrementalOffsetPointer {
                                            initial_data: Rc::clone(&initial_data),
                                            preceding_data,
                                            current_data: pointer_data,
                                        },
                                    ],
                                    PinchUpdatedPointer::Second,
                                )
                            };
                            handler.borrow_mut()(PinchUpdateData {
                                pointers,
                                updated_pointer,
                            });
                        }
                    }
                };
            }
            _ => {}
        }
    }

    fn remove_known_down_pointer_event(&mut self, index: usize, pointer_data: Rc<PointerData>) {
        let pointer = self.pointers.remove(index);
        let initial_data = pointer.initial;

        match self.pointers.len() {
            0 => {
                match self.gesture_state {
                    GestureState::Pending => (),
                    GestureState::Started => {
                        if let Some(handler) = &self.drag.on_end {
                            handler.borrow_mut()(DragEndData {
                                pointer: OffsetPointer {
                                    initial_data,
                                    final_data: pointer_data,
                                },
                            });
                        }
                    }
                };
            }
            1 => {
                match self.gesture_state {
                    GestureState::Pending => (),
                    GestureState::Started => {
                        if let Some(handler) = &self.pinch.on_end {
                            let removed = OffsetPointer {
                                initial_data,
                                final_data: pointer_data,
                            };
                            let pointer = &self.pointers[0];
                            let retained = OffsetPointer {
                                initial_data: Rc::clone(&pointer.initial),
                                final_data: Rc::clone(&pointer.current),
                            };
                            handler.borrow_mut()(PinchEndData {
                                pointers: if index == 0 {
                                    [removed, retained]
                                } else {
                                    [retained, removed]
                                },
                            });
                        }
                    }
                };

                self.pointers[0].initial = Rc::clone(&self.pointers[0].current);

                self.gesture_state = match (self.drag.has_started)([
                    &self.pointers[0].current,
                    &self.pointers[0].current,
                ]) {
                    false => GestureState::Pending,
                    true => GestureState::Started,
                };

                match self.gesture_state {
                    GestureState::Started => {
                        if let Some(handler) = &self.drag.on_start {
                            handler.borrow_mut()(DragStartData {
                                pointer: InitialPointer {
                                    data: Rc::clone(&self.pointers[0].initial),
                                },
                            });
                        }
                    }
                    _ => (),
                }
            }
            2 => {
                self.pointers
                    .iter_mut()
                    .for_each(|pointer| pointer.initial = Rc::clone(&pointer.current));

                let [first, second]: &[DownPointerState; 2] =
                    self.pointers[0..=1].try_into().unwrap();

                self.gesture_state = match (self.pinch.has_started)(
                    [&first.initial, &first.current],
                    [&second.initial, &second.current],
                ) {
                    false => GestureState::Pending,
                    true => GestureState::Started,
                };

                match self.gesture_state {
                    GestureState::Started => {
                        if let Some(handler) = &self.pinch.on_start {
                            handler.borrow_mut()(PinchStartData {
                                pointers: [
                                    InitialPointer {
                                        data: Rc::clone(&first.initial),
                                    },
                                    InitialPointer {
                                        data: Rc::clone(&second.initial),
                                    },
                                ],
                            });
                        }
                    }
                    _ => (),
                }
            }
            _ => {}
        }
    }

    fn cancel_known_down_pointer_event(&mut self, index: usize, pointer_data: Rc<PointerData>) {
        let pointer = self.pointers.remove(index);
        let initial_data = pointer.initial;

        match self.pointers.len() {
            0 => {
                match self.gesture_state {
                    GestureState::Pending => (),
                    GestureState::Started => {
                        if let Some(handler) = &self.drag.on_cancel {
                            handler.borrow_mut()(DragCancelData {
                                pointer: OffsetPointer {
                                    initial_data,
                                    final_data: pointer_data,
                                },
                            });
                        }
                    }
                };
            }
            1 => {
                match self.gesture_state {
                    GestureState::Pending => (),
                    GestureState::Started => {
                        if let Some(handler) = &self.pinch.on_cancel {
                            let removed = OffsetPointer {
                                initial_data,
                                final_data: pointer_data,
                            };
                            let pointer = &self.pointers[0];
                            let retained = OffsetPointer {
                                initial_data: Rc::clone(&pointer.initial),
                                final_data: Rc::clone(&pointer.current),
                            };
                            handler.borrow_mut()(PinchCancelData {
                                pointers: if index == 0 {
                                    [removed, retained]
                                } else {
                                    [retained, removed]
                                },
                            });
                        }
                    }
                };

                self.pointers[0].initial = Rc::clone(&self.pointers[0].current);

                self.gesture_state = match (self.drag.has_started)([
                    &self.pointers[0].current,
                    &self.pointers[0].current,
                ]) {
                    false => GestureState::Pending,
                    true => GestureState::Started,
                };

                match self.gesture_state {
                    GestureState::Started => {
                        if let Some(handler) = &self.drag.on_start {
                            handler.borrow_mut()(DragStartData {
                                pointer: InitialPointer {
                                    data: Rc::clone(&self.pointers[0].initial),
                                },
                            });
                        }
                    }
                    _ => (),
                }
            }
            2 => {
                self.pointers
                    .iter_mut()
                    .for_each(|pointer| pointer.initial = Rc::clone(&pointer.current));

                let [first, second]: &[DownPointerState; 2] =
                    self.pointers[0..=1].try_into().unwrap();

                self.gesture_state = match (self.pinch.has_started)(
                    [&first.initial, &first.current],
                    [&second.initial, &second.current],
                ) {
                    false => GestureState::Pending,
                    true => GestureState::Started,
                };

                match self.gesture_state {
                    GestureState::Started => {
                        if let Some(handler) = &self.pinch.on_start {
                            handler.borrow_mut()(PinchStartData {
                                pointers: [
                                    InitialPointer {
                                        data: Rc::clone(&first.initial),
                                    },
                                    InitialPointer {
                                        data: Rc::clone(&second.initial),
                                    },
                                ],
                            });
                        }
                    }
                    _ => (),
                }
            }
            _ => {}
        }
    }

    fn add_or_update(&mut self, event: &Event<PointerData>) {
        let pointer_data = event.data();
        let pointer_id = pointer_data.pointer_id();
        let associated_down_pointer_event = self
            .pointers
            .iter()
            .position(|p| p.id.is_equal_i32(pointer_id));
        match associated_down_pointer_event {
            Some(position) => {
                self.update_known_down_pointer_event(position, pointer_data);
            }
            None => {
                self.add_down_pointer_event(pointer_data);
            }
        };
    }

    fn update(&mut self, event: &Event<PointerData>) {
        let pointer_data = event.data();
        let pointer_id = pointer_data.pointer_id();
        let associated_down_pointer_event = self
            .pointers
            .iter()
            .position(|p| p.id.is_equal_i32(pointer_id));
        match associated_down_pointer_event {
            Some(position) => {
                self.update_known_down_pointer_event(position, pointer_data);
            }
            None => {}
        };
    }

    fn remove(&mut self, event: &Event<PointerData>) {
        let pointer_data = event.data();
        let pointer_id = pointer_data.pointer_id();
        let associated_down_pointer_event = self
            .pointers
            .iter()
            .position(|p| p.id.is_equal_i32(pointer_id));
        match associated_down_pointer_event {
            Some(position) => {
                self.remove_known_down_pointer_event(position, pointer_data);
            }
            None => {}
        };
    }

    fn cancel(&mut self, event: &Event<PointerData>) {
        let pointer_data = event.data();
        let pointer_id = pointer_data.pointer_id();
        let associacted_hover_event = self
            .pointers
            .iter()
            .position(|p| p.id.is_equal_i32(pointer_id));
        match associacted_hover_event {
            Some(position) => {
                self.cancel_known_down_pointer_event(position, pointer_data);
            }
            None => {}
        };
    }
}

impl PointerEventReceiver<&Event<PointerData>> for DownPointerGestureState {
    fn pointer_over(&mut self, _: &Event<PointerData>) {}

    fn pointer_enter(&mut self, _: &Event<PointerData>) {}

    fn pointer_down(&mut self, event: &Event<PointerData>) {
        self.add_or_update(event);
    }

    fn pointer_move(&mut self, event: &Event<PointerData>) {
        self.update(event);
    }

    fn pointer_up(&mut self, event: &Event<PointerData>) {
        self.remove(event);
    }

    fn pointer_cancel(&mut self, event: &Event<PointerData>) {
        self.cancel(event);
    }

    fn pointer_out(&mut self, event: &Event<PointerData>) {
        self.remove(event);
    }

    fn pointer_leave(&mut self, event: &Event<PointerData>) {
        self.remove(event);
    }
}
