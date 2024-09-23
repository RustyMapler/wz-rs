use crate::{WzNode, WzObject, WzProperty, WzReader};
use std::{collections::HashMap, io::Error};

pub struct WzDirectory {
    pub reader: *mut WzReader,
    pub offset: u32,
    pub name: String,
    pub sub_directories: HashMap<String, WzDirectory>,
    pub objects: HashMap<String, WzObject>,
}

impl WzProperty for WzDirectory {}

impl WzNode for WzDirectory {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_child_mut(&mut self, name: &str) -> Option<&mut dyn WzNode> {
        if let Some(child) = self.objects.get_mut(name) {
            return Some(child as &mut dyn WzNode);
        }

        if let Some(child) = self.sub_directories.get_mut(name) {
            return Some(child as &mut dyn WzNode);
        }

        None
    }

    fn list_children(&self) -> Vec<String> {
        let mut subs = self
            .sub_directories
            .keys()
            .cloned()
            .collect::<Vec<String>>();

        let mut objs = self.objects.keys().cloned().collect::<Vec<String>>();

        subs.append(&mut objs);

        subs
    }
}

impl WzDirectory {
    pub fn parse_directory(&mut self, eager: bool) -> Result<(), Error> {
        unsafe {
            // Seek to the current directory position
            (*self.reader).seek(self.offset as u64)?;

            let num_entries = (*self.reader).read_wz_int()?;
            log::trace!("num entries {}", num_entries);

            for _ in 0..num_entries {
                let mut obj_type = (*self.reader).read_u8()?;
                log::trace!("obj_type {}", obj_type);

                let mut remember_pos = 0;
                let mut node_name = String::from("");

                match obj_type {
                    2 => {
                        let offset = (*self.reader).read_u32()?;
                        remember_pos = (*self.reader).get_position()?;

                        (*self.reader).seek(((*self.reader).file_start + offset) as u64)?;

                        obj_type = (*self.reader).read_u8()?;
                        node_name = (*self.reader).read_wz_string()?;
                    }
                    3 | 4 => {
                        node_name = (*self.reader).read_wz_string()?;
                        remember_pos = (*self.reader).get_position()?;
                    }
                    _ => {}
                }

                // Seek back to the original position
                (*self.reader).seek(remember_pos)?;
                log::trace!("name {}", node_name);

                // Fetch some additional info
                let fsize = (*self.reader).read_wz_int()?;
                let checksum = (*self.reader).read_wz_int()?;
                let offset = (*self.reader).read_wz_offset()?;
                log::trace!("fsize {}", fsize);
                log::trace!("checksum {}", checksum);
                log::trace!("offset {}", offset);

                // Build sub directories and objects
                match obj_type {
                    3 => {
                        let sub_dir = WzDirectory {
                            reader: self.reader,
                            offset,
                            name: node_name.clone(),
                            sub_directories: HashMap::new(),
                            objects: HashMap::new(),
                        };

                        self.sub_directories.insert(node_name.clone(), sub_dir);
                    }
                    _ => {
                        let obj = WzObject {
                            reader: self.reader,
                            offset,
                            name: node_name.clone(),
                            properties: HashMap::new(),
                            is_parsed: false,
                        };

                        self.objects.insert(node_name.clone(), obj);
                    }
                }
            }

            // Continue parsing sub directories
            if eager {
                for (_name, sub_dir) in self.sub_directories.iter_mut() {
                    sub_dir.parse_directory(eager)?;
                }
            }

            Ok(())
        }
    }
}
