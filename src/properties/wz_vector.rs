use crate::{Vec2, WzNode, WzProperty};

pub struct WzVectorProperty {
    pub name: String,
    pub value: (i32, i32),
}

impl WzProperty for WzVectorProperty {
    fn get_vec2(&self) -> Option<Vec2> {
        Some(Vec2 {
            x: self.value.0,
            y: self.value.1,
        })
    }
}

impl WzNode for WzVectorProperty {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
