use std::rc::Rc;

use dioxus::{
    events::{InteractionLocation, Modifiers, ModifiersInteraction, PointerInteraction},
    html::{
        geometry::{euclid::Vector2D, ClientSpace},
        input_data::MouseButtonSet,
        PointerData,
    },
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PointerId(pub i32);

impl PointerId {
    pub fn is_equal_i32(&self, probe: i32) -> bool {
        self.0 == probe
    }
}

impl From<i32> for PointerId {
    fn from(value: i32) -> Self {
        PointerId(value)
    }
}

pub struct IncrementalOffsetPointer {
    pub initial_data: Rc<PointerData>,
    pub preceding_data: Rc<PointerData>,
    pub current_data: Rc<PointerData>,
}

impl IncrementalOffsetPointer {
    pub fn delta_movement(&self) -> Vector2D<f64, ClientSpace> {
        PointerDataDifference::coordinates(&self.current_data, &self.preceding_data)
    }

    pub fn delta_width(&self) -> i32 {
        PointerDataDifference::width(&self.current_data, &self.preceding_data)
    }

    pub fn delta_height(&self) -> i32 {
        PointerDataDifference::height(&self.current_data, &self.preceding_data)
    }

    pub fn delta_pressure(&self) -> f32 {
        PointerDataDifference::pressure(&self.current_data, &self.preceding_data)
    }

    pub fn delta_tangential_pressure(&self) -> f32 {
        PointerDataDifference::tangential_pressure(&self.current_data, &self.preceding_data)
    }

    pub fn delta_tilt_x(&self) -> i32 {
        PointerDataDifference::tilt_x(&self.current_data, &self.preceding_data)
    }

    pub fn delta_tilt_y(&self) -> i32 {
        PointerDataDifference::tilt_y(&self.current_data, &self.preceding_data)
    }

    pub fn delta_twist(&self) -> i32 {
        PointerDataDifference::twist(&self.current_data, &self.preceding_data)
    }

    pub fn delta_is_primary(&self) -> bool {
        PointerDataDifference::is_primary(&self.current_data, &self.preceding_data)
    }

    pub fn delta_modifiers(&self) -> Modifiers {
        PointerDataDifference::modifiers(&self.current_data, &self.preceding_data)
    }

    pub fn delta_held_buttons(&self) -> MouseButtonSet {
        PointerDataDifference::held_buttons(&self.current_data, &self.preceding_data)
    }
}

impl IncrementalOffsetPointer {
    pub fn offset_movement(&self) -> Vector2D<f64, ClientSpace> {
        PointerDataDifference::coordinates(&self.current_data, &self.initial_data)
    }

    pub fn offset_width(&self) -> i32 {
        PointerDataDifference::width(&self.current_data, &self.initial_data)
    }

    pub fn offset_height(&self) -> i32 {
        PointerDataDifference::height(&self.current_data, &self.initial_data)
    }

    pub fn offset_pressure(&self) -> f32 {
        PointerDataDifference::pressure(&self.current_data, &self.initial_data)
    }

    pub fn offset_tangential_pressure(&self) -> f32 {
        PointerDataDifference::tangential_pressure(&self.current_data, &self.initial_data)
    }

    pub fn offset_tilt_x(&self) -> i32 {
        PointerDataDifference::tilt_x(&self.current_data, &self.initial_data)
    }

    pub fn offset_tilt_y(&self) -> i32 {
        PointerDataDifference::tilt_y(&self.current_data, &self.initial_data)
    }

    pub fn offset_twist(&self) -> i32 {
        PointerDataDifference::twist(&self.current_data, &self.initial_data)
    }

    pub fn offset_is_primary(&self) -> bool {
        PointerDataDifference::is_primary(&self.current_data, &self.initial_data)
    }

    pub fn offset_modifiers(&self) -> Modifiers {
        PointerDataDifference::modifiers(&self.current_data, &self.initial_data)
    }

    pub fn offset_held_buttons(&self) -> MouseButtonSet {
        PointerDataDifference::held_buttons(&self.current_data, &self.initial_data)
    }
}

pub struct OffsetPointer {
    pub initial_data: Rc<PointerData>,
    pub final_data: Rc<PointerData>,
}

impl OffsetPointer {
    pub fn offset_movement(&self) -> Vector2D<f64, ClientSpace> {
        PointerDataDifference::coordinates(&self.final_data, &self.initial_data)
    }

    pub fn offset_width(&self) -> i32 {
        PointerDataDifference::width(&self.final_data, &self.initial_data)
    }

    pub fn offset_height(&self) -> i32 {
        PointerDataDifference::height(&self.final_data, &self.initial_data)
    }

    pub fn offset_pressure(&self) -> f32 {
        PointerDataDifference::pressure(&self.final_data, &self.initial_data)
    }

    pub fn offset_tangential_pressure(&self) -> f32 {
        PointerDataDifference::tangential_pressure(&self.final_data, &self.initial_data)
    }

    pub fn offset_tilt_x(&self) -> i32 {
        PointerDataDifference::tilt_x(&self.final_data, &self.initial_data)
    }

    pub fn offset_tilt_y(&self) -> i32 {
        PointerDataDifference::tilt_y(&self.final_data, &self.initial_data)
    }

    pub fn offset_twist(&self) -> i32 {
        PointerDataDifference::twist(&self.final_data, &self.initial_data)
    }

    pub fn offset_is_primary(&self) -> bool {
        PointerDataDifference::is_primary(&self.final_data, &self.initial_data)
    }

    pub fn offset_modifiers(&self) -> Modifiers {
        PointerDataDifference::modifiers(&self.final_data, &self.initial_data)
    }

    pub fn offset_held_buttons(&self) -> MouseButtonSet {
        PointerDataDifference::held_buttons(&self.final_data, &self.initial_data)
    }
}

pub struct InitialPointer {
    pub data: Rc<PointerData>,
}

struct PointerDataDifference;

impl PointerDataDifference {
    fn coordinates(a: &PointerData, b: &PointerData) -> Vector2D<f64, ClientSpace> {
        a.client_coordinates() - b.client_coordinates()
    }

    fn width(a: &PointerData, b: &PointerData) -> i32 {
        a.width() - b.width()
    }

    fn height(a: &PointerData, b: &PointerData) -> i32 {
        a.height() - b.height()
    }

    fn pressure(a: &PointerData, b: &PointerData) -> f32 {
        a.pressure() - b.pressure()
    }

    fn tangential_pressure(a: &PointerData, b: &PointerData) -> f32 {
        a.tangential_pressure() - b.tangential_pressure()
    }

    fn tilt_x(a: &PointerData, b: &PointerData) -> i32 {
        a.tilt_x() - b.tilt_x()
    }

    fn tilt_y(a: &PointerData, b: &PointerData) -> i32 {
        a.tilt_y() - b.tilt_y()
    }

    fn twist(a: &PointerData, b: &PointerData) -> i32 {
        a.twist() - b.twist()
    }

    fn is_primary(a: &PointerData, b: &PointerData) -> bool {
        a.is_primary() ^ b.is_primary()
    }

    fn modifiers(a: &PointerData, b: &PointerData) -> Modifiers {
        a.modifiers().symmetric_difference(b.modifiers())
    }

    fn held_buttons(a: &PointerData, b: &PointerData) -> MouseButtonSet {
        a.held_buttons().symmetrical_difference(b.held_buttons())
    }
}
