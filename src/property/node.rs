use super::WzValue;
use crate::{Vec2, WzReader, WzSound};
use std::{
    collections::HashMap,
    fmt,
    io::{Error, ErrorKind},
    sync::Arc,
};

pub struct DynamicWzNode {
    pub name: String,
    pub value: WzValue,
    pub children: HashMap<String, Arc<DynamicWzNode>>,
}

pub type ArcDynamicWzNode = Arc<DynamicWzNode>;

impl DynamicWzNode {
    pub fn new(name: &String, value: impl Into<WzValue>) -> Self {
        Self::new_with_children(name, value, HashMap::new())
    }

    pub fn new_with_children(
        name: &String,
        value: impl Into<WzValue>,
        children: HashMap<String, Arc<DynamicWzNode>>,
    ) -> Self {
        let result = Self {
            name: name.clone(),
            value: value.into(),
            children,
        };
        log::trace!("node::new | {}", result);
        result
    }
}

impl fmt::Display for DynamicWzNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let children: Vec<String> = self.children.keys().cloned().collect();
        write!(
            f,
            "name(\"{}\"), value({:?}), children({:?})",
            self.name, self.value, children
        )
    }
}

pub fn parse_directory(
    name: String,
    reader: &Arc<WzReader>,
    offset: u32,
) -> Result<ArcDynamicWzNode, Error> {
    let mut children = HashMap::new();

    reader.seek(offset as u64)?;

    let count = reader.read_wz_int()?;

    log::trace!(
        "parse_directory | name({}) offset({}) children({})",
        name,
        offset,
        count
    );

    for _ in 0..count {
        let remember_pos: u64;
        let mut entry_name = String::from("");
        let mut entry_type = reader.read_u8()?;

        match entry_type {
            2 => {
                let offset = reader.read_u32()?;
                remember_pos = reader.get_position()?;

                reader.seek((*reader.file_start.borrow() + offset) as u64)?;

                entry_type = reader.read_u8()?;
                entry_name = reader.read_wz_string()?;
            }
            3 | 4 => {
                entry_name = reader.read_wz_string()?;
                remember_pos = reader.get_position()?;
            }
            _ => {
                remember_pos = reader.get_position()?;
            }
        }

        // Seek back to the original position
        reader.seek(remember_pos)?;

        // Fetch some additional info
        let entry_fsize = reader.read_wz_int()?;
        let entry_checksum = reader.read_wz_int()?;
        let entry_offset = reader.read_wz_offset()?;

        log::trace!(
            "entry | type({}) name({}) fsize({}) checksum({}) offset({})",
            entry_type,
            entry_name,
            entry_fsize,
            entry_checksum,
            entry_offset
        );

        // Build directories and .imgs
        match entry_type {
            3 => {
                let remember_pos = reader.get_position()?;
                let node = parse_directory(entry_name.clone(), reader, entry_offset)?;
                reader.seek(remember_pos)?;
                children.insert(entry_name.clone(), node);
            }
            _ => {
                let remember_pos = reader.get_position()?;
                let node = parse_img(entry_name.clone(), reader, entry_offset)?;
                reader.seek(remember_pos)?;
                children.insert(entry_name.clone(), node);
            }
        }
    }

    let node = DynamicWzNode::new_with_children(&name, WzValue::Directory, children);
    Ok(Arc::new(node))
}

pub fn parse_img(
    name: String,
    reader: &Arc<WzReader>,
    offset: u32,
) -> Result<ArcDynamicWzNode, Error> {
    log::trace!("parse_img | name({}) offset({})", name, offset);

    reader.seek(offset as u64)?;

    // Read the first byte and check that this node is a .img
    let byte = reader.read_u8()?;
    match byte {
        WzReader::HEADERBYTE_WITHOUT_OFFSET => {
            let prop = reader.read_wz_string()?;
            let val = reader.read_u16()?;
            if prop != "Property" || val != 0 {
                let error_type = ErrorKind::Unsupported;
                let error_message = format!("Unsupported .img type: {} {}", prop, val);
                Err(Error::new(error_type, error_message))?
            }
        }
        _ => {
            let error_type = ErrorKind::Unsupported;
            let error_message = format!("Unsupported .img header: {}", byte);
            Err(Error::new(error_type, error_message))?
        }
    }

    // Continue parsing all properties for this node
    if let Ok(children) = parse_property(reader, offset) {
        Ok(Arc::new(DynamicWzNode::new_with_children(
            &name,
            WzValue::Img,
            children,
        )))
    } else {
        Ok(Arc::new(DynamicWzNode::new(&name, WzValue::Img)))
    }
}

pub fn parse_property(
    reader: &Arc<WzReader>,
    offset: u32,
) -> Result<HashMap<String, ArcDynamicWzNode>, Error> {
    log::trace!("parse_property | offset({})", offset);

    let mut children = HashMap::new();

    let num_entries = reader.read_wz_int()?;
    for _ in 0..num_entries {
        // Continue to the next entry without panicking if reading fails
        let name = reader.read_string_block(offset).unwrap_or_default();
        let property_type = reader.read_u8()?;
        let node = match property_type {
            0 => DynamicWzNode::new(&name, WzValue::Null),
            2 | 11 => {
                let value = reader.read_i16()?;
                DynamicWzNode::new(&name, WzValue::Short(value))
            }
            3 | 19 => {
                let value = reader.read_wz_int()?;
                DynamicWzNode::new(&name, WzValue::Int(value))
            }
            20 => {
                let value = reader.read_wz_long()?;
                DynamicWzNode::new(&name, WzValue::Long(value))
            }
            4 => {
                let value = match reader.read_u8()? {
                    0x80 => reader.read_f32()?,
                    _ => 0.0,
                };
                DynamicWzNode::new(&name, WzValue::Float(value))
            }
            5 => {
                let value = reader.read_f64()?;
                DynamicWzNode::new(&name, WzValue::Double(value))
            }
            8 => {
                let value = reader.read_string_block(offset)?;
                DynamicWzNode::new(&name, WzValue::String(value))
            }
            9 => {
                let remember_pos = reader.read_u32()? + reader.get_position()? as u32;
                let extended_children = parse_extended_property(name.clone(), reader, offset)?;
                reader.seek(remember_pos as u64)?;
                DynamicWzNode::new_with_children(&name, WzValue::Extended, extended_children)
            }
            _ => {
                log::warn!("unsupported property type: {}, {}", name, property_type);
                continue;
            }
        };

        children.insert(name, Arc::new(node));
    }

    Ok(children)
}

pub fn parse_extended_property(
    name: String,
    reader: &Arc<WzReader>,
    offset: u32,
) -> Result<HashMap<String, ArcDynamicWzNode>, Error> {
    log::trace!(
        "parse_extended_property | name({}) offset({})",
        name,
        offset
    );

    let mut extended_children = HashMap::new();

    let extended_property_type = reader.read_string_block(offset)?;
    match extended_property_type.as_str() {
        "Property" => {
            reader.skip(2)?;
            let properties = parse_property(reader, offset)?;
            extended_children.extend(properties);
        }
        "Canvas" => {
            // reader.skip(1)?;

            // let byte = reader.read_u8()?;

            // let mut properties = HashMap::new();
            // if byte == 1 {
            //     reader.skip(2)?;
            //     properties = parse_property(reader, offset)?;
            // }

            // let width = reader.read_wz_int()? as u32;
            // let height = reader.read_wz_int()? as u32;
            // let format1 = reader.read_wz_int()? as u32;
            // let format2 = reader.read_u8()?;

            // reader.skip(4)?;
            // let offset = reader.get_position()? as u32;

            // let len = reader.read_i32()? - 1;

            // reader.skip(1)?;

            // if len > 0 {
            //     reader.skip(len as usize)?;
            // }

            // let node = DynamicWzNode::new(&name, WzValue::Null);

            // extended_children.insert(name.clone(), Arc::new(node));
        }
        "Shape2D#Vector2D" => {
            let x = reader.read_wz_int()?;
            let y = reader.read_wz_int()?;

            let node = DynamicWzNode::new(&name, WzValue::Vector(Vec2 { x, y }));
            extended_children.insert(name.clone(), Arc::new(node));
        }
        "Shape2D#Convex2D" => {
            // let mut properties = HashMap::new();

            // let num_entries = reader.read_wz_int()?;
            // for _ in 0..num_entries {
            //     let convex_properties = parse_extended_property(name.clone(), reader, offset)?;
            //     let convex_node = DynamicWzNode::new_with_children(
            //         &name.clone(),
            //         WzValue::Null,
            //         convex_properties,
            //     );
            //     properties.insert(name.clone(), Arc::new(convex_node));
            // }

            // let node = DynamicWzNode::new_with_children(&name, WzValue::Null, properties);
            // extended_children.insert(name.clone(), Arc::new(node));
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

            let node = DynamicWzNode::new(&name, WzValue::Sound(sound));
            extended_children.insert(name.clone(), Arc::new(node));
        }
        "UOL" => {
            reader.skip(1)?;
            let value = reader.read_string_block(offset)?;
            let node = DynamicWzNode::new(&name, WzValue::Uol(value));
            extended_children.insert(name.clone(), Arc::new(node));
        }
        _ => {
            let error_type = ErrorKind::Unsupported;
            let error_message = format!(
                "Unsupported extended property type: {}",
                extended_property_type
            );
            Err(Error::new(error_type, error_message))?
        }
    }

    Ok(extended_children)
}

// Function to recursively print the node names and their children
pub fn print_node(node: &Arc<DynamicWzNode>, depth: usize) {
    let indent = "-".repeat(depth);
    println!("{}{}({})", indent, node.name, node.value);

    for child in node.children.values() {
        print_node(child, depth + 1)
    }
}

// Function to resolve a path to a child node
pub fn resolve(node: &Arc<DynamicWzNode>, path: &str) -> Result<Arc<DynamicWzNode>, Error> {
    let parts: Vec<&str> = path.split('/').collect();
    let mut current_node = Arc::clone(node);

    for part in parts.iter() {
        if let Some(child) = current_node.children.get(*part) {
            current_node = Arc::clone(child);
        } else {
            let error_type = ErrorKind::NotFound;
            let error_message = format!("Child node '{}' not found.", part);
            Err(Error::new(error_type, error_message))?
        }
    }

    Ok(current_node)
}
