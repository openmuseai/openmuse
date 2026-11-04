#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import java.io.PushbackInputStream

use std::io::{Read, Write};
use std::sync::OnceLock;

// Java field: _pem_array: &[char]
const PEM_ARRAY: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

// Java field: _pem_convert_array: &[u8]
fn pem_convert_array() -> &'static [i8; 256] {
    static ARRAY: OnceLock<[i8; 256]> = OnceLock::new();
    ARRAY.get_or_init(|| {
        let mut arr = [-1i8; 256];
        for (i, &ch) in PEM_ARRAY.iter().enumerate() {
            arr[ch as usize] = i as i8;
        }
        arr
    })
}

// Java: public class BASE64Decoder extends CharacterDecoder
pub struct BASE64Decoder {
    // Java field: _decode_buffer: &[u8]
    decode_buffer: [u8; 4],
}

// Java: public class BASE64Decoder extends CharacterDecoder
impl BASE64Decoder {
    // Java: BASE64Decoder()
    pub fn new() -> Self {
        // Java: super();
        Self {
            decode_buffer: [0u8; 4],
        }
    }

    // Java: int bytesPerAtom()
    pub fn bytesPerAtom(&self) -> i32 {
        // Java: return 4;
        4
    }

    // Java: int bytesPerLine()
    pub fn bytesPerLine(&self) -> i32 {
        // Java: return 72;
        72
    }

    // Java: void decodeAtom(PushbackInputStream inStream, OutputStream outStream, int rem)
    pub fn decodeAtom(&mut self, instream: &mut dyn Read, outstream: &mut dyn Write, rem: i32) {
        // Java: byte a = -1;
        let mut a: i8 = -1;
        // Java: byte b = -1;
        let mut b: i8 = -1;
        // Java: byte c = -1;
        let mut c: i8 = -1;
        // Java: byte d = -1;
        let mut d: i8 = -1;
        // Java: if (rem < 2)
        if rem < 2
        {
            // Java: throw new IOException("BASE64Decoder: Not enough bytes for an atom.");
            panic!("BASE64Decoder: Not enough bytes for an atom.");
        }
        // Java: else
        {
            // Java: int i;
            let mut i_val: i32;
            // Java: do
            loop
            {
                // Java: i = inStream.read();
                let mut single = [0u8; 1];
                let n = in_stream.read(&mut single).unwrap_or(0);
                i_val = if n == 0 { -1 } else { single[0] as i32 };
                // Java: if (i == -1)
                if i_val == -1
                {
                    // Java: throw new IOException();
                    panic!("IOException");
                }
                if i_val != 10 && i_val != 13 { break; }
            }
            // Java: this.decode_buffer[0] = (byte)i;
            self.decode_buffer[0] = i_val as u8;
            // Java: i = this.readFully(inStream, this.decode_buffer, 1, rem - 1);
            let read_count = self.readFully(in_stream, 1, (rem - 1) as usize);
            // Java: if (i == -1)
            if read_count == -1
            {
                // Java: throw new IOException();
                panic!("IOException");
            }
            // Java: else
            {
                let pca = pem_convert_array();
                // Java: if (rem > 3 && this.decode_buffer[3] == 61)
                if rem > 3 && self.decode_buffer[3] == 61
                {
                    // Java: rem = 3;
                    rem = 3;
                }
                // Java: if (rem > 2 && this.decode_buffer[2] == 61)
                if rem > 2 && self.decode_buffer[2] == 61
                {
                    // Java: rem = 2;
                    rem = 2;
                }
                // Java: switch (rem)
                // Java: case 4:
                if rem >= 4 {
                    // Java: d = pem_convert_array[this.decode_buffer[3] & 255];
                    d = pca[(self.decode_buffer[3] & 255) as usize];
                }
                // Java: case 3:
                if rem >= 3 {
                    // Java: c = pem_convert_array[this.decode_buffer[2] & 255];
                    c = pca[(self.decode_buffer[2] & 255) as usize];
                }
                // Java: case 2:
                if rem >= 2 {
                    // Java: b = pem_convert_array[this.decode_buffer[1] & 255];
                    b = pca[(self.decode_buffer[1] & 255) as usize];
                    // Java: a = pem_convert_array[this.decode_buffer[0] & 255];
                    a = pca[(self.decode_buffer[0] & 255) as usize];
                }
                // Java: default:
                // Java: switch (rem)
                match rem {
                    // Java: case 2:
                    2 => {
                        // Java: outStream.write((byte)(a << 2 & 252 | b >>> 4 & 3));
                        let _ = out_stream.write(&[((a as i32) << 2 & 252 | ((b as u8) >> 4) as i32 & 3) as u8]);
                        // Java: break;
                    }
                    // Java: case 3:
                    3 => {
                        // Java: outStream.write((byte)(a << 2 & 252 | b >>> 4 & 3));
                        let _ = out_stream.write(&[((a as i32) << 2 & 252 | ((b as u8) >> 4) as i32 & 3) as u8]);
                        // Java: outStream.write((byte)(b << 4 & 240 | c >>> 2 & 15));
                        let _ = out_stream.write(&[((b as i32) << 4 & 240 | ((c as u8) >> 2) as i32 & 15) as u8]);
                        // Java: break;
                    }
                    // Java: case 4:
                    4 => {
                        // Java: outStream.write((byte)(a << 2 & 252 | b >>> 4 & 3));
                        let _ = out_stream.write(&[((a as i32) << 2 & 252 | ((b as u8) >> 4) as i32 & 3) as u8]);
                        // Java: outStream.write((byte)(b << 4 & 240 | c >>> 2 & 15));
                        let _ = out_stream.write(&[((b as i32) << 4 & 240 | ((c as u8) >> 2) as i32 & 15) as u8]);
                        // Java: outStream.write((byte)(c << 6 & 192 | d & 63));
                        let _ = out_stream.write(&[((c as i32) << 6 & 192 | (d as i32) & 63) as u8]);
                    }
                    _ => {}
                }
            }
        }
    }

    fn readFully(&mut self, input: &mut dyn Read, offset: usize, len: usize) -> i32 {
        for i in 0..len {
            let mut single = [0u8; 1];
            let n = input.read(&mut single).unwrap_or(0);
            if n == 0 {
                return if i == 0 { -1 } else { i as i32 };
            }
            self.decode_buffer[i + offset] = single[0];
        }
        len as i32
    }
}
