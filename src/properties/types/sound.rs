use crate::WzReader;
use std::{
    fmt,
    fs::File,
    io::{BufWriter, Error, Write},
};

const WAV_HEADER_SIZE: usize = 44;
const PCM_SUBCHUNK_SIZE: usize = 16;

#[derive(Default, Debug, Clone)]
pub struct WzSound {
    pub name: String,
    pub duration: u32,
    pub header_offset: u64,
    pub header_size: usize,
    pub buffer_offset: u64,
    pub buffer_size: usize,
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

pub fn parse_sound_header(sound: &WzSound, reader: &WzReader) -> Result<Vec<u8>, Error> {
    let current_position = reader.get_position()?;
    reader.seek(sound.header_offset)?;
    let header_bytes = reader.read_bytes(sound.header_size as u64)?;
    reader.seek(current_position)?;
    Ok(header_bytes)
}

pub fn parse_sound_buffer(sound: &WzSound, reader: &WzReader) -> Result<Vec<u8>, Error> {
    let current_position = reader.get_position()?;
    reader.seek(sound.buffer_offset)?;
    let buffer_bytes = reader.read_bytes(sound.buffer_size as u64)?;
    reader.seek(current_position)?;
    Ok(buffer_bytes)
}

pub fn save_sound(path: &str, sound: &WzSound, reader: &WzReader) -> std::io::Result<()> {
    let sound_header = parse_sound_header(sound, reader)?;
    let sound_buffer = parse_sound_buffer(sound, reader)?;

    let sound_type = match sound_header.len() {
        0x46 => "wav",
        _ => "mp3",
    };

    let file_path = format!("{}/{}.{}", path, sound.name, sound_type);
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    match sound_type {
        "wav" => {
            // Ensure the header has enough data for the PCM subchunk
            if sound_header.len() < 0x34 + PCM_SUBCHUNK_SIZE {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid WAV header data",
                ));
            }

            let sound_format: [u8; PCM_SUBCHUNK_SIZE] = sound_header
                [0x34..0x34 + PCM_SUBCHUNK_SIZE]
                .try_into()
                .map_err(|_| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Failed to parse WAV format",
                    )
                })?;

            let wav_header = create_wav_header(sound.buffer_size, &sound_format);

            writer.write_all(&wav_header)?;
            writer.write_all(&sound_buffer)?;
        }
        "mp3" => {
            // Directly write the MP3 data
            writer.write_all(&sound_buffer)?;
        }
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Unsupported sound format",
            ));
        }
    }

    Ok(())
}

// Helper function to create a WAV header
fn create_wav_header(
    buffer_size: usize,
    sound_format: &[u8; PCM_SUBCHUNK_SIZE],
) -> [u8; WAV_HEADER_SIZE] {
    //https://docs.fileformat.com/audio/wav/
    const WAV_HEADER_TEMPLATE: [u8; WAV_HEADER_SIZE] = [
        // RIFF chunk descriptor
        0x52, 0x49, 0x46, 0x46, // "RIFF" in ASCII
        0, 0, 0, 0, // Chunk size (file size - 8 bytes)
        0x57, 0x41, 0x56, 0x45, // "WAVE" in ASCII
        // fmt sub-chunk
        0x66, 0x6d, 0x74, 0x20, // "fmt " in ASCII
        0x10, 0, 0, 0, // Sub-chunk size (16 for PCM)
        0, 0, // Audio format (1 = PCM, other values indicate compression)
        0, 0, // Number of channels (1 for mono, 2 for stereo, etc.)
        0, 0, 0, 0, // Sample rate (e.g., 44100 Hz)
        0, 0, 0, 0, // Byte rate (sample rate * num channels * bits per sample / 8)
        0, 0, // Block align (num channels * bits per sample / 8)
        0, 0, // Bits per sample (e.g., 16 bits)
        // data sub-chunk
        0x64, 0x61, 0x74, 0x61, // "data" in ASCII
        0, 0, 0, 0, // Sub-chunk 2 size (data size)
    ];

    let riff_chunk_size = (buffer_size + 36).to_le_bytes();
    let data_chunk_size = buffer_size.to_le_bytes();

    let mut wav_header = WAV_HEADER_TEMPLATE;

    // Fill chunk sizes
    wav_header[4..8].copy_from_slice(&riff_chunk_size);
    wav_header[40..44].copy_from_slice(&data_chunk_size);
    wav_header[20..36].copy_from_slice(sound_format);

    wav_header
}
