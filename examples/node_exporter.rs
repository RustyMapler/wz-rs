use std::io;
use wz::{resolve, to_json, write_json_to_file, WzFile, WzVersion};

fn to_node_path(map_id: &str) -> String {
    format!("Map/Map{}/{}.img", &map_id[0..1], map_id)
}

fn main() -> io::Result<()> {
    simple_logger::SimpleLogger::new()
        .env()
        .with_module_level("wz", log::LevelFilter::Error)
        .init()
        .unwrap();

    let input_file = "assets/Map002.wz";
    let input_map_id = "000020000";
    let output_file = format!("assets/{}.json", input_map_id);

    let mut wz_file = WzFile::new(input_file, WzVersion::GMS);
    wz_file.open()?;

    let root = wz_file.parse_root_directory()?;

    let path = to_node_path(input_map_id);

    if let Ok(node) = resolve(&root, &path) {
        let json = to_json(&node);
        write_json_to_file(&json, &output_file)?;
        log::info!("JSON data written to {}", output_file);
    } else {
        log::error!("Node not found for path: {}", path);
    }

    Ok(())
}
