// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use super::ATNBE::ATNBE;
use std::io::Write;

fn le_put_short(buf: &mut [u8], offset: i32, value: i16) {
    let o = offset as usize;
    let v = value as u16;
    buf[o] = (v & 0xFF) as u8;
    buf[o + 1] = (v >> 8) as u8;
}

// Java field: int LENGTH_ZERO_TAG
pub(crate) const LENGTH_ZERO_TAG: i32 = -1;

// Java: public class SttbfAtnbkmk
pub struct SttbfAtnbkmk {
    // Java field: short _fExtend
    _fExtend: i16,
    // Java field: short _nCount
    pub(crate) _nCount: i16,
    // Java field: short _cbExtra
    _cbExtra: i16,
    // Java field: short _cchData
    _cchData: i16,
    // Java field: ArrayList<ATNBE> _atnbeList
    _atnbeList: Vec<ATNBE>,
    // Java field: ArrayList<Integer> _atnbkmkTag
    pub(crate) _atnbkmkTag: Vec<i32>,
}

// Java: public class SttbfAtnbkmk
impl SttbfAtnbkmk {
    // Java: SttbfAtnbkmk()
    pub fn new() -> Self {
      // Java: super();
      SttbfAtnbkmk { _fExtend: -1, _nCount: 0, _cbExtra: 10, _cchData: 0, _atnbeList: Vec::new(), _atnbkmkTag: Vec::new() }
    }

    // Java: SttbfAtnbkmk(DocumentInputStream buf, int offset, int size)
    pub fn new_from_stream(buf: &mut DocumentInputStream, mut offset: i32, _size: i32) -> Self {
      // Java: super();
      let mut this = SttbfAtnbkmk { _fExtend: -1, _nCount: 0, _cbExtra: 10, _cchData: 0, _atnbeList: Vec::new(), _atnbkmkTag: Vec::new() };
      // Java: buf.seek((long)offset);
      buf.seek(offset as i64);
      // Java: this._fExtend = buf.readShort();
      this._fExtend = buf.readShort();
      // Java: offset += 2;
      offset += 2;
      // Java: this._nCount = buf.readShort();
      this._nCount = buf.readShort();
      // Java: offset += 2;
      offset += 2;
      // Java: this._cbExtra = buf.readShort();
      this._cbExtra = buf.readShort();
      // Java: offset += 2;
      offset += 2;
      // Java: this._cchData = buf.readShort();
      this._cchData = buf.readShort();
      // Java: offset += 4;
      offset += 4;
      // Java: for(int i = 0; i < this._nCount; ++i)
      for _i in 0..this._nCount
      {
         // Java: buf.seek((long)offset);
         buf.seek(offset as i64);
         // Java: this._atnbkmkTag.add(buf.readInt());
         this._atnbkmkTag.push(buf.readInt());
         // Java: offset += 12;
         offset += 12;
      }
      this
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return this._nCount;
      self._nCount as i32
    }

    // Java: void setSize(int count)
    pub fn setSize(&mut self, count: i32) {
      // Java: this._nCount = (short)count;
      self._nCount = count as i16;
    }

    // Java: void addATNBE(ATNBE atnbe)
    pub fn addATNBE(&mut self, atnbe: ATNBE) {
      // Java: this._atnbeList.add(atnbe);
      self._atnbeList.push(atnbe);
    }

    // Java: int getIndexByTag(int tag)
    pub fn getIndexByTag(&self, tag: i32) -> i32 {
      // Java: return this._atnbkmkTag.indexOf(tag);
      self._atnbkmkTag.iter().position(|&t| t == tag).map_or(-1, |p| p as i32)
    }

    // Java: void writeTo(HWPFOutputStream out)
    pub fn writeTo(&self, out: &mut HWPFOutputStream) {
      // Java: if (this._nCount > 0)
      if self._nCount > 0
      {
         // Java: int offset = 0;
         let mut offset: i32 = 0;
         // Java: byte[] buf = new byte[8];
         let mut buf = vec![0u8; 8];
         // Java: LittleEndian.putShort(buf, offset, this._fExtend);
         le_put_short(&mut buf, offset, self._fExtend);
         // Java: offset += 2;
         offset += 2;
         // Java: LittleEndian.putShort(buf, offset, this._nCount);
         le_put_short(&mut buf, offset, self._nCount);
         // Java: offset += 2;
         offset += 2;
         // Java: LittleEndian.putShort(buf, offset, this._cbExtra);
         le_put_short(&mut buf, offset, self._cbExtra);
         // Java: offset += 2;
         offset += 2;
         // Java: LittleEndian.putShort(buf, offset, this._cchData);
         le_put_short(&mut buf, offset, self._cchData);
         // Java: offset += 2;
         offset += 2;
         // Java: out.write(buf);
         let _ = out.write(&buf);
         // Java: offset = 0;
         offset = 0;
         // Java: buf = new byte[12 * this._nCount - 2];
         let buf_size = (12 * self._nCount as i32 - 2) as usize;
         let mut buf = vec![0u8; buf_size];
         // Java: for(int i = 0; i < this._nCount; ++i)
         for i in 0..self._nCount as usize
         {
            // Java: ((ATNBE)this._atnbeList.get(i)).serialize(buf, offset);
            self._atnbeList[i].serialize(&mut buf, offset);
            // Java: offset += 12;
            offset += 12;
         }
         // Java: out.write(buf);
         let _ = out.write(&buf);
      }
    }

}
