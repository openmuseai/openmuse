// Java: import java.io.IOException
// Java: import java.util.Arrays
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use super::ListFormatOverrideLevel::ListFormatOverrideLevel;

fn le_put_int(buf: &mut [u8], offset: i32, value: i32) {
    let o = offset as usize;
    let bytes = value.to_le_bytes();
    buf[o] = bytes[0]; buf[o+1] = bytes[1]; buf[o+2] = bytes[2]; buf[o+3] = bytes[3];
}

// Java: public class ListFormatOverride
pub struct ListFormatOverride {
    // Java field: int _lsid
    _lsid: i32,
    // Java field: int _reserved1
    _reserved1: i32,
    // Java field: int _reserved2
    _reserved2: i32,
    // Java field: byte _clfolvl
    _clfolvl: i8,
    // Java field: byte _ibstFltAutoNum
    _ibstFltAutoNum: i8,
    // Java field: byte _grfhic
    _grfhic: i8,
    // Java field: byte _reserved3
    _reserved3: i8,
    // Java field: ListFormatOverrideLevel[] _levelOverrides
    _levelOverrides: Vec<ListFormatOverrideLevel>,
}

// Java: public class ListFormatOverride
impl ListFormatOverride {
    // Java: ListFormatOverride(int lsid)
    pub fn new(lsid: i32) -> Self {
      // Java: super();
      // Java: this._lsid = lsid;
      // Java: this._levelOverrides = new ListFormatOverrideLevel[0];
      ListFormatOverride {
          _lsid: lsid,
          _reserved1: 0,
          _reserved2: 0,
          _clfolvl: 0,
          _ibstFltAutoNum: 0,
          _grfhic: 0,
          _reserved3: 0,
          _levelOverrides: Vec::new(),
      }
    }

    // Java: ListFormatOverride(DocumentInputStream stream, int offset)
    pub fn new_from_stream(stream: &mut DocumentInputStream, offset: i32) -> Self {
      // Java: super();
      // Java: stream.seek((long)offset);
      stream.seek(offset as i64);
      // Java: this._lsid = stream.readInt();
      let _lsid = stream.readInt();
      // Java: offset += 4;
      // Java: this._reserved1 = stream.readInt();
      let _reserved1 = stream.readInt();
      // Java: offset += 4;
      // Java: this._reserved2 = stream.readInt();
      let _reserved2 = stream.readInt();
      // Java: offset += 4;
      // Java: this._clfolvl = stream.readByte();
      let _clfolvl = stream.readByte();
      // Java: ++offset;
      // Java: this._ibstFltAutoNum = stream.readByte();
      let _ibstFltAutoNum = stream.readByte();
      // Java: ++offset;
      // Java: this._grfhic = stream.readByte();
      let _grfhic = stream.readByte();
      // Java: ++offset;
      // Java: this._reserved3 = stream.readByte();
      let _reserved3 = stream.readByte();
      // Java: ++offset;
      // Java: this._levelOverrides = new ListFormatOverrideLevel[this._clfolvl];
      let _levelOverrides = Vec::with_capacity(_clfolvl as usize);
      ListFormatOverride {
          _lsid, _reserved1, _reserved2, _clfolvl,
          _ibstFltAutoNum, _grfhic, _reserved3, _levelOverrides,
      }
    }

    // Java: byte numOverrides()
    pub fn numOverrides(&self) -> i32 {
      // Java: return this._clfolvl;
      self._clfolvl as i32
    }

    // Java: void setNumOverrides(byte clfolvl)
    pub fn setNumOverrides(&mut self, clfolvl: i8) {
      // Java: this._clfolvl = clfolvl;
      self._clfolvl = clfolvl;
    }

    // Java: int getLsid()
    pub fn getLsid(&self) -> i32 {
      // Java: return this._lsid;
      self._lsid
    }

    // Java: void setLsid(int lsid)
    pub fn setLsid(&mut self, lsid: i32) {
      // Java: this._lsid = lsid;
      self._lsid = lsid;
    }

    // Java: byte getIbstFltAutoNum()
    pub fn getIbstFltAutoNum(&self) -> i8 {
      // Java: return this._ibstFltAutoNum;
      self._ibstFltAutoNum
    }

    // Java: void setIbstFltAutoNum(byte val)
    pub fn setIbstFltAutoNum(&mut self, val: i8) {
      // Java: this._ibstFltAutoNum = val;
      self._ibstFltAutoNum = val;
    }

    // Java: byte getGrfhic()
    pub fn getGrfhic(&self) -> i8 {
      // Java: return this._grfhic;
      self._grfhic
    }

    // Java: void setGrfhic(byte val)
    pub fn setGrfhic(&mut self, val: i8) {
      // Java: this._grfhic = val;
      self._grfhic = val;
    }

    // Java: ListFormatOverrideLevel[] getLevelOverrides()
    pub fn getLevelOverrides(&self) -> &Vec<ListFormatOverrideLevel> {
      // Java: return this._levelOverrides;
      &self._levelOverrides
    }

    // Java: void setLevelOverrides(ListFormatOverrideLevel[] levelOverrides)
    pub fn setLevelOverrides(&mut self, level_overrides: Option<&Vec<ListFormatOverrideLevel>>) {
      // Java: this._levelOverrides = null;
      self._levelOverrides = Vec::new();
      // Java: if (levelOverrides != null)
      if let Some(overrides) = level_overrides {
         // Java: this._levelOverrides = (ListFormatOverrideLevel[]) levelOverrides.clone();
         self._levelOverrides = overrides.clone();
      }
    }

    // Java: void setOverride(int index, ListFormatOverrideLevel lfolvl)
    pub fn setOverride(&mut self, index: i32, lfolvl: ListFormatOverrideLevel) {
      // Java: if (this._levelOverrides != null && index < this._levelOverrides.length)
      if (index as usize) < self._levelOverrides.len() {
         // Java: this._levelOverrides[index] = lfolvl;
         self._levelOverrides[index as usize] = lfolvl;
      }
    }

    // Java: ListFormatOverrideLevel getOverrideLevel(int level)
    pub fn getOverrideLevel(&self, level: i32) -> Option<&ListFormatOverrideLevel> {
      // Java: ListFormatOverrideLevel retLevel = null;
      let mut ret_level: Option<&ListFormatOverrideLevel> = None;
      // Java: int x = 0;
      // Java: for(int len = this._levelOverrides.length; x < len; ++x)
      for x in 0..self._levelOverrides.len() {
         // Java: if (this._levelOverrides[x].getLevelNum() == level)
         if self._levelOverrides[x].getLevelNum() == level {
            // Java: retLevel = this._levelOverrides[x];
            ret_level = Some(&self._levelOverrides[x]);
         }
      }
      // Java: return retLevel;
      ret_level
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return super.hashCode();
      self._lsid ^ self._reserved1 ^ (self._clfolvl as i32)
    }

    // Java: boolean equals(Object obj)
    pub fn equals(&self, obj: &ListFormatOverride) -> bool {
      // Java: if (obj == null)
      // Java: return false;
      // Java: else
      // Java: ListFormatOverride lfo = (ListFormatOverride)obj;
      let lfo = obj;
      // Java: return lfo._clfolvl == this._clfolvl && lfo._lsid == this._lsid && lfo._reserved1 == this._reserved1 && lfo._reserved2 == this._reserved2 && lfo._clfolvl == this._clfolvl && lfo._ibstFltAutoNum == this._ibstFltAutoNum && lfo._grfhic == this._grfhic && lfo._reserved3 == this._reserved3 && Arrays.equals(lfo._levelOverrides, this._levelOverrides);
      lfo._clfolvl == self._clfolvl && lfo._lsid == self._lsid
          && lfo._reserved1 == self._reserved1 && lfo._reserved2 == self._reserved2
          && lfo._ibstFltAutoNum == self._ibstFltAutoNum && lfo._grfhic == self._grfhic
          && lfo._reserved3 == self._reserved3
    }

    // Java: byte[] toByteArray()
    pub fn toByteArray(&self) -> Vec<u8> {
      // Java: byte[] buf = new byte[16];
      let mut buf = vec![0u8; 16];
      // Java: int offset = 0;
      let mut offset: i32 = 0;
      // Java: LittleEndian.putInt(buf, offset, this._lsid);
      le_put_int(&mut buf, offset, self._lsid);
      // Java: offset += 4;
      offset += 4;
      // Java: LittleEndian.putInt(buf, offset, this._reserved1);
      le_put_int(&mut buf, offset, self._reserved1);
      // Java: offset += 4;
      offset += 4;
      // Java: LittleEndian.putInt(buf, offset, this._reserved2);
      le_put_int(&mut buf, offset, self._reserved2);
      // Java: offset += 4;
      offset += 4;
      // Java: buf[offset++] = this._clfolvl;
      buf[offset as usize] = self._clfolvl as u8; offset += 1;
      // Java: buf[offset++] = this._ibstFltAutoNum;
      buf[offset as usize] = self._ibstFltAutoNum as u8; offset += 1;
      // Java: buf[offset++] = this._grfhic;
      buf[offset as usize] = self._grfhic as u8; offset += 1;
      // Java: buf[offset++] = this._reserved3;
      buf[offset as usize] = self._reserved3 as u8;
      // Java: return buf;
      buf
    }

}
