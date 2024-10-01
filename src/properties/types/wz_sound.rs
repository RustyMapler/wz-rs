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
    pub buffer_offset: u64,
    pub buffer: Vec<u8>,
    pub buffer_size: u32,
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
            "WzSound(name: {}, sound_duration: {}, header_offset: {}, header_size: {}, sound_data_offset: {}, sound_size: {})",
            self.name, self.duration, self.header_offset, self.header_size, self.buffer_offset, self.buffer_size
        )
    }
}

pub fn save_sound(path: &str, sound: &WzSound) -> std::io::Result<()> {
    let sound_type = if sound.header_data.len() == 0x46 {
        "wav"
    } else {
        "mp3"
    };

    let file_path = format!("{}/{}.{}", path, sound.name, sound_type);
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    match sound_type {
        "wav" => {
            //https://docs.fileformat.com/audio/wav/
            const WAV_HEADER_TEMPLATE: [u8; 44] = [
                // RIFF chunk descriptor
                0x52, 0x49, 0x46, 0x46, // "RIFF" in ASCII
                0, 0, 0, 0, // Chunk size (file size - 8 bytes)
                0x57, 0x41, 0x56, 0x45, // "WAVE" in ASCII
                // fmt sub-chunk
                0x66, 0x6d, 0x74, 0x20, // "fmt " in ASCII
                0x10, 0, 0, 0, // Sub-chunk size (16 for PCM)
                0, 0, // Audio format (1 = PCM, other values indicate compression)
                0, 0, // Number of channels (1 for mono, 2 for stereo, etc.)
                0, 0, 0, 0, // Sample rate (e.g., 44100 Hz), to be filled later
                0, 0, 0,
                0, // Byte rate (sample rate * num channels * bits per sample / 8), to be filled later
                0, 0, // Block align (num channels * bits per sample / 8), to be filled later
                0, 0, // Bits per sample (e.g., 16 bits), to be filled later
                // data sub-chunk
                0x64, 0x61, 0x74, 0x61, // "data" in ASCII
                // Sub-chunk 2 size (data size = num samples * num channels * bits per sample / 8)
                0, 0, 0, 0,
            ];

            let sound_format = &sound.header_data[0x34..0x34 + 16];
            let riff_chunk_size = (sound.buffer_size + 36).to_le_bytes();
            let data_chunk_size = sound.buffer_size.to_le_bytes();

            let mut wav_header = WAV_HEADER_TEMPLATE.to_vec();

            wav_header[4..8].copy_from_slice(&riff_chunk_size);
            wav_header[20..36].copy_from_slice(sound_format);
            wav_header[40..44].copy_from_slice(&data_chunk_size);

            writer.write_all(&wav_header)?;
            writer.write_all(&sound.buffer)?;
        }
        "mp3" => {
            writer.write_all(&sound.buffer)?;
        }
        _ => {}
    }

    Ok(())
}
