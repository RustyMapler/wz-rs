use std::fmt;

use crate::{Vec2, WzSound};

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
    Vector(Vec2),
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
            WzValue::Vector(val) => write!(f, "Vector({})", val),
            WzValue::Sound(val) => write!(f, "Sound({})", val),
            WzValue::Uol(val) => write!(f, "Uol({})", val),
        }
    }
}
