extern crate wz;

use std::io;
use std::time::Instant;
use wz::{WzFile, WzVersion};

fn main() -> io::Result<()> {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let now = Instant::now();

    let mut wz_file = WzFile::new("assets/Item.wz", WzVersion::GMS);
    wz_file.open()?;

    let node = wz_file.parse_main_dir().unwrap();

    // wz_file.parse_wz_main_directory().unwrap();

    // if let Some(node) = map_wz.resolve("MapHelper.img/weather/snow") {
    //     log::info!("node: {}", node.get_name());

    //     for (child, _) in node.get_children() {
    //         log::info!("child: {}", child);
    //     }
    // }

    let elapsed = now.elapsed();
    log::info!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
