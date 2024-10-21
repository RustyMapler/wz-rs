use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Serialize, Deserialize, Debug)]
struct World {
    id: String,
    project_id: String,
    uri: String,
    mimetype: String,
    children: Vec<Child>,
    field11: u32,
    version: String,
    sub_version: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(rename = "MOD.Core.MapComponent")]
    MapComponent(MapComponent),
    #[serde(rename = "MOD.Core.FootholdComponent")]
    FootholdComponent(FootholdComponent),
    #[serde(rename = "MOD.Core.SoundComponent")]
    SoundComponent(SoundComponent),
    #[serde(rename = "MOD.Core.BackgroundComponent")]
    BackgroundComponent(BackgroundComponent),
    #[serde(rename = "MOD.Core.TileMapComponent")]
    TileMapComponent(TileMapComponent),
    #[serde(rename = "MOD.Core.MapLayerComponent")]
    MapLayerComponent(MapLayerComponent),
    #[serde(rename = "MOD.Core.TransformComponent")]
    TransformComponent(TransformComponent),
    #[serde(other)]
    Unknown, // Catch-all for unknown component types
}

#[derive(Serialize, Deserialize, Debug)]
struct MapComponent {
    #[serde(rename = "AirAccelerationXFactor")]
    air_acceleration_x_factor: f32,
    #[serde(rename = "AirDecelerationXFactor")]
    air_deceleration_x_factor: f32,
    #[serde(rename = "FallSpeedMaxXFactor")]
    fall_speed_max_x_factor: f32,
    #[serde(rename = "FallSpeedMaxYFactor")]
    fall_speed_max_y_factor: f32,
    #[serde(rename = "Gravity")]
    gravity: f32,
    #[serde(rename = "IsInstanceMap")]
    is_instance_map: bool,
    #[serde(rename = "TileMapMode")]
    tile_map_mode: u32,
    #[serde(rename = "WalkAccelerationFactor")]
    walk_acceleration_factor: f32,
    #[serde(rename = "WalkDrag")]
    walk_drag: f32,
    #[serde(rename = "Enable")]
    enable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct FootholdComponent {
    #[serde(rename = "FootholdsByLayer")]
    footholds_by_layer: HashMap<String, Vec<Foothold>>,
    #[serde(rename = "Enable")]
    enable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct SoundComponent {
    #[serde(rename = "AudioClipRUID")]
    audio_clip_ruid: String,
    #[serde(rename = "Bgm")]
    bgm: bool,
    #[serde(rename = "PlayOnEnable")]
    play_on_enable: bool,
    #[serde(rename = "Volume")]
    volume: f32,
    #[serde(rename = "Enable")]
    enable: bool,
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

// Define the struct for origin
#[derive(Serialize, Deserialize, Debug)]
struct TileMapComponent {
    #[serde(rename = "Color", skip_serializing_if = "Option::is_none")]
    color: Option<SolidColor>,
    #[serde(rename = "FootholdDrag", skip_serializing_if = "Option::is_none")]
    foothold_drag: Option<f32>,
    #[serde(rename = "FootholdForce", skip_serializing_if = "Option::is_none")]
    foothold_force: Option<f32>,
    #[serde(
        rename = "FootholdWalkSpeedFactor",
        skip_serializing_if = "Option::is_none"
    )]
    foothold_walk_speed_factor: Option<f32>,
    #[serde(
        rename = "IgnoreMapLayerCheck",
        skip_serializing_if = "Option::is_none"
    )]
    ignore_map_layer_check: Option<bool>,
    #[serde(rename = "IsOddGridPosition")]
    is_odd_grid_position: bool,
    #[serde(rename = "OrderInLayer", skip_serializing_if = "Option::is_none")]
    order_in_layer: Option<u32>,
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

// Enum to represent different types of components
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct TransformComponent {
    #[serde(rename = "Position")]
    position: Position,
    #[serde(rename = "QuaternionRotation")]
    quaternion_rotation: QuaternionRotation,
    #[serde(rename = "Scale", skip_serializing_if = "Option::is_none")]
    scale: Option<Scale>,
    #[serde(rename = "Enable")]
    enable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Foothold {
    #[serde(rename = "Length")]
    length: f32,
    #[serde(rename = "NextFootholdId")]
    next_foothold_id: u32,
    #[serde(rename = "PreviousFootholdId")]
    previous_foothold_id: u32,
    #[serde(rename = "groupID")]
    group_id: u32,
    #[serde(rename = "layer")]
    layer: u32,
    #[serde(rename = "sortingLayerName")]
    sorting_layer_name: String,
    #[serde(rename = "attribute")]
    attribute: FootholdAttribute,
    #[serde(rename = "OwnerId")]
    owner_id: String,
    #[serde(rename = "Id")]
    id: u32,
    #[serde(rename = "StartPoint")]
    start_point: Point,
    #[serde(rename = "EndPoint")]
    end_point: Point,
    #[serde(rename = "Variance")]
    variance: Point,
    #[serde(rename = "IsDynamic")]
    is_dynamic: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct FootholdAttribute {
    #[serde(rename = "walk")]
    walk: f32,
    #[serde(rename = "force")]
    force: f32,
    #[serde(rename = "drag")]
    drag: f32,
    #[serde(rename = "isBlockVertical")]
    is_block_vertical: bool,
    #[serde(rename = "isDynamic")]
    is_dynamic: bool,
    #[serde(rename = "isCustomFoothold")]
    is_custom_foothold: bool,
    #[serde(rename = "inertiaOption")]
    inertia_option: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: f32,
    y: f32,
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

// Define the structs for each component type
#[derive(Serialize, Deserialize, Debug)]
struct Tile {
    #[serde(rename = "type")]
    tile_type: u32,
    position: TilePosition,
    #[serde(rename = "tileIndex")]
    tile_index: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct TilePosition {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct QuaternionRotation {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Scale {
    x: f32,
    y: f32,
    z: f32,
}

fn deserialize_json_field<'de, D>(deserializer: D) -> Result<FieldJson, D::Error>
where
    D: Deserializer<'de>,
{
    let json_str: String = Deserialize::deserialize(deserializer)?;
    serde_json::from_str(&json_str).map_err(serde::de::Error::custom)
}

fn serialize_json_field<S>(field_json: &FieldJson, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let json_str = serde_json::to_string(field_json).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&json_str)
}

fn main() -> io::Result<()> {
    // Read the JSON file
    let mut file = File::open("assets/000020000-string.json")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    // Deserialize the JSON data
    let world: World = serde_json::from_str(&data).expect("Invalid JSON");

    // Serialize the data back to JSON
    let serialized_data = serde_json::to_string_pretty(&world).expect("Serialization failed");

    // Optionally, write the serialized data back to a file
    let mut output_file = File::create("assets/000020000-string-serialized.json")?;
    output_file.write_all(serialized_data.as_bytes())?;

    Ok(())
}
