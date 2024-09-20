use crate::{WzNode, WzProperty};

pub struct WzFloatProperty {
    pub name: String,
    pub value: f32,
}

impl WzProperty for WzFloatProperty {
    fn get_float(&self) -> Option<f32> {
        Some(self.value)
    }
}

impl WzNode for WzFloatProperty {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
