use std::{
    fmt,
    io::{Error, ErrorKind},
    sync::Arc,
};

use byteorder::{ByteOrder, LittleEndian};
use inflate::inflate_bytes_zlib;

use crate::{
    convert_image_bgra8888_to_rgba8888, decompress_image_bgr565_to_rgba8888,
    decompress_image_bgra4444_to_rgba8888, decompress_image_dxt5_to_rgba8888, Vec2, WzImage,
    WzReader,
};

#[derive(Default, Debug, Clone)]

pub struct WzCanvas {
    pub width: u32,
    pub height: u32,
    pub format1: u32,
    pub format2: u8,
    pub offset: u32,
    pub origin: Vec2,
}

impl fmt::Display for WzCanvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WzCanvas(width: {}, height: {}, format1: {}, format2: {}, offset: {})",
            self.width, self.height, self.format1, self.format2, self.offset
        )
    }
}

pub fn parse_canvas(canvas: &WzCanvas, reader: Arc<WzReader>) -> Result<WzImage, Error> {
    let raw_image_bytes = get_raw_image(canvas, reader)?;
    let canvas_format = canvas.format1 + canvas.format2 as u32;

    match canvas_format {
        // bgra4444
        1 => {
            let decompressed = decompress_image_bgra4444_to_rgba8888(
                &raw_image_bytes,
                canvas.width,
                canvas.height,
            );
            Ok(WzImage {
                width: canvas.width,
                height: canvas.height,
                data: decompressed,
                origin: canvas.origin.clone(),
            })
        }
        // bgra8888
        2 => {
            let converted = convert_image_bgra8888_to_rgba8888(raw_image_bytes);
            Ok(WzImage {
                width: canvas.width,
                height: canvas.height,
                data: converted,
                origin: canvas.origin.clone(),
            })
        }
        // bgr565
        517 => {
            let decompressed =
                decompress_image_bgr565_to_rgba8888(&raw_image_bytes, canvas.width, canvas.height);
            Ok(WzImage {
                width: canvas.width,
                height: canvas.height,
                data: decompressed,
                origin: canvas.origin.clone(),
            })
        }
        // dxt5
        1026 | 2050 => {
            let decompressed =
                decompress_image_dxt5_to_rgba8888(&raw_image_bytes, canvas.width, canvas.height);
            Ok(WzImage {
                width: canvas.width,
                height: canvas.height,
                data: decompressed,
                origin: canvas.origin.clone(),
            })
        }
        _ => Err(Error::new(
            ErrorKind::Unsupported,
            format!("Unsupported image format {}", canvas_format),
        ))?,
    }
}

fn get_raw_image(canvas: &WzCanvas, reader: Arc<WzReader>) -> Result<Vec<u8>, Error> {
    let compressed_bytes = get_compressed_bytes(canvas, reader)?;

    let header_buf = &compressed_bytes[0..2];
    let header = LittleEndian::read_u16(header_buf);
    // let header = reader.read_u16::<LittleEndian>().unwrap();

    let used_list_wz = header != 0x9C78 && header != 0xDA78 && header != 0x0178 && header != 0x5E78;

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

        data = vec![];
        Err(Error::new(
            ErrorKind::Unsupported,
            format!("Unsupported list wz image"),
        ))?
    }

    let format = canvas.format1 + canvas.format2 as u32;
    let uncompressed_size = match format {
        // inflate returns a vector with a size larger than the actual uncompressed image
        // so we need to calculate the uncompressed_size and splice the vector
        1 => (canvas.width * canvas.height * 2) as usize,
        2 => (canvas.width * canvas.height * 4) as usize,
        517 => (canvas.width * canvas.height / 128) as usize,
        1026 | 2050 => (canvas.width * canvas.height) as usize,
        _ => {
            panic!("unhandled image format {}", format);
        }
    };

    let buf = inflate_bytes_zlib(&data).unwrap();
    Ok(buf[..uncompressed_size].to_vec())
}

fn get_compressed_bytes(canvas: &WzCanvas, reader: Arc<WzReader>) -> Result<Vec<u8>, Error> {
    let current_position = reader.get_position()?;
    reader.seek(canvas.offset.into())?;
    let len = reader.read_u32()? - 1;

    reader.skip(1)?;

    let compressed_bytes = reader.read_bytes(len as u64)?;
    reader.seek(current_position)?;

    Ok(compressed_bytes)
}
