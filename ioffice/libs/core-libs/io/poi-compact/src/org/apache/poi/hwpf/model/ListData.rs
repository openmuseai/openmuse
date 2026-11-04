// Java: import cn.wps.base.assertion.IOAssert
// Java: import java.util.Arrays
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.BitField
// Java: import org.apache.poi.util.BitFieldFactory
// Java: import org.apache.poi.util.LittleEndian

use super::ListLevel::ListLevel;

struct BitField {
    mask: i32,
}

impl BitField {
    fn new(mask: i32) -> Self { BitField { mask } }
    fn getValue(&self, holder: i8) -> i32 {
        let shifted = (holder as i32) & self.mask;
        let mut m = self.mask;
        let mut shift = 0;
        while m > 0 && (m & 1) == 0 { m >>= 1; shift += 1; }
        shifted >> shift
    }
    fn setValue(&self, holder: i8, value: i32) -> i32 {
        let mut m = self.mask;
        let mut shift = 0;
        while m > 0 && (m & 1) == 0 { m >>= 1; shift += 1; }
        ((holder as i32) & !self.mask) | ((value << shift) & self.mask)
    }
    fn setByteBoolean(&self, holder: i8, flag: bool) -> i8 {
        if flag {
            ((holder as i32) | self.mask) as i8
        } else {
            ((holder as i32) & !self.mask) as i8
        }
    }
}

fn le_put_int(buf: &mut [u8], offset: i32, value: i32) {
    let o = offset as usize;
    let bytes = value.to_le_bytes();
    buf[o] = bytes[0]; buf[o+1] = bytes[1]; buf[o+2] = bytes[2]; buf[o+3] = bytes[3];
}

fn le_put_short(buf: &mut [u8], offset: i32, value: i16) {
    let o = offset as usize;
    let v = value as u16;
    buf[o] = (v & 0xFF) as u8;
    buf[o+1] = (v >> 8) as u8;
}

// Java: public class ListData
#[derive(Clone)]
pub struct ListData {
    // Java field: int SIZE_LSTF
    // Java field: int LEN_RGISTD
    // Java field: int _lsid
    _lsid: i32,
    // Java field: int _tplc
    _tplc: i32,
    // Java field: short[] _rgistd
    _rgistd: Vec<i16>,
    // Java field: byte _info
    _info: i8,
    // Java field: BitField _fSimpleList
    // Java field: BitField _fRestartHdn
    // Java field: BitField _fAutoNum
    // Java field: BitField _fHybrid
    // Java field: byte _grfhic
    _grfhic: i8,
    // Java field: ListLevel[] _levels
    _levels: Vec<ListLevel>,
}

// Java: public class ListData
impl ListData {
    // Java field: int SIZE_LSTF
    pub const SIZE_LSTF: i32 = 28;
    // Java field: int LEN_RGISTD
    pub const LEN_RGISTD: i32 = 9;

    const F_SIMPLE_LIST: BitField = BitField { mask: 1 };
    const F_RESTART_HDN: BitField = BitField { mask: 2 };
    const F_AUTO_NUM: BitField = BitField { mask: 4 };
    const F_HYBRID: BitField = BitField { mask: 16 };

    // Java: ListData(int listID, boolean numbered)
    pub fn new(list_id: i32, numbered: bool) -> Self {
      // Java: this(listID, false, numbered);
      Self::new_full(list_id, false, numbered)
    }

    // Java: ListData(int listID, boolean fSimpleList, boolean numbered)
    pub fn new_full(list_id: i32, f_simple_list: bool, numbered: bool) -> Self {
      // Java: super();
      // Java: this._lsid = listID;
      let mut data = ListData {
          _lsid: list_id,
          _tplc: 0,
          _rgistd: vec![0i16; 9],
          _info: 0,
          _grfhic: 0,
          _levels: Vec::new(),
      };
      // Java: this._rgistd = new short[9];
      // Java: for(int x = 0; x < 9; ++x)
      for x in 0..9 {
         // Java: this._rgistd[x] = 4095;
         data._rgistd[x] = 4095;
      }
      // Java: this.setFSimpleList(fSimpleList);
      data.setFSimpleList(f_simple_list);
      // Java: if (fSimpleList)
      if f_simple_list {
         // Java: this._levels = new ListLevel[1];
         data._levels = Vec::with_capacity(1);
      }
      // Java: else
      else {
         // Java: this._levels = new ListLevel[9];
         data._levels = Vec::with_capacity(9);
      }
      // Java: int x = 0;
      let len = if f_simple_list { 1 } else { 9 };
      // Java: for(int len = this._levels.length; x < len; ++x)
      for x in 0..len {
         // Java: this._levels[x] = new ListLevel(x, numbered);
         data._levels.push(ListLevel::new(x as i32, numbered));
      }
      data
    }

    pub fn new_from_stream(stream: &mut DocumentInputStream, offset: i32) -> Self {
        stream.seek(offset as i64);
        let _lsid = stream.readInt();
        let _tplc = stream.readInt();
        let mut _rgistd = vec![0i16; 9];
        for x in 0..9 {
            _rgistd[x] = stream.readShort();
        }
        let _info = stream.readByte();
        let _grfhic = stream.readByte();
        let f_simple = BitField::new(1);
        let num_levels = if f_simple.getValue(_info) > 0 { 1 } else { 9 };
        ListData {
            _lsid,
            _tplc,
            _rgistd,
            _info,
            _grfhic,
            _levels: Vec::with_capacity(num_levels),
        }
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

    // Java: int getTplc()
    pub fn getTplc(&self) -> i32 {
      // Java: return this._tplc;
      self._tplc
    }

    // Java: void setTplc(int tplc)
    pub fn setTplc(&mut self, tplc: i32) {
      // Java: this._tplc = tplc;
      self._tplc = tplc;
    }

    // Java: boolean isFSimpleList()
    pub fn isFSimpleList(&self) -> bool {
      // Java: return _fSimpleList.getValue(this._info) == 1;
      Self::F_SIMPLE_LIST.getValue(self._info) == 1
    }

    // Java: void setFSimpleList(boolean fSimpleList)
    pub fn setFSimpleList(&mut self, f_simple_list: bool) {
      // Java: int simpleList = 0;
      let mut simple_list = 0;
      // Java: if (fSimpleList)
      if f_simple_list {
         // Java: simpleList = 1;
         simple_list = 1;
      }
      // Java: this._info = (byte)_fSimpleList.setValue(this._info, simpleList);
      self._info = Self::F_SIMPLE_LIST.setValue(self._info, simple_list) as i8;
    }

    // Java: boolean isFRestartHdn()
    pub fn isFRestartHdn(&self) -> bool {
      // Java: return _fRestartHdn.getValue(this._info) == 1;
      Self::F_RESTART_HDN.getValue(self._info) == 1
    }

    // Java: void setFRestartHdn(boolean fRestartHdn)
    pub fn setFRestartHdn(&mut self, f_restart_hdn: bool) {
      // Java: int restartHdn = 0;
      let mut restart_hdn = 0;
      // Java: if (fRestartHdn)
      if f_restart_hdn {
         // Java: restartHdn = 1;
         restart_hdn = 1;
      }
      // Java: this._info = (byte)_fRestartHdn.setValue(this._info, restartHdn);
      self._info = Self::F_RESTART_HDN.setValue(self._info, restart_hdn) as i8;
    }

    // Java: boolean isFAutoNum()
    pub fn isFAutoNum(&self) -> bool {
      // Java: return _fAutoNum.getValue(this._info) == 1;
      Self::F_AUTO_NUM.getValue(self._info) == 1
    }

    // Java: void setFAutoNum(boolean fAutoNum)
    pub fn setFAutoNum(&mut self, f_auto_num: bool) {
      // Java: this._info = _fAutoNum.setByteBoolean(this._info, fAutoNum);
      self._info = Self::F_AUTO_NUM.setByteBoolean(self._info, f_auto_num);
    }

    // Java: boolean isFHybrid()
    pub fn isFHybrid(&self) -> bool {
      // Java: return _fHybrid.getValue(this._info) == 1;
      Self::F_HYBRID.getValue(self._info) == 1
    }

    // Java: void setFHybrid(boolean fHybrid)
    pub fn setFHybrid(&mut self, f_hybrid: bool) {
      // Java: this._info = _fHybrid.setByteBoolean(this._info, fHybrid);
      self._info = Self::F_HYBRID.setByteBoolean(self._info, f_hybrid);
    }

    // Java: byte getGrfhic()
    pub fn getGrfhic(&self) -> i8 {
      // Java: return this._grfhic;
      self._grfhic
    }

    // Java: void setGrfhic(byte grfhic)
    pub fn setGrfhic(&mut self, grfhic: i8) {
      // Java: this._grfhic = grfhic;
      self._grfhic = grfhic;
    }

    // Java: int numLevels()
    pub fn numLevels(&self) -> i32 {
      // Java: return this._levels.length;
      self._levels.len() as i32
    }

    // Java: void setLevel(int index, ListLevel level)
    pub fn setLevel(&mut self, index: i32, level: ListLevel) {
      // Java: this._levels[index] = level;
      let idx = index as usize;
      if idx < self._levels.len() {
          self._levels[idx] = level;
      } else {
          while self._levels.len() <= idx {
              self._levels.push(ListLevel::new(0, false));
          }
          self._levels[idx] = level;
      }
    }

    // Java: ListLevel[] getLevels()
    pub fn getLevels(&self) -> &Vec<ListLevel> {
      // Java: return this._levels;
      &self._levels
    }

    // Java: void getLevels(ListLevel[] levels)
    pub fn setLevels(&mut self, levels: Option<&Vec<ListLevel>>) {
      // Java: if (levels != null)
      if let Some(lvls) = levels {
         // Java: this._levels = (ListLevel[]) levels.clone();
         self._levels = lvls.clone();
      }
    }

    // Java: int getLevelStyle(int index)
    pub fn getLevelStyle(&self, index: i32) -> i16 {
      // Java: return this._rgistd[index];
      self._rgistd[index as usize]
    }

    // Java: void setLevelStyle(int index, int styleIndex)
    pub fn setLevelStyle(&mut self, index: i32, style_index: i32) {
      // Java: IOAssert.assertTrue("index of rgistd should be in 0 to 8: " + index, index >= 0 && index <= 8);
      assert!(index >= 0 && index <= 8, "index of rgistd should be in 0 to 8: {}", index);
      // Java: this._rgistd[index] = (short)styleIndex;
      self._rgistd[index as usize] = style_index as i16;
    }

    // Java: int getLevelStyleSize()
    pub fn getLevelStyleSize(&self) -> i32 {
      // Java: return this._rgistd.length;
      self._rgistd.len() as i32
    }

    // Java: void setLevelStyles(short[] rgistd)
    pub fn setLevelStyles(&mut self, rgistd: Option<&Vec<i16>>) {
      // Java: if (rgistd != null)
      if let Some(r) = rgistd {
         // Java: this._rgistd = (short[]) rgistd.clone();
         self._rgistd = r.clone();
      }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return super.hashCode();
      self._lsid ^ self._tplc ^ (self._info as i32) ^ (self._grfhic as i32)
    }

    // Java: boolean equals(Object obj)
    pub fn equals(&self, obj: &ListData) -> bool {
      // Java: if (obj == null)
      // Java: return false;
      // Java: else
      // Java: ListData lst = (ListData)obj;
      let lst = obj;
      // Java: return lst._info == this._info && Arrays.equals(lst._levels, this._levels) && lst._lsid == this._lsid && lst._grfhic == this._grfhic && lst._tplc == this._tplc && Arrays.equals(lst._rgistd, this._rgistd);
      lst._info == self._info && lst._lsid == self._lsid && lst._grfhic == self._grfhic && lst._tplc == self._tplc && lst._rgistd == self._rgistd
    }

    // Java: int resetListID()
    pub fn resetListID(&mut self) -> i32 {
      // Java: this._lsid = (int)(Math.random() * (double)System.currentTimeMillis());
      self._lsid = (std::time::SystemTime::now()
          .duration_since(std::time::UNIX_EPOCH)
          .unwrap_or_default()
          .as_millis() as i32).wrapping_mul(31);
      // Java: return this._lsid;
      self._lsid
    }

    // Java: byte[] toByteArray()
    pub fn toByteArray(&self) -> Vec<u8> {
      // Java: byte[] buf = new byte[28];
      let mut buf = vec![0u8; 28];
      // Java: int offset = 0;
      let mut offset: i32 = 0;
      // Java: LittleEndian.putInt(buf, this._lsid);
      le_put_int(&mut buf, 0, self._lsid);
      // Java: offset += 4;
      offset += 4;
      // Java: LittleEndian.putInt(buf, offset, this._tplc);
      le_put_int(&mut buf, offset, self._tplc);
      // Java: offset += 4;
      offset += 4;
      // Java: for(int x = 0; x < 9; ++x)
      for x in 0..9 {
         // Java: LittleEndian.putShort(buf, offset, this._rgistd[x]);
         le_put_short(&mut buf, offset, self._rgistd[x]);
         // Java: offset += 2;
         offset += 2;
      }
      // Java: buf[offset++] = this._info;
      buf[offset as usize] = self._info as u8;
      offset += 1;
      // Java: buf[offset] = this._grfhic;
      buf[offset as usize] = self._grfhic as u8;
      // Java: return buf;
      buf
    }

    pub fn getLevelAt(&self, index: i32) -> &ListLevel {
        &self._levels[index as usize]
    }
}
