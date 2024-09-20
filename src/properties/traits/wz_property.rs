use crate::{Vec2, WzImage};

pub trait WzProperty {
    fn get_string(&self) -> Option<String> {
        None
    }

    fn get_short(&self) -> Option<i16> {
        None
    }

    fn get_int(&self) -> Option<i32> {
        None
    }

    fn get_long(&self) -> Option<i64> {
        None
    }

    fn get_float(&self) -> Option<f32> {
        None
    }

    fn get_double(&self) -> Option<f64> {
        None
    }

    fn get_image(&self) -> Option<WzImage> {
        None
    }

    fn get_vec2(&self) -> Option<Vec2> {
        None
    }

    fn get_sound(&self) -> Option<Vec<u8>> {
        None
    }

    fn is_inlink(&self) -> bool {
        false
    }

    fn get_inlink(&self) -> Option<String> {
        None
    }

    fn is_outlink(&self) -> bool {
        false
    }

    fn get_outlink(&self) -> Option<String> {
        None
    }

    fn is_uol(&self) -> bool {
        false
    }

    fn get_uol(&self) -> Option<String> {
        None
    }
}
