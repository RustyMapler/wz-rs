use std::{
    collections::HashMap, error::Error, io::ErrorKind, sync::{Arc, Weak}
};

use crate::WzReader;

use super::WzValue;

pub struct DynamicWzNode {
    pub name: String,
    pub value: WzValue,
    pub parent: Weak<DynamicWzNode>,
    pub children: HashMap<String, Arc<DynamicWzNode>>,
}

pub type ArcDynamicWzNode = Arc<DynamicWzNode>;

impl DynamicWzNode {
    pub fn new(
        name: &String,
        value: impl Into<WzValue>,
        parent: Option<&ArcDynamicWzNode>,
    ) -> Self {
        Self::new_with_children(name, value, parent, HashMap::new())
    }

    pub fn new_with_children(
        name: &String,
        value: impl Into<WzValue>,
        parent: Option<&ArcDynamicWzNode>,
        children: HashMap<String, Arc<DynamicWzNode>>,
    ) -> Self {
        Self {
            name: name.clone(),
            value: value.into(),
            parent: parent.map(Arc::downgrade).unwrap_or_default(),
            children,
        }
    }
}

pub fn parse_property(
    parent: Option<&ArcDynamicWzNode>,
    reader: &Arc<WzReader>,
    offset: u32,
) -> Result<HashMap<String, Arc<DynamicWzNode>>, Box<dyn std::error::Error>> {
    let mut properties = HashMap::new();

    let num_entries = reader.read_wz_int()?;
    for _ in 0..num_entries {
        // Continue to the next entry without panicking if reading fails
        let name = reader.read_string_block(offset).unwrap_or_default();
        let property_type = reader.read_u8()?;

        let node = match property_type {
            0 => DynamicWzNode::new(&name, WzValue::Null, parent),
            2 | 11 => {
                let value = reader.read_i16()?;
                DynamicWzNode::new(&name, WzValue::Short(value), parent)
            }
            3 | 19 => {
                let value = reader.read_wz_int()?;
                DynamicWzNode::new(&name, WzValue::Int(value), parent)
            }
            20 => {
                let value = reader.read_wz_long()?;
                DynamicWzNode::new(&name, WzValue::Long(value), parent)
            }
            4 => {
                let value = match reader.read_u8()? {
                    0x80 => reader.read_f32()?,
                    _ => 0.0,
                };
                DynamicWzNode::new(&name, WzValue::Float(value), parent)
            }
            5 => {
                let value = reader.read_f64()?;
                DynamicWzNode::new(&name, WzValue::Double(value), parent)
            }
            8 => {
                let value = reader.read_string_block(offset)?;
                DynamicWzNode::new(&name, WzValue::String(value), parent)
            }
            9 => {
                continue;
            }
            _ => {
                log::warn!("unsupported property type: {}, {}", name, property_type);
                continue; // Skip unsupported types
            }
        };

        properties.insert(name, Arc::new(node));
    }

    Ok(properties)
}
