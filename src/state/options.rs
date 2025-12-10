use nanoid::nanoid;

#[derive(Clone)]
pub struct UseGesturesOptions {
    pub target_id_attribute_name: &'static str,
    pub target_id: Option<String>,
}

impl UseGesturesOptions {
    pub fn target_id_attribute_name(mut self, attribute_name: &'static str) -> Self {
        self.target_id_attribute_name = attribute_name;
        self
    }
}

impl UseGesturesOptions {
    pub fn target_id(mut self, target_id: String) -> Self {
        self.target_id = Some(target_id);
        self
    }
}

impl Default for UseGesturesOptions {
    fn default() -> Self {
        Self { target_id_attribute_name: "data-gestures-id", target_id: None }
    }
}