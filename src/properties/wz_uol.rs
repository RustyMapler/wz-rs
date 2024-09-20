use crate::{WzNode, WzProperty};

pub struct WzUolProperty {
    pub name: String,
    pub value: String,
}

impl WzProperty for WzUolProperty {
    fn get_string(&self) -> Option<String> {
        Some(self.value.clone())
    }

    fn is_uol(&self) -> bool {
        true
    }

    fn get_uol(&self) -> Option<String> {
        Some(self.value.clone())
    }
}

impl WzNode for WzUolProperty {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
