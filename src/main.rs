extern crate wz;

use std::io;
use std::time::Instant;
use wz::{resolve, WzFile, WzVersion};

fn time_code_block<F: FnOnce() -> R, R>(f: F, label: &str) -> R {
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();

    log::info!("{}: Duration: {:?}", label, duration);

    result
}

fn main() -> io::Result<()> {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let file_path = "assets/Map.wz";
    let node_path = "MapHelper.img/weather/snow";

    let mut wz_file = WzFile::new(file_path, WzVersion::GMS);
    wz_file.open()?;

    let root = time_code_block(
        || {
            return wz_file.parse_root_directory().unwrap();
        },
        "New | parse root",
    );

    time_code_block(
        || {
            if let Ok(node) = resolve(&root, node_path) {
                log::info!("node: {}", node.name);
            }
        },
        &format!("New | resolve {}", node_path),
    );

    //print_node(&root, 0);

    Ok(())
}
