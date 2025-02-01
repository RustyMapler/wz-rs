use crate::Vec2;
use std::fmt;

#[derive(Default, Debug, Clone)]
pub struct WzImage {
    pub width: u32,
    pub height: u32,
    pub origin: Vec2,
    pub data: Vec<u8>,
}

impl fmt::Display for WzImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WzImage(width: {}, height: {}, origin: {}, data: {})",
            self.width,
            self.height,
            self.origin,
            self.data.len()
        )
    }
}
