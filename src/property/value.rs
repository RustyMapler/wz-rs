#[derive(Debug, Clone)]
pub enum WzValue {
    Null,
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Vector((i32, i32)),
    String(String),
}

impl Default for WzValue {
  fn default() -> Self {
      Self::Null
  }
}
