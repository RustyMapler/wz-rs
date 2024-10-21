use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Serialize, Deserialize, Debug)]
struct World {
    id: String,
    project_id: String,
    uri: String,
    mimetype: String,
    children: Vec<Child>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Child {
    field1: u32,
    json: String,
    field3: Vec<Field>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Field {
    id: String,
    path: String,
    #[serde(
        deserialize_with = "deserialize_json_field",
        serialize_with = "serialize_json_field"
    )]
    json: FieldJson,
    mimetype: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FieldJson {
    name: String,
    path: String,
    #[serde(rename = "nameEditable")]
    name_editable: bool,
    enable: bool,
    visible: bool,
    #[serde(rename = "displayOrder")]
    display_order: u32,
    #[serde(rename = "pathConstraints")]
    path_constraints: String,
    revision: u32,
    origin: Option<Origin>,
    #[serde(rename = "modelId")]
    model_id: Option<String>,
    #[serde(rename = "@components")]
    components: Vec<Component>,
    #[serde(rename = "@version")]
    version: u32,
}

// Define the struct for origin
#[derive(Serialize, Deserialize, Debug)]
struct Origin {
    #[serde(rename = "type")]
    origin_type: String,
    entry_id: String,
    sub_entity_id: Option<String>,
}

// Enum to represent different types of components
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "@type")]
enum Component {
    #[serde(rename = "MOD.Core.BackgroundComponent")]
    BackgroundComponent(BackgroundComponent),
    #[serde(rename = "MOD.Core.TileMapComponent")]
    TileMapComponent(TileMapComponent),
    #[serde(rename = "MOD.Core.MapLayerComponent")]
    MapLayerComponent(MapLayerComponent),
    #[serde(other)]
    Unknown, // Catch-all for unknown component types
}

// Define the structs for each component type
#[derive(Serialize, Deserialize, Debug)]
struct BackgroundComponent {
    #[serde(rename = "SolidColor")]
    solid_color: SolidColor,
    #[serde(rename = "TemplateRUID")]
    template_ruid: String,
    #[serde(rename = "Type")]
    component_type: u32,
    #[serde(rename = "WebUrl")]
    web_url: String,
    #[serde(rename = "Enable")]
    enable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct MapLayerComponent {
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
}

#[derive(Serialize, Deserialize, Debug)]
struct TileMapComponent {
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
}

#[derive(Serialize, Deserialize, Debug)]
struct SolidColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct TileSetRUID {
    #[serde(rename = "DataId")]
    data_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tile {
    #[serde(rename = "type")]
    tile_type: u32,
    position: Position,
    #[serde(rename = "tileIndex")]
    tile_index: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Position {
    x: i32,
    y: i32,
}

// Custom deserialization function for the json field
fn deserialize_json_field<'de, D>(deserializer: D) -> Result<FieldJson, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let json_str: String = Deserialize::deserialize(deserializer)?;
    serde_json::from_str(&json_str).map_err(serde::de::Error::custom)
}

// Custom serialization function for the json field
fn serialize_json_field<S>(field_json: &FieldJson, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let json_str = serde_json::to_string(field_json).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&json_str)
}

fn main() -> io::Result<()> {
    // Read the JSON file
    let mut file = File::open("assets/map-000020000.json")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    // Deserialize the JSON data
    let world: World = serde_json::from_str(&data).expect("Invalid JSON");

    // Serialize the data back to JSON
    let serialized_data = serde_json::to_string_pretty(&world).expect("Serialization failed");

    // Optionally, write the serialized data back to a file
    let mut output_file = File::create("assets/map-serialized-000020000.json")?;
    output_file.write_all(serialized_data.as_bytes())?;

    Ok(())
}
