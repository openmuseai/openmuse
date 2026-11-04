#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments)]

// Java: import cn.wps.base.log.Log
// Java: import java.io.IOException
use std::io;
// Java: import java.io.InputStream
use std::io::{Read, Seek, SeekFrom};

use crate::cn::wps::base::BufferedInputStream::BufferedInputStream;

// Java field: String TAG
pub(crate) const TAG: &str = "PicTypeParser";
// Java field: byte[] bytes
fn get_bytes() -> Vec<u8> { vec![0u8; 16] }

// Java: public class PicTypeParser
pub struct PicTypeParser {
}

// Java: public class PicTypeParser
impl PicTypeParser {
    // Java: PicTypeParser()
    pub fn new() -> Self {
      // Java: super();
      PicTypeParser {}
    }

    // Java: static boolean isEMF(InputStream stream)
    pub fn isEMF(stream: &mut BufferedInputStream) -> io::Result<bool> {
      let mut bytes = get_bytes();
      // Java: stream.mark(152);
      stream.mark(152);
      // Java: stream.read(bytes, 0, 4);
      stream.read_to_buf(&mut bytes, 0, 4)?;
      // Java: int type = (bytes[0] & 255) + ((bytes[1] & 255) << 8) + ((bytes[2] & 255) << 16) + ((bytes[3] & 255) << 24);
      let type_val = (bytes[0] as i32 & 255) + ((bytes[1] as i32 & 255) << 8) + ((bytes[2] as i32 & 255) << 16) + ((bytes[3] as i32 & 255) << 24);
      // Java: if (type != 1)
      if type_val != 1
      {
         // Java: stream.reset();
         let _ = stream.reset();
         // Java: return false;
         return Ok(false);
      }
      // Java: else
      else
      {
         // Java: stream.skip(36L);
         stream.skip(36)?;
         // Java: stream.read(bytes, 0, 4);
         stream.read_to_buf(&mut bytes, 0, 4)?;
         // Java: int signature = (bytes[0] & 255) + ((bytes[1] & 255) << 8) + ((bytes[2] & 255) << 16) + ((bytes[3] & 255) << 24);
         let signature = (bytes[0] as i32 & 255) + ((bytes[1] as i32 & 255) << 8) + ((bytes[2] as i32 & 255) << 16) + ((bytes[3] as i32 & 255) << 24);
         // Java: if (signature != 1179469088)
         if signature != 1179469088
         {
            // Java: stream.reset();
            let _ = stream.reset();
            // Java: return false;
            return Ok(false);
         }
         // Java: else
         else
         {
            // Java: stream.skip(14L);
            stream.skip(14)?;
            // Java: stream.read(bytes, 0, 2);
            stream.read_to_buf(&mut bytes, 0, 2)?;
            // Java: int reserved = (bytes[0] & 255) + ((bytes[1] & 255) << 8);
            let reserved = (bytes[0] as i32 & 255) + ((bytes[1] as i32 & 255) << 8);
            // Java: stream.reset();
            let _ = stream.reset();
            // Java: return reserved == 0;
            return Ok(reserved == 0);
         }
      }
    }

    // Java: static boolean isWMF(InputStream stream)
    pub fn isWMF(stream: &mut BufferedInputStream) -> io::Result<bool> {
      let mut bytes = get_bytes();
      // Java: stream.mark(40);
      stream.mark(40);
      // Java: int Signature = -1698247209;
      let signature_val: i32 = -1698247209;
      // Java: stream.read(bytes, 0, 4);
      stream.read_to_buf(&mut bytes, 0, 4)?;
      // Java: int testKey = (bytes[0] & 255) + ((bytes[1] & 255) << 8) + ((bytes[2] & 255) << 16) + ((bytes[3] & 255) << 24);
      let testKey = (bytes[0] as i32 & 255) + ((bytes[1] as i32 & 255) << 8) + ((bytes[2] as i32 & 255) << 16) + ((bytes[3] as i32 & 255) << 24);
      // Java: if (testKey == Signature)
      if testKey == signature_val
      {
         // Java: stream.skip(18L);
         stream.skip(18)?;
      }
      // Java: else
      else
      {
         // Java: stream.reset();
         let _ = stream.reset();
         // Java: stream.mark(40);
         stream.mark(40);
      }
      // Java: stream.read(bytes, 0, 4);
      stream.read_to_buf(&mut bytes, 0, 4)?;
      // Java: int fileType = (bytes[0] & 255) + ((bytes[1] & 255) << 8);
      let fileType = (bytes[0] as i32 & 255) + ((bytes[1] as i32 & 255) << 8);
      // Java: int headerSize = (bytes[2] & 255) + ((bytes[3] & 255) << 8);
      let headerSize = (bytes[2] as i32 & 255) + ((bytes[3] as i32 & 255) << 8);
      // Java: stream.skip(12L);
      stream.skip(12)?;
      // Java: stream.read(bytes, 0, 2);
      stream.read_to_buf(&mut bytes, 0, 2)?;
      // Java: int numOfParams = (bytes[0] & 255) + ((bytes[1] & 255) << 8);
      let numOfParams = (bytes[0] as i32 & 255) + ((bytes[1] as i32 & 255) << 8);
      // Java: stream.reset();
      let _ = stream.reset();
      // Java: return fileType == 1 && headerSize == 9 && numOfParams == 0;
      Ok(fileType == 1 && headerSize == 9 && numOfParams == 0)
    }

    // Java: static boolean isJPG(InputStream stream)
    pub fn isJPG(stream: &mut BufferedInputStream) -> io::Result<bool> {
      // Java: stream.mark(2);
      stream.mark(2);
      // Java: int byte1 = stream.read();
      let byte1 = stream.read_one()?;
      // Java: int byte2 = stream.read();
      let byte2 = stream.read_one()?;
      // Java: stream.reset();
      let _ = stream.reset();
      // Java: return byte1 == 255 && byte2 == 216;
      Ok(byte1 == 255 && byte2 == 216)
    }

    // Java: static boolean isPNG(InputStream stream)
    pub fn isPNG(stream: &mut BufferedInputStream) -> io::Result<bool> {
      let mut bytes = get_bytes();
      // Java: stream.mark(8);
      stream.mark(8);
      // Java: stream.read(bytes, 0, 8);
      stream.read_to_buf(&mut bytes, 0, 8)?;
      // Java: stream.reset();
      let _ = stream.reset();
      // Java: return (bytes[0] & 255) == 137 && bytes[1] == 80 && bytes[2] == 78 && bytes[3] == 71 && bytes[4] == 13 && bytes[5] == 10 && bytes[6] == 26 && bytes[7] == 10;
      Ok((bytes[0] as i32 & 255) == 137 && bytes[1] == 80 && bytes[2] == 78 && bytes[3] == 71 && bytes[4] == 13 && bytes[5] == 10 && bytes[6] == 26 && bytes[7] == 10)
    }

    // Java: static boolean isGIF(InputStream stream)
    pub fn isGIF(stream: &mut BufferedInputStream) -> io::Result<bool> {
      let mut bytes = get_bytes();
      // Java: stream.mark(6);
      stream.mark(6);
      // Java: stream.read(bytes, 0, 6);
      stream.read_to_buf(&mut bytes, 0, 6)?;
      // Java: stream.reset();
      let _ = stream.reset();
      // Java: return bytes[0] == 71 && bytes[1] == 73 && bytes[2] == 70 && (bytes[4] == 57 || bytes[4] == 55);
      Ok(bytes[0] == 71 && bytes[1] == 73 && bytes[2] == 70 && (bytes[4] == 57 || bytes[4] == 55))
    }

    // Java: static boolean isBMP(InputStream stream)
    pub fn isBMP(stream: &mut BufferedInputStream) -> io::Result<bool> {
      // Java: stream.mark(2);
      stream.mark(2);
      // Java: int byte1 = stream.read();
      let byte1 = stream.read_one()?;
      // Java: int byte2 = stream.read();
      let byte2 = stream.read_one()?;
      // Java: stream.reset();
      let _ = stream.reset();
      // Java: return byte1 == 66 && byte2 == 77;
      Ok(byte1 == 66 && byte2 == 77)
    }

    // Java: static boolean isNormalType(InputStream stream)
    pub fn isNormalType(stream: &mut BufferedInputStream) -> bool {
      // Java: try
      {
         // Java: return isGIF(stream) || isJPG(stream) || isPNG(stream) || isBMP(stream);
         match (|| -> io::Result<bool> {
             Ok(Self::isGIF(stream)? || Self::isJPG(stream)? || Self::isPNG(stream)? || Self::isBMP(stream)?)
         })() {
             Ok(result) => return result,
             Err(_) => {}
         }
      }
      // Java: catch (IOException e)
      {
         // Java: Log.e(TAG, "IOException", e);
         // Java: return false;
         return false;
      }
    }

    // Java: static boolean isTIFF(InputStream stream)
    pub fn isTIFF(stream: &mut BufferedInputStream) -> io::Result<bool> {
      let mut bytes = get_bytes();
      // Java: stream.mark(4);
      stream.mark(4);
      // Java: stream.read(bytes, 0, 4);
      stream.read_to_buf(&mut bytes, 0, 4)?;
      // Java: stream.reset();
      let _ = stream.reset();
      // Java: if (bytes[0] == 73 && bytes[1] == 73 || bytes[0] == 77 && bytes[1] == 77)
      if (bytes[0] == 73 && bytes[1] == 73) || (bytes[0] == 77 && bytes[1] == 77)
      {
         // Java: boolean isSmallEndian = bytes[0] == 73 && bytes[1] == 73;
         let isSmallEndian = bytes[0] == 73 && bytes[1] == 73;
         // Java: if (isSmallEndian)
         if isSmallEndian
         {
            // Java: return bytes[2] == 42 && bytes[3] == 0;
            return Ok(bytes[2] == 42 && bytes[3] == 0);
         }
         // Java: else
         else
         {
            // Java: return bytes[3] == 42 && bytes[2] == 0;
            return Ok(bytes[3] == 42 && bytes[2] == 0);
         }
      }
      // Java: else
      else
      {
         // Java: return false;
         return Ok(false);
      }
    }

    // Java: static boolean isWebp(InputStream stream)
    pub fn isWebp(stream: &mut BufferedInputStream) -> io::Result<bool> {
      let mut bytes = get_bytes();
      // Java: stream.mark(15);
      stream.mark(15);
      // Java: stream.read(bytes, 0, 15);
      stream.read_to_buf(&mut bytes, 0, 15)?;
      // Java: stream.reset();
      let _ = stream.reset();
      // Java: return bytes[0] == 82 && bytes[1] == 73 && bytes[2] == 70 && bytes[3] == 70 && bytes[8] == 87 && bytes[9] == 69 && bytes[10] == 66 && bytes[11] == 80;
      Ok(bytes[0] == 82 && bytes[1] == 73 && bytes[2] == 70 && bytes[3] == 70 && bytes[8] == 87 && bytes[9] == 69 && bytes[10] == 66 && bytes[11] == 80)
    }

    // Java: static boolean isHeic(InputStream stream)
    pub fn isHeic(stream: &mut BufferedInputStream) -> io::Result<bool> {
      let mut bytes = get_bytes();
      // Java: stream.mark(16);
      stream.mark(16);
      // Java: stream.read(bytes, 0, 16);
      stream.read_to_buf(&mut bytes, 0, 16)?;
      // Java: stream.reset();
      let _ = stream.reset();
      // Java: return bytes[4] == 102 && bytes[5] == 116 && bytes[6] == 121 && bytes[7] == 112 && (bytes[8] == 104 && bytes[9] == 101 && bytes[10] == 105 && bytes[11] == 99 || bytes[8] == 104 && bytes[9] == 101 && bytes[10] == 105 && bytes[11] == 115 || bytes[8] == 104 && bytes[9] == 101 && bytes[10] == 105 && bytes[11] == 120 || bytes[8] == 104 && bytes[9] == 101 && bytes[10] == 118 && bytes[11] == 99 || bytes[8] == 104 && bytes[9] == 101 && bytes[10] == 118 && bytes[11] == 120 || bytes[8] == 109 && bytes[9] == 105 && bytes[10] == 102 && bytes[11] == 49 || bytes[8] == 109 && bytes[9] == 115 && bytes[10] == 102 && bytes[11] == 49);
      Ok(bytes[4] == 102 && bytes[5] == 116 && bytes[6] == 121 && bytes[7] == 112
         && ((bytes[8] == 104 && bytes[9] == 101 && bytes[10] == 105 && bytes[11] == 99)
             || (bytes[8] == 104 && bytes[9] == 101 && bytes[10] == 105 && bytes[11] == 115)
             || (bytes[8] == 104 && bytes[9] == 101 && bytes[10] == 105 && bytes[11] == 120)
             || (bytes[8] == 104 && bytes[9] == 101 && bytes[10] == 118 && bytes[11] == 99)
             || (bytes[8] == 104 && bytes[9] == 101 && bytes[10] == 118 && bytes[11] == 120)
             || (bytes[8] == 109 && bytes[9] == 105 && bytes[10] == 102 && bytes[11] == 49)
             || (bytes[8] == 109 && bytes[9] == 115 && bytes[10] == 102 && bytes[11] == 49)))
    }

}
