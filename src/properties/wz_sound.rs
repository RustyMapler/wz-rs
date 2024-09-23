use std::{io::Error, sync::Arc};

use crate::{WzNode, WzProperty, WzReader};

pub struct WzSoundProperty {
    pub name: String,
    pub reader: Arc<WzReader>,
    pub offset: u32,
    pub len: u32,
    pub data_len: u32,
    pub header: Vec<u8>,
    pub sound_offset: u64,
}

impl WzProperty for WzSoundProperty {
    fn get_sound(&self) -> Option<Vec<u8>> {
        match self.parse_sound() {
            Ok(sound) => Some(sound),
            Err(_) => None,
        }
    }
}

impl WzNode for WzSoundProperty {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl WzSoundProperty {
    const SOUND_HEADER: [u8; 51] = [
        0x02, 0x83, 0xEB, 0x36, 0xE4, 0x4F, 0x52, 0xCE, 0x11, 0x9F, 0x53, 0x00, 0x20, 0xAF, 0x0B,
        0xA7, 0x70, 0x8B, 0xEB, 0x36, 0xE4, 0x4F, 0x52, 0xCE, 0x11, 0x9F, 0x53, 0x00, 0x20, 0xAF,
        0x0B, 0xA7, 0x70, 0x00, 0x01, 0x81, 0x9F, 0x58, 0x05, 0x56, 0xC3, 0xCE, 0x11, 0xBF, 0x01,
        0x00, 0xAA, 0x00, 0x55, 0x59, 0x5A,
    ];

    pub fn create(
        reader: Arc<WzReader>,
        offset: u32,
        name: String,
    ) -> Result<WzSoundProperty, Error> {
        reader.skip(1)?;
        let data_len = reader.read_wz_int()?;
        let len = reader.read_wz_int()?;

        // Get the wav_len
        let header_offset = reader.get_position()?;
        reader.skip(WzSoundProperty::SOUND_HEADER.len())?;
        let wav_len = reader.read_u8()?;
        reader.seek(header_offset)?;

        let header = (*reader)
            .read_bytes(WzSoundProperty::SOUND_HEADER.len() as u64 + 1 + wav_len as u64)?;

        let sound_offset = reader.get_position()?;
        reader.skip(data_len as usize)?; // skip bytes to read later

        Ok(WzSoundProperty {
            reader,
            offset,
            name,
            len: len as u32,
            data_len: data_len as u32,
            header,
            sound_offset,
        })
    }

    pub fn parse_sound(&self) -> Result<Vec<u8>, Error> {
        let current_position = self.reader.get_position()?;
        self.reader.seek(self.sound_offset)?;

        let sound_bytes = self.reader.read_bytes(self.data_len.into())?;
        self.reader.seek(current_position)?;

        Ok(sound_bytes)
    }
}
