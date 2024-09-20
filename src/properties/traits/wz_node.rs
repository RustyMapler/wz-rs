use crate::WzProperty;
use std::collections::HashMap;

#[allow(unused_variables)]
pub trait WzNode: WzProperty {
    fn get_name(&self) -> String;

    fn get_child(&self, name: &str) -> Option<&dyn WzNode> {
        None
    }

    fn get_child_mut(&mut self, name: &str) -> Option<&mut dyn WzNode> {
        None
    }

    fn list_children(&self) -> Vec<String> {
        vec![]
    }

    fn get_children(&self) -> &HashMap<String, Box<dyn WzNode>> {
        todo!()
    }

    fn parse(&mut self) {}

    fn resolve(&mut self, path: &str) -> Option<&mut dyn WzNode>
    where
        Self: Sized,
    {
        let mut current_node: &mut dyn WzNode = self;

        // Traverse node's children
        for p in path.split('/') {
            let tmp = &mut (*current_node);
            if let Some(next_node) = tmp.get_child_mut(p) {
                current_node = next_node;
            } else {
                log::warn!("Failed to resolve: '{}' at '{}'", path, p);
                return None;
            }
        }

        current_node.parse();

        Some(current_node)
    }
}
