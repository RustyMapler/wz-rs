#[derive(Debug, Clone)]
pub struct WzSound {
    pub offset: u32,
    pub len: u32,
    pub header_offset: u64,
    pub header: Vec<u8>,
    pub data_offset: u64,
    pub data: Vec<u8>,
    pub data_len: u32,
}

#[derive(Debug, Clone)]
pub enum WzValue {
    Null,
    Directory,
    Img,
    Extended,
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    Vector(i32, i32),
    Sound(WzSound),
    Uol(String),
}

impl Default for WzValue {
    fn default() -> Self {
        Self::Null
    }
}
