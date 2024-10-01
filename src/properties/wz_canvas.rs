use crate::{
    convert_image_bgra8888_to_rgba8888, decompress_image_bgr565_to_rgba8888,
    decompress_image_bgra4444_to_rgba8888, decompress_image_dxt5_to_rgba8888, Vec2, WzImage,
    WzNode, WzObject, WzProperty, WzReader,
};
use byteorder::{ByteOrder, LittleEndian};
use core::panic;
use inflate::inflate_bytes_zlib;
use squish::Format;
use std::{collections::HashMap, sync::Arc, vec};

pub struct WzCanvasProperty {
    pub root_obj: *mut WzObject,
    pub name: String,
    pub properties: HashMap<String, Box<dyn WzNode>>,
    pub width: u32,
    pub height: u32,
    pub format1: u32,
    pub format2: u8,
    pub reader: Arc<WzReader>,
    pub offset: u32,
}

impl WzProperty for WzCanvasProperty {
    fn get_image(&self) -> Option<WzImage> {
        // if this canvas property is an inlink, we'll want to navigate and
        // return the actual canvas property
        if self.is_inlink() {
            let inlink_path = self.get_inlink().unwrap();
            unsafe {
                let img_node = (*self.root_obj).resolve(&inlink_path).unwrap();
                let inlink_img = img_node.get_image();
                return match inlink_img {
                    Some(mut inlink_img) => {
                        // Get the original image's intended origin
                        // and swap it with the inlink
                        let origin = self.get_origin();
                        inlink_img.origin = origin;
                        Some(inlink_img)
                    }
                    None => None,
                };
            }
        }

        self.parse_image()
    }

    /// Checks if this canvas property contains an inlink.
    /// Inlinks reference another property with a relative path. The inlink
    /// is relative from the parent object.
    /// ex: {Path}
    ///     portal/editor/pv
    /// The parent object is MapHelper.img
    fn is_inlink(&self) -> bool {
        self.properties.contains_key("_inlink")
    }

    fn get_inlink(&self) -> Option<String> {
        match self.properties.get("_inlink") {
            Some(inlink) => Some(inlink.get_string().unwrap().clone()),
            None => None,
        }
    }

    /// Checks if this canvas property contains an outlink.
    /// Outlinks reference another property with a full absolute path. The path
    /// contains the name of the WzFile and the path.
    /// ex: {WzFile}/{Path}
    ///     Map/Tile/logMarble2.img/bsc/0
    fn is_outlink(&self) -> bool {
        self.properties.contains_key("_outlink")
    }

    fn get_outlink(&self) -> Option<String> {
        match self.properties.get("_outlink") {
            Some(outlink) => match outlink.get_string() {
                Some(string) => Some(string.split_once('/').unwrap().1.to_string()),
                _ => None,
            },
            None => None,
        }
    }
}

impl WzNode for WzCanvasProperty {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_child(&self, name: &str) -> Option<&dyn WzNode> {
        self.properties
            .get(name)
            .and_then(|child| Some(child.as_ref()))
    }

    fn list_children(&self) -> Vec<String> {
        self.properties.keys().cloned().collect()
    }
}

impl WzCanvasProperty {
    fn get_origin(&self) -> Vec2 {
        match self.properties.get("origin") {
            Some(node) => node.get_vec2().unwrap(),
            None => Vec2::default(),
        }
    }

    fn parse_image(&self) -> Option<WzImage> {
        let img_bytes = self.get_raw_image().unwrap();
        let format = self.format1 + self.format2 as u32;

        let origin = self.get_origin();

        match format {
            // bgra4444
            1 => {
                let decompressed =
                    decompress_image_bgra4444_to_rgba8888(&img_bytes, self.width, self.height);
                Some(WzImage {
                    width: self.width,
                    height: self.height,
                    data: decompressed,
                    origin,
                })
            }
            // bgra8888
            2 => {
                let converted = convert_image_bgra8888_to_rgba8888(img_bytes);
                Some(WzImage {
                    width: self.width,
                    height: self.height,
                    data: converted,
                    origin,
                })
            }
            517 => {
                let decompressed =
                    decompress_image_bgr565_to_rgba8888(&img_bytes, self.width, self.height);
                Some(WzImage {
                    width: self.width,
                    height: self.height,
                    data: decompressed,
                    origin,
                })
            }
            // dxt5
            1026 | 2050 => {
                let decompressed =
                    decompress_image_dxt5_to_rgba8888(&img_bytes, self.width, self.height);
                Some(WzImage {
                    width: self.width,
                    height: self.height,
                    data: decompressed,
                    origin,
                })
            }
            _ => {
                panic!("unsupported image format {}", format)
            }
        }
    }

    fn get_raw_image(&self) -> Result<Vec<u8>, String> {
        let compressed_bytes = self.get_compressed_bytes();

        let header_buf = &compressed_bytes[0..2];
        let header = LittleEndian::read_u16(header_buf);
        // let header = reader.read_u16::<LittleEndian>().unwrap();

        let used_list_wz =
            header != 0x9C78 && header != 0xDA78 && header != 0x0178 && header != 0x5E78;

        let data: Vec<u8>;
        if !used_list_wz {
            data = compressed_bytes;
        } else {
            // let mut reader = Cursor::new(&compressed_bytes);

            // let mut blocksize = 0;
            // while reader.position() < compressed_bytes.len() as u64 {
            //     blocksize = reader.read_i32::<LittleEndian>().unwrap();
            //     for i in 0..blocksize {}
            // }
            // data = vec![];
            panic!("using list wz image");
        }

        let format = self.format1 + self.format2 as u32;
        let uncompressed_size = match format {
            // inflate returns a vector with a size larger than the actual uncompressed image
            // so we need to calculate the uncompressed_size and splice the vector
            1 => (self.width * self.height * 2) as usize,
            2 => (self.width * self.height * 4) as usize,
            517 => (self.width * self.height / 128) as usize,
            1026 | 2050 => (self.width * self.height) as usize,
            _ => {
                panic!("unhandled image format {}", format);
            }
        };

        let buf = inflate_bytes_zlib(&data).unwrap();
        Ok(buf[..uncompressed_size].to_vec())
    }

    fn get_compressed_bytes(&self) -> Vec<u8> {
        let current_position = self.reader.get_position().unwrap();
        self.reader.seek(self.offset.into()).unwrap();
        let len = self.reader.read_u32().unwrap() - 1;

        self.reader.skip(1).unwrap();

        let compressed_bytes = self.reader.read_bytes(len as u64).unwrap();
        self.reader.seek(current_position).unwrap();

        compressed_bytes
    }
}
