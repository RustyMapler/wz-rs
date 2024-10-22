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
        let mut state = serializer.serialize_map(None)?;

        if self.children.is_empty() {
            match &self.value {
                WzValue::Short(val) => state.serialize_entry(&self.name, val)?,
                WzValue::Int(val) => state.serialize_entry(&self.name, val)?,
                WzValue::Long(val) => state.serialize_entry(&self.name, val)?,
                WzValue::Float(val) => state.serialize_entry(&self.name, val)?,
                WzValue::Double(val) => state.serialize_entry(&self.name, val)?,
                WzValue::String(val) => state.serialize_entry(&self.name, val)?,
                WzValue::Vector(val) => state.serialize_entry(&self.name, val)?,
                _ => {} // Skip other types
            }
        } else {
            for (key, child) in &self.children {
                if let WzValue::Extended = &self.value {
                    match &child.value {
                        WzValue::Short(val) => state.serialize_entry(key, val)?,
                        WzValue::Int(val) => state.serialize_entry(key, val)?,
                        WzValue::Long(val) => state.serialize_entry(key, val)?,
                        WzValue::Float(val) => state.serialize_entry(key, val)?,
                        WzValue::Double(val) => state.serialize_entry(key, val)?,
                        WzValue::String(val) => state.serialize_entry(key, val)?,
                        WzValue::Vector(val) => state.serialize_entry(key, val)?,
                        _ => state.serialize_entry(key, child.as_ref())?,
                    }
                } else {
                    state.serialize_entry(key, child.as_ref())?;
                }
            }
        }

        state.end()
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
