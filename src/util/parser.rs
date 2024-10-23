use crate::{ArcWzNode, Vec2, WzCanvas, WzNode, WzReader, WzSound, WzValue, WzValueCast};
use indexmap::IndexMap;
use std::{
    io::{Error, ErrorKind},
    sync::Arc,
};

/// Parse the header for a .wz file. Get the file start for the reader.
pub fn parse_wz_header(reader: &WzReader) -> Result<u32, Error> {
    let ident = reader.read_string(4)?;

    if ident != "PKG1" {
        return Err(Error::new(ErrorKind::Other, "Invalid .wz file"));
    }

    let _size = reader.read_u64()?;
    let start = reader.read_u32()?;
    let _copyright = reader.read_string_to_end()?;

    Ok(start)
}

pub fn parse_directory(
    reader: &Arc<WzReader>,
    offset: usize,
    name: String,
    level: usize,
) -> Result<ArcWzNode, Error> {
    let mut children = IndexMap::new();

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
                if level > 0 {
                    let remember_pos = reader.get_position()?;
                    let node = parse_directory(
                        reader,
                        entry_offset as usize,
                        entry_name.clone(),
                        level - 1,
                    )?;
                    reader.seek(remember_pos)?;
                    children.insert(entry_name.clone(), node);
                } else {
                    let node = Arc::new(WzNode::new(
                        &entry_name,
                        entry_offset as usize,
                        WzValue::Directory,
                    ));
                    children.insert(entry_name.clone(), node);
                }
            }
            _ => {
                if level > 0 {
                    let remember_pos = reader.get_position()?;
                    let node = parse_img(reader, entry_offset as usize, entry_name.clone())?;
                    reader.seek(remember_pos)?;
                    children.insert(entry_name.clone(), node);
                } else {
                    let node = Arc::new(WzNode::new(
                        &entry_name,
                        entry_offset as usize,
                        WzValue::Directory,
                    ));
                    children.insert(entry_name.clone(), node);
                }
            }
        }
    }

    let node = WzNode::new_with_children(&name, offset, WzValue::Directory, children);
    Ok(Arc::new(node))
}

pub fn parse_img(reader: &Arc<WzReader>, offset: usize, name: String) -> Result<ArcWzNode, Error> {
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
) -> Result<IndexMap<String, ArcWzNode>, Error> {
    let mut children = IndexMap::new();

    let num_entries = reader.read_wz_int()?;
    for _ in 0..num_entries {
        let name = reader.read_string_block(offset as u32)?;
        let node = parse_property(reader, offset, name.clone())?;
        children.insert(name, Arc::new(node));
    }

    Ok(children)
}

pub fn parse_property(
    reader: &Arc<WzReader>,
    offset: usize,
    name: String,
) -> Result<WzNode, Error> {
    let property_offset = reader.get_position()? as usize;
    let property_type = reader.read_u8()?;
    let property_node = match property_type {
        0 => WzNode::new(&name, property_offset, WzValue::Null),
        2 | 11 => {
            let value = reader.read_i16()?;
            WzNode::new(&name, property_offset, WzValue::Short(value))
        }
        3 | 19 => {
            let value = reader.read_wz_int()?;
            WzNode::new(&name, property_offset, WzValue::Int(value))
        }
        20 => {
            let value = reader.read_wz_long()?;
            WzNode::new(&name, property_offset, WzValue::Long(value))
        }
        4 => {
            let value = match reader.read_u8()? {
                0x80 => reader.read_f32()?,
                _ => 0.0,
            };
            WzNode::new(&name, property_offset, WzValue::Float(value))
        }
        5 => {
            let value = reader.read_f64()?;
            WzNode::new(&name, property_offset, WzValue::Double(value))
        }
        8 => {
            let value = reader.read_string_block(offset as u32)?;
            WzNode::new(&name, property_offset, WzValue::String(value))
        }
        9 => {
            let remember_pos = reader.read_u32()? + reader.get_position()? as u32;
            let extended_property_node = parse_extended_property(reader, offset, name.clone())?;
            reader.seek(remember_pos as u64)?;
            extended_property_node
        }
        _ => Err(Error::new(
            ErrorKind::Unsupported,
            format!(
                "Unsupported property: {} {} {}",
                name, property_offset, property_type
            ),
        ))?,
    };

    Ok(property_node)
}

pub fn parse_extended_property(
    reader: &Arc<WzReader>,
    offset: usize,
    name: String,
) -> Result<WzNode, Error> {
    let extended_property_offset = reader.get_position()? as usize;
    let extended_property_type = reader.read_string_block(offset as u32)?;
    let extended_property_node = match extended_property_type.as_str() {
        "Property" => {
            reader.skip(2)?;

            let properties = parse_property_list(reader, offset)?;

            WzNode::new_with_children(
                &name,
                extended_property_offset,
                WzValue::Extended,
                properties,
            )
        }
        "Canvas" => {
            reader.skip(1)?;

            let mut properties = IndexMap::new();

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
                extended_property_offset,
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

            WzNode::new(
                &name,
                extended_property_offset,
                WzValue::Vector(Vec2 { x, y }),
            )
        }
        "Shape2D#Convex2D" => {
            let mut properties = IndexMap::new();

            let num_entries = reader.read_wz_int()?;
            for index in 0..num_entries {
                let entry_name = index.to_string();
                let entry_node = parse_extended_property(reader, offset, entry_name.clone())?;
                properties.insert(entry_name.clone(), Arc::new(entry_node));
            }

            WzNode::new_with_children(&name, extended_property_offset, WzValue::Convex, properties)
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
            reader.skip(header_size as usize)?;

            // Extract the sound data
            let buffer_offset = reader.get_position()?;
            reader.skip(buffer_size as usize)?;

            let value = WzSound {
                name: name.clone(),
                duration: duration as u32,
                header_offset,
                header_size: header_size as usize,
                buffer_offset,
                buffer_size: buffer_size as usize,
            };

            WzNode::new(&name, extended_property_offset, WzValue::Sound(value))
        }
        "UOL" => {
            reader.skip(1)?;
            let value = reader.read_string_block(offset as u32)?;
            WzNode::new(&name, extended_property_offset, WzValue::Uol(value))
        }
        _ => Err(Error::new(
            ErrorKind::Unsupported,
            format!(
                "Unsupported extended property: {} {} {}",
                name, extended_property_offset, extended_property_type
            ),
        ))?,
    };

    Ok(extended_property_node)
}
