use crate::mutable_key::WzMutableKey;
use byteorder::{LittleEndian, ReadBytesExt};
use std::{
    cell::RefCell,
    io::{prelude::*, Cursor, Error, ErrorKind, SeekFrom},
    u64,
};

#[derive(Clone)]
pub struct WzReader {
    pub file: RefCell<Cursor<Vec<u8>>>,
    /// WZ key used to decrypt strings. In newer WZ versions, decryption is not used
    pub wz_key: Option<WzMutableKey>,
    pub file_start: RefCell<u32>,
    pub version_hash: RefCell<u32>,
}

impl Default for WzReader {
    fn default() -> Self {
        WzReader::new(Cursor::new(Vec::new()), None)
    }
}

impl WzReader {
    pub const HEADERBYTE_LUA: u8 = 0x1;
    pub const HEADERBYTE_WITH_OFFSET: u8 = 0x1B;
    pub const HEADERBYTE_WITHOUT_OFFSET: u8 = 0x73;

    pub fn new(buffer: Cursor<Vec<u8>>, wz_key: Option<WzMutableKey>) -> WzReader {
        WzReader {
            file: RefCell::new(buffer),
            wz_key: wz_key,
            file_start: 0.into(),
            version_hash: 0.into(),
        }
    }

    pub fn set_wz_key(&mut self, wz_key: Option<WzMutableKey>) {
        self.wz_key = wz_key;
    }

    pub fn set_file_start(&self, file_start: u32) {
        *self.file_start.borrow_mut() = file_start;
    }

    pub fn set_version_hash(&self, version_hash: u32) {
        *self.version_hash.borrow_mut() = version_hash;
    }

    pub fn seek(&self, pos: u64) -> Result<u64, Error> {
        self.file.borrow_mut().seek(SeekFrom::Start(pos))
    }

    pub fn get_position(&self) -> Result<u64, Error> {
        self.file.borrow_mut().stream_position()
    }

    pub fn skip(&self, len: usize) -> Result<u64, Error> {
        self.file.borrow_mut().seek(SeekFrom::Current(len as i64))
    }

    pub fn read_u8(&self) -> Result<u8, Error> {
        self.file.borrow_mut().read_u8()
    }

    pub fn read_u16(&self) -> Result<u16, Error> {
        self.file.borrow_mut().read_u16::<LittleEndian>()
    }

    pub fn read_u32(&self) -> Result<u32, Error> {
        self.file.borrow_mut().read_u32::<LittleEndian>()
    }

    pub fn read_u64(&self) -> Result<u64, Error> {
        self.file.borrow_mut().read_u64::<LittleEndian>()
    }

    pub fn read_i8(&self) -> Result<i8, Error> {
        self.file.borrow_mut().read_i8()
    }

    pub fn read_i16(&self) -> Result<i16, Error> {
        self.file.borrow_mut().read_i16::<LittleEndian>()
    }

    pub fn read_i32(&self) -> Result<i32, Error> {
        self.file.borrow_mut().read_i32::<LittleEndian>()
    }

    pub fn read_i64(&self) -> Result<i64, Error> {
        self.file.borrow_mut().read_i64::<LittleEndian>()
    }

    pub fn read_f32(&self) -> Result<f32, Error> {
        self.file.borrow_mut().read_f32::<LittleEndian>()
    }

    pub fn read_f64(&self) -> Result<f64, Error> {
        self.file.borrow_mut().read_f64::<LittleEndian>()
    }

    pub fn read_bytes(&self, length: u64) -> Result<Vec<u8>, Error> {
        let mut buffer: Vec<u8> = vec![];

        for _ in 0..length {
            let val = self.read_u8()?;
            buffer.push(val);
        }

        Ok(buffer)
    }

    pub fn read_string(&self, length: u64) -> Result<String, Error> {
        let buffer = self.read_bytes(length)?;

        match String::from_utf8(buffer) {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::new(ErrorKind::NotFound, e)),
        }
    }

    pub fn read_string_to_end(&self) -> Result<String, Error> {
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

    pub fn read_string_block(&self, offset: u32) -> Result<String, Error> {
        let string_type = self.read_u8()?;

        match string_type {
            0 | WzReader::HEADERBYTE_WITHOUT_OFFSET => self.read_wz_string(),
            1 | WzReader::HEADERBYTE_WITH_OFFSET => {
                let another_offset = self.read_u32()?;
                self.read_wz_string_at_offset(offset + another_offset)
            }
            _ => Err(Error::new(ErrorKind::NotFound, "Unknown type")),
        }
    }

    pub fn read_wz_int(&self) -> Result<i32, Error> {
        let possible_size = self.read_i8()?;

        if possible_size == -128 {
            let wz_int = self.read_i32()?;
            Ok(wz_int)
        } else {
            Ok(possible_size as i32)
        }
    }

    pub fn read_wz_long(&self) -> Result<i64, Error> {
        let possible_size = self.read_i8()?;

        if possible_size == -128 {
            let wz_long = self.read_i64()?;
            Ok(wz_long)
        } else {
            Ok(possible_size as i64)
        }
    }

    pub fn read_wz_string_at_offset(&self, offset: u32) -> Result<String, Error> {
        let position = self.get_position()?;
        self.seek(offset.into())?;
        let result = self.read_wz_string();
        self.seek(position)?;

        result
    }

    pub fn read_wz_string(&self) -> Result<String, Error> {
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

    pub fn read_wz_offset(&self) -> Result<u32, Error> {
        let file_start = *self.file_start.borrow();
        let version_hash = *self.version_hash.borrow();

        let mut offset = self.get_position()?;
        offset = (offset - (file_start as u64)) ^ 0xFFFFFFFF;
        offset = offset * (version_hash as u64);
        offset -= 0x581C3F6D;
        offset = rotate_left(offset as u32, (offset & 0x1F) as u8) as u64;

        let encrypted_offset = self.read_u32()?;
        offset ^= encrypted_offset as u64;
        offset += (file_start * 2) as u64;

        Ok(offset as u32)
    }

    fn read_wz_string_as_unicode(&self, size: u32) -> Result<String, Error> {
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

            // Newer versions do not use encryption
            let key = self.wz_key.clone();
            if let Some(mut key) = key {
                encrypted_char ^= ((key.at(i * 2 + 1) as u16) << 8) + (key.at(i * 2) as u16)
            }

            res_string.push(encrypted_char);
            mask += 1;
        }

        match String::from_utf16(&res_string) {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::new(ErrorKind::Other, e)),
        }
    }

    fn read_wz_string_as_ascii(&self, size: u32) -> Result<String, Error> {
        let mut mask: u8 = 0xAA;
        let mut res_string: Vec<u8> = vec![];
        for i in 0..(size as usize) {
            let mut encrypted_char = self.read_u8()?;
            encrypted_char ^= mask;

            // Newer versions do not use encryption
            let key = self.wz_key.clone();
            if let Some(mut key) = key {
                encrypted_char ^= key.at(i) as u8
            }

            res_string.push(encrypted_char as u8);

            mask = mask.wrapping_add(1);
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
