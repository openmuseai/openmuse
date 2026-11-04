#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_parens)]
use super::Memory::ByteOrder;

// Java: public class ModifiedUtf8
#[derive(Default)]
pub struct ModifiedUtf8 {}

// Java: public class ModifiedUtf8
impl ModifiedUtf8 {
    // Java: public static String decode(byte[] in, char[] out, int offset, int utfSize) throws java.io.UTFDataFormatException
    pub fn decode(r#in: &[u8], out: &mut [u16], offset: i32, utfSize: i32) -> String {
        // Java: int count = 0;
        let mut count: usize = 0;
        // Java: int s = 0;
        let mut s: usize = 0;
        let off = offset as usize;
        let utf_size = utfSize as usize;
        // Java: while(count < utfSize)
        while count < utf_size {
            // Java: if ((out[s] = (char)in[offset + count++]) >= 128)
            out[s] = r#in[off + count] as u16;
            count += 1;
            if out[s] >= 128 {
                // Java: int a;
                // Java: if (((a = out[s]) & 224) != 192)
                let a = out[s] as i32;
                if (a & 224) != 192 {
                    // Java: if ((a & 240) != 224)
                    if (a & 240) != 224 {
                        // Java: throw new java.io.UTFDataFormatException("bad byte at " + (count - 1));
                        panic!("UTFDataFormatException: bad byte at {}", count - 1);
                    }
                    // Java: if (count + 1 >= utfSize)
                    if count + 1 >= utf_size {
                        // Java: throw new java.io.UTFDataFormatException("bad third byte at " + (count + 1));
                        panic!("UTFDataFormatException: bad third byte at {}", count + 1);
                    }
                    // Java: int b = in[offset + count++];
                    let b = r#in[off + count] as i32; count += 1;
                    // Java: int c = in[offset + count++];
                    let c = r#in[off + count] as i32; count += 1;
                    // Java: if ((b & 192) != 128 || (c & 192) != 128)
                    if (b & 192) != 128 || (c & 192) != 128 {
                        // Java: throw new java.io.UTFDataFormatException("bad second or third byte at " + (count - 2));
                        panic!("UTFDataFormatException: bad second or third byte at {}", count - 2);
                    }
                    // Java: out[s++] = (char)((a & 15) << 12 | (b & 63) << 6 | c & 63);
                    out[s] = (((a & 15) << 12) | ((b & 63) << 6) | (c & 63)) as u16;
                    s += 1;
                }
                // Java: else
                else {
                    // Java: if (count >= utfSize)
                    if count >= utf_size {
                        // Java: throw new java.io.UTFDataFormatException("bad second byte at " + count);
                        panic!("UTFDataFormatException: bad second byte at {}", count);
                    }
                    // Java: int b = in[offset + count++];
                    let b = r#in[off + count] as i32; count += 1;
                    // Java: if ((b & 192) != 128)
                    if (b & 192) != 128 {
                        // Java: throw new java.io.UTFDataFormatException("bad second byte at " + (count - 1));
                        panic!("UTFDataFormatException: bad second byte at {}", count - 1);
                    }
                    // Java: out[s++] = (char)((a & 31) << 6 | b & 63);
                    out[s] = (((a & 31) << 6) | (b & 63)) as u16;
                    s += 1;
                }
            }
            // Java: else
            else {
                // Java: ++s;
                s += 1;
            }
        }
        // Java: return new String(out, 0, s);
        String::from_utf16_lossy(&out[..s])
    }

    // Java: public static long countBytes(String s, boolean shortLength) throws java.io.UTFDataFormatException
    pub fn countBytes(s: &str, shortLength: bool) -> i64 {
        // Java: long result = 0L;
        let mut result: i64 = 0;
        // Java: int length = s.length();
        let chars: Vec<u16> = s.encode_utf16().collect();
        let length = chars.len();
        // Java: for(int i = 0; i < length; ++i)
        for i in 0..length {
            // Java: char ch = s.charAt(i);
            let ch = chars[i];
            // Java: if (ch != 0 && ch <= 127)
            if ch != 0 && ch <= 127 {
                // Java: ++result;
                result += 1;
            }
            // Java: else if (ch <= 2047)
            else if ch <= 2047 {
                // Java: result += 2L;
                result += 2;
            }
            // Java: else
            else {
                // Java: result += 3L;
                result += 3;
            }
            // Java: if (shortLength && result > 65535L)
            if shortLength && result > 65535 {
                // Java: throw new java.io.UTFDataFormatException("String more than 65535 UTF bytes long");
                panic!("UTFDataFormatException: String more than 65535 UTF bytes long");
            }
        }
        // Java: return result;
        result
    }

    // Java: public static void encode(byte[] dst, int offset, String s)
    pub fn encode(dst: &mut [u8], offset: i32, s: &str) {
        // Java: int length = s.length();
        let chars: Vec<u16> = s.encode_utf16().collect();
        let length = chars.len();
        let mut off = offset as usize;
        // Java: for(int i = 0; i < length; ++i)
        for i in 0..length {
            // Java: char ch = s.charAt(i);
            let ch = chars[i];
            // Java: if (ch != 0 && ch <= 127)
            if ch != 0 && ch <= 127 {
                // Java: dst[offset++] = (byte)ch;
                dst[off] = ch as u8; off += 1;
            }
            // Java: else if (ch <= 2047)
            else if ch <= 2047 {
                // Java: dst[offset++] = (byte)(192 | 31 & ch >> 6);
                dst[off] = (192 | (31 & (ch >> 6))) as u8; off += 1;
                // Java: dst[offset++] = (byte)(128 | 63 & ch);
                dst[off] = (128 | (63 & ch)) as u8; off += 1;
            }
            // Java: else
            else {
                // Java: dst[offset++] = (byte)(224 | 15 & ch >> 12);
                dst[off] = (224 | (15 & (ch >> 12))) as u8; off += 1;
                // Java: dst[offset++] = (byte)(128 | 63 & ch >> 6);
                dst[off] = (128 | (63 & (ch >> 6))) as u8; off += 1;
                // Java: dst[offset++] = (byte)(128 | 63 & ch);
                dst[off] = (128 | (63 & ch)) as u8; off += 1;
            }
        }
    }

    // Java: public static byte[] encode(String s) throws java.io.UTFDataFormatException
    pub fn encode__string(s: &str) -> Vec<u8> {
        // Java: int utfCount = (int)countBytes(s, true);
        let utfCount = Self::countBytes(s, true) as usize;
        // Java: byte[] result = new byte[2 + utfCount];
        let mut result = vec![0u8; 2 + utfCount];
        // Java: Memory.pokeShort(result, 0, (short)utfCount, ByteOrder.BIG_ENDIAN);
        super::Memory::Memory::pokeShort(&mut result, 0, utfCount as i16, ByteOrder::BIG_ENDIAN);
        // Java: encode(result, 2, s);
        Self::encode(&mut result, 2, s);
        // Java: return result;
        result
    }

    // Java: private ModifiedUtf8()
    pub fn new() -> ModifiedUtf8 { Self {} }
}
