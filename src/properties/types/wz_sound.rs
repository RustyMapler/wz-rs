use std::fmt;

#[derive(Default, Debug, Clone)]
pub struct WzSound {
    pub offset: u32,
    pub len: u32,
    pub header_offset: u64,
    pub header: Vec<u8>,
    pub data_offset: u64,
    pub data: Vec<u8>,
    pub data_len: u32,
}

impl WzSound {
    pub const SOUND_HEADER: [u8; 51] = [
        0x02, 0x83, 0xEB, 0x36, 0xE4, 0x4F, 0x52, 0xCE, 0x11, 0x9F, 0x53, 0x00, 0x20, 0xAF, 0x0B,
        0xA7, 0x70, 0x8B, 0xEB, 0x36, 0xE4, 0x4F, 0x52, 0xCE, 0x11, 0x9F, 0x53, 0x00, 0x20, 0xAF,
        0x0B, 0xA7, 0x70, 0x00, 0x01, 0x81, 0x9F, 0x58, 0x05, 0x56, 0xC3, 0xCE, 0x11, 0xBF, 0x01,
        0x00, 0xAA, 0x00, 0x55, 0x59, 0x5A,
    ];
}

impl fmt::Display for WzSound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WzSound(offset: {}, len: {}, header_offset: {}, data_offset: {}, data_len: {})",
            self.offset, self.len, self.header_offset, self.data_offset, self.data_len
        )
    }
}
