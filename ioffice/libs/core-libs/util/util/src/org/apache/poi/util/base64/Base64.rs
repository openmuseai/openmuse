#![allow(non_snake_case, dead_code, unused_variables)]

// Java: import java.io.ByteArrayInputStream
// Java: import java.io.ByteArrayOutputStream
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream

const ENCODE_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn decode_char(c: u8) -> u8 {
    match c {
        b'A'..=b'Z' => c - b'A',
        b'a'..=b'z' => c - b'a' + 26,
        b'0'..=b'9' => c - b'0' + 52,
        b'+' => 62,
        b'/' => 63,
        _ => 0,
    }
}

// Java: public class Base64
pub struct Base64 {
}

// Java: public class Base64
impl Base64 {
    // Java: Base64()
    pub fn new() -> Self {
        // Java: super();
        Self {}
    }

    // Java: static _encode: &[u8](_bytes: &[u8])
    pub fn encode(bytes: &[u8]) -> Vec<u8> {
        // Java: return encode(bytes, 0);
        Self::encode_with_wrap(bytes, 0)
    }

    // Java: static _encode: &[u8](_bytes: &[u8], int wrapAt)
    pub fn encode_with_wrap(bytes: &[u8], wrap_at: i32) -> Vec<u8> {
        // Java: ByteArrayInputStream inputStream = new ByteArrayInputStream(bytes);
        // Java: ByteArrayOutputStream outputStream = new ByteArrayOutputStream();
        let mut output = Vec::new();
        let mut line_len = 0i32;

        // Java: try
        {
            // Java: encode(inputStream, outputStream, wrapAt);
            let mut i = 0;
            while i + 2 < bytes.len() {
                let b0 = bytes[i] as u32;
                let b1 = bytes[i + 1] as u32;
                let b2 = bytes[i + 2] as u32;
                output.push(ENCODE_TABLE[((b0 >> 2) & 0x3F) as usize]);
                output.push(ENCODE_TABLE[(((b0 << 4) | (b1 >> 4)) & 0x3F) as usize]);
                output.push(ENCODE_TABLE[(((b1 << 2) | (b2 >> 6)) & 0x3F) as usize]);
                output.push(ENCODE_TABLE[(b2 & 0x3F) as usize]);
                line_len += 4;
                if wrap_at > 0 && line_len >= wrap_at {
                    output.push(b'\r');
                    output.push(b'\n');
                    line_len = 0;
                }
                i += 3;
            }

            let remaining = bytes.len() - i;
            if remaining == 1 {
                let b0 = bytes[i] as u32;
                output.push(ENCODE_TABLE[((b0 >> 2) & 0x3F) as usize]);
                output.push(ENCODE_TABLE[((b0 << 4) & 0x3F) as usize]);
                output.push(b'=');
                output.push(b'=');
            } else if remaining == 2 {
                let b0 = bytes[i] as u32;
                let b1 = bytes[i + 1] as u32;
                output.push(ENCODE_TABLE[((b0 >> 2) & 0x3F) as usize]);
                output.push(ENCODE_TABLE[(((b0 << 4) | (b1 >> 4)) & 0x3F) as usize]);
                output.push(ENCODE_TABLE[((b1 << 2) & 0x3F) as usize]);
                output.push(b'=');
            }
        }
        // Java: catch (IOException e)
        // (Rust does not have checked exceptions; handled inline)
        {
            // Java: throw new RuntimeException("Unexpected I/O error", e);
        }
        // Java: finally
        {
            // Java: try
            {
                // Java: inputStream.close();
            }
            // Java: catch (Throwable var14)
            {
            }
            // Java: try
            {
                // Java: outputStream.close();
            }
            // Java: catch (Throwable var13)
            {
            }
        }
        // Java: return outputStream.toByteArray();
        output
    }

    // Java: static void encode(InputStream inputStream, OutputStream outputStream)
    pub fn encode_stream(input: &[u8], output: &mut Vec<u8>) {
        // Java: encode(inputStream, outputStream, 0);
        *output = Self::encode_with_wrap(input, 0);
    }

    // Java: static void encode(InputStream inputStream, OutputStream outputStream, int wrapAt)
    pub fn encode_stream_wrap(input: &[u8], output: &mut Vec<u8>, wrap_at: i32) {
        // Java: Base64OutputStream aux = new Base64OutputStream(outputStream, wrapAt);
        // Java: copy(inputStream, aux);
        // Java: aux.commit();
        *output = Self::encode_with_wrap(input, wrap_at);
    }

    // Java: static _decode: &[u8](_bytes: &[u8])
    pub fn decode(bytes: &[u8]) -> Vec<u8> {
        // Java: ByteArrayInputStream inputStream = new ByteArrayInputStream(bytes);
        // Java: ByteArrayOutputStream outputStream = new ByteArrayOutputStream();
        let filtered: Vec<u8> = bytes.iter()
            .copied()
            .filter(|&b| b != b'\r' && b != b'\n' && b != b' ')
            .collect();

        let mut output = Vec::new();
        // Java: try
        {
            // Java: decode(inputStream, outputStream);
            let mut i = 0;
            while i + 3 < filtered.len() {
                let c0 = decode_char(filtered[i]) as u32;
                let c1 = decode_char(filtered[i + 1]) as u32;
                let c2 = decode_char(filtered[i + 2]) as u32;
                let c3 = decode_char(filtered[i + 3]) as u32;

                output.push(((c0 << 2) | (c1 >> 4)) as u8);
                if filtered[i + 2] != b'=' {
                    output.push((((c1 << 4) | (c2 >> 2)) & 0xFF) as u8);
                }
                if filtered[i + 3] != b'=' {
                    output.push((((c2 << 6) | c3) & 0xFF) as u8);
                }
                i += 4;
            }
        }
        // Java: catch (IOException e)
        {
            // Java: throw new RuntimeException("Unexpected I/O error", e);
        }
        // Java: finally
        {
            // Java: try
            {
                // Java: inputStream.close();
            }
            // Java: catch (Throwable var13)
            {
            }
            // Java: try
            {
                // Java: outputStream.close();
            }
            // Java: catch (Throwable var12)
            {
            }
        }
        // Java: return outputStream.toByteArray();
        output
    }

    // Java: static void decode(InputStream inputStream, OutputStream outputStream)
    pub fn decode_stream(input: &[u8], output: &mut Vec<u8>) {
        // Java: copy(new Base64InputStream(inputStream), outputStream);
        *output = Self::decode(input);
    }

    // Java: static void copy(InputStream inputStream, OutputStream outputStream)
    fn copy(input: &[u8], output: &mut Vec<u8>) {
        // Java: _b: &[u8] = new byte[10240];
        // Java: int len;
        // Java: while((len = inputStream.read(b)) != -1)
        // Java: outputStream.write(b, 0, len);
        output.extend_from_slice(input);
    }
}
