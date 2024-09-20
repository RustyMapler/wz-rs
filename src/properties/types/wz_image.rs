use crate::Vec2;

#[derive(Default, Debug)]
pub struct WzImage {
    pub width: u32,
    pub height: u32,
    pub origin: Vec2,
    pub data: Vec<u8>,
}
