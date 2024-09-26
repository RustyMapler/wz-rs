use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use crate::WzReader;

use super::{WzSound, WzValue};

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
                let eob = reader.read_u32()? + reader.get_position()? as u32;
                let extended = parse_extended_property(parent, reader, offset, name.clone())?;
                reader.seek(eob as u64)?;

                DynamicWzNode::new_with_children(&name, WzValue::Null, parent, extended)
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

pub fn parse_extended_property(
    parent: Option<&ArcDynamicWzNode>,
    reader: &Arc<WzReader>,
    offset: u32,
    property_name: String,
) -> Result<HashMap<String, Arc<DynamicWzNode>>, Box<dyn std::error::Error>> {
    let mut properties = HashMap::new();

    let property_type = reader.read_string_block(offset)?;
    match property_type.as_str() {
        "Property" => {
            reader.skip(2)?;
            properties.extend(parse_property(parent, &reader, offset)?);
        }
        "Canvas" => {
            // TODO
        }
        "Shape2D#Vector2D" => {
            let x = reader.read_wz_int()?;
            let y = reader.read_wz_int()?;

            properties.insert(
                property_name.clone(),
                Arc::new(DynamicWzNode::new(
                    &property_name,
                    WzValue::Vector(x, y),
                    parent,
                )),
            );
        }
        "Shape2D#Convex2D" => {
            // TODO
        }
        "Sound_DX8" => {
            const SOUND_HEADER: [u8; 51] = [
                0x02, 0x83, 0xEB, 0x36, 0xE4, 0x4F, 0x52, 0xCE, 0x11, 0x9F, 0x53, 0x00, 0x20, 0xAF,
                0x0B, 0xA7, 0x70, 0x8B, 0xEB, 0x36, 0xE4, 0x4F, 0x52, 0xCE, 0x11, 0x9F, 0x53, 0x00,
                0x20, 0xAF, 0x0B, 0xA7, 0x70, 0x00, 0x01, 0x81, 0x9F, 0x58, 0x05, 0x56, 0xC3, 0xCE,
                0x11, 0xBF, 0x01, 0x00, 0xAA, 0x00, 0x55, 0x59, 0x5A,
            ];

            // Skip the first byte
            reader.skip(1)?;

            let data_len = reader.read_wz_int()?;
            let len = reader.read_wz_int()?;

            //  Read the header
            let header_offset = reader.get_position()?;
            reader.skip(SOUND_HEADER.len())?;
            let wav_len = reader.read_u8()?;
            reader.seek(header_offset)?;
            let header_len = SOUND_HEADER.len() as u64 + 1 + wav_len as u64;
            let header = reader.read_bytes(header_len)?;

            // Read the data
            let data_offset = reader.get_position()?;
            let data = reader.read_bytes(data_len as u64)?;

            let sound = WzSound {
                offset,
                len: len as u32,
                header_offset,
                header,
                data_offset,
                data,
                data_len: data_len as u32,
            };

            properties.insert(
                property_name.clone(),
                Arc::new(DynamicWzNode::new(
                    &property_name,
                    WzValue::Sound(sound),
                    parent,
                )),
            );
        }
        "UOL" => {
            reader.skip(1)?;
            let value = reader.read_string_block(offset)?;

            properties.insert(
                property_name.clone(),
                Arc::new(DynamicWzNode::new(
                    &property_name,
                    WzValue::Uol(value),
                    parent,
                )),
            );
        }
        _ => {
            let error_type = std::io::ErrorKind::Unsupported;
            let error_message = format!("Unsupported extended property type: {}", property_type);
            Err(Box::new(std::io::Error::new(error_type, error_message)))?
        }
    }

    Ok(properties)
}