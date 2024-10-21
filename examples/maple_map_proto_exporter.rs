use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Root {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "GameId")]
    game_id: String,
    #[serde(rename = "EntryKey")]
    entry_key: String,
    #[serde(rename = "ContentType")]
    content_type: String,
    #[serde(rename = "Content")]
    content: String,
    #[serde(rename = "Usage")]
    usage: u32,
    #[serde(rename = "UsePublish")]
    use_publish: u32,
    #[serde(rename = "UseService")]
    use_service: u32,
    #[serde(rename = "CoreVersion")]
    core_version: String,
    #[serde(rename = "StudioVersion")]
    studio_version: String,
    #[serde(rename = "DynamicLoading")]
    dynamic_loading: u32,
    #[serde(rename = "ContentProto")]
    content_proto: ContentProto,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContentProto {
    #[serde(rename = "Use")]
    use_type: String,
    #[serde(rename = "Entities")]
    entities: Vec<Entity>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Entity {
    id: String,
    path: String,
    #[serde(rename = "componentNames")]
    component_names: String,
    #[serde(rename = "jsonString")]
    json_string: JsonString,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonString {
    name: String,
    path: String,
    #[serde(rename = "nameEditable")]
    name_editable: bool,
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
    enable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Origin {
    #[serde(rename = "type")]
    origin_type: String,
    entry_id: String,
    sub_entity_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "@type")]
enum Component {
    #[serde(rename = "MOD.Core.TransformComponent")]
    TransformComponent(TransformComponent),
    #[serde(rename = "MOD.Core.MapComponent")]
    MapComponent(MapComponent),
    #[serde(rename = "MOD.Core.BackgroundComponent")]
    BackgroundComponent(BackgroundComponent),
    #[serde(rename = "MOD.Core.MapLayerComponent")]
    MapLayerComponent(MapLayerComponent),
    #[serde(rename = "MOD.Core.TileMapComponent")]
    TileMapComponent(TileMapComponent),
    #[serde(rename = "MOD.Core.FootholdComponent")]
    FootholdComponent(FootholdComponent),
    #[serde(rename = "MOD.Core.SpriteRendererComponent")]
    SpriteRendererComponent(SpriteRendererComponent),
    #[serde(rename = "MOD.Core.ClimbableSpriteRendererComponent")]
    ClimbableSpriteRendererComponent(ClimbableSpriteRendererComponent),
    #[serde(rename = "MOD.Core.ClimbableComponent")]
    ClimbableComponent(ClimbableComponent),
    #[serde(rename = "MOD.Core.PortalComponent")]
    PortalComponent(PortalComponent),
    #[serde(rename = "MOD.Core.SpawnLocationComponent")]
    SpawnLocationComponent(SpawnLocationComponent),
    #[serde(rename = "MOD.Core.TagComponent")]
    TagComponent(TagComponent),
    #[serde(other)]
    Unknown, // Catch-all for unknown component types
}

#[derive(Serialize, Deserialize, Debug)]
struct TransformComponent {
    #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
    position: Option<Transform>,
    #[serde(rename = "Rotation", skip_serializing_if = "Option::is_none")]
    rotation: Option<Transform>,
    #[serde(rename = "Scale", skip_serializing_if = "Option::is_none")]
    scale: Option<Transform>,
    #[serde(rename = "QuaternionRotation", skip_serializing_if = "Option::is_none")]
    quaternion_rotation: Option<Quaternion>,
    #[serde(rename = "ZRotation", skip_serializing_if = "Option::is_none")]
    z_rotation: Option<f32>,
    #[serde(rename = "Enable")]
    enable: bool,
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
struct BackgroundComponent {
    #[serde(rename = "Type")]
    background_type: u32,
    #[serde(rename = "SolidColor")]
    solid_color: Color,
    #[serde(rename = "TemplateRUID")]
    template_ruid: String,
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
    #[serde(rename = "OrderInLayer", skip_serializing_if = "Option::is_none")]
    order_in_layer: Option<u32>,
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
struct SpriteRendererComponent {
    #[serde(rename = "ActionSheet")]
    action_sheet: HashMap<String, String>,
    #[serde(rename = "EndFrameIndex")]
    end_frame_index: i32,
    #[serde(rename = "RenderSetting")]
    render_setting: u32,
    #[serde(rename = "SortingLayer")]
    sorting_layer: String,
    #[serde(rename = "SpriteRUID")]
    sprite_ruid: String,
    #[serde(rename = "StartFrameIndex")]
    start_frame_index: i32,
    #[serde(rename = "TiledSize", skip_serializing_if = "Option::is_none")]
    tiled_size: Option<Point>,
    #[serde(rename = "Color", skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(rename = "Enable")]
    enable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ClimbableSpriteRendererComponent {
    #[serde(rename = "BodySpriteFlipInfo")]
    body_sprite_flip_info: Point,
    #[serde(rename = "BodySpriteLocalPos")]
    body_sprite_local_pos: Point,
    #[serde(rename = "NeedGizmo")]
    need_gizmo: bool,
    #[serde(rename = "OrderInLayer", skip_serializing_if = "Option::is_none")]
    order_in_layer: Option<u32>,
    #[serde(rename = "OriginOffset")]
    origin_offset: Point,
    #[serde(rename = "OriginRectSize")]
    origin_rect_size: Point,
    #[serde(rename = "RenderSetting")]
    render_setting: u32,
    #[serde(rename = "SortingLayer")]
    sorting_layer: String,
    #[serde(rename = "SpriteRUID")]
    sprite_ruid: String,
    #[serde(rename = "SpriteRUIDHead")]
    sprite_ruid_head: String,
    #[serde(rename = "SpriteRUIDTail")]
    sprite_ruid_tail: String,
    #[serde(rename = "TiledSize", skip_serializing_if = "Option::is_none")]
    tiled_size: Option<Point>,
    #[serde(rename = "Color", skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(rename = "Enable")]
    enable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ClimbableComponent {
    #[serde(rename = "BoxOffset")]
    box_offset: Point,
    #[serde(rename = "BoxSize")]
    box_size: Point,
    #[serde(rename = "Enable")]
    enable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct PortalComponent {
    #[serde(rename = "BoxOffset")]
    box_offset: Point,
    #[serde(rename = "BoxSize")]
    box_size: Point,
    #[serde(rename = "IsLegacy")]
    is_legacy: bool,
    #[serde(rename = "PortalEntityRef")]
    portal_entity_ref: PortalEntityRef,
    #[serde(rename = "Enable")]
    enable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct SpawnLocationComponent {
    #[serde(rename = "Enable")]
    enable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct TagComponent {
    #[serde(rename = "Tags")]
    tags: Vec<String>,
    #[serde(rename = "Enable")]
    enable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct PortalEntityRef {
    #[serde(rename = "tempEntityId")]
    temp_entity_id: Option<String>,
    #[serde(rename = "IsRelative")]
    is_relative: bool,
    #[serde(rename = "EntityId")]
    entity_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tile {
    #[serde(rename = "type")]
    tile_type: u32,
    position: Point,
    #[serde(rename = "tileIndex")]
    tile_index: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct TileSetRUID {
    #[serde(rename = "DataId")]
    data_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Transform {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
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

fn main() -> io::Result<()> {
    // Read the JSON file
    let mut file = File::open("assets/proto-000020000.json")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    // Deserialize the JSON data
    let root: Root = serde_json::from_str(&data).expect("Invalid JSON");

    // Serialize the data back to JSON
    let serialized_data = serde_json::to_string_pretty(&root).expect("Serialization failed");

    // Optionally, write the serialized data back to a file
    let mut output_file = File::create("assets/proto-serialized-000020000.json")?;
    output_file.write_all(serialized_data.as_bytes())?;

    Ok(())
}
