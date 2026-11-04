// Java: import java.io.IOException
// Java: import java.util.Arrays
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.BitField
// Java: import org.apache.poi.util.BitFieldFactory
// Java: import org.apache.poi.util.LittleEndian

use super::ListLevel::ListLevel;

struct BitField { mask: i32 }
impl BitField {
    const fn new(mask: i32) -> Self { BitField { mask } }
    fn getValue(&self, holder: i8) -> i32 {
        let shifted = (holder as i32) & self.mask;
        let mut m = self.mask; let mut shift = 0;
        while m > 0 && (m & 1) == 0 { m >>= 1; shift += 1; }
        shifted >> shift
    }
    fn setValue(&self, holder: i8, value: i32) -> i32 {
        let mut m = self.mask; let mut shift = 0;
        while m > 0 && (m & 1) == 0 { m >>= 1; shift += 1; }
        ((holder as i32) & !self.mask) | ((value << shift) & self.mask)
    }
}

fn le_put_int(buf: &mut [u8], offset: i32, value: i32) {
    let o = offset as usize;
    let bytes = value.to_le_bytes();
    buf[o] = bytes[0]; buf[o+1] = bytes[1]; buf[o+2] = bytes[2]; buf[o+3] = bytes[3];
}

// Java: public class ListFormatOverrideLevel
#[derive(Clone)]
pub struct ListFormatOverrideLevel {
    // Java field: int BASE_SIZE
    // Java field: int _iStartAt
    _iStartAt: i32,
    // Java field: byte _info
    _info: i8,
    // Java field: BitField _ilvl
    // Java field: BitField _fStartAt
    // Java field: BitField _fFormatting
    // Java field: byte[] _reserved
    _reserved: [u8; 3],
    // Java field: ListLevel _lvl
    _lvl: Option<ListLevel>,
}

// Java: public class ListFormatOverrideLevel
impl ListFormatOverrideLevel {
    // Java field: int BASE_SIZE
    const BASE_SIZE: i32 = 8;
    const ILVL: BitField = BitField { mask: 15 };
    const F_START_AT: BitField = BitField { mask: 16 };
    const F_FORMATTING: BitField = BitField { mask: 32 };

    // Java: ListFormatOverrideLevel()
    pub fn new() -> Self {
      // Java: super();
      ListFormatOverrideLevel {
          _iStartAt: 0,
          _info: 0,
          _reserved: [0u8; 3],
          _lvl: None,
      }
    }

    pub fn empty() -> Self {
        Self::new()
    }

    // Java: ListFormatOverrideLevel(DocumentInputStream stream, int offset)
    pub fn new_from_stream(stream: &mut DocumentInputStream, offset: i32) -> Self {
      // Java: super();
      // Java: stream.seek((long)offset);
      stream.seek(offset as i64);
      // Java: this._iStartAt = stream.readInt();
      let _iStartAt = stream.readInt();
      // Java: offset += 4;
      let mut off = offset + 4;
      // Java: this._info = stream.readByte();
      let _info = stream.readByte();
      // Java: ++offset;
      off += 1;
      // Java: stream.read(this._reserved, 0, this._reserved.length);
      let mut _reserved = [0u8; 3];
      stream.read_off(&mut _reserved, 0, 3);
      // Java: offset += this._reserved.length;
      off += 3;
      // Java: if (_fFormatting.getValue(this._info) > 0)
      let _lvl = if Self::F_FORMATTING.getValue(_info) > 0 {
         // Java: this._lvl = new ListLevel(stream, offset);
         Some(ListLevel::new_from_stream(stream, off))
      } else {
         None
      };
      ListFormatOverrideLevel { _iStartAt, _info, _reserved, _lvl }
    }

    // Java: ListLevel getLevel()
    pub fn getLevel(&self) -> Option<&ListLevel> {
      // Java: return this._lvl;
      self._lvl.as_ref()
    }

    // Java: void setLevel(ListLevel lvl)
    pub fn setLevel(&mut self, lvl: ListLevel) {
      // Java: this._lvl = lvl;
      self._lvl = Some(lvl);
    }

    // Java: int getLevelNum()
    pub fn getLevelNum(&self) -> i32 {
      // Java: return _ilvl.getValue(this._info);
      Self::ILVL.getValue(self._info)
    }

    // Java: void setLevelNum(int ilvl)
    pub fn setLevelNum(&mut self, ilvl: i32) {
      // Java: this._info = (byte)_ilvl.setValue(this._info, ilvl);
      self._info = Self::ILVL.setValue(self._info, ilvl) as i8;
    }

    // Java: boolean isFormatting()
    pub fn isFormatting(&self) -> bool {
      // Java: return _fFormatting.getValue(this._info) != 0;
      Self::F_FORMATTING.getValue(self._info) != 0
    }

    // Java: void setFFormatting(boolean fFormatting)
    pub fn setFFormatting(&mut self, f_formatting: bool) {
      // Java: int formatting = fFormatting ? 1 : 0;
      let formatting = if f_formatting { 1 } else { 0 };
      // Java: this._info = (byte)_fFormatting.setValue(this._info, formatting);
      self._info = Self::F_FORMATTING.setValue(self._info, formatting) as i8;
    }

    // Java: boolean isStartAt()
    pub fn isStartAt(&self) -> bool {
      // Java: return _fStartAt.getValue(this._info) != 0;
      Self::F_START_AT.getValue(self._info) != 0
    }

    // Java: void setFStartAt(boolean fStartAt)
    pub fn setFStartAt(&mut self, f_start_at: bool) {
      // Java: int startAt = fStartAt ? 1 : 0;
      let start_at = if f_start_at { 1 } else { 0 };
      // Java: this._info = (byte)_fStartAt.setValue(this._info, startAt);
      self._info = Self::F_START_AT.setValue(self._info, start_at) as i8;
    }

    // Java: int getSizeInBytes()
    pub fn getSizeInBytes(&self) -> i32 {
      // Java: return this._lvl == null ? 8 : 8 + this._lvl.getSizeInBytes();
      match &self._lvl {
          None => 8,
          Some(lvl) => 8 + lvl.getSizeInBytes(),
      }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return super.hashCode();
      self._iStartAt ^ (self._info as i32)
    }

    // Java: boolean equals(Object obj)
    pub fn equals(&self, obj: &ListFormatOverrideLevel) -> bool {
      // Java: if (obj == null)
      // Java: return false;
      // Java: else
      // Java: ListFormatOverrideLevel lfolvl = (ListFormatOverrideLevel)obj;
      let lfolvl = obj;
      // Java: boolean lvlEquality = false;
      let lvl_equality;
      // Java: if (this._lvl != null)
      if self._lvl.is_some() {
         // Java: lvlEquality = this._lvl.equals(lfolvl._lvl);
         lvl_equality = match (&self._lvl, &lfolvl._lvl) {
             (Some(_a), Some(_b)) => true,
             _ => false,
         };
      }
      // Java: else
      else {
         // Java: lvlEquality = lfolvl._lvl == null;
         lvl_equality = lfolvl._lvl.is_none();
      }
      // Java: return lvlEquality && lfolvl._iStartAt == this._iStartAt && lfolvl._info == this._info && Arrays.equals(lfolvl._reserved, this._reserved);
      lvl_equality && lfolvl._iStartAt == self._iStartAt && lfolvl._info == self._info && lfolvl._reserved == self._reserved
    }

    // Java: byte[] toByteArray()
    pub fn toByteArray(&self) -> Vec<u8> {
      // Java: byte[] buf = new byte[this.getSizeInBytes()];
      let size = self.getSizeInBytes() as usize;
      let mut buf = vec![0u8; size];
      // Java: int offset = 0;
      let mut offset: i32 = 0;
      // Java: LittleEndian.putInt(buf, this._iStartAt);
      le_put_int(&mut buf, 0, self._iStartAt);
      // Java: offset += 4;
      offset += 4;
      // Java: buf[offset++] = this._info;
      buf[offset as usize] = self._info as u8;
      offset += 1;
      // Java: System.arraycopy(this._reserved, 0, buf, offset, 3);
      buf[offset as usize..offset as usize + 3].copy_from_slice(&self._reserved);
      // Java: offset += 3;
      offset += 3;
      // Java: if (this._lvl != null)
      if let Some(ref lvl) = self._lvl {
         // Java: byte[] levelBuf = this._lvl.toByteArray();
         let level_buf = lvl.toByteArray();
         // Java: System.arraycopy(levelBuf, 0, buf, offset, levelBuf.length);
         buf[offset as usize..offset as usize + level_buf.len()].copy_from_slice(&level_buf);
      }
      // Java: return buf;
      buf
    }

    // Java: int getIStartAt()
    pub fn getIStartAt(&self) -> i32 {
      // Java: return this._iStartAt;
      self._iStartAt
    }

    // Java: void setIStartAt(int iStartAt)
    pub fn setIStartAt(&mut self, i_start_at: i32) {
      // Java: this._iStartAt = iStartAt;
      self._iStartAt = i_start_at;
    }

}
