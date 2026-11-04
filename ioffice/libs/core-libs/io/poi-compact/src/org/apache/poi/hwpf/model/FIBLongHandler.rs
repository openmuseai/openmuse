// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

fn le_get_short(buf: &[u8], offset: usize) -> i16 {
    ((buf[offset] as i16) & 0xFF) | (((buf[offset+1] as i16) & 0xFF) << 8)
}
fn le_get_int(buf: &[u8], offset: usize) -> i32 {
    (buf[offset] as i32 & 0xFF) | ((buf[offset+1] as i32 & 0xFF) << 8) |
    ((buf[offset+2] as i32 & 0xFF) << 16) | ((buf[offset+3] as i32 & 0xFF) << 24)
}
fn le_put_short(buf: &mut [u8], offset: usize, value: i16) {
    buf[offset] = (value & 0xFF) as u8;
    buf[offset+1] = ((value >> 8) & 0xFF) as u8;
}
fn le_put_int(buf: &mut [u8], offset: usize, value: i32) {
    buf[offset] = (value & 0xFF) as u8;
    buf[offset+1] = ((value >> 8) & 0xFF) as u8;
    buf[offset+2] = ((value >> 16) & 0xFF) as u8;
    buf[offset+3] = ((value >> 24) & 0xFF) as u8;
}

// Java field: int CBMAC
pub(crate) const CBMAC: i32 = 0;
// Java field: int PRODUCTCREATED
pub(crate) const PRODUCTCREATED: i32 = 1;
// Java field: int PRODUCTREVISED
pub(crate) const PRODUCTREVISED: i32 = 2;
// Java field: int CCPTEXT
pub(crate) const CCPTEXT: i32 = 3;
// Java field: int CCPFTN
pub(crate) const CCPFTN: i32 = 4;
// Java field: int CCPHDD
pub(crate) const CCPHDD: i32 = 5;
// Java field: int CCPMCR
pub(crate) const CCPMCR: i32 = 6;
// Java field: int CCPATN
pub(crate) const CCPATN: i32 = 7;
// Java field: int CCPEDN
pub(crate) const CCPEDN: i32 = 8;
// Java field: int CCPTXBX
pub(crate) const CCPTXBX: i32 = 9;
// Java field: int CCPHDRTXBX
pub(crate) const CCPHDRTXBX: i32 = 10;
// Java field: int PNFBPCHPFIRST
pub(crate) const PNFBPCHPFIRST: i32 = 11;
// Java field: int PNCHPFIRST
pub(crate) const PNCHPFIRST: i32 = 12;
// Java field: int CPNBTECHP
pub(crate) const CPNBTECHP: i32 = 13;
// Java field: int PNFBPPAPFIRST
pub(crate) const PNFBPPAPFIRST: i32 = 14;
// Java field: int PNPAPFIRST
pub(crate) const PNPAPFIRST: i32 = 15;
// Java field: int CPNBTEPAP
pub(crate) const CPNBTEPAP: i32 = 16;
// Java field: int PNFBPLVCFIRST
pub(crate) const PNFBPLVCFIRST: i32 = 17;
// Java field: int PNLVCFIRST
pub(crate) const PNLVCFIRST: i32 = 18;
// Java field: int CPNBTELVC
pub(crate) const CPNBTELVC: i32 = 19;
// Java field: int FCISLANDFIRST
pub(crate) const FCISLANDFIRST: i32 = 20;
// Java field: int FCISLANDLIM
pub(crate) const FCISLANDLIM: i32 = 21;

// Java: public class FIBLongHandler
pub struct FIBLongHandler {
    // Java field: int[] _longs
    pub(crate) _longs: Vec<i32>,
}

// Java: public class FIBLongHandler
impl FIBLongHandler {
    // Java: FIBLongHandler(byte[] mainStream, int offset)
    pub fn new_from_bytes(main_stream: &[u8], offset: usize) -> Self {
      // Java: super();
      // Java: int longCount = LittleEndian.getShort(mainStream, offset);
      let long_count = le_get_short(main_stream, offset) as i32;
      // Java: offset += 2;
      let offset = offset + 2;
      // Java: this._longs = new int[longCount];
      let mut longs = vec![0i32; long_count as usize];
      // Java: for(int x = 0; x < longCount; ++x)
      for x in 0..long_count as usize {
         // Java: this._longs[x] = LittleEndian.getInt(mainStream, offset + x * 4);
         longs[x] = le_get_int(main_stream, offset + x * 4);
      }
      FIBLongHandler { _longs: longs }
    }

    // Java: FIBLongHandler(DocumentInputStream mainStream, int offset)
    pub fn new_from_stream(main_stream: &mut DocumentInputStream, offset: i32) -> Self {
      // Java: super();
      // Java: mainStream.seek((long)offset);
      main_stream.seek(offset as i64);
      // Java: int longCount = mainStream.readUShort();
      let long_count = main_stream.readUShort();
      // Java: this._longs = new int[longCount];
      let mut longs = vec![0i32; long_count as usize];
      // Java: for(int x = 0; x < longCount; ++x)
      for x in 0..long_count as usize {
         // Java: this._longs[x] = mainStream.readInt();
         longs[x] = main_stream.readInt();
      }
      FIBLongHandler { _longs: longs }
    }

    // Java: int getLong(int longCode)
    pub fn getLong(&self, long_code: i32) -> i32 {
      // Java: return this._longs[longCode];
      self._longs[long_code as usize]
    }

    // Java: void setLong(int longCode, int value)
    pub fn setLong(&mut self, long_code: i32, value: i32) {
      // Java: this._longs[longCode] = value;
      self._longs[long_code as usize] = value;
    }

    // Java: void serialize(byte[] mainStream, int offset)
    pub fn serialize(&self, main_stream: &mut [u8], offset: usize) {
      // Java: LittleEndian.putShort(mainStream, offset, (short)this._longs.length);
      le_put_short(main_stream, offset, self._longs.len() as i16);
      // Java: offset += 2;
      let mut offset = offset + 2;
      // Java: for(int x = 0; x < this._longs.length; ++x)
      for x in 0..self._longs.len() {
         // Java: LittleEndian.putInt(mainStream, offset, this._longs[x]);
         le_put_int(main_stream, offset, self._longs[x]);
         // Java: offset += 4;
         offset += 4;
      }
    }

    // Java: int sizeInBytes()
    pub fn sizeInBytes(&self) -> i32 {
      // Java: return this._longs.length * 4 + 2;
      (self._longs.len() as i32) * 4 + 2
    }

    // Java: int getLongLength()
    pub fn getLongLength(&self) -> i32 {
      // Java: return this._longs.length;
      self._longs.len() as i32
    }

}
