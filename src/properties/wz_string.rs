use crate::{WzNode, WzProperty};

pub struct WzStringProperty {
    pub name: String,
    pub value: String,
}

impl WzProperty for WzStringProperty {
    fn get_string(&self) -> Option<String> {
        Some(self.value.clone())
    }
}

impl WzNode for WzStringProperty {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
