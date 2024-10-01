use squish::Format;

pub fn decompress_image_bgra4444_to_rgba8888(data: &[u8], width: u32, height: u32) -> Vec<u8> {
    fn extract_lower_bits(bits: u8) -> u8 {
        let byte = bits & 0x0F;
        (byte << 4) | byte
    }

    fn extract_upper_bits(bits: u8) -> u8 {
        let byte = bits >> 4;
        (byte << 4) | byte
    }

    let pixel_count = (width * height) as usize;
    let mut result: Vec<u8> = Vec::with_capacity(pixel_count * 4);
    result.resize(pixel_count * 4, 0);

    for i in 0..pixel_count {
        let index = i * 2;
        let b = extract_lower_bits(data[index]);
        let g = extract_upper_bits(data[index]);
        let r = extract_lower_bits(data[index + 1]);
        let a = extract_upper_bits(data[index + 1]);

        let output_index = i * 4;
        result[output_index] = r;
        result[output_index + 1] = g;
        result[output_index + 2] = b;
        result[output_index + 3] = a;
    }

    result
}

pub fn convert_image_bgra8888_to_rgba8888(data: Vec<u8>) -> Vec<u8> {
    let mut result = vec![0u8; data.len()];

    for i in (0..data.len()).step_by(4) {
        result[i] = data[i + 2]; // Red
        result[i + 1] = data[i + 1]; // Green
        result[i + 2] = data[i]; // Blue
        result[i + 3] = data[i + 3]; // Alpha
    }

    result
}

pub fn decompress_image_dxt5_to_rgba8888(data: &[u8], width: u32, height: u32) -> Vec<u8> {
    let mut result = vec![0u8; (4 * width * height) as usize];

    Format::Bc3.decompress(data, width as usize, height as usize, &mut result);

    result
}

pub fn decompress_image_bgr565_to_rgba8888(data: &[u8], width: u32, height: u32) -> Vec<u8> {
    let pixel_count = (width * height) as usize;
    let mut result = vec![0u8; pixel_count * 4];

    for i in 0..pixel_count {
        let index = i * 2;
        let b0 = data[index];
        let b1 = data[index + 1];

        // BGR565 -> 16 bits:
        let bgr565 = u16::from_le_bytes([b0, b1]);

        // Extract individual components
        let blue = ((bgr565 & 0x1F) << 3) as u8; // 5 bits -> shift to 8-bit range
        let green = (((bgr565 >> 5) & 0x3F) << 2) as u8; // 6 bits -> shift to 8-bit range
        let red = (((bgr565 >> 11) & 0x1F) << 3) as u8; // 5 bits -> shift to 8-bit range

        // Set RGBA values in the output buffer
        let output_index = i * 4;
        result[output_index] = red;
        result[output_index + 1] = green;
        result[output_index + 2] = blue;
        result[output_index + 3] = 255; // Alpha set to fully opaque
    }

    result
}
