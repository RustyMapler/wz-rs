extern crate wz;

use std::io;
use std::time::Instant;
use wz::{print_node, resolve, WzFile, WzVersion};

fn main() -> io::Result<()> {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let now = Instant::now();

    let mut wz_file = WzFile::new("assets/Map.wz", WzVersion::GMS);
    wz_file.open()?;

    // New way

    let node = wz_file.parse_root_directory().unwrap();
    print_node(&node, 0);

    let resolved_node = resolve(&node, "MapHelper.img/weather/snow")?;
    log::info!("node: {}", resolved_node.name);

    // Old way

    wz_file.parse_wz_main_directory()?;

    if let Some(node) = wz_file.resolve("MapHelper.img/weather/snow") {
        log::info!("node: {}", node.get_name());
    }

    let elapsed = now.elapsed();
    log::info!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
