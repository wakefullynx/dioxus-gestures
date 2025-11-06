use std::{cell::RefCell, rc::Rc};


use dioxus::html::point_interaction::{InteractionElementOffset, InteractionLocation};
use dioxus::{
    html::{
        geometry::{
            euclid::{Angle, Point2D, Vector2D},
            ClientPoint, ClientSpace, ElementPoint, PagePoint, ScreenPoint,
        },
        PointerData,
    },
};

use crate::state::gestures::pointer::{IncrementalOffsetPointer, InitialPointer, OffsetPointer};

/// ```rust
/// use dioxus::{html::geometry::euclid::Angle, prelude::*};
/// use dioxus_gestures::{
///     state::gestures::pinch::Pinch,
///     use_gestures::{use_gestures, Gestures},
/// };
///
/// const UNDRAGGED_TEXT: &str = "Pinch me!";
/// const DRAGGED_TEXT: &str = "Pinching ...";
///
/// #[component]
/// pub fn PinchExample() -> Element {
///     let mut rotation = use_signal(|| Angle::<f64>::default());
///     let mut scale = use_signal(|| 1.0);
///     let mut text = use_signal(|| UNDRAGGED_TEXT);
///
///     let gestures = use_gestures(
///         Gestures::default().pinch(
///             Pinch::default()
///                 .on_start(move |_| text.set(DRAGGED_TEXT))
///                 .on_update(move |data| {
///                     scale.set(scale() * data.delta_distance_scale());
///                     rotation.set(rotation() + data.delta_angle());
///                 })
///                 .on_end(move |_| text.set(UNDRAGGED_TEXT))
///                 .on_cancel(move |_| text.set(UNDRAGGED_TEXT)),
///         ),
///     );
///
///     rsx! {
///         div {
///         class: "target pinch",
///         style: format!("
///             user-select: none;
///             touch-action: none;
///             position: relative;
///             transform: scale({}) rotate({}rad);
///         ", scale(), rotation().get()),
///         ..gestures.event_handlers(),
///             "{text}"
///         }
///     }
/// }
/// ```
#[derive(Clone)]
pub struct Pinch {
    pub on_start: Option<Rc<RefCell<dyn FnMut(PinchStartData)>>>,
    pub on_update: Option<Rc<RefCell<dyn FnMut(PinchUpdateData)>>>,
    pub on_end: Option<Rc<RefCell<dyn FnMut(PinchEndData)>>>,
    pub on_cancel: Option<Rc<RefCell<dyn FnMut(PinchCancelData)>>>,
    pub has_started: Rc<dyn Fn([&PointerData; 2], [&PointerData; 2]) -> bool>,
}

impl Pinch {
    pub fn on_start(mut self, handler: impl FnMut(PinchStartData) + 'static) -> Self {
        self.on_start = Some(Rc::new(RefCell::new(handler)));
        self
    }

    pub fn on_update(mut self, handler: impl FnMut(PinchUpdateData) + 'static) -> Self {
        self.on_update = Some(Rc::new(RefCell::new(handler)));
        self
    }

    pub fn on_end(mut self, handler: impl FnMut(PinchEndData) + 'static) -> Self {
        self.on_end = Some(Rc::new(RefCell::new(handler)));
        self
    }

    pub fn on_cancel(mut self, handler: impl FnMut(PinchCancelData) + 'static) -> Self {
        self.on_cancel = Some(Rc::new(RefCell::new(handler)));
        self
    }

    pub fn has_started(
        mut self,
        predicate: impl Fn([&PointerData; 2], [&PointerData; 2]) -> bool + 'static,
    ) -> Self {
        self.has_started = Rc::new(predicate);
        self
    }
}

impl Default for Pinch {
    fn default() -> Self {
        Self {
            on_start: Default::default(),
            on_update: Default::default(),
            on_end: Default::default(),
            on_cancel: Default::default(),
            has_started: Rc::new(|[a_initial, a_current], [b_initial, b_current]| {
                (a_current.client_coordinates() - a_initial.client_coordinates()).length() >= 5.0
                    || (b_current.client_coordinates() - b_initial.client_coordinates()).length()
                        >= 5.0
            }),
        }
    }
}

pub struct PinchStartData {
    pub pointers: [InitialPointer; 2],
}

impl PinchStartData {
    pub fn client_center(&self) -> ClientPoint {
        PinchData::client_center(&self.pointers[0].data, &self.pointers[1].data)
    }

    pub fn screen_center(&self) -> ScreenPoint {
        PinchData::screen_center(&self.pointers[0].data, &self.pointers[1].data)
    }

    pub fn page_center(&self) -> PagePoint {
        PinchData::page_center(&self.pointers[0].data, &self.pointers[1].data)
    }

    pub fn element_center(&self) -> ElementPoint {
        PinchData::element_center(&self.pointers[0].data, &self.pointers[1].data)
    }

    pub fn distance(&self) -> f64 {
        PinchData::distance(&self.pointers[0].data, &self.pointers[1].data)
    }

    pub fn angle(&self) -> Angle<f64> {
        PinchData::angle(&self.pointers[0].data, &self.pointers[1].data)
    }
}

pub struct PinchUpdateData {
    pub pointers: [IncrementalOffsetPointer; 2],
    pub updated_pointer: PinchUpdatedPointer,
}

pub enum PinchUpdatedPointer {
    First,
    Second,
}

impl PinchUpdateData {
    pub fn start_client_center(&self) -> ClientPoint {
        PinchData::client_center(
            &self.pointers[0].initial_data,
            &self.pointers[1].initial_data,
        )
    }

    pub fn start_screen_center(&self) -> ScreenPoint {
        PinchData::screen_center(
            &self.pointers[0].initial_data,
            &self.pointers[1].initial_data,
        )
    }

    pub fn start_page_center(&self) -> PagePoint {
        PinchData::page_center(
            &self.pointers[0].initial_data,
            &self.pointers[1].initial_data,
        )
    }

    pub fn start_element_center(&self) -> ElementPoint {
        PinchData::element_center(
            &self.pointers[0].initial_data,
            &self.pointers[1].initial_data,
        )
    }

    pub fn start_distance(&self) -> f64 {
        PinchData::distance(
            &self.pointers[0].initial_data,
            &self.pointers[1].initial_data,
        )
    }

    pub fn start_angle(&self) -> Angle<f64> {
        PinchData::angle(
            &self.pointers[0].initial_data,
            &self.pointers[1].initial_data,
        )
    }

    pub fn preceding_client_center(&self) -> ClientPoint {
        PinchData::client_center(
            &self.pointers[0].preceding_data,
            &self.pointers[1].preceding_data,
        )
    }

    pub fn preceding_screen_center(&self) -> ScreenPoint {
        PinchData::screen_center(
            &self.pointers[0].preceding_data,
            &self.pointers[1].preceding_data,
        )
    }

    pub fn preceding_page_center(&self) -> PagePoint {
        PinchData::page_center(
            &self.pointers[0].preceding_data,
            &self.pointers[1].preceding_data,
        )
    }

    pub fn preceding_element_center(&self) -> ElementPoint {
        PinchData::element_center(
            &self.pointers[0].preceding_data,
            &self.pointers[1].preceding_data,
        )
    }

    pub fn preceding_distance(&self) -> f64 {
        PinchData::distance(
            &self.pointers[0].preceding_data,
            &self.pointers[1].preceding_data,
        )
    }

    pub fn preceding_angle(&self) -> Angle<f64> {
        PinchData::angle(
            &self.pointers[0].preceding_data,
            &self.pointers[1].preceding_data,
        )
    }

    pub fn delta_movement(&self) -> Vector2D<f64, ClientSpace> {
        self.current_client_center() - self.preceding_client_center()
    }

    pub fn delta_distance(&self) -> f64 {
        self.current_distance() - self.preceding_distance()
    }

    pub fn delta_distance_scale(&self) -> f64 {
        self.current_distance() / self.preceding_distance()
    }

    pub fn delta_angle(&self) -> Angle<f64> {
        PinchData::angle_difference(self.current_angle(), self.preceding_angle())
    }

    pub fn current_client_center(&self) -> ClientPoint {
        PinchData::client_center(
            &self.pointers[0].current_data,
            &self.pointers[1].current_data,
        )
    }

    pub fn current_screen_center(&self) -> ScreenPoint {
        PinchData::screen_center(
            &self.pointers[0].current_data,
            &self.pointers[1].current_data,
        )
    }

    pub fn current_page_center(&self) -> PagePoint {
        PinchData::page_center(
            &self.pointers[0].current_data,
            &self.pointers[1].current_data,
        )
    }

    pub fn current_element_center(&self) -> ElementPoint {
        PinchData::element_center(
            &self.pointers[0].current_data,
            &self.pointers[1].current_data,
        )
    }

    pub fn current_distance(&self) -> f64 {
        PinchData::distance(
            &self.pointers[0].current_data,
            &self.pointers[1].current_data,
        )
    }

    pub fn current_angle(&self) -> Angle<f64> {
        PinchData::angle(
            &self.pointers[0].current_data,
            &self.pointers[1].current_data,
        )
    }

    pub fn offset_movement(&self) -> Vector2D<f64, ClientSpace> {
        self.current_client_center() - self.start_client_center()
    }

    pub fn offset_distance(&self) -> f64 {
        self.current_distance() - self.start_distance()
    }

    pub fn offset_distance_scale(&self) -> f64 {
        self.current_distance() / self.start_distance()
    }

    pub fn offset_angle(&self) -> Angle<f64> {
        PinchData::angle_difference(self.current_angle(), self.start_angle())
    }
}

pub struct PinchEndData {
    pub pointers: [OffsetPointer; 2],
}

impl PinchEndData {
    pub fn start_client_center(&self) -> ClientPoint {
        PinchData::client_center(
            &self.pointers[0].initial_data,
            &self.pointers[1].initial_data,
        )
    }

    pub fn start_screen_center(&self) -> ScreenPoint {
        PinchData::screen_center(
            &self.pointers[0].initial_data,
            &self.pointers[1].initial_data,
        )
    }

    pub fn start_page_center(&self) -> PagePoint {
        PinchData::page_center(
            &self.pointers[0].initial_data,
            &self.pointers[1].initial_data,
        )
    }

    pub fn start_element_center(&self) -> ElementPoint {
        PinchData::element_center(
            &self.pointers[0].initial_data,
            &self.pointers[1].initial_data,
        )
    }

    pub fn start_distance(&self) -> f64 {
        PinchData::distance(
            &self.pointers[0].initial_data,
            &self.pointers[1].initial_data,
        )
    }

    pub fn start_angle(&self) -> Angle<f64> {
        PinchData::angle(
            &self.pointers[0].initial_data,
            &self.pointers[1].initial_data,
        )
    }

    pub fn end_client_center(&self) -> ClientPoint {
        PinchData::client_center(&self.pointers[0].final_data, &self.pointers[1].final_data)
    }

    pub fn end_screen_center(&self) -> ScreenPoint {
        PinchData::screen_center(&self.pointers[0].final_data, &self.pointers[1].final_data)
    }

    pub fn end_page_center(&self) -> PagePoint {
        PinchData::page_center(&self.pointers[0].final_data, &self.pointers[1].final_data)
    }

    pub fn end_element_center(&self) -> ElementPoint {
        PinchData::element_center(&self.pointers[0].final_data, &self.pointers[1].final_data)
    }

    pub fn end_distance(&self) -> f64 {
        PinchData::distance(&self.pointers[0].final_data, &self.pointers[1].final_data)
    }

    pub fn end_angle(&self) -> Angle<f64> {
        PinchData::angle(&self.pointers[0].final_data, &self.pointers[1].final_data)
    }

    pub fn offset_movement(&self) -> Vector2D<f64, ClientSpace> {
        self.end_client_center() - self.start_client_center()
    }

    pub fn offset_distance(&self) -> f64 {
        self.end_distance() - self.start_distance()
    }

    pub fn offset_distance_scale(&self) -> f64 {
        self.end_distance() / self.start_distance()
    }

    pub fn offset_angle(&self) -> Angle<f64> {
        PinchData::angle_difference(self.end_angle(), self.start_angle())
    }
}

pub struct PinchCancelData {
    pub pointers: [OffsetPointer; 2],
}

impl PinchCancelData {
    pub fn start_client_center(&self) -> ClientPoint {
        PinchData::client_center(
            &self.pointers[0].initial_data,
            &self.pointers[1].initial_data,
        )
    }

    pub fn start_screen_center(&self) -> ScreenPoint {
        PinchData::screen_center(
            &self.pointers[0].initial_data,
            &self.pointers[1].initial_data,
        )
    }

    pub fn start_page_center(&self) -> PagePoint {
        PinchData::page_center(
            &self.pointers[0].initial_data,
            &self.pointers[1].initial_data,
        )
    }

    pub fn start_element_center(&self) -> ElementPoint {
        PinchData::element_center(
            &self.pointers[0].initial_data,
            &self.pointers[1].initial_data,
        )
    }

    pub fn start_distance(&self) -> f64 {
        PinchData::distance(
            &self.pointers[0].initial_data,
            &self.pointers[1].initial_data,
        )
    }

    pub fn start_angle(&self) -> Angle<f64> {
        PinchData::angle(
            &self.pointers[0].initial_data,
            &self.pointers[1].initial_data,
        )
    }

    pub fn end_client_center(&self) -> ClientPoint {
        PinchData::client_center(&self.pointers[0].final_data, &self.pointers[1].final_data)
    }

    pub fn end_screen_center(&self) -> ScreenPoint {
        PinchData::screen_center(&self.pointers[0].final_data, &self.pointers[1].final_data)
    }

    pub fn end_page_center(&self) -> PagePoint {
        PinchData::page_center(&self.pointers[0].final_data, &self.pointers[1].final_data)
    }

    pub fn end_element_center(&self) -> ElementPoint {
        PinchData::element_center(&self.pointers[0].final_data, &self.pointers[1].final_data)
    }

    pub fn end_distance(&self) -> f64 {
        PinchData::distance(&self.pointers[0].final_data, &self.pointers[1].final_data)
    }

    pub fn end_angle(&self) -> Angle<f64> {
        PinchData::angle(&self.pointers[0].final_data, &self.pointers[1].final_data)
    }

    pub fn offset_movement(&self) -> Vector2D<f64, ClientSpace> {
        self.end_client_center() - self.start_client_center()
    }

    pub fn offset_distance(&self) -> f64 {
        self.end_distance() - self.start_distance()
    }

    pub fn offset_distance_scale(&self) -> f64 {
        self.end_distance() / self.start_distance()
    }

    pub fn offset_angle(&self) -> Angle<f64> {
        PinchData::angle_difference(self.end_angle(), self.start_angle())
    }
}

struct PinchData;

impl PinchData {
    fn client_center(a: &PointerData, b: &PointerData) -> ClientPoint {
        Self::center(a.client_coordinates(), b.client_coordinates())
    }

    fn screen_center(a: &PointerData, b: &PointerData) -> ScreenPoint {
        Self::center(a.screen_coordinates(), b.screen_coordinates())
    }

    fn page_center(a: &PointerData, b: &PointerData) -> PagePoint {
        Self::center(a.page_coordinates(), b.page_coordinates())
    }

    fn element_center(a: &PointerData, b: &PointerData) -> ElementPoint {
        Self::center(a.element_coordinates(), b.element_coordinates())
    }

    fn center<T>(a: Point2D<f64, T>, b: Point2D<f64, T>) -> Point2D<f64, T> {
        a.lerp(b, 0.5)
    }

    fn distance(a: &PointerData, b: &PointerData) -> f64 {
        a.client_coordinates().distance_to(b.client_coordinates())
    }

    fn angle(a: &PointerData, b: &PointerData) -> Angle<f64> {
        (b.client_coordinates() - a.client_coordinates()).angle_from_x_axis()
    }

    fn angle_difference(a: Angle<f64>, b: Angle<f64>) -> Angle<f64> {
        (a - b).signed()
    }
}
