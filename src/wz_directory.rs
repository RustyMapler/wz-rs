use crate::{WzNode, WzObject, WzProperty, WzReader};
use std::{collections::HashMap, io::Error, sync::Arc};

pub struct WzDirectory {
    pub reader: Arc<WzReader>,
    pub offset: u32,
    pub name: String,
    pub directories: HashMap<String, WzDirectory>,
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

        if let Some(child) = self.directories.get_mut(name) {
            return Some(child as &mut dyn WzNode);
        }

        None
    }

    fn list_children(&self) -> Vec<String> {
        let mut subs = self.directories.keys().cloned().collect::<Vec<String>>();

        let mut objs = self.objects.keys().cloned().collect::<Vec<String>>();

        subs.append(&mut objs);

        subs
    }
}

impl WzDirectory {
    pub fn parse_directory(&mut self, eager: bool) -> Result<(), Error> {
        // Seek to the current directory position
        self.reader.seek(self.offset as u64)?;

        let entry_count = self.reader.read_wz_int()?;
        log::trace!("entry_count {}", entry_count);

        for _ in 0..entry_count {
            let mut remember_pos = 0;
            let mut entry_name = String::from("");
            let mut entry_type = self.reader.read_u8()?;

            match entry_type {
                2 => {
                    let offset = self.reader.read_u32()?;
                    remember_pos = self.reader.get_position()?;

                    self.reader.seek((self.reader.file_start + offset) as u64)?;

                    entry_type = self.reader.read_u8()?;
                    entry_name = self.reader.read_wz_string()?;
                }
                3 | 4 => {
                    entry_name = self.reader.read_wz_string()?;
                    remember_pos = self.reader.get_position()?;
                }
                _ => {}
            }

            // Seek back to the original position
            self.reader.seek(remember_pos)?;

            // Fetch some additional info
            let fsize = self.reader.read_wz_int()?;
            let checksum = self.reader.read_wz_int()?;
            let offset = self.reader.read_wz_offset()?;

            log::trace!("entry: {} {}", entry_type, entry_name);
            log::trace!("fsize {}, checksum {}, offset {}", fsize, checksum, offset);

            // Build sub directories and objects
            match entry_type {
                3 => {
                    let sub_dir = WzDirectory {
                        reader: self.reader.clone(),
                        offset,
                        name: entry_name.clone(),
                        directories: HashMap::new(),
                        objects: HashMap::new(),
                    };

                    self.directories.insert(entry_name.clone(), sub_dir);
                }
                _ => {
                    let obj = WzObject {
                        reader: self.reader.clone(),
                        offset,
                        name: entry_name.clone(),
                        properties: HashMap::new(),
                        is_parsed: false,
                    };

                    self.objects.insert(entry_name.clone(), obj);
                }
            }
        }

        // Continue parsing sub directories
        if eager {
            for (_name, sub_dir) in self.directories.iter_mut() {
                sub_dir.parse_directory(eager)?;
            }
        }

        Ok(())
    }
}
