use crate::{wz_mutable_key::WzMutableKey, wz_object::WzObject};
use byteorder::{LittleEndian, ReadBytesExt};
use std::{
    io::{prelude::*, Cursor, Error, ErrorKind, SeekFrom},
    u64,
};

pub struct WzReader {
    pub file: Cursor<Vec<u8>>,
    pub file_start: u32,
    pub hash: u32,
    /// WZ key used to decrypt strings. In newer WZ versions, decryption is not used
    pub wz_key: Option<WzMutableKey>,
}

impl WzReader {
    pub fn seek(&mut self, pos: u64) -> Result<u64, Error> {
        self.file.seek(SeekFrom::Start(pos))
    }

    pub fn get_position(&mut self) -> Result<u64, Error> {
        self.file.stream_position()
    }

    pub fn skip(&mut self, len: usize) -> Result<u64, Error> {
        self.file.seek(SeekFrom::Current(len as i64))
    }

    pub fn read_u8(&mut self) -> Result<u8, Error> {
        self.file.read_u8()
    }

    pub fn read_u16(&mut self) -> Result<u16, Error> {
        self.file.read_u16::<LittleEndian>()
    }

    pub fn read_u32(&mut self) -> Result<u32, Error> {
        self.file.read_u32::<LittleEndian>()
    }

    pub fn read_u64(&mut self) -> Result<u64, Error> {
        self.file.read_u64::<LittleEndian>()
    }

    pub fn read_i8(&mut self) -> Result<i8, Error> {
        self.file.read_i8()
    }

    pub fn read_i16(&mut self) -> Result<i16, Error> {
        self.file.read_i16::<LittleEndian>()
    }

    pub fn read_i32(&mut self) -> Result<i32, Error> {
        self.file.read_i32::<LittleEndian>()
    }

    pub fn read_i64(&mut self) -> Result<i64, Error> {
        self.file.read_i64::<LittleEndian>()
    }

    pub fn read_f32(&mut self) -> Result<f32, Error> {
        self.file.read_f32::<LittleEndian>()
    }

    pub fn read_f64(&mut self) -> Result<f64, Error> {
        self.file.read_f64::<LittleEndian>()
    }

    pub fn read_bytes(&mut self, length: u64) -> Result<Vec<u8>, Error> {
        let mut buffer: Vec<u8> = vec![];

        for _ in 0..length {
            let val = self.read_u8()?;
            buffer.push(val);
        }

        Ok(buffer)
    }

    pub fn read_string(&mut self, length: u64) -> Result<String, Error> {
        let buffer = self.read_bytes(length)?;

        match String::from_utf8(buffer) {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::new(ErrorKind::NotFound, e)),
        }
    }

    pub fn read_string_to_end(&mut self) -> Result<String, Error> {
        let mut buffer: Vec<u8> = vec![];
        let mut val = self.read_u8()?;
        while val != 0 {
            buffer.push(val);
            val = self.read_u8()?;
        }

        match String::from_utf8(buffer) {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::new(ErrorKind::NotFound, e)),
        }
    }

    pub fn read_string_block(&mut self, offset: u32) -> Result<String, Error> {
        let string_type = self.read_u8()?;

        match string_type {
            0 | WzObject::HEADERBYTE_WITHOUT_OFFSET => self.read_wz_string(),
            1 | WzObject::HEADERBYTE_WITH_OFFSET => {
                let another_offset = self.read_u32()?;
                self.read_wz_string_at_offset(offset + another_offset)
            }
            _ => Err(Error::new(ErrorKind::NotFound, "Unknown type")),
        }
    }

    pub fn read_wz_int(&mut self) -> Result<i32, Error> {
        let possible_size = self.read_i8()?;

        if possible_size == -128 {
            let wz_int = self.read_i32()?;
            Ok(wz_int)
        } else {
            Ok(possible_size as i32)
        }
    }

    pub fn read_wz_long(&mut self) -> Result<i64, Error> {
        let possible_size = self.read_i8()?;

        if possible_size == -128 {
            let wz_long = self.read_i64()?;
            Ok(wz_long)
        } else {
            Ok(possible_size as i64)
        }
    }

    pub fn read_wz_string_at_offset(&mut self, offset: u32) -> Result<String, Error> {
        let position = self.get_position()?;
        self.seek(offset.into())?;
        let result = self.read_wz_string();
        self.seek(position)?;

        result
    }

    pub fn read_wz_string(&mut self) -> Result<String, Error> {
        let mut size: i32 = self.read_i8()?.into();

        if size == 0 {
            return Ok(String::new());
        }

        if size > 0 {
            if size == 127 {
                size = self.read_i32()?;
            }

            return self.read_wz_string_as_unicode(size as u32);
        }

        if size == -128 {
            size = self.read_i32()?;
        } else {
            size *= -1;
        }

        return self.read_wz_string_as_ascii(size as u32);
    }

    pub fn read_wz_offset(&mut self) -> Result<u32, Error> {
        let mut offset = self.get_position()?;
        offset = (offset - self.file_start as u64) ^ 0xFFFFFFFF;
        offset = offset * self.hash as u64;
        offset -= 0x581C3F6D;
        offset = rotate_left(offset as u32, (offset & 0x1F) as u8) as u64;

        let encrypted_offset = self.read_u32()?;
        offset ^= encrypted_offset as u64;
        offset += (self.file_start * 2) as u64;

        Ok(offset as u32)
    }

    fn read_wz_string_as_unicode(&mut self, size: u32) -> Result<String, Error> {
        let mut mask: u16 = 0xAAAA;
        let mut res_string: Vec<u16> = vec![];

        // while i < (size as usize) {
        //     let mut character = (characters[i] | characters[i + 1] << 8) as u16;
        //     character ^= mask;
        //     characters[i] = character as u8;
        //     characters[i + 1] = (character >> 8) as u8;

        //     mask += 1;
        //     i += 2;
        // }

        for i in 0..(size as usize) {
            let mut encrypted_char = self.read_u16()?;
            encrypted_char ^= mask;
            match &mut self.wz_key {
                // Newer versions do not use encryption
                Some(key) => {
                    encrypted_char ^= ((key.at(i * 2 + 1) as u16) << 8) + (key.at(i * 2) as u16)
                }
                None => {}
            };
            res_string.push(encrypted_char);
            mask += 1;
        }

        match String::from_utf16(&res_string) {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::new(ErrorKind::Other, e)),
        }
    }

    fn read_wz_string_as_ascii(&mut self, size: u32) -> Result<String, Error> {
        let mut mask: u8 = 0xAA;
        let mut res_string: Vec<u8> = vec![];
        for i in 0..(size as usize) {
            let mut encrypted_char = self.read_u8()? as u8;
            encrypted_char ^= mask;
            match &mut self.wz_key {
                // Newer versions do not use encryption
                Some(key) => encrypted_char ^= key.at(i) as u8,
                None => {}
            }
            res_string.push(encrypted_char as u8);
            mask += 1;
        }

        match String::from_utf8(res_string) {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::new(ErrorKind::Other, e)),
        }
    }
}

fn rotate_left(x: u32, n: u8) -> u32 {
    ((x) << (n)) | ((x) >> (32 - (n)))
}
