use std::fmt;

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

impl fmt::Display for WzValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WzValue::Null => write!(f, "Null"),
            WzValue::Directory => write!(f, "Directory"),
            WzValue::Img => write!(f, "Img"),
            WzValue::Extended => write!(f, "Extended"),
            WzValue::Short(val) => write!(f, "Short({})", val),
            WzValue::Int(val) => write!(f, "Int({})", val),
            WzValue::Long(val) => write!(f, "Long({})", val),
            WzValue::Float(val) => write!(f, "Float({})", val),
            WzValue::Double(val) => write!(f, "Double({})", val),
            WzValue::String(val) => write!(f, "String({})", val),
            WzValue::Vector(x, y) => write!(f, "Vector({}, {})", x, y),
            WzValue::Sound(sound) => write!(
                f,
                "Sound(offset: {}, len: {}, header_offset: {}, data_offset: {}, data_len: {})",
                sound.offset, sound.len, sound.header_offset, sound.data_offset, sound.data_len
            ),
            WzValue::Uol(val) => write!(f, "Uol({})", val),
        }
    }
}
