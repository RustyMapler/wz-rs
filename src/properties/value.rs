use std::fmt;

use crate::{Vec2, WzCanvas, WzSound};

#[derive(Default, Debug, Clone)]
pub enum WzValue {
    #[default]
    Null,
    Directory,
    Img,
    Extended,
    Convex,
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    Vector(Vec2),
    Canvas(WzCanvas),
    Sound(WzSound),
    Uol(String),
}

impl fmt::Display for WzValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WzValue::Null => write!(f, "Null"),
            WzValue::Directory => write!(f, "Directory"),
            WzValue::Img => write!(f, "Img"),
            WzValue::Extended => write!(f, "Extended"),
            WzValue::Convex => write!(f, "Convex"),
            WzValue::Short(val) => write!(f, "Short: {}", val),
            WzValue::Int(val) => write!(f, "Int: {}", val),
            WzValue::Long(val) => write!(f, "Long: {}", val),
            WzValue::Float(val) => write!(f, "Float: {}", val),
            WzValue::Double(val) => write!(f, "Double: {}", val),
            WzValue::String(val) => write!(f, "String: {}", val),
            WzValue::Vector(val) => write!(f, "Vector: {}", val),
            WzValue::Canvas(val) => write!(f, "Canvas: {}", val),
            WzValue::Sound(val) => write!(f, "Sound: {}", val),
            WzValue::Uol(val) => write!(f, "Uol: {}", val),
        }
    }
}

macro_rules! try_as {
    ($func_name:ident, $variant:ident, $result:ty) => {
        fn $func_name(&self) -> Option<&$result> {
            match self {
                WzValue::$variant(inner) => Some(inner),
                _ => None,
            }
        }
    };
}

pub trait WzValueCast {
    fn is_null(&self) -> bool;
    fn is_directory(&self) -> bool;
    fn is_img(&self) -> bool;
    fn is_extended(&self) -> bool;
    fn is_convex(&self) -> bool;

    fn as_short(&self) -> Option<&i16>;
    fn as_int(&self) -> Option<&i32>;
    fn as_long(&self) -> Option<&i64>;
    fn as_float(&self) -> Option<&f32>;
    fn as_double(&self) -> Option<&f64>;
    fn as_string(&self) -> Option<&String>;
    fn as_vector(&self) -> Option<&Vec2>;
    fn as_canvas(&self) -> Option<&WzCanvas>;
    fn as_sound(&self) -> Option<&WzSound>;
    fn as_uol(&self) -> Option<&String>;
}

impl WzValueCast for WzValue {
    fn is_null(&self) -> bool {
        matches!(self, WzValue::Null)
    }

    fn is_directory(&self) -> bool {
        matches!(self, WzValue::Directory)
    }

    fn is_img(&self) -> bool {
        matches!(self, WzValue::Img)
    }

    fn is_extended(&self) -> bool {
        matches!(self, WzValue::Extended)
    }

    fn is_convex(&self) -> bool {
        matches!(self, WzValue::Convex)
    }

    try_as!(as_short, Short, i16);
    try_as!(as_int, Int, i32);
    try_as!(as_long, Long, i64);
    try_as!(as_float, Float, f32);
    try_as!(as_double, Double, f64);
    try_as!(as_string, String, String);
    try_as!(as_vector, Vector, Vec2);
    try_as!(as_canvas, Canvas, WzCanvas);
    try_as!(as_sound, Sound, WzSound);
    try_as!(as_uol, Uol, String);
}
