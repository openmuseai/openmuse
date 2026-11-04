// Java: import cn.wps.base.assertion.IOAssert
// Java: import java.io.IOException
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use std::io::Write;

fn le_put_int(buf: &mut [u8], offset: i32, value: i32) {
    let o = offset as usize;
    buf[o] = (value & 0xFF) as u8;
    buf[o+1] = ((value >> 8) & 0xFF) as u8;
    buf[o+2] = ((value >> 16) & 0xFF) as u8;
    buf[o+3] = ((value >> 24) & 0xFF) as u8;
}

// Java: public class Plcfbkl
pub struct Plcfbkl {
    // Java field: int _count
    _count: i32,
    // Java field: int[] _bklCp
    pub(crate) _bklCp: Vec<i32>,
    // Java field: boolean[] mFindMark
    mFindMark: Option<Vec<bool>>,
}

// Java: public class Plcfbkl
impl Plcfbkl {
    // Java: Plcfbkl()
    pub fn new() -> Self {
      // Java: super();
      Plcfbkl { _count: 0, _bklCp: Vec::new(), mFindMark: None }
    }

    // Java: Plcfbkl(DocumentInputStream tableStream, int offset, int size)
    pub fn new_from_stream(table_stream: &mut DocumentInputStream, offset: i32, size: i32) -> Self {
      // Java: super();
      let mut this = Plcfbkl { _count: 0, _bklCp: Vec::new(), mFindMark: None };
      // Java: tableStream.seek((long)offset);
      table_stream.seek(offset as i64);
      // Java: this._count = size / 4;
      this._count = size / 4;
      // Java: this._bklCp = new int[this._count];
      this._bklCp = vec![0i32; this._count as usize];
      // Java: for(int i = 0; i < this._count; ++i)
      for i in 0..this._count
      {
         // Java: this._bklCp[i] = tableStream.readInt();
         this._bklCp[i as usize] = table_stream.readInt();
         // Java: offset += 4;
      }
      this
    }

    // Java: void init(int count)
    pub fn init(&mut self, count: i32) {
      // Java: this._count = count;
      self._count = count;
      // Java: this._bklCp = new int[this._count];
      self._bklCp = vec![0i32; self._count as usize];
      // Java: this.mFindMark = new boolean[this._count];
      self.mFindMark = Some(vec![false; self._count as usize]);
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return this._bklCp.length;
      self._bklCp.len() as i32
    }

    // Java: int getCpByIndex(int index)
    pub fn getCpByIndex(&self, index: i32) -> i32 {
      // Java: return index >= 0 && index < this._count ? this._bklCp[index] : -1;
      if index >= 0 && index < self._count { self._bklCp[index as usize] } else { -1 }
    }

    // Java: int getIndexByCp(int cp)
    pub fn getIndexByCp(&mut self, cp: i32) -> i32 {
      // Java: for(int i = 0; i < this._count; ++i)
      for i in 0..self._count
      {
         // Java: if (this._bklCp[i] == cp && !this.mFindMark[i])
         if self._bklCp[i as usize] == cp && !self.mFindMark.as_ref().map_or(false, |m| m[i as usize])
         {
            // Java: this.mFindMark[i] = true;
            if let Some(ref mut marks) = self.mFindMark {
                marks[i as usize] = true;
            }
            // Java: return i;
            return i;
         }
      }
      // Java: return -1;
      -1
    }

    // Java: void addCpByIndex(int index, int cp)
    pub fn addCpByIndex(&mut self, index: i32, cp: i32) {
      // Java: IOAssert.assertTrue("index >= 0 && index < _count should be true!", index >= 0 && index < this._count);
      assert!(index >= 0 && index < self._count, "index >= 0 && index < _count should be true!");
      // Java: this._bklCp[index] = cp;
      self._bklCp[index as usize] = cp;
    }

    // Java: void writeTo(HWPFOutputStream tableStream)
    pub fn writeTo(&self, table_stream: &mut HWPFOutputStream) {
      // Java: if (this._count > 0)
      if self._count > 0
      {
         // Java: int offset = 0;
         let mut offset: i32 = 0;
         // Java: byte[] buf = new byte[this._count * 4];
         let mut buf = vec![0u8; (self._count * 4) as usize];
         // Java: for(int i = 0; i < this._count; ++i)
         for i in 0..self._count
         {
            // Java: LittleEndian.putInt(buf, offset, this._bklCp[i]);
            le_put_int(&mut buf, offset, self._bklCp[i as usize]);
            // Java: offset += 4;
            offset += 4;
         }
         // Java: tableStream.write(buf);
         let _ = table_stream.write(&buf);
      }
    }

}
