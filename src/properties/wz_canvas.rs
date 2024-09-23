use crate::{Vec2, WzImage, WzNode, WzObject, WzProperty, WzReader};
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
                let decompressed = WzCanvasProperty::decompress_image_bgra4444_to_rgba8888(
                    &img_bytes,
                    self.width,
                    self.height,
                );
                Some(WzImage {
                    width: self.width,
                    height: self.height,
                    data: decompressed,
                    origin,
                })
            }
            // bgra8888
            2 => {
                let converted = WzCanvasProperty::convert_image_bgra8888_to_rgba8888(img_bytes);
                Some(WzImage {
                    width: self.width,
                    height: self.height,
                    data: converted,
                    origin,
                })
            }
            517 => {
                let decompressed = WzCanvasProperty::decompress_image_bgr565_to_rgba8888(
                    &img_bytes,
                    self.width,
                    self.height,
                );
                Some(WzImage {
                    width: self.width,
                    height: self.height,
                    data: decompressed,
                    origin,
                })
            }
            // dxt5
            1026 | 2050 => {
                let decompressed = WzCanvasProperty::decompress_image_dxt5_to_rgba8888(
                    &img_bytes,
                    self.width,
                    self.height,
                );
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

    fn decompress_image_bgra4444_to_rgba8888(data: &[u8], width: u32, height: u32) -> Vec<u8> {
        fn extract_lower_bits(bits: u8) -> u8 {
            let byte = bits & 0x0F;
            byte | byte << 4
        }
        fn extract_upper_bits(bits: u8) -> u8 {
            let byte = bits & 0xF0;
            byte | byte >> 4
        }

        let uncompressed_size = width * height * 2;
        let mut decoded_bytes: Vec<u8> = Vec::with_capacity((uncompressed_size * 2) as usize);
        unsafe {
            decoded_bytes.set_len((uncompressed_size * 2) as usize);
        }

        for i in (0..uncompressed_size as usize).step_by(2) {
            let b = extract_lower_bits(data[i]);
            let g = extract_upper_bits(data[i]);
            let r = extract_lower_bits(data[i + 1]);
            let a = extract_upper_bits(data[i + 1]);

            decoded_bytes[i * 2] = r;
            decoded_bytes[i * 2 + 1] = g;
            decoded_bytes[(i + 1) * 2] = b;
            decoded_bytes[(i + 1) * 2 + 1] = a;
        }

        decoded_bytes
    }

    fn convert_image_bgra8888_to_rgba8888(data: Vec<u8>) -> Vec<u8> {
        let mut bytes = data;
        for i in (0..bytes.len()).step_by(4) {
            bytes.swap(i, i + 2);
        }
        bytes
    }

    fn decompress_image_dxt5_to_rgba8888(data: &[u8], width: u32, height: u32) -> Vec<u8> {
        let mut decompressed = vec![0u8; (4 * width * height).try_into().unwrap()];

        Format::Bc3.decompress(data, width as usize, height as usize, &mut decompressed);

        decompressed
    }

    // not 100% sure if this is correct
    fn decompress_image_bgr565_to_rgba8888(data: &[u8], width: u32, height: u32) -> Vec<u8> {
        let uncompressed_size = width * height * 2;
        let mut decoded_bytes: Vec<u8> = Vec::with_capacity((uncompressed_size * 2) as usize);
        unsafe {
            decoded_bytes.set_len((uncompressed_size * 2) as usize);
        }

        let mut line_index = 0;
        let mut j0 = 0;
        let j1 = height / 16;
        while j0 < j1 {
            let mut dst_index = line_index;
            let mut i0 = 0;
            let i1 = width / 16;
            while i0 < i1 {
                let index = (i0 + j0 * i1) * 2;
                let b0 = data[index as usize];
                let b1 = data[(index + 1) as usize];
                for _ in 0..16 {
                    decoded_bytes[dst_index] = b0;
                    dst_index += 1;
                    decoded_bytes[dst_index] = b1;
                    dst_index += 1;
                }

                i0 += 1;
            }

            for _ in 1..16 {
                dst_index += (width * 2) as usize;
            }

            line_index += (width * 32) as usize;

            j0 += 1;
        }

        decoded_bytes
    }
}
