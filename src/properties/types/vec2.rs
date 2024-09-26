use std::fmt;

#[derive(Default, Debug, Clone)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
