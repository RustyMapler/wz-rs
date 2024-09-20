extern crate wz;

use image::{DynamicImage, ImageBuffer};
use std::io;
use std::time::Instant;
use wz::{WzFile, WzVersion};

fn main() -> io::Result<()> {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let now = Instant::now();

    // NPC
    // let mut wz_file = WzFile::open("../game/assets/Npc.wz", wz::wzfile::WzVersion::GMS)?;
    // let node = wz_file.resolve("0002008.img/stand/0").unwrap();
    // let image = node.get_image().unwrap();
    // let dynamic_image = ImageBuffer::from_raw(image.width, image.height, image.data)
    //     .map(DynamicImage::ImageRgba8)
    //     .unwrap();
    // dynamic_image.save("npc.png");

    // Loading UI
    let mut wz_file = WzFile::new("../assets/UI.wz", WzVersion::GMS);
    wz_file.open()?;
    // let node = wz_file.resolve("Login.img").unwrap();
    // let node = wz_file.resolve("Logo.img/Loading/repeat/1/0").unwrap();
    let node = wz_file.resolve("Login.img/CharSelect/charInfo1").unwrap();
    let image = node.get_image().unwrap();
    let dynamic_image = ImageBuffer::from_raw(image.width, image.height, image.data)
        .map(DynamicImage::ImageRgba8)
        .unwrap();
    dynamic_image.save("char info.png");

    // let mut wz_file = WzFile::open("../game/assets/Map.wz", wz::wzfile::WzVersion::GMS_OLD)?;

    // let mut wz_file = WzFile::open("../game/assets/Map002.wz", wz::wzfile::WzVersion::GMS)?;

    // wz_file.parse_wz_main_directory()?;

    // let root = wz_file.root.unwrap();
    // dbg!(root.list_children());

    // let node = wz_file_set.resolve("Map").unwrap();
    // dbg!(node.list_children());

    // let mut wz_file_set = WzFileSet::from_paths(
    //     vec![
    //         "../game/assets/Map.wz".to_string(),
    //         "../game/assets/Map001.wz".to_string(),
    //         "../game/assets/Map002.wz".to_string(),
    //         "../game/assets/Map2.wz".to_string(),
    //     ],
    //     wz::wzfile::WzVersion::GMS,
    // )?;

    // // Save portal image
    // let node = wz_file_set
    //     .resolve("MapHelper.img/portal/game/pv/default/0")
    //     .unwrap();
    // let image = node.get_image().unwrap();
    // dbg!(image.width);
    // dbg!(image.height);
    // let dynamic_image = ImageBuffer::from_raw(image.width, image.height, image.data)
    //     .map(DynamicImage::ImageRgba8)
    //     .unwrap();
    // dynamic_image.save("portal0.png");

    // dbg!(node.list_children());
    // for (child_name, child) in node.get_children() {
    //     dbg!(child.list_children());
    //     let image = child.get_image().unwrap();
    //     let dynamic_image = ImageBuffer::from_raw(image.width, image.height, image.data)
    //         .map(DynamicImage::ImageRgba8)
    //         .unwrap();
    //     dynamic_image.save(format!("image{}.png", child_name));

    // }
    // let image = node.get_image().unwrap();
    // let dynamic_image = ImageBuffer::from_raw(image.width, image.height, image.data)
    //     .map(DynamicImage::ImageRgba8)
    //     .unwrap();
    // dynamic_image.save("image.png");

    // let node = wz_file_set.resolve("Tile/woodMarble.img/bsc/0").unwrap();
    // dbg!(node.list_children());

    // dbg!(node.get_child("_outlink").unwrap().get_string().unwrap());

    // let node = wz_file_set
    //     .resolve("Map/Map0/000010000.img/0/tile")
    //     .unwrap();
    // dbg!(node.list_children());
    // let node = root.get_child(&root.list_children()[0]);
    // if node.is_some() {
    //     dbg!(node.unwrap().list_children());
    // }
    // let node = root.get_child("Obj").unwrap();
    // dbg!(node.list_children());

    // let obj_node = wz_file.resolve("Obj").unwrap();
    // dbg!(obj_node.list_children());

    // let tile_path = "Map/Map0/000010000.img/0/tile";
    // let node = wz_file.resolve(tile_path).unwrap();
    // for tile in node.get_children() {
    //     let full_path = format!("{}/{}", tile_path, tile.get_name());
    //     println!("{}", &full_path);
    //     let tile_placement = TilePlacement {
    //         x: tile.get_child("x").unwrap().get_int().unwrap(),
    //         y: tile.get_child("y").unwrap().get_int().unwrap(),
    //         u: tile
    //             .get_child("u")
    //             .unwrap()
    //             .get_string()
    //             .unwrap()
    //             .to_string(),
    //         no: tile.get_child("no").unwrap().get_int().unwrap(),
    //         zM: tile.get_child("zM").unwrap().get_int().unwrap(),
    //     };
    //     dbg!(tile_placement);
    // }

    // let node = wz_file.resolve("Map/Map0/000010000.img/info/bgm").unwrap();
    // log::info!("bgm name {}", node.get_string());

    // let mut sound_wz_file = WzFile::open("../game/assets/Sound.wz")?;
    // let sound_node = sound_wz_file.resolve("Bgm04.img/WhiteChristmas2").unwrap();
    // println!("{}", sound_node.get_name());
    // let sound_bytes = sound_node.get_sound().unwrap();

    // unsafe {
    //     let mut wav = audio::Wav::default();
    //     wav.load_raw_wav_8(&sound_bytes).unwrap();
    //     sl.play(&wav);
    //     while sl.voice_count() > 0 {
    //         std::thread::sleep(std::time::Duration::from_millis(100));
    //     }
    // }
    // let node = wz_file_set.resolve("Tile/grassySoil.img/enV0/2").unwrap();
    // dbg!(node.list_children());
    // // let node = wz_file.resolve("Tile/grassySoil.img/bsc/0").unwrap();
    // let image = node.get_image().unwrap();
    // let dynamic_image = ImageBuffer::from_raw(image.width, image.height, image.data)
    //     .map(DynamicImage::ImageRgba8)
    //     .unwrap();
    // dynamic_image.save("image.png");

    // let node = wz_file.resolve("Map/Map0/000010000.img/back").unwrap();
    // log::debug!("node name {}", node.get_name());
    // dbg!(node.list_children());
    // let child_node = node.get_child_mut("0").unwrap();
    // dbg!(child_node.list_children());

    // let bs = child_node
    //     .get_child("bS")
    //     .unwrap()
    //     .get_string()
    //     .unwrap()
    //     .clone();
    // let inst_no = child_node
    //     .get_child("no")
    //     .unwrap()
    //     .get_int()
    //     .unwrap()
    //     .clone();

    // let back_str = format!("Back/{}.img/ani/{}", bs, inst_no);
    // log::debug!("{}", back_str);
    // let back_node = wz_file.resolve(&back_str).unwrap();
    // dbg!(back_node.list_children());

    // let resolved_path = resolve_uol_path("00012000.img", "../../front/head");

    let elapsed = now.elapsed();
    log::info!("Elapsed: {:.2?}", elapsed);

    Ok(())
}

#[cfg(test)]
mod main_tests {
    use wz::{WzFileSet, WzVersion};

    #[test]
    fn wzfileset() {
        let mut file_set = WzFileSet::from_paths(
            vec![
                "../assets/Map.wz".to_string(),
                "../assets/Map001.wz".to_string(),
                "../assets/Map002.wz".to_string(),
                "../assets/Map2.wz".to_string(),
            ],
            WzVersion::GMS,
        );

        file_set.open();

        let node = file_set.resolve("Map/Map0/000010000.img/info").unwrap();
        let children = node.list_children();
        assert_eq!(children.len(), 24);
    }
}
