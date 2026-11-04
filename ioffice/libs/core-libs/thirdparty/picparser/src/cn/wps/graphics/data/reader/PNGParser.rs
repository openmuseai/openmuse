#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports, unused_assignments, unused_mut)]

// Java: import java.io.IOException
// Java: import java.io.InputStream

use std::io::{self, Read, Seek, SeekFrom};
use std::sync::Mutex;

// Java field: String TAG
pub(crate) const TAG: &str = "PNGParser";
// Java field: int USER_MAX_TRUNK_SIZE
pub(crate) const USER_MAX_TRUNK_SIZE: i32 = 8000000;
// Java field: byte[] iTxt
static I_TXT: &[u8] = &[105, 84, 88, 116];
// Java field: byte[] fileHead
static FILE_HEAD: &[u8] = &[0x89u8, 80, 78, 71, 13, 10, 26, 10];
// Java field: byte[] tempBytes
// Java field: byte[] headerBytes
// Java field: Chunk chunk
static CHUNK: Mutex<Chunk> = Mutex::new(Chunk { length: 0, chunk_type: None });

// Java: public class PNGParser
pub struct PNGParser {
}

// Java: public class Chunk
pub struct Chunk {
    // Java field: int length
    pub(crate) length: i32,
    // Java field: byte[] type
    pub(crate) chunk_type: Option<Vec<u8>>,
}

// Java: public class PNGParser
impl PNGParser {
    // Java: PNGParser()
    pub fn new() -> Self {
        // Java: super();
        PNGParser {}
    }

    // Java: static boolean isUserTrunkDataTooLarge(InputStream is)
    pub fn isUserTrunkDataTooLarge(is: &mut (impl Read + Seek)) -> bool {
        // Java: boolean e;
        let mut e: bool = false;
        let mark_pos = is.stream_position().unwrap_or(0);
        let mut caught = false;
        let mut catch_result: bool = false;
        // Java: try
        {
            let try_result: Result<(), io::Error> = (|| -> Result<(), io::Error> {
                let available = {
                    let current = is.stream_position()?;
                    let end = is.seek(SeekFrom::End(0))?;
                    is.seek(SeekFrom::Start(current))?;
                    (end - current) as i64
                };
                // Java: if (is.available() >= 8000000)
                if available >= 8000000 {
                    // Java: is.mark(is.available());
                    // Java: is.read(headerBytes);
                    let mut header_bytes = [0u8; 8];
                    is.read_exact(&mut header_bytes)?;
                    // Java: while(readChunk(is) != null)
                    while PNGParser::readChunk_impl(is) {
                        let chunk = CHUNK.lock().unwrap();
                        // Java: if (chunk.length > 8000000 && isITXt(chunk))
                        if chunk.length > 8000000 && PNGParser::isITXt(&chunk) {
                            // Java: e = true;
                            e = true;
                            // Java: return e;
                            return Ok(());
                        }
                        let skip_len = chunk.length as i64 + 4;
                        drop(chunk);
                        // Java: is.skip((long)(chunk.length + 4));
                        is.seek(SeekFrom::Current(skip_len))?;
                    }
                    // Java: e = false;
                    e = false;
                    // Java: return e;
                    return Ok(());
                }
                // Java: e = false;
                e = false;
                Ok(())
            })();
            if let Err(_) = try_result {
                caught = true;
            }
        }
        // Java: catch (IOException var14)
        if caught {
            // Java: boolean var2 = false;
            let var2 = false;
            catch_result = var2;
            // Java: return var2;
        }
        // Java: finally
        {
            // Java: try
            {
                // Java: is.reset();
                let _ = is.seek(SeekFrom::Start(mark_pos));
            }
            // Java: catch (IOException var13)
            {
            }
        }
        if caught {
            return catch_result;
        }
        // Java: return e;
        return e;
    }

    // Java: static Chunk readChunk(InputStream is)
    fn readChunk_impl(is: &mut (impl Read + Seek)) -> bool {
        // Java: try
        let result: Result<bool, io::Error> = (|| -> Result<bool, io::Error> {
            // Java: int nRead = is.read(tempBytes);
            let mut temp_bytes = [0u8; 4];
            let n_read = is.read(&mut temp_bytes)?;
            // Java: if (nRead != 4)
            if n_read != 4 {
                // Java: return null;
                return Ok(false);
            }
            // Java: else
            {
                let mut chunk = CHUNK.lock().unwrap();
                // Java: chunk.length = tempBytes[3] & 255 | (tempBytes[2] & 255) << 8 | (tempBytes[1] & 255) << 16 | (tempBytes[0] & 255) << 24;
                chunk.length = (temp_bytes[3] as i32 & 255)
                    | ((temp_bytes[2] as i32 & 255) << 8)
                    | ((temp_bytes[1] as i32 & 255) << 16)
                    | ((temp_bytes[0] as i32 & 255) << 24);
                // Java: chunk.type = new byte[4];
                let mut type_buf = vec![0u8; 4];
                // Java: is.read(chunk.type);
                is.read_exact(&mut type_buf)?;
                chunk.chunk_type = Some(type_buf);
                // Java: return chunk;
                Ok(true)
            }
        })();
        // Java: catch (IOException var2)
        match result {
            Ok(val) => val,
            Err(_) => {
                // Java: return null;
                false
            }
        }
    }

    // Java: static boolean isITXt(Chunk chunk)
    pub fn isITXt(chunk: &Chunk) -> bool {
        // Java: return chunk != null && chunk.type != null && chunk.type[0] == iTxt[0] && chunk.type[1] == iTxt[1] && chunk.type[2] == iTxt[2] && chunk.type[3] == iTxt[3];
        if let Some(ref t) = chunk.chunk_type {
            t.len() >= 4 && t[0] == I_TXT[0] && t[1] == I_TXT[1] && t[2] == I_TXT[2] && t[3] == I_TXT[3]
        } else {
            false
        }
    }
}

// Java: public class Chunk
impl Chunk {
    // Java: Chunk()
    pub fn new() -> Self {
        // Java: super();
        Chunk { length: 0, chunk_type: None }
    }
}
