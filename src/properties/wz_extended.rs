use crate::{WzNode, WzProperty};
use std::collections::HashMap;

pub struct WzExtendedProperty {
    pub name: String,
    pub properties: HashMap<String, Box<dyn WzNode>>,
}

impl WzProperty for WzExtendedProperty {}

impl WzNode for WzExtendedProperty {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_child(&self, name: &str) -> Option<&dyn WzNode> {
        self.properties
            .get(name)
            .and_then(|child| Some(child.as_ref()))
    }

    fn get_child_mut(&mut self, name: &str) -> Option<&mut dyn WzNode> {
        match self.properties.get_mut(name) {
            Some(child) => Some(child.as_mut()),
            None => None,
        }
    }

    fn list_children(&self) -> Vec<String> {
        self.properties.keys().cloned().collect()
    }

    fn get_children(&self) -> &HashMap<String, Box<dyn WzNode>> {
        &self.properties
    }
}
