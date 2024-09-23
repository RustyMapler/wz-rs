use crate::{
    WzCanvasProperty, WzConvexProperty, WzDoubleProperty, WzExtendedProperty, WzFloatProperty,
    WzIntProperty, WzLongProperty, WzNode, WzProperty, WzReader, WzShortProperty, WzSoundProperty,
    WzStringProperty, WzUolProperty, WzVectorProperty,
};

use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
    sync::Arc,
    vec,
};

pub struct WzObject {
    pub reader: Arc<WzReader>,
    pub offset: u32,
    pub name: String,
    pub properties: HashMap<String, Box<dyn WzNode>>,
    pub is_parsed: bool,
}

impl WzProperty for WzObject {}

impl WzNode for WzObject {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_child(&self, name: &str) -> Option<&dyn WzNode> {
        self.properties
            .get(name)
            .and_then(|child| Some(child.as_ref()))
    }

    fn get_child_mut(&mut self, name: &str) -> Option<&mut dyn WzNode> {
        self.parse();

        match self.properties.get_mut(name) {
            Some(child) => Some(child.as_mut()),
            None => None,
        }
    }

    fn list_children(&self) -> Vec<String> {
        self.properties.keys().cloned().collect()
    }

    fn get_children(&self) -> &HashMap<String, Box<dyn WzNode>> {
        &self.properties
    }

    fn parse(&mut self) {
        if !self.is_parsed {
            self.is_parsed = self.parse_object().is_ok();
        }
    }
}

impl WzObject {
    pub const HEADERBYTE_LUA: u8 = 0x1; // TODO: Support lua
    pub const HEADERBYTE_WITH_OFFSET: u8 = 0x1B;
    pub const HEADERBYTE_WITHOUT_OFFSET: u8 = 0x73;

    fn parse_object(&mut self) -> Result<(), Error> {
        // Seek to this object offset
        self.reader.seek(self.offset as u64)?;

        // This should be a .img
        let byte = self.reader.read_u8()?;
        match byte {
            WzObject::HEADERBYTE_WITHOUT_OFFSET => {
                let prop = self.reader.read_wz_string()?;
                let val = self.reader.read_u16()?;
                if prop != "Property" || val != 0 {
                    let msg = format!("Unsupported .img type: {} {}", prop, val);
                    return Err(Error::new(ErrorKind::Unsupported, msg));
                }
            }
            _ => {
                let msg = format!("Unsupported .img header: {}", byte);
                return Err(Error::new(ErrorKind::Unsupported, msg));
            }
        }

        let property_reader = self.reader.clone();
        if let Ok(properties) = parse_property(self, property_reader, self.offset) {
            self.properties = properties;
        }

        Ok(())
    }
}

fn parse_property(
    root_obj: *mut WzObject,
    reader: Arc<WzReader>,
    offset: u32,
) -> Result<HashMap<String, Box<dyn WzNode>>, Error> {
    let mut properties: HashMap<String, Box<dyn WzNode>> = HashMap::new();

    let num_entries = reader.read_wz_int()?;
    for _ in 0..num_entries {
        // We want to continue with the next entry if this fails (Doesn't panic)
        let name = reader.read_string_block(offset).unwrap_or_default();

        let property_type = reader.read_u8()?;
        match property_type {
            0 => {
                log::trace!("property type is null: {}", name);
            }
            2 | 11 => {
                let value = reader.read_i16()?;
                properties.insert(name.clone(), Box::new(WzShortProperty { name, value }));
            }
            3 | 19 => {
                let value = reader.read_wz_int()?;
                properties.insert(name.clone(), Box::new(WzIntProperty { name, value }));
            }
            20 => {
                let value = reader.read_wz_long()?;
                properties.insert(name.clone(), Box::new(WzLongProperty { name, value }));
            }
            4 => {
                let value = reader.read_u8().and_then(|sub| {
                    if sub == 0x80 {
                        reader.read_f32()
                    } else {
                        Ok(0.0)
                    }
                })?;
                properties.insert(name.clone(), Box::new(WzFloatProperty { name, value }));
            }
            5 => {
                let value = reader.read_f64()?;
                properties.insert(name.clone(), Box::new(WzDoubleProperty { name, value }));
            }
            8 => {
                let value = reader.read_string_block(offset)?;
                properties.insert(name.clone(), Box::new(WzStringProperty { name, value }));
            }
            9 => {
                let eob = reader.read_u32()? + reader.get_position()? as u32;

                let extended_property_reader = reader.clone();
                let extended = parse_extended_property(
                    root_obj,
                    extended_property_reader,
                    offset,
                    name.clone(),
                )?;
                properties.insert(name.clone(), extended);
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

fn parse_extended_property(
    root_obj: *mut WzObject,
    reader: Arc<WzReader>,
    offset: u32,
    name: String,
) -> Result<Box<dyn WzNode>, Error> {
    let property_type = reader.read_string_block(offset)?;
    match property_type.as_str() {
        "Property" => {
            reader.skip(2)?;
            let properties = parse_property(root_obj, reader, offset)?;

            Ok(Box::new(WzExtendedProperty { name, properties }))
        }
        "Canvas" => {
            reader.skip(1)?;

            let byte = reader.read_u8()?;

            let mut properties = HashMap::new();
            if byte == 1 {
                reader.skip(2)?;
                let property_reader = reader.clone();
                properties = parse_property(root_obj, property_reader, offset)?;
            }

            let width = reader.read_wz_int()? as u32;
            let height = reader.read_wz_int()? as u32;
            let format1 = reader.read_wz_int()? as u32;
            let format2 = reader.read_u8()?;

            reader.skip(4)?;
            let offset = reader.get_position()? as u32;

            let len = reader.read_i32()? - 1;

            reader.skip(1)?;

            if len > 0 {
                reader.skip(len as usize)?;
            }

            Ok(Box::new(WzCanvasProperty {
                root_obj,
                name,
                properties,
                width,
                height,
                format1,
                format2,
                offset,
                reader,
            }))
        }
        "Shape2D#Vector2D" => {
            let x = reader.read_wz_int()?;
            let y = reader.read_wz_int()?;
            let value = (x, y);

            Ok(Box::new(WzVectorProperty { name, value }))
        }
        "Shape2D#Convex2D" => {
            let mut convex = WzConvexProperty {
                name: name.clone(),
                properties: vec![],
            };

            let num_entries = reader.read_wz_int()?;
            for _ in 0..num_entries {
                let extended_property_reader = reader.clone();
                let extended = parse_extended_property(
                    root_obj,
                    extended_property_reader,
                    offset,
                    name.clone(),
                )?;
                convex.properties.push(extended)
            }

            Ok(Box::new(convex))
        }
        "Sound_DX8" => {
            let sound = WzSoundProperty::create(reader, offset, name)?;

            Ok(Box::new(sound))
        }
        "UOL" => {
            reader.skip(1)?;
            let value = reader.read_string_block(offset)?;

            Ok(Box::new(WzUolProperty { name, value }))
        }
        _ => {
            let msg = format!("Unsupported extended property type: {}", property_type);
            Err(Error::new(ErrorKind::Unsupported, msg))
        }
    }
}
