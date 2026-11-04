#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_parens)]

use std::io;

use crate::kfs::zip::ZipErrors;

// Java: public class ModifiedUtf8 
pub struct ModifiedUtf8 {}

// Java: public class ModifiedUtf8 
impl ModifiedUtf8 {
    // Java: public static String decode(byte[] in, char[] out, int offset, int utfSize) throws UTFDataFormatException 
    pub fn decode(r#in: &[u8], out: &mut [u16], offset: usize, utfSize: usize) -> String {
        Self::try_decode(r#in, out, offset, utfSize)
            .unwrap_or_else(|e| ZipErrors::panicLikeJava(&e))
    }

    // Rust-native: decode 的可恢复版本。
    // Java 的 UTFDataFormatException extends IOException —— 坏编码的文件名是可恢复的数据错误，
    // 不该让整个进程 panic（调用方 `catch (IOException e)` 会跳过这个 entry）。
    pub fn try_decode(
        r#in: &[u8],
        out: &mut [u16],
        offset: usize,
        utfSize: usize,
    ) -> io::Result<String> {
        let mut count: usize = 0;
        let mut s: usize = 0;
        // Java: while(count < utfSize)
        while count < utfSize {
            let b = r#in[offset + count] as u16;
            count += 1;
            out[s] = b;
            // Java: if ((out[s] = (char)in[offset + count++]) >= 128)
            if b >= 128 {
                // Java: int a;
                let a = out[s] as i32;
                // Java: if (((a = out[s]) & 224) != 192)
                if (a & 224) != 192 {
                    // Java: if ((a & 240) != 224)
                    if (a & 240) != 224 {
                        // Java: throw new UTFDataFormatException("bad byte at " + (count - 1));
                        return Err(ZipErrors::utfDataFormatException(format!(
                            "bad byte at {}",
                            count - 1
                        )));
                    }
                    // Java: if (count + 1 >= utfSize)
                    if count + 1 >= utfSize {
                        // Java: throw new UTFDataFormatException("bad third byte at " + (count + 1));
                        return Err(ZipErrors::utfDataFormatException(format!(
                            "bad third byte at {}",
                            count + 1
                        )));
                    }
                    // Java: int b = in[offset + count++];
                    let b2 = r#in[offset + count] as i32;
                    count += 1;
                    // Java: int c = in[offset + count++];
                    let c = r#in[offset + count] as i32;
                    count += 1;
                    // Java: if ((b & 192) != 128 || (c & 192) != 128)
                    if (b2 & 192) != 128 || (c & 192) != 128 {
                        // Java: throw new UTFDataFormatException("bad second or third byte at " + (count - 2));
                        return Err(ZipErrors::utfDataFormatException(format!(
                            "bad second or third byte at {}",
                            count - 2
                        )));
                    }
                    // Java: out[s++] = (char)((a & 15) << 12 | (b & 63) << 6 | c & 63);
                    out[s] = (((a & 15) << 12) | ((b2 & 63) << 6) | (c & 63)) as u16;
                    s += 1;
                } else {
                    // Java: if (count >= utfSize)
                    if count >= utfSize {
                        // Java: throw new UTFDataFormatException("bad second byte at " + count);
                        return Err(ZipErrors::utfDataFormatException(format!(
                            "bad second byte at {}",
                            count
                        )));
                    }
                    // Java: int b = in[offset + count++];
                    let b2 = r#in[offset + count] as i32;
                    count += 1;
                    // Java: if ((b & 192) != 128)
                    if (b2 & 192) != 128 {
                        // Java: throw new UTFDataFormatException("bad second byte at " + (count - 1));
                        return Err(ZipErrors::utfDataFormatException(format!(
                            "bad second byte at {}",
                            count - 1
                        )));
                    }
                    // Java: out[s++] = (char)((a & 31) << 6 | b & 63);
                    out[s] = (((a & 31) << 6) | (b2 & 63)) as u16;
                    s += 1;
                }
            } else {
                // Java: ++s;
                s += 1;
            }
        }
        // Java: return new String(out, 0, s);
        Ok(String::from_utf16_lossy(&out[..s]))
    }

    // Java: public static long countBytes(String s, boolean shortLength) throws UTFDataFormatException 
    pub fn countBytes(s: &str, shortLength: bool) -> i64 {
        // Java: long result = 0L;
        let mut result: i64 = 0;
        // Java: int length = s.length();
        // Java: for(int i = 0; i < length; ++i)
        for ch in s.encode_utf16() {
            // Java: if (ch != 0 && ch <= 127)
            if ch != 0 && ch <= 127 {
                // Java: ++result;
                result += 1;
            }
            // Java: else if (ch <= 2047)
            else if ch <= 2047 {
                // Java: result += 2L;
                result += 2;
            } else {
                // Java: result += 3L;
                result += 3;
            }
            // Java: if (shortLength && result > 65535L)
            if shortLength && result > 65535 {
                // Java: throw new UTFDataFormatException("String more than 65535 UTF bytes long");
                panic!("UTFDataFormatException: String more than 65535 UTF bytes long");
            }
        }
        // Java: return result;
        result
    }

    // Java: public static void encode(byte[] dst, int offset, String s) 
    pub fn encode(dst: &mut [u8], mut offset: usize, s: &str) {
        // Java: int length = s.length();
        // Java: for(int i = 0; i < length; ++i)
        for ch in s.encode_utf16() {
            // Java: if (ch != 0 && ch <= 127)
            if ch != 0 && ch <= 127 {
                // Java: dst[offset++] = (byte)ch;
                dst[offset] = ch as u8;
                offset += 1;
            }
            // Java: else if (ch <= 2047)
            else if ch <= 2047 {
                // Java: dst[offset++] = (byte)(192 | 31 & ch >> 6);
                dst[offset] = (192 | ((ch >> 6) & 31)) as u8;
                offset += 1;
                // Java: dst[offset++] = (byte)(128 | 63 & ch);
                dst[offset] = (128 | (ch & 63)) as u8;
                offset += 1;
            } else {
                // Java: dst[offset++] = (byte)(224 | 15 & ch >> 12);
                dst[offset] = (224 | ((ch >> 12) & 15)) as u8;
                offset += 1;
                // Java: dst[offset++] = (byte)(128 | 63 & ch >> 6);
                dst[offset] = (128 | ((ch >> 6) & 63)) as u8;
                offset += 1;
                // Java: dst[offset++] = (byte)(128 | 63 & ch);
                dst[offset] = (128 | (ch & 63)) as u8;
                offset += 1;
            }
        }
    }

    // Java: public static byte[] encode(String s) throws UTFDataFormatException 
    pub fn encode__string(s: &str) -> Vec<u8> {
        // Java: int utfCount = (int)countBytes(s, true);
        let utfCount = Self::countBytes(s, true) as usize;
        // Java: byte[] result = new byte[2 + utfCount];
        let mut result = vec![0u8; 2 + utfCount];
        // Java: Memory.pokeShort(result, 0, (short)utfCount, ByteOrder.BIG_ENDIAN);
        result[0] = ((utfCount >> 8) & 0xFF) as u8;
        result[1] = (utfCount & 0xFF) as u8;
        // Java: encode(result, 2, s);
        Self::encode(&mut result, 2, s);
        // Java: return result;
        result
    }
}
