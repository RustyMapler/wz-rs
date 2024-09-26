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

impl fmt::Display for WzSound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WzSound(offset: {}, len: {}, header_offset: {}, data_offset: {}, data_len: {})",
            self.offset, self.len, self.header_offset, self.data_offset, self.data_len
        )
    }
}
