use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::Write;
use std::sync::Arc;
use wz::{properties::node::WzNode, resolve, WzFile, WzVersion};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Origin {
    #[serde(rename = "type")]
    origin_type: String,
    entry_id: String,
    sub_entity_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Position {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct QuaternionRotation {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Scale {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct TileSetRUID {
    #[serde(rename = "DataId")]
    data_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TilePosition {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Tile {
    #[serde(rename = "type")]
    tile_type: u32,
    position: TilePosition,
    tile_index: i32,
}

// Enum for Components
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "@type")]
enum Component {
    #[serde(rename = "MOD.Core.TransformComponent")]
    TransformComponent {
        #[serde(rename = "Position")]
        position: Position,
        #[serde(rename = "QuaternionRotation")]
        quaternion_rotation: QuaternionRotation,
        #[serde(rename = "Scale")]
        scale: Scale,
        #[serde(rename = "Enable")]
        enable: bool,
    },
    #[serde(rename = "MOD.Core.MapLayerComponent")]
    MapLayerComponent {
        #[serde(rename = "IsVisible")]
        is_visible: bool,
        #[serde(rename = "LayerSortOrder")]
        layer_sort_order: u32,
        #[serde(rename = "Locked")]
        locked: bool,
        #[serde(rename = "MapLayerName")]
        map_layer_name: String,
        #[serde(rename = "Thumbnail")]
        thumbnail: String,
        #[serde(rename = "Enable")]
        enable: bool,
    },
    #[serde(rename = "MOD.Core.TileMapComponent")]
    TileMapComponent {
        #[serde(rename = "IsOddGridPosition")]
        is_odd_grid_position: bool,
        #[serde(rename = "SortingLayer")]
        sorting_layer: String,
        #[serde(rename = "TileMapVersion")]
        tile_map_version: u32,
        #[serde(rename = "TileSetRUID")]
        tile_set_ruid: TileSetRUID,
        #[serde(rename = "Tiles")]
        tiles: Vec<Tile>,
        #[serde(rename = "Enable")]
        enable: bool,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MapLayer {
    name: String,
    path: String,
    name_editable: bool,
    enable: bool,
    visible: bool,
    display_order: u32,
    path_constraints: String,
    revision: u32,
    origin: Origin,
    model_id: String,
    #[serde(rename = "@components")]
    components: Vec<Component>,
    #[serde(rename = "@version")]
    version: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TileMap {
    name: String,
    path: String,
    name_editable: bool,
    enable: bool,
    visible: bool,
    display_order: u32,
    path_constraints: String,
    revision: u32,
    origin: Origin,
    model_id: String,
    #[serde(rename = "@components")]
    components: Vec<Component>,
    #[serde(rename = "@version")]
    version: u32,
}

fn to_json(node: &Arc<WzNode>) -> String {
    serde_json::to_string_pretty(node.as_ref()).unwrap()
}

fn write_json_to_file(json: &str, output_file: &str) -> io::Result<()> {
    let mut file = File::create(output_file)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

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
    let input_map_id = "000010000";
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
