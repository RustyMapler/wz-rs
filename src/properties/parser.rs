use crate::{ArcWzNode, Vec2, WzCanvas, WzNode, WzReader, WzSound, WzValue, WzValueCast};
use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
    sync::Arc,
};

pub fn parse_directory(
    name: String,
    reader: &Arc<WzReader>,
    offset: usize,
) -> Result<ArcWzNode, Error> {
    let mut children = HashMap::new();

    reader.seek(offset as u64)?;

    let count = reader.read_wz_int()?;

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
        let _entry_fsize = reader.read_wz_int()?;
        let _entry_checksum = reader.read_wz_int()?;
        let entry_offset = reader.read_wz_offset()?;

        // Build directories and .imgs
        match entry_type {
            3 => {
                let remember_pos = reader.get_position()?;
                let node = parse_directory(entry_name.clone(), reader, entry_offset as usize)?;
                reader.seek(remember_pos)?;
                children.insert(entry_name.clone(), node);
            }
            _ => {
                let remember_pos = reader.get_position()?;
                let node = parse_img(entry_name.clone(), reader, entry_offset as usize)?;
                reader.seek(remember_pos)?;
                children.insert(entry_name.clone(), node);
            }
        }
    }

    let node = WzNode::new_with_children(&name, offset, WzValue::Directory, children);
    Ok(Arc::new(node))
}

pub fn parse_img(name: String, reader: &Arc<WzReader>, offset: usize) -> Result<ArcWzNode, Error> {
    reader.seek(offset as u64)?;

    // Read the first byte and check that this node is a .img
    let byte = reader.read_u8()?;
    match byte {
        WzReader::HEADERBYTE_WITHOUT_OFFSET => {
            let prop = reader.read_wz_string()?;
            let val = reader.read_u16()?;
            if prop != "Property" || val != 0 {
                Err(Error::new(
                    ErrorKind::Unsupported,
                    format!("Unsupported .img type: {} {}", prop, val),
                ))?
            }
        }
        _ => Err(Error::new(
            ErrorKind::Unsupported,
            format!("Unsupported .img header: {}", byte),
        ))?,
    }

    // Continue parsing all properties for this node
    if let Ok(children) = parse_property_list(reader, offset) {
        Ok(Arc::new(WzNode::new_with_children(
            &name,
            offset,
            WzValue::Img,
            children,
        )))
    } else {
        Ok(Arc::new(WzNode::new(&name, offset, WzValue::Img)))
    }
}

pub fn parse_property_list(
    reader: &Arc<WzReader>,
    offset: usize,
) -> Result<HashMap<String, ArcWzNode>, Error> {
    let mut children = HashMap::new();

    let num_entries = reader.read_wz_int()?;
    for _ in 0..num_entries {
        let name = reader.read_string_block(offset as u32)?;
        let node = parse_property(name.clone(), reader, offset)?;
        children.insert(name, Arc::new(node));
    }

    Ok(children)
}

pub fn parse_property(
    name: String,
    reader: &Arc<WzReader>,
    offset: usize,
) -> Result<WzNode, Error> {
    let property_type = reader.read_u8()?;
    let property_node = match property_type {
        0 => WzNode::new(&name, offset, WzValue::Null),
        2 | 11 => {
            let value = reader.read_i16()?;
            WzNode::new(&name, offset, WzValue::Short(value))
        }
        3 | 19 => {
            let value = reader.read_wz_int()?;
            WzNode::new(&name, offset, WzValue::Int(value))
        }
        20 => {
            let value = reader.read_wz_long()?;
            WzNode::new(&name, offset, WzValue::Long(value))
        }
        4 => {
            let value = match reader.read_u8()? {
                0x80 => reader.read_f32()?,
                _ => 0.0,
            };
            WzNode::new(&name, offset, WzValue::Float(value))
        }
        5 => {
            let value = reader.read_f64()?;
            WzNode::new(&name, offset, WzValue::Double(value))
        }
        8 => {
            let value = reader.read_string_block(offset as u32)?;
            WzNode::new(&name, offset, WzValue::String(value))
        }
        9 => {
            let remember_pos = reader.read_u32()? + reader.get_position()? as u32;
            let extended_property_node = parse_extended_property(name.clone(), reader, offset)?;
            reader.seek(remember_pos as u64)?;
            extended_property_node
        }
        _ => Err(Error::new(
            ErrorKind::Unsupported,
            format!(
                "Unsupported property: {} {} {}",
                name, offset, property_type
            ),
        ))?,
    };

    Ok(property_node)
}

pub fn parse_extended_property(
    name: String,
    reader: &Arc<WzReader>,
    offset: usize,
) -> Result<WzNode, Error> {
    let extended_property_type = reader.read_string_block(offset as u32)?;
    let extended_property_node = match extended_property_type.as_str() {
        "Property" => {
            reader.skip(2)?;

            let properties = parse_property_list(reader, offset)?;

            WzNode::new_with_children(&name, offset, WzValue::Extended, properties)
        }
        "Canvas" => {
            reader.skip(1)?;

            let mut properties = HashMap::new();

            let has_children = reader.read_u8()? == 1;
            if has_children {
                reader.skip(2)?;
                properties = parse_property_list(reader, offset)?;
            }

            let width = reader.read_wz_int()? as u32;
            let height = reader.read_wz_int()? as u32;
            let format1 = reader.read_wz_int()? as u32;
            let format2 = reader.read_u8()?;

            reader.skip(4)?;

            let offset = reader.get_position()? as u32;
            let len = reader.read_i32()? - 1;

            reader.skip(1)?;

            // Skip reading this for now.
            if len > 0 {
                reader.skip(len as usize)?;
            }

            // Get the origin now
            let mut origin = Vec2::default();
            if let Some(node) = properties.get("origin") {
                if let Some(vector) = node.value.as_vector() {
                    origin = vector.clone();
                }
            }

            WzNode::new_with_children(
                &name,
                offset as usize,
                WzValue::Canvas(WzCanvas {
                    width,
                    height,
                    format1,
                    format2,
                    offset,
                    origin,
                }),
                properties,
            )
        }
        "Shape2D#Vector2D" => {
            let x = reader.read_wz_int()?;
            let y = reader.read_wz_int()?;

            WzNode::new(&name, offset, WzValue::Vector(Vec2 { x, y }))
        }
        "Shape2D#Convex2D" => {
            let mut properties = HashMap::new();

            let num_entries = reader.read_wz_int()?;
            for index in 0..num_entries {
                let entry_name = index.to_string();
                let entry_node = parse_extended_property(entry_name.clone(), reader, offset)?;
                properties.insert(entry_name.clone(), Arc::new(entry_node));
            }

            WzNode::new_with_children(&name, offset, WzValue::Convex, properties)
        }
        "Sound_DX8" => {
            reader.skip(1)?;

            // Sound metadata
            let buffer_size = reader.read_wz_int()?;
            let duration = reader.read_wz_int()?;

            // Sound header, extract wav len
            let header_offset = reader.get_position()?;
            reader.skip(WzSound::SOUND_HEADER.len())?;
            let wav_len = reader.read_u8()?;
            reader.seek(header_offset)?;

            // Determine the header len and extract the header data
            let header_size = WzSound::SOUND_HEADER.len() as u64 + 1 + wav_len as u64;
            let header_data = reader.read_bytes(header_size)?;

            // Extract the sound data
            let buffer_offset = reader.get_position()?;
            let buffer = reader.read_bytes(buffer_size as u64)?;

            let value = WzSound {
                name: name.clone(),
                duration: duration as u32,
                header_offset,
                header_data,
                header_size: header_size as usize,
                buffer_offset,
                buffer,
                buffer_size: buffer_size as usize,
            };

            WzNode::new(&name, offset, WzValue::Sound(value))
        }
        "UOL" => {
            reader.skip(1)?;
            let value = reader.read_string_block(offset as u32)?;
            WzNode::new(&name, offset, WzValue::Uol(value))
        }
        _ => Err(Error::new(
            ErrorKind::Unsupported,
            format!(
                "Unsupported extended property: {} {} {}",
                name, offset, extended_property_type
            ),
        ))?,
    };

    Ok(extended_property_node)
}
