use crate::properties::node::WzNode;
use std::{
    fs::File,
    io::{Error, ErrorKind, Result, Write},
    sync::Arc,
};

pub fn to_json(node: &Arc<WzNode>) -> Result<String> {
    serde_json::to_string_pretty(node.as_ref()).map_err(|e| Error::new(ErrorKind::Other, e))
}

pub fn write_json_to_file(json: &str, output_file: &str) -> Result<()> {
    let mut file = File::create(output_file)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
