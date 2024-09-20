use crate::{WzNode, WzProperty};

pub struct WzShortProperty {
    pub name: String,
    pub value: i16,
}

impl WzProperty for WzShortProperty {
    fn get_short(&self) -> Option<i16> {
        Some(self.value)
    }
}

impl WzNode for WzShortProperty {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
