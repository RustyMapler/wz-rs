use std::{
    fmt,
    fs::File,
    io::{BufWriter, Write},
};

#[derive(Default, Debug, Clone)]
pub struct WzSound {
    pub name: String,
    pub duration: u32,
    pub header_offset: u64,
    pub header_data: Vec<u8>,
    pub header_size: u32,
    pub sound_data_offset: u64,
    pub sound_data: Vec<u8>,
    pub sound_size: u32,
}

impl WzSound {
    pub const SOUND_HEADER: [u8; 51] = [
        0x02, 0x83, 0xEB, 0x36, 0xE4, 0x4F, 0x52, 0xCE, 0x11, 0x9F, 0x53, 0x00, 0x20, 0xAF, 0x0B,
        0xA7, 0x70, 0x8B, 0xEB, 0x36, 0xE4, 0x4F, 0x52, 0xCE, 0x11, 0x9F, 0x53, 0x00, 0x20, 0xAF,
        0x0B, 0xA7, 0x70, 0x00, 0x01, 0x81, 0x9F, 0x58, 0x05, 0x56, 0xC3, 0xCE, 0x11, 0xBF, 0x01,
        0x00, 0xAA, 0x00, 0x55, 0x59, 0x5A,
    ];
}

impl fmt::Display for WzSound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WzSound(sound_duration: {}, header_offset: {}, header_size: {}, sound_data_offset: {}, sound_size: {})",
            self.duration, self.header_offset, self.header_size, self.sound_data_offset, self.sound_size
        )
    }
}

pub fn save_sound(sound: &WzSound) -> std::io::Result<()> {
    let sound_type = if sound.header_data.len() == 0x46 {
        "wav"
    } else {
        "mp3"
    };

    let file_path = format!("assets/sound/{}.{}", sound.name, sound_type);
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    match sound_type {
        "wav" => {
            const WAV_HEADER: [u8; 44] = [
                0x52, 0x49, 0x46, 0x46, // "RIFF"
                0, 0, 0, 0, // ChunkSize
                0x57, 0x41, 0x56, 0x45, // "WAVE"
                0x66, 0x6d, 0x74, 0x20, // "fmt"
                0x10, 0, 0, 0, // chunk1Size
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // copy16char
                0x64, 0x61, 0x74, 0x61, // "data"
                0, 0, 0, 0, // chunk2Size
            ];

            let u8_16_from_header = &sound.header_data[0x34..0x34 + 16];
            let chunk1_size = (sound.sound_size + 36).to_le_bytes();
            let chunk2_size = sound.sound_size.to_le_bytes();

            let mut wav_header = WAV_HEADER.to_vec();

            wav_header[4..8].copy_from_slice(&chunk1_size);
            wav_header[20..36].copy_from_slice(u8_16_from_header);
            wav_header[40..44].copy_from_slice(&chunk2_size);

            writer.write_all(&wav_header)?;
            writer.write_all(&sound.sound_data)?;
        }
        "mp3" => {
            writer.write_all(&sound.sound_data)?;
        }
        _ => {}
    }

    Ok(())
}
