extern crate wz;

use std::io;
use wz::{build_lookup_table, get_description, resolve, WzFile, WzVersion};

fn main() -> io::Result<()> {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let file_path = "assets/Item.wz";
    let node_path = "ItemOption.img";

    let mut wz_file = WzFile::new(file_path, WzVersion::GMS);
    wz_file.open()?;

    let root = wz_file.parse_root_directory()?;

    if let Ok(node) = resolve(&root, node_path) {
        let lookup_table = build_lookup_table(&node);

        let potentials = vec!["040041", "030041", "030044"];
        let level = 15;

        for potential in potentials {
            if let Some(description) = get_description((potential, level), &lookup_table) {
                println!("Looking up {:?} -- Description: {}", potential, description);
            } else {
                println!("Item not found for ID {}", potential);
            }
        }

        // let json_data = serde_json::to_string_pretty(&lookup_table).unwrap();
        // println!("{}", json_data);
    }

    Ok(())
}
