use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use uuid::Uuid;

// the below are WZ structs
#[derive(Serialize, Deserialize, Debug)]
struct MapData {
    #[serde(rename = "1")]
    layer_one: Layer,
}

#[derive(Serialize, Deserialize, Debug)]
struct Layer {
    tile: HashMap<String, TileSub>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TileSub {
    x: IntValue,
    y: IntValue,
    no: IntValue,
    u: StringValue,
    zM: IntValue,
}

#[derive(Serialize, Deserialize, Debug)]
struct IntValue {
    _dirName: String,
    _dirType: String,
    _value: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct StringValue {
    _dirName: String,
    _dirType: String,
    _value: String,
}

// break point the below are MSW structs

#[derive(Serialize, Deserialize, Default, Debug)]
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

#[derive(Serialize, Deserialize, Default, Debug)]
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

#[derive(Serialize, Deserialize, Default, Debug)]
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
    #[serde(rename = "modelId")]
    model_id: Option<String>,
    #[serde(rename = "@components")]
    components: Vec<Component>,
    #[serde(rename = "@version")]
    version: u32,
    enable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<Origin>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "@type")]
enum Component {
    #[serde(rename = "MOD.Core.BackgroundComponent")]
    BackgroundComponent(BackgroundComponent),
    #[serde(rename = "MOD.Core.ClimbableComponent")]
    ClimbableComponent(ClimbableComponent),
    #[serde(rename = "MOD.Core.ClimbableSpriteRendererComponent")]
    ClimbableSpriteRendererComponent(ClimbableSpriteRendererComponent),
    #[serde(rename = "MOD.Core.FootholdComponent")]
    FootholdComponent(FootholdComponent),
    #[serde(rename = "MOD.Core.MapComponent")]
    MapComponent(MapComponent),
    #[serde(rename = "MOD.Core.MapLayerComponent")]
    MapLayerComponent(MapLayerComponent),
    #[serde(rename = "MOD.Core.PortalComponent")]
    PortalComponent(PortalComponent),
    #[serde(rename = "MOD.Core.SpawnLocationComponent")]
    SpawnLocationComponent(SpawnLocationComponent),
    #[serde(rename = "MOD.Core.SpriteRendererComponent")]
    SpriteRendererComponent(SpriteRendererComponent),
    #[serde(rename = "MOD.Core.TagComponent")]
    TagComponent(TagComponent),
    #[serde(rename = "MOD.Core.TileMapComponent")]
    TileMapComponent(TileMapComponent),
    #[serde(rename = "MOD.Core.TransformComponent")]
    TransformComponent(TransformComponent),
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
struct BackgroundComponent {
    #[serde(rename = "Enable")]
    enable: bool,
    // #[serde(rename = "SolidColor")]
    // solid_color: Color,
    #[serde(rename = "TemplateRUID")]
    template_ruid: String,
    #[serde(rename = "Type")]
    background_type: u32,
    // #[serde(rename = "WebUrl")]
    // web_url: String,
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
struct ClimbableSpriteRendererComponent {
    #[serde(rename = "BodySpriteFlipInfo")]
    body_sprite_flip_info: Point,
    #[serde(rename = "BodySpriteLocalPos")]
    body_sprite_local_pos: Point,
    #[serde(rename = "Color", skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(rename = "Enable")]
    enable: bool,
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
}

#[derive(Serialize, Deserialize, Debug)]
struct FootholdComponent {
    #[serde(rename = "Enable")]
    enable: bool,
    #[serde(rename = "FootholdsByLayer")]
    footholds_by_layer: HashMap<String, Vec<Foothold>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MapComponent {
    // #[serde(rename = "AirAccelerationXFactor")]
    // air_acceleration_x_factor: f32,
    // #[serde(rename = "AirDecelerationXFactor")]
    // air_deceleration_x_factor: f32,
    #[serde(rename = "Enable")]
    enable: bool,
    // #[serde(rename = "FallSpeedMaxXFactor")]
    // fall_speed_max_x_factor: f32,
    // #[serde(rename = "FallSpeedMaxYFactor")]
    // fall_speed_max_y_factor: f32,
    // #[serde(rename = "Gravity")]
    // gravity: f32,
    #[serde(rename = "IsInstanceMap")]
    is_instance_map: bool,
    #[serde(rename = "TileMapMode")]
    tile_map_mode: u32,
    // #[serde(rename = "WalkAccelerationFactor")]
    // walk_acceleration_factor: f32,
    // #[serde(rename = "WalkDrag")]
    // walk_drag: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct MapLayerComponent {
    #[serde(rename = "Enable")]
    enable: bool,
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
}

#[derive(Serialize, Deserialize, Debug)]
struct PortalComponent {
    #[serde(rename = "BoxOffset")]
    box_offset: Point,
    #[serde(rename = "BoxSize")]
    box_size: Point,
    #[serde(rename = "Enable")]
    enable: bool,
    #[serde(rename = "IsLegacy")]
    is_legacy: bool,
    #[serde(rename = "PortalEntityRef")]
    portal_entity_ref: PortalEntityRef,
}

#[derive(Serialize, Deserialize, Debug)]
struct SpawnLocationComponent {
    #[serde(rename = "Enable")]
    enable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct SpriteRendererComponent {
    #[serde(rename = "ActionSheet")]
    action_sheet: HashMap<String, String>,
    #[serde(rename = "Color", skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(rename = "DrawMode", skip_serializing_if = "Option::is_none")]
    draw_mode: Option<u32>,
    #[serde(rename = "Enable")]
    enable: bool,
    #[serde(rename = "EndFrameIndex")]
    end_frame_index: i32,
    #[serde(rename = "FlipX", skip_serializing_if = "Option::is_none")]
    flip_x: Option<bool>,
    #[serde(rename = "FlipY", skip_serializing_if = "Option::is_none")]
    flip_y: Option<bool>,
    #[serde(
        rename = "IgnoreMapLayerCheck",
        skip_serializing_if = "Option::is_none"
    )]
    ignore_map_layer_check: Option<bool>,
    #[serde(rename = "OrderInLayer", skip_serializing_if = "Option::is_none")]
    order_in_layer: Option<u32>,
    #[serde(rename = "PlayRate", skip_serializing_if = "Option::is_none")]
    play_rate: Option<f32>,
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
}

#[derive(Serialize, Deserialize, Debug)]
struct TagComponent {
    #[serde(rename = "Enable")]
    enable: bool,
    #[serde(rename = "Tags")]
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct TileMapComponent {
    #[serde(rename = "Color", skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(rename = "Enable")]
    enable: bool,
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
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct TransformComponent {
    #[serde(rename = "Enable")]
    enable: bool,
    #[serde(rename = "Position", skip_serializing_if = "Option::is_none")]
    position: Option<Transform>,
    #[serde(rename = "QuaternionRotation", skip_serializing_if = "Option::is_none")]
    quaternion_rotation: Option<Quaternion>,
    #[serde(rename = "Rotation", skip_serializing_if = "Option::is_none")]
    rotation: Option<Transform>,
    #[serde(rename = "Scale", skip_serializing_if = "Option::is_none")]
    scale: Option<Transform>,
    #[serde(rename = "ZRotation", skip_serializing_if = "Option::is_none")]
    z_rotation: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Color {
    a: f32,
    b: f32,
    g: f32,
    r: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Foothold {
    #[serde(rename = "EndPoint")]
    end_point: Point,
    #[serde(rename = "Id")]
    id: u32,
    #[serde(rename = "IsDynamic")]
    is_dynamic: bool,
    #[serde(rename = "Length")]
    length: f32,
    #[serde(rename = "NextFootholdId")]
    next_foothold_id: u32,
    #[serde(rename = "OwnerId")]
    owner_id: String,
    #[serde(rename = "PreviousFootholdId")]
    previous_foothold_id: u32,
    #[serde(rename = "StartPoint")]
    start_point: Point,
    #[serde(rename = "Variance")]
    variance: Point,
    #[serde(rename = "attribute")]
    attribute: FootholdAttribute,
    #[serde(rename = "groupID")]
    group_id: u32,
    #[serde(rename = "layer")]
    layer: u32,
    #[serde(rename = "sortingLayerName")]
    sorting_layer_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FootholdAttribute {
    #[serde(rename = "drag")]
    drag: f32,
    #[serde(rename = "force")]
    force: f32,
    #[serde(rename = "inertiaOption")]
    inertia_option: u32,
    #[serde(rename = "isBlockVertical")]
    is_block_vertical: bool,
    #[serde(rename = "isCustomFoothold")]
    is_custom_foothold: bool,
    #[serde(rename = "isDynamic")]
    is_dynamic: bool,
    #[serde(rename = "walk")]
    walk: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Origin {
    entry_id: String,
    sub_entity_id: Option<String>,
    #[serde(rename = "type")]
    origin_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct PortalEntityRef {
    #[serde(rename = "EntityId")]
    entity_id: String,
    #[serde(rename = "IsRelative")]
    is_relative: bool,
    #[serde(rename = "tempEntityId")]
    temp_entity_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tile {
    position: Point,
    #[serde(rename = "tileIndex")]
    tile_index: i32,
    #[serde(rename = "type")]
    tile_type: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct TileSetRUID {
    #[serde(rename = "DataId")]
    data_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Transform {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Quaternion {
    w: f32,
    x: f32,
    y: f32,
    z: f32,
}

fn main() -> io::Result<()> {
    let map_id = "000040001";

    // Read the JSON file
    let mut file = File::open(format!("{}.img.json", map_id))?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    // Deserialize the JSON data
    let map_data: MapData = serde_json::from_str(&data).expect("Invalid JSON");

    fn get_tile_type(tile: &str) -> u32 {
        match tile {
            "enH0" => 9, // horizontal top
            "enH1" => 7, // horizontal bottom
            "edU" => 11, // circlar top
            "edD" => 0,  // circular bottom, probably want to skip
            "enV1" => 8, // vertical left
            "enV0" => 8, // vertical right & left
            "bsc" => 5,  // middle
            "slLU" => 1, // slope left up
            "slRU" => 3, // slope right up
            "slLD" => 2, // slope left down
            "slRD" => 4, // slope right down
            _ => 0,
        }
    }

    fn get_tile_width(tile: &str) -> u32 {
        match tile {
            "enH0" => 90, // horizontal top
            "enH1" => 90, // horizontal bottom
            "edU" => 54,  // circlar top
            "edD" => 48,  // circular bottom, probably want to skip
            "enV1" => 29, // vertical left
            "enV0" => 29, // vertical right & left
            "bsc" => 90,  // middle
            "slLU" => 90, // slope left up
            "slRU" => 90, // slope right up
            "slLD" => 90, // slope left down
            "slRD" => 90, // slope right down
            _ => {
                println!("Unknown tile: {}", tile);
                0
            }
        }
    }

    fn get_tile_height(tile: &str) -> u32 {
        match tile {
            "enH0" => 38, // horizontal top
            "enH1" => 26, // horizontal bottom
            "edU" => 38,  // circlar top
            "edD" => 17,  // circular bottom, probably want to skip
            "enV1" => 60, // vertical left
            "enV0" => 60, // vertical right & left
            "bsc" => 60,  // middle
            "slLU" => 94, // slope left up
            "slRU" => 94, // slope right up
            "slLD" => 77, // slope left down
            "slRD" => 77, // slope right down
            _ => {
                println!("Unknown tile: {}", tile);
                0
            }
        }
    }

    let layer_one_tiles = map_data
        .layer_one
        .tile
        .iter()
        .map(|(_, tile)| {
            // if tile type is bsc, then we need to skip it
            // if get_tile_type(&tile.u._value) == 5
            //     || get_tile_type(&tile.u._value) == 0
            //     || get_tile_type(&tile.u._value) == 7
            // {
            //     return None;
            // }

            // println!("----");
            // println!("({},{})", tile.x._value, tile.y._value);
            // println!(
            //     "{} / {} = {}",
            //     tile.x._value,
            //     45,
            //     (tile.x._value / 45 as i32) as f32
            // );

            // println!(
            //     "{} / {} = {}",
            //     tile.y._value,
            //     30,
            //     -(tile.y._value / 30 as i32) as f32
            // );

            Some(Tile {
                position: Point {
                    x: (tile.x._value / 45 as i32) as f32,
                    y: -(tile.y._value / 30 as i32) as f32,
                },
                tile_index: 0,
                tile_type: get_tile_type(&tile.u._value),
            })
        })
        // filter out none
        .filter_map(|tile| tile)
        .collect();

    let msw_map = Root {
        entry_key: format!("map://{}", map_id),
        content_type: "x-mod/map".to_string(),
        use_publish: 1,
        core_version: "1.21.0.0".to_string(),
        studio_version: "0.1.0.0".to_string(),
        content_proto: ContentProto {
            use_type: "Binary".to_string(),
            entities: vec![
                Entity {
                    id: Uuid::new_v4().to_string(),
                    path: format!("/maps/{}", map_id),
                    component_names: "MOD.Core.MapComponent".to_string(),
                    json_string: JsonString {
                        name: map_id.to_string(),
                        path: format!("/maps/{}", map_id),
                        name_editable: true,
                        enable: true,
                        visible: true,
                        display_order: 0,
                        path_constraints: "//".to_string(),
                        revision: 1,
                        version: 1,
                        components: vec![Component::MapComponent(MapComponent {
                            is_instance_map: false,
                            tile_map_mode: 0,
                            enable: true,
                        })],
                        ..Default::default()
                    },
                },
                Entity {
                    id: Uuid::new_v4().to_string(),
                    path: format!("/maps/{}/Background", map_id),
                    component_names: "MOD.Core.BackgroundComponent".to_string(),
                    json_string: JsonString {
                        name: "Background".to_string(),
                        path: format!("/maps/{}/Background", map_id),
                        name_editable: false,
                        enable: true,
                        visible: true,
                        display_order: 0,
                        path_constraints: "///".to_string(),
                        revision: 1,
                        version: 1,
                        components: vec![Component::BackgroundComponent(BackgroundComponent {
                            enable: true,
                            template_ruid: "9cbbc80343ed406388d581f45b4861fb".to_string(),
                            background_type: 1,
                        })],
                        ..Default::default()
                    },
                },
                Entity {
                    id: Uuid::new_v4().to_string(),
                    path: format!("/maps/{}/MapleMapLayer", map_id),
                    component_names: "MOD.Core.MapLayerComponent".to_string(),
                    json_string: JsonString {
                        name: "MapleMapLayer".to_string(),
                        path: format!("/maps/{}/Background", map_id),
                        name_editable: false,
                        enable: true,
                        visible: true,
                        display_order: 1,
                        path_constraints: "///".to_string(),
                        revision: 1,
                        version: 1,
                        origin: Some(Origin {
                            entry_id: "maplemaplayer".to_string(),
                            sub_entity_id: None,
                            origin_type: "Model".to_string(),
                        }),
                        components: vec![Component::MapLayerComponent(MapLayerComponent {
                            enable: true,
                            is_visible: true,
                            layer_sort_order: 0,
                            locked: false,
                            map_layer_name: "Layer1".to_string(),
                            thumbnail: "".to_string(),
                        })],
                        ..Default::default()
                    },
                },
                Entity {
                    id: Uuid::new_v4().to_string(),
                    path: format!("/maps/{}/TileMap", map_id),
                    component_names: "MOD.Core.TransformComponent,MOD.Core.TileMapComponent"
                        .to_string(),
                    json_string: JsonString {
                        name: "TileMap".to_string(),
                        path: format!("/maps/{}/TileMap", map_id),
                        name_editable: false,
                        enable: true,
                        visible: true,
                        display_order: 2,
                        path_constraints: "///".to_string(),
                        revision: 1,
                        version: 1,
                        origin: Some(Origin {
                            origin_type: "Model".to_string(),
                            entry_id: "tilemap".to_string(),
                            sub_entity_id: None,
                        }),
                        components: vec![
                            Component::TransformComponent(TransformComponent {
                                enable: true,
                                position: Some(Transform {
                                    x: -0.225,
                                    y: -0.15,
                                    z: 1000.0,
                                }),
                                quaternion_rotation: Some(Quaternion {
                                    w: 1.0,
                                    x: 0.0,
                                    y: 0.0,
                                    z: 0.0,
                                }),
                                scale: Some(Transform {
                                    x: 1.0,
                                    y: 1.0,
                                    z: 1.0,
                                }),
                                ..Default::default()
                            }),
                            Component::TileMapComponent(TileMapComponent {
                                enable: true,
                                is_odd_grid_position: false,
                                sorting_layer: "MapLayer0".to_string(),
                                tile_map_version: 1,
                                tile_set_ruid: TileSetRUID {
                                    data_id: "46701ff2021b4d1fb21fbf5790b1ab14".to_string(),
                                },
                                tiles: layer_one_tiles,
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    },
                },
            ],
        },
        ..Default::default()
    };

    // Serialize the data back to JSON
    let serialized_data = serde_json::to_string_pretty(&msw_map).expect("Serialization failed");

    // Optionally, write the serialized data back to a file
    let mut output_file = File::create(format!("{}.map", map_id))?;
    output_file.write_all(serialized_data.as_bytes())?;

    Ok(())
}
