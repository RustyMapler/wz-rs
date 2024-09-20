use crate::{WzNode, WzProperty};

pub struct WzDoubleProperty {
    pub name: String,
    pub value: f64,
}

impl WzProperty for WzDoubleProperty {
    fn get_double(&self) -> Option<f64> {
        Some(self.value)
    }
}

impl WzNode for WzDoubleProperty {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
