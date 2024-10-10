use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use indexmap::IndexMap;
use serde::Serialize;
use wz::{resolve, WzFile, WzNode, WzValue, WzVersion};

#[derive(Serialize, Debug)]
pub struct ItemOption {
    name: String,
    description: String,
    levels: IndexMap<i32, IndexMap<String, i32>>,
}

impl ItemOption {
    pub fn from_node(node: &WzNode) -> Option<Self> {
        let name = node.name.clone();
        let mut description = String::new();
        let mut levels = IndexMap::new();

        // Extract description
        if let Some(info_node) = node.children.get("info") {
            if let Some(string_node) = info_node.children.get("string") {
                if let WzValue::String(value) = &string_node.value {
                    description = value.clone();
                }
            }
        }

        // Process each level and extract all properties
        if let Some(level_node) = node.children.get("level") {
            for (level_name, level_val) in &level_node.children {
                if let Ok(level_num) = level_name.parse::<i32>() {
                    let mut properties = IndexMap::new();

                    // Extract all properties under this level
                    for (prop_name, prop_node) in &level_val.children {
                        if let WzValue::Int(prop_value) = prop_node.value {
                            properties.insert(prop_name.clone(), prop_value);
                        }
                    }

                    // Only add levels with at least one property
                    if !properties.is_empty() {
                        levels.insert(level_num, properties);
                    }
                }
            }
        }

        // Return the new ItemOption struct
        Some(Self {
            name,
            description,
            levels,
        })
    }
}

pub fn build_lookup_table(root_node: &WzNode) -> IndexMap<String, ItemOption> {
    let mut table = IndexMap::new();

    for (name, child_node) in &root_node.children {
        if let Some(item_option) = ItemOption::from_node(child_node) {
            table.insert(name.clone(), item_option);
        }
    }

    table
}

pub fn get_description(
    lookup: (&str, i32),
    item_options: &IndexMap<String, ItemOption>,
) -> Option<String> {
    let (item_id, level) = lookup;

    // Look up the item by item_id
    if let Some(item_option) = item_options.get(item_id) {
        // Clone the description to modify it
        let mut description = item_option.description.clone();

        // Check if the specified level exists in the item's levels map
        if let Some(level_data) = item_option.levels.get(&level) {
            // Replace each placeholder in the format #variable with the actual value
            for (key, value) in level_data {
                let placeholder = format!("#{}", key);
                description = description.replace(&placeholder, &value.to_string());
            }
        }

        // Return the modified description
        Some(description)
    } else {
        None
    }
}

fn main() -> io::Result<()> {
    simple_logger::SimpleLogger::new()
        .env()
        .with_module_level("wz", log::LevelFilter::Error)
        .init()
        .unwrap();

    let input_file = "assets/Item.wz";
    let input_node_path = "ItemOption.img";
    let output_file = "assets/itemOption.json";

    let lookup_potentials = vec!["040041", "030041", "030044"];
    let lookup_level = 15;

    let write_to_file = true;
    let lookup = false;

    let mut wz_file = WzFile::new(input_file, WzVersion::GMS);

    wz_file.open()?;

    let root = wz_file.parse_root_directory()?;

    let mut lookup_table = IndexMap::new();

    if let Ok(node) = resolve(&root, input_node_path) {
        lookup_table = build_lookup_table(&node);

        if write_to_file {
            let json_data = serde_json::to_string_pretty(&lookup_table).unwrap();
            let mut file = File::create(&Path::new(output_file))?;
            file.write_all(json_data.as_bytes())?;
        }
    }

    if lookup {
        for potential in lookup_potentials {
            if let Some(description) = get_description((potential, lookup_level), &lookup_table) {
                log::error!("Looking up {:?} -- Description: {}", potential, description);
            } else {
                log::error!("Item not found for ID {}", potential);
            }
        }
    }

    Ok(())
}
