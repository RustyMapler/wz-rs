use std::fmt;

#[derive(Default, Debug, Clone)]

pub struct WzCanvas {
    pub width: u32,
    pub height: u32,
    pub format1: u32,
    pub format2: u8,
    pub offset: u32,
}

impl fmt::Display for WzCanvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WzCanvas(width: {}, height: {}, format1: {}, format2: {}, offset: {})",
            self.width, self.height, self.format1, self.format2, self.offset
        )
    }
}
