// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

fn le_get_short(buf: &[u8], offset: usize) -> i16 {
    ((buf[offset] as i16) & 0xFF) | (((buf[offset+1] as i16) & 0xFF) << 8)
}
fn le_get_ushort(buf: &[u8], offset: usize) -> i32 {
    ((buf[offset] as i32) & 0xFF) | (((buf[offset+1] as i32) & 0xFF) << 8)
}
fn le_put_short(buf: &mut [u8], offset: usize, value: i16) {
    buf[offset] = (value & 0xFF) as u8;
    buf[offset+1] = ((value >> 8) & 0xFF) as u8;
}

// Java field: int MAGICCREATED
pub(crate) const MAGICCREATED: i32 = 0;
// Java field: int MAGICREVISED
pub(crate) const MAGICREVISED: i32 = 1;
// Java field: int MAGICCREATEDPRIVATE
pub(crate) const MAGICCREATEDPRIVATE: i32 = 2;
// Java field: int MAGICREVISEDPRIVATE
pub(crate) const MAGICREVISEDPRIVATE: i32 = 3;
// Java field: int LIDFE
pub(crate) const LIDFE: i32 = 13;
// Java field: int START
pub(crate) const START: i32 = 32;

// Java: public class FIBShortHandler
pub struct FIBShortHandler {
    // Java field: int[] _shorts
    pub(crate) _shorts: Vec<i32>,
}

// Java: public class FIBShortHandler
impl FIBShortHandler {
    // Java: FIBShortHandler(byte[] mainStream)
    pub fn new_from_bytes(main_stream: &[u8]) -> Self {
      // Java: super();
      // Java: int offset = 32;
      let mut offset: usize = 32;
      // Java: int shortCount = LittleEndian.getShort(mainStream, offset);
      let short_count = le_get_short(main_stream, offset) as i32;
      // Java: offset += 2;
      offset += 2;
      // Java: this._shorts = new int[shortCount];
      let mut shorts = vec![0i32; short_count as usize];
      // Java: for(int x = 0; x < shortCount; ++x)
      for x in 0..short_count as usize {
         // Java: this._shorts[x] = LittleEndian.getUShort(mainStream, offset);
         shorts[x] = le_get_ushort(main_stream, offset);
         // Java: offset += 2;
         offset += 2;
      }
      FIBShortHandler { _shorts: shorts }
    }

    // Java: FIBShortHandler(DocumentInputStream mainStream)
    pub fn new_from_stream(main_stream: &mut DocumentInputStream) -> Self {
      // Java: super();
      // Java: int offset = 32;
      let mut offset: i32 = 32;
      // Java: mainStream.seek((long)offset);
      main_stream.seek(offset as i64);
      // Java: int shortCount = mainStream.readUShort();
      let short_count = main_stream.readUShort();
      // Java: offset += 2;
      offset += 2;
      // Java: this._shorts = new int[shortCount];
      let mut shorts = vec![0i32; short_count as usize];
      // Java: for(int x = 0; x < shortCount; ++x)
      for x in 0..short_count as usize {
         // Java: this._shorts[x] = mainStream.readShort();
         shorts[x] = main_stream.readShort() as i32;
         // Java: offset += 2;
         offset += 2;
      }
      FIBShortHandler { _shorts: shorts }
    }

    // Java: int getShort(int shortCode)
    pub fn getShort(&self, short_code: i32) -> i32 {
      // Java: return this._shorts[shortCode];
      self._shorts[short_code as usize]
    }

    // Java: void setShort(int shortCode, short value)
    pub fn setShort(&mut self, short_code: i32, value: i16) {
      // Java: this._shorts[shortCode] = value;
      self._shorts[short_code as usize] = value as i32;
    }

    // Java: int sizeInBytes()
    pub fn sizeInBytes(&self) -> i32 {
      // Java: return this._shorts.length * 2 + 2;
      (self._shorts.len() as i32) * 2 + 2
    }

    // Java: void serialize(byte[] mainStream)
    pub fn serialize(&self, main_stream: &mut [u8]) {
      // Java: int offset = 32;
      let mut offset: usize = 32;
      // Java: LittleEndian.putShort(mainStream, offset, (short)this._shorts.length);
      le_put_short(main_stream, offset, self._shorts.len() as i16);
      // Java: offset += 2;
      offset += 2;
      // Java: for(int x = 0; x < this._shorts.length; ++x)
      for x in 0..self._shorts.len() {
         // Java: LittleEndian.putShort(mainStream, offset, (short)this._shorts[x]);
         le_put_short(main_stream, offset, self._shorts[x] as i16);
         // Java: offset += 2;
         offset += 2;
      }
    }

    // Java: int getShortLength()
    pub fn getShortLength(&self) -> i32 {
      // Java: return this._shorts.length;
      self._shorts.len() as i32
    }

}
