pub mod base64_util {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    pub fn encode(data: &[u8]) -> String {
        let mut result = String::new();
        let chunks = data.chunks(3);
        for chunk in chunks {
            let b0 = chunk[0] as u32;
            let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
            let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
            let triple = (b0 << 16) | (b1 << 8) | b2;
            result.push(ALPHABET[((triple >> 18) & 0x3F) as usize] as char);
            result.push(ALPHABET[((triple >> 12) & 0x3F) as usize] as char);
            if chunk.len() > 1 {
                result.push(ALPHABET[((triple >> 6) & 0x3F) as usize] as char);
            } else {
                result.push('=');
            }
            if chunk.len() > 2 {
                result.push(ALPHABET[(triple & 0x3F) as usize] as char);
            } else {
                result.push('=');
            }
        }
        result
    }

    pub fn decode(encoded: &str) -> Vec<u8> {
        let mut lookup = [255u8; 256];
        for (i, &c) in ALPHABET.iter().enumerate() {
            lookup[c as usize] = i as u8;
        }
        let mut result = Vec::new();
        let bytes: Vec<u8> = encoded.bytes().filter(|&b| b != b'\n' && b != b'\r' && b != b' ').collect();
        for chunk in bytes.chunks(4) {
            if chunk.len() < 2 { break; }
            let a = lookup[chunk[0] as usize] as u32;
            let b = lookup[chunk[1] as usize] as u32;
            let c = if chunk.len() > 2 && chunk[2] != b'=' { lookup[chunk[2] as usize] as u32 } else { 0 };
            let d = if chunk.len() > 3 && chunk[3] != b'=' { lookup[chunk[3] as usize] as u32 } else { 0 };
            let triple = (a << 18) | (b << 12) | (c << 6) | d;
            result.push(((triple >> 16) & 0xFF) as u8);
            if chunk.len() > 2 && chunk[2] != b'=' {
                result.push(((triple >> 8) & 0xFF) as u8);
            }
            if chunk.len() > 3 && chunk[3] != b'=' {
                result.push((triple & 0xFF) as u8);
            }
        }
        result
    }
}

pub mod type_conversion {
    pub fn string2Integer(s: Option<&str>) -> Option<i32> {
        s.and_then(|v| v.trim().parse::<i32>().ok())
    }
}
