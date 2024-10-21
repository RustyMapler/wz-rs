use super::WzValue;
use indexmap::IndexMap;
use serde::{ser::SerializeMap, Serialize, Serializer};
use std::{
    fmt,
    io::{Error, ErrorKind},
    sync::Arc,
};

pub struct WzNode {
    pub name: String,
    pub offset: usize,
    pub value: WzValue,
    pub children: IndexMap<String, Arc<WzNode>>,
}

pub type ArcWzNode = Arc<WzNode>;

impl WzNode {
    pub fn new(name: &String, offset: usize, value: impl Into<WzValue>) -> Self {
        Self::new_with_children(name, offset, value, IndexMap::new())
    }

    pub fn new_with_children(
        name: &String,
        offset: usize,
        value: impl Into<WzValue>,
        children: IndexMap<String, ArcWzNode>,
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

    pub fn display(&self) -> String {
        format!("{}({})", self.name, self.offset)
    }
}

impl fmt::Display for WzNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let children: Vec<String> = self.children.keys().cloned().collect();
        write!(
            f,
            "name(\"{}\"), offset({}), value({:?}), children({:?})",
            self.name, self.offset, self.value, children
        )
    }
}

impl Serialize for WzNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(4))?;
        state.serialize_entry("name", &self.name)?;
        state.serialize_entry("offset", &self.offset)?;
        state.serialize_entry("value", &self.value)?;

        let children: IndexMap<_, _> = self
            .children
            .iter()
            .map(|(k, v)| (k.clone(), &**v))
            .collect();
        state.serialize_entry("children", &children)?;

        state.end()
    }
}

// Function to recursively print the node names and their children
pub fn print_node(node: &ArcWzNode, depth: usize) {
    let indent = "-".repeat(depth);
    println!("{}{}({})", indent, node.name, node.value);

    for child in node.children.values() {
        print_node(child, depth + 1)
    }
}

// Function to resolve a path to a child node
pub fn resolve(node: &ArcWzNode, path: &str) -> Result<ArcWzNode, Error> {
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
