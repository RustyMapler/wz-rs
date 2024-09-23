extern crate wz;

use std::io;
use std::time::Instant;
use wz::{WzFile, WzVersion};

fn main() -> io::Result<()> {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let now = Instant::now();

    let mut map_wz = WzFile::new("assets/Map.wz", WzVersion::GMS);
    map_wz.open()?;

    if let Some(node) = map_wz.resolve("MapHelper.img/weather/snow") {
        log::info!("node: {}", node.get_name());

        for (child, _) in node.get_children() {
            log::info!("child: {}", child);
        }
    }

    let elapsed = now.elapsed();
    log::info!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
