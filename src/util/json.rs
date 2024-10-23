use crate::properties::node::WzNode;
use std::{
    fs::File,
    io::{self, Write},
    sync::Arc,
};

pub fn to_json(node: &Arc<WzNode>) -> String {
    serde_json::to_string_pretty(node.as_ref()).unwrap()
}

pub fn write_json_to_file(json: &str, output_file: &str) -> io::Result<()> {
    let mut file = File::create(output_file)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
