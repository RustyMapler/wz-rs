use crate::{WzNode, WzProperty};

pub struct WzLongProperty {
    pub name: String,
    pub value: i64,
}

impl WzProperty for WzLongProperty {
    fn get_long(&self) -> Option<i64> {
        Some(self.value)
    }
}

impl WzNode for WzLongProperty {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
