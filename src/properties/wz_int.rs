use crate::{WzNode, WzProperty};

pub struct WzIntProperty {
    pub name: String,
    pub value: i32,
}

impl WzProperty for WzIntProperty {
    fn get_int(&self) -> Option<i32> {
        Some(self.value)
    }
}

impl WzNode for WzIntProperty {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
