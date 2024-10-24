use crate::{ArcDynamicWzNode, DynamicWzNode, Vec2, WzCanvas, WzReader, WzSound, WzValue};
use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
    sync::Arc,
};

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
                let extended_node = parse_extended_property(name.clone(), reader, offset)?;
                reader.seek(remember_pos as u64)?;
                extended_node
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
) -> Result<DynamicWzNode, Error> {
    log::trace!(
        "parse_extended_property | name({}) offset({})",
        name,
        offset
    );
    let extended_property_type = reader.read_string_block(offset)?;
    let node = match extended_property_type.as_str() {
        "Property" => {
            reader.skip(2)?;
            let properties = parse_property(reader, offset)?;
            DynamicWzNode::new_with_children(&name, WzValue::Extended, properties)
        }
        "Canvas" => {
            reader.skip(1)?;
            let mut properties = HashMap::new();

            let byte = reader.read_u8()?;
            if byte == 1 {
                reader.skip(2)?;
                properties = parse_property(reader, offset)?;
            }

            let width = reader.read_wz_int()? as u32;
            let height = reader.read_wz_int()? as u32;
            let format1 = reader.read_wz_int()? as u32;
            let format2 = reader.read_u8()?;

            reader.skip(4)?;

            let offset = reader.get_position()? as u32;
            let len = reader.read_i32()? - 1;

            reader.skip(1)?;

            // Skip reading this for now
            // TODO: Fix
            if len > 0 {
                reader.skip(len as usize)?;
            }

            DynamicWzNode::new_with_children(
                &name,
                WzValue::Canvas(WzCanvas {
                    width,
                    height,
                    format1,
                    format2,
                    offset,
                }),
                properties,
            )
        }
        "Shape2D#Vector2D" => {
            let x = reader.read_wz_int()?;
            let y = reader.read_wz_int()?;

            DynamicWzNode::new(&name, WzValue::Vector(Vec2 { x, y }))
        }
        "Shape2D#Convex2D" => {
            let mut properties = HashMap::new();

            let num_entries = reader.read_wz_int()?;
            for _ in 0..num_entries {
                let node = parse_extended_property(name.clone(), reader, offset)?;
                properties.insert(name.clone(), Arc::new(node));
            }

            DynamicWzNode::new_with_children(&name, WzValue::Convex, properties)
        }
        "Sound_DX8" => {
            reader.skip(1)?;

            let data_len = reader.read_wz_int()?;
            let len = reader.read_wz_int()?;

            //  Read the header
            let header_offset = reader.get_position()?;
            reader.skip(WzSound::SOUND_HEADER.len())?;
            let wav_len = reader.read_u8()?;
            reader.seek(header_offset)?;
            let header_len = WzSound::SOUND_HEADER.len() as u64 + 1 + wav_len as u64;
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

            DynamicWzNode::new(&name, WzValue::Sound(sound))
        }
        "UOL" => {
            reader.skip(1)?;
            let value = reader.read_string_block(offset)?;
            DynamicWzNode::new(&name, WzValue::Uol(value))
        }
        _ => Err(Error::new(
            ErrorKind::Unsupported,
            format!(
                "Unsupported extended property type: {}",
                extended_property_type
            ),
        ))?,
    };

    Ok(node)
}
