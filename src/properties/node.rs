use super::WzValue;
use std::{
    collections::HashMap,
    fmt,
    io::{Error, ErrorKind},
    sync::Arc,
};
pub struct WzNode {
    pub name: String,
    pub offset: usize,
    pub value: WzValue,
    pub children: HashMap<String, Arc<WzNode>>,
}

pub type ArcWzNode = Arc<WzNode>;

impl WzNode {
    pub fn new(name: &String, offset: usize, value: impl Into<WzValue>) -> Self {
        Self::new_with_children(name, offset, value, HashMap::new())
    }

    pub fn new_with_children(
        name: &String,
        offset: usize,
        value: impl Into<WzValue>,
        children: HashMap<String, Arc<WzNode>>,
    ) -> Self {
        let result = Self {
            name: name.clone(),
            offset,
            value: value.into(),
            children,
        };
        log::trace!("node::new | {}", result);
        result
    }
}

impl fmt::Display for WzNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let children: Vec<String> = self.children.keys().cloned().collect();
        write!(
            f,
            "name(\"{}\"), value({:?}), children({:?})",
            self.name, self.value, children
        )
    }
}

// Function to recursively print the node names and their children
pub fn print_node(node: &Arc<WzNode>, depth: usize) {
    let indent = "-".repeat(depth);
    println!("{}{}({})", indent, node.name, node.value);

    for child in node.children.values() {
        print_node(child, depth + 1)
    }
}

// Function to resolve a path to a child node
pub fn resolve(node: &Arc<WzNode>, path: &str) -> Result<Arc<WzNode>, Error> {
    let parts: Vec<&str> = path.split('/').collect();
    let mut current_node = Arc::clone(node);

    for part in parts.iter() {
        if let Some(child) = current_node.children.get(*part) {
            current_node = Arc::clone(child);
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                format!("Child node '{}' not found.", part),
            ))?
        }
    }

    Ok(current_node)
}
