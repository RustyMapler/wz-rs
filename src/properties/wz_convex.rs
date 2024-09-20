use crate::{WzNode, WzProperty};

pub struct WzConvexProperty {
    pub name: String,
    pub properties: Vec<Box<dyn WzNode>>,
}

impl WzProperty for WzConvexProperty {}

impl WzNode for WzConvexProperty {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_child_mut(&mut self, name: &str) -> Option<&mut dyn WzNode> {
        for prop in self.properties.iter_mut() {
            if prop.get_name() == name {
                return Some(prop.as_mut());
            }
        }

        None
    }
}
