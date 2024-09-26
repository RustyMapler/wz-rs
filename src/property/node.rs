use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, Weak},
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
        Self {
            name: name.clone(),
            value: value.into(),
            parent: parent.map(Arc::downgrade).unwrap_or_default(),
            children: HashMap::new(),
        }
    }
}

pub fn parse_property_list(
    parent: Option<&ArcDynamicWzNode>,
    reader: &Arc<WzReader>,
    offset: u32,
) -> Result<HashMap<String, Arc<DynamicWzNode>>, Box<dyn std::error::Error>> {
    let mut properties: HashMap<String, Arc<DynamicWzNode>> = HashMap::new();

    let num_entries = reader.read_wz_int()?;
    for _ in 0..num_entries {
        // We want to continue with the next entry if this fails (Doesn't panic)
        let name = reader.read_string_block(offset).unwrap_or_default();

        let property_type = reader.read_u8()?;
        match property_type {
            0 => {
                properties.insert(
                    name.clone(),
                    Arc::new(DynamicWzNode::new(&name, WzValue::Null, parent)),
                );
            }
            2 | 11 => {
                let value = reader.read_i16()?;
                properties.insert(
                    name.clone(),
                    Arc::new(DynamicWzNode::new(&name, WzValue::Short(value), parent)),
                );
            }
            3 | 19 => {
                let value = reader.read_wz_int()?;
                properties.insert(
                    name.clone(),
                    Arc::new(DynamicWzNode::new(&name, WzValue::Int(value), parent)),
                );
            }
            20 => {
                let value = reader.read_wz_long()?;
                properties.insert(
                    name.clone(),
                    Arc::new(DynamicWzNode::new(&name, WzValue::Long(value), parent)),
                );
            }
            4 => {
                let value = reader.read_u8().and_then(|sub| {
                    if sub == 0x80 {
                        reader.read_f32()
                    } else {
                        Ok(0.0)
                    }
                })?;
                properties.insert(
                    name.clone(),
                    Arc::new(DynamicWzNode::new(&name, WzValue::Float(value), parent)),
                );
            }
            5 => {
                let value = reader.read_f64()?;
                properties.insert(
                    name.clone(),
                    Arc::new(DynamicWzNode::new(&name, WzValue::Double(value), parent)),
                );
            }
            8 => {
                let value = reader.read_string_block(offset)?;
                properties.insert(
                    name.clone(),
                    Arc::new(DynamicWzNode::new(&name, WzValue::String(value), parent)),
                );
            }
            9 => {
                let eob = reader.read_u32()? + reader.get_position()? as u32;
                // let extended_property_reader = reader.clone();
                // let extended = parse_extended_property(
                //     root_obj,
                //     extended_property_reader,
                //     offset,
                //     name.clone(),
                // )?;
                // properties.insert(name.clone(), extended);
                reader.seek(eob as u64)?;
            }
            _ => {
                // We want to continue with the next entry (Do not throw an error)
                log::warn!("unsupported property type: {}, {}", name, property_type);
            }
        }
    }

    Ok(properties)
}
