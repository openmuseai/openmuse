// Java: import cn.wps.base.assertion.IOAssert
// Java: import java.io.IOException
// Java: import java.util.Arrays
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.BitField
// Java: import org.apache.poi.util.BitFieldFactory
use std::sync::Mutex;

struct BitField {
    _mask: i32,
    _shift_count: i32,
}

impl BitField {
    fn new(mask: i32) -> Self {
        let mut count = 0;
        let mut bit_pattern = mask;
        if mask != 0 {
            while (bit_pattern & 1) == 0 {
                count += 1;
                bit_pattern >>= 1;
            }
        }
        BitField { _mask: mask, _shift_count: count }
    }
    fn getValue(&self, holder: i32) -> i32 {
        ((self.getRawValue(holder) as u32) >> self._shift_count) as i32
    }
    fn getRawValue(&self, holder: i32) -> i32 {
        holder & self._mask
    }
    fn setValue(&self, holder: i32, value: i32) -> i32 {
        (holder & !self._mask) | ((value << self._shift_count) & self._mask)
    }
}

fn le_get_int(buf: &[u8], offset: usize) -> i32 {
    (buf[offset] as i32 & 0xFF) | ((buf[offset+1] as i32 & 0xFF) << 8) |
    ((buf[offset+2] as i32 & 0xFF) << 16) | ((buf[offset+3] as i32 & 0xFF) << 24)
}

fn le_put_int(buf: &mut [u8], offset: usize, value: i32) {
    buf[offset] = (value & 0xFF) as u8;
    buf[offset+1] = ((value >> 8) & 0xFF) as u8;
    buf[offset+2] = ((value >> 16) & 0xFF) as u8;
    buf[offset+3] = ((value >> 24) & 0xFF) as u8;
}

fn le_get_ushort(buf: &[u8], offset: usize) -> u16 {
    (buf[offset] as u16 & 0xFF) | ((buf[offset+1] as u16 & 0xFF) << 8)
}

fn le_put_ushort(buf: &mut [u8], offset: usize, value: u16) {
    buf[offset] = (value & 0xFF) as u8;
    buf[offset+1] = ((value >> 8) & 0xFF) as u8;
}

// Java field: int RGBXCH_NUMS_SIZE
pub(crate) const RGBXCH_NUMS_SIZE: i32 = 9;

fn get_jc() -> BitField { BitField::new(3) }
fn get_f_legal() -> BitField { BitField::new(4) }
fn get_f_no_restart() -> BitField { BitField::new(8) }
fn get_f_indent_sav() -> BitField { BitField::new(16) }
fn get_f_converted() -> BitField { BitField::new(32) }
fn get_f_tentative() -> BitField { BitField::new(128) }

// Java field: BitField _jc
static _jc: std::sync::OnceLock<std::sync::Mutex<BitField>> = std::sync::OnceLock::new();
// Java field: BitField _fLegal
static _fLegal: std::sync::OnceLock<std::sync::Mutex<BitField>> = std::sync::OnceLock::new();
// Java field: BitField _fNoRestart
static _fNoRestart: std::sync::OnceLock<std::sync::Mutex<BitField>> = std::sync::OnceLock::new();
// Java field: BitField _fIndentSav
static _fIndentSav: std::sync::OnceLock<std::sync::Mutex<BitField>> = std::sync::OnceLock::new();
// Java field: BitField _fConverted
static _fConverted: std::sync::OnceLock<std::sync::Mutex<BitField>> = std::sync::OnceLock::new();
// Java field: BitField _fTentative
static _fTentative: std::sync::OnceLock<std::sync::Mutex<BitField>> = std::sync::OnceLock::new();

// Java: public class ListLevel
#[derive(Clone)]
pub struct ListLevel {
    // Java field: int _iStartAt
    pub(crate) _iStartAt: i32,
    // Java field: byte _nfc
    pub(crate) _nfc: i8,
    // Java field: byte _info
    _info: i8,
    // Java field: byte[] _rgbxchNums
    pub(crate) _rgbxchNums: Vec<u8>,
    // Java field: byte _ixchFollow
    pub(crate) _ixchFollow: i8,
    // Java field: int _dxaIndentSav
    pub(crate) _dxaIndentSav: i32,
    // Java field: int _unused
    _unused: i32,
    // Java field: short _cbGrpprlChpx
    pub(crate) _cbGrpprlChpx: i16,
    // Java field: short _cbGrpprlPapx
    pub(crate) _cbGrpprlPapx: i16,
    // Java field: byte _ilvlRestartLim
    pub(crate) _ilvlRestartLim: i8,
    // Java field: byte _grfhic
    pub(crate) _grfhic: i8,
    // Java field: byte[] _grpprlPapx
    pub(crate) _grpprlPapx: Vec<u8>,
    // Java field: byte[] _grpprlChpx
    pub(crate) _grpprlChpx: Vec<u8>,
    // Java field: char[] _numberText
    pub(crate) _numberText: Vec<char>,
}

// Java: public class ListLevel
impl ListLevel {
    // Java: ListLevel(int level, boolean numbered)
    pub fn new(level: i32, numbered: bool) -> Self {
      // Java: super();
      // Java: this._iStartAt = 1;
      // Java: this._grpprlPapx = new byte[0];
      // Java: this._grpprlChpx = new byte[0];
      // Java: this._numberText = new char[0];
      // Java: this._rgbxchNums = new byte[9];
      let mut rgbxch_nums = vec![0u8; 9];
      let number_text;
      // Java: if (numbered)
        if numbered
      {
         // Java: this._rgbxchNums[0] = 1;
         rgbxch_nums[0] = 1;
         // Java: this._numberText = new char[]{(char)level, '.'};
         number_text = vec![char::from_u32(level as u32).unwrap_or('\0'), '.'];
      }
      // Java: else
        else
      {
         // Java: this._numberText = new char[]{'•'};
         number_text = vec!['\u{2022}'];
      }
      ListLevel {
          _iStartAt: 1,
          _nfc: 0,
          _info: 0,
          _rgbxchNums: rgbxch_nums,
          _ixchFollow: 0,
          _dxaIndentSav: 0,
          _unused: 0,
          _cbGrpprlChpx: 0,
          _cbGrpprlPapx: 0,
          _ilvlRestartLim: 0,
          _grfhic: 0,
          _grpprlPapx: Vec::new(),
          _grpprlChpx: Vec::new(),
          _numberText: number_text,
      }
    }

    // Java: ListLevel(DocumentInputStream stream, int offset)
    pub fn new_from_stream(stream: &mut DocumentInputStream, offset: i32) -> Self {
      // Java: super();
      // Java: stream.seek((long)offset);
      stream.seek(offset as i64);
      // Java: byte[] buff = new byte[28];
      let mut buff = vec![0u8; 28];
      // Java: stream.read(buff, 0, buff.length);
      stream.read(&mut buff);
      // Java: int off = 0;
      let mut off = 0usize;
      // Java: this._iStartAt = LittleEndian.getInt(buff, off);
      let i_start_at = le_get_int(&buff, off);
      // Java: off += 4;
      off += 4;
      // Java: this._nfc = LittleEndian.getByte(buff, off++);
      let nfc = buff[off] as i8; off += 1;
      // Java: this._info = LittleEndian.getByte(buff, off++);
      let info = buff[off] as i8; off += 1;
      // Java: this._rgbxchNums = new byte[9];
      let mut rgbxch_nums = vec![0u8; 9];
      // Java: System.arraycopy(buff, off, this._rgbxchNums, 0, 9);
      rgbxch_nums.copy_from_slice(&buff[off..off+9]);
      // Java: off += 9;
      off += 9;
      // Java: this._ixchFollow = LittleEndian.getByte(buff, off++);
      let ixch_follow = buff[off] as i8; off += 1;
      // Java: this._dxaIndentSav = LittleEndian.getInt(buff, off);
      let dxa_indent_sav = le_get_int(&buff, off);
      // Java: off += 4;
      off += 4;
      // Java: this._unused = LittleEndian.getInt(buff, off);
      let unused = le_get_int(&buff, off);
      // Java: off += 4;
      off += 4;
      // Java: this._cbGrpprlChpx = LittleEndian.getUByte(buff, off++);
      let cb_grpprl_chpx = (buff[off] as u8) as i16; off += 1;
      // Java: this._cbGrpprlPapx = LittleEndian.getUByte(buff, off++);
      let cb_grpprl_papx = (buff[off] as u8) as i16; off += 1;
      // Java: this._ilvlRestartLim = LittleEndian.getByte(buff, off++);
      let ilvl_restart_lim = buff[off] as i8; off += 1;
      // Java: this._grfhic = LittleEndian.getByte(buff, off++);
      let grfhic = buff[off] as i8;
      // Java: this._grpprlPapx = new byte[this._cbGrpprlPapx];
      let mut grpprl_papx = vec![0u8; cb_grpprl_papx as usize];
      // Java: this._grpprlChpx = new byte[this._cbGrpprlChpx];
      let mut grpprl_chpx = vec![0u8; cb_grpprl_chpx as usize];
      // Java: stream.read(this._grpprlPapx, 0, this._cbGrpprlPapx);
      stream.read(&mut grpprl_papx);
      // Java: stream.read(this._grpprlChpx, 0, this._cbGrpprlChpx);
      stream.read(&mut grpprl_chpx);
      // Java: int numberTextLength = stream.readShort();
      let number_text_length = stream.readShort() as i32;
      let mut number_text = Vec::new();
      // Java: if (numberTextLength > 0)
        if number_text_length > 0
      {
         // Java: this._numberText = new char[numberTextLength];
         number_text = vec!['\0'; number_text_length as usize];
         // Java: if (numberTextLength << 1 <= off)
            if (number_text_length << 1) <= (off as i32)
         {
            // Java: off = 0;
            let mut read_off = 0usize;
            // Java: stream.read(buff, 0, numberTextLength << 1);
            let read_len = (number_text_length << 1) as i32;
            let mut read_buf = vec![0u8; read_len as usize];
            stream.read(&mut read_buf);
            // Java: for(int i = 0; i < numberTextLength; ++i)
                for i in 0..(number_text_length as usize)
            {
               // Java: this._numberText[i] = (char)LittleEndian.getUShort(buff, off);
               number_text[i] = char::from_u32(le_get_ushort(&read_buf, read_off) as u32).unwrap_or('\0');
               // Java: off += 2;
               read_off += 2;
            }
         }
         // Java: else
            else
         {
            // Java: for(int i = 0; i < numberTextLength; ++i)
                for i in 0..(number_text_length as usize)
            {
               // Java: this._numberText[i] = (char)stream.readShort();
               number_text[i] = char::from_u32(stream.readShort() as u16 as u32).unwrap_or('\0');
            }
         }
      }
      ListLevel {
          _iStartAt: i_start_at,
          _nfc: nfc,
          _info: info,
          _rgbxchNums: rgbxch_nums,
          _ixchFollow: ixch_follow,
          _dxaIndentSav: dxa_indent_sav,
          _unused: unused,
          _cbGrpprlChpx: cb_grpprl_chpx,
          _cbGrpprlPapx: cb_grpprl_papx,
          _ilvlRestartLim: ilvl_restart_lim,
          _grfhic: grfhic,
          _grpprlPapx: grpprl_papx,
          _grpprlChpx: grpprl_chpx,
          _numberText: number_text,
      }
    }

    // Java: int getStartAt()
    pub fn getStartAt(&self) -> i32 {
      // Java: return this._iStartAt;
      self._iStartAt
    }

    // Java: void setStartAt(int iStartAt)
    pub fn setStartAt(&mut self, i_start_at: i32) {
      // Java: this._iStartAt = iStartAt;
      self._iStartAt = i_start_at;
    }

    // Java: byte getNumberFormat()
    pub fn getNumberFormat(&self) -> i8 {
      // Java: return this._nfc;
      self._nfc
    }

    // Java: void setNumberFormat(byte numberFormatCode)
    pub fn setNumberFormat(&mut self, number_format_code: i8) {
      // Java: this._nfc = numberFormatCode;
      self._nfc = number_format_code;
    }

    // Java: byte getAlignment()
    pub fn getAlignment(&self) -> i8 {
      // Java: return (byte)_jc.getValue(this._info);
      get_jc().getValue(self._info as i32) as i8
    }

    // Java: void setAlignment(byte alignment)
    pub fn setAlignment(&mut self, alignment: i8) {
      // Java: this._info = (byte)_jc.setValue(this._info, alignment);
      self._info = get_jc().setValue(self._info as i32, alignment as i32) as i8;
    }

    // Java: boolean isLegal()
    pub fn isLegal(&self) -> bool {
      // Java: return _fLegal.getValue(this._info) == 1;
      get_f_legal().getValue(self._info as i32) == 1
    }

    // Java: void setFLegal(boolean fLegal)
    pub fn setFLegal(&mut self, f_legal_val: bool) {
      // Java: int legal = 0;
      let mut legal = 0;
      // Java: if (fLegal)
        if f_legal_val
      {
         // Java: legal = 1;
         legal = 1;
      }
      // Java: this._info = (byte)_fLegal.setValue(this._info, legal);
      self._info = get_f_legal().setValue(self._info as i32, legal) as i8;
    }

    // Java: boolean isNoRestart()
    pub fn isNoRestart(&self) -> bool {
      // Java: return _fNoRestart.getValue(this._info) == 1;
      get_f_no_restart().getValue(self._info as i32) == 1
    }

    // Java: void setFNoRestart(boolean fNoRestart)
    pub fn setFNoRestart(&mut self, f_no_restart_val: bool) {
      // Java: int noRestart = 0;
      let mut no_restart = 0;
      // Java: if (fNoRestart)
        if f_no_restart_val
      {
         // Java: noRestart = 1;
         no_restart = 1;
      }
      // Java: this._info = (byte)_fNoRestart.setValue(this._info, noRestart);
      self._info = get_f_no_restart().setValue(self._info as i32, no_restart) as i8;
    }

    // Java: boolean isIndentSav()
    pub fn isIndentSav(&self) -> bool {
      // Java: return _fIndentSav.getValue(this._info) == 1;
      get_f_indent_sav().getValue(self._info as i32) == 1
    }

    // Java: void setFIndentSav(boolean fIndentSav)
    pub fn setFIndentSav(&mut self, f_indent_sav_val: bool) {
      // Java: int indentSav = 0;
      let mut indent_sav = 0;
      // Java: if (fIndentSav)
        if f_indent_sav_val
      {
         // Java: indentSav = 1;
         indent_sav = 1;
      }
      // Java: this._info = (byte)_fIndentSav.setValue(this._info, indentSav);
      self._info = get_f_indent_sav().setValue(self._info as i32, indent_sav) as i8;
    }

    // Java: boolean isConverted()
    pub fn isConverted(&self) -> bool {
      // Java: return _fConverted.getValue(this._info) == 1;
      get_f_converted().getValue(self._info as i32) == 1
    }

    // Java: void setFConverted(boolean fConverted)
    pub fn setFConverted(&mut self, f_converted_val: bool) {
      // Java: int converted = 0;
      let mut converted = 0;
      // Java: if (fConverted)
        if f_converted_val
      {
         // Java: converted = 1;
         converted = 1;
      }
      // Java: this._info = (byte)_fConverted.setValue(this._info, converted);
      self._info = get_f_converted().setValue(self._info as i32, converted) as i8;
    }

    // Java: boolean isTentative()
    pub fn isTentative(&self) -> bool {
      // Java: return _fTentative.getValue(this._info) == 1;
      get_f_tentative().getValue(self._info as i32) == 1
    }

    // Java: void setFTentative(boolean fTentative)
    pub fn setFTentative(&mut self, f_tentative_val: bool) {
      // Java: int tentative = 0;
      let mut tentative = 0;
      // Java: if (fTentative)
        if f_tentative_val
      {
         // Java: tentative = 1;
         tentative = 1;
      }
      // Java: this._info = (byte)_fNoRestart.setValue(this._info, tentative);
      self._info = get_f_no_restart().setValue(self._info as i32, tentative) as i8;
    }

    // Java: byte getIxchFollow()
    pub fn getIxchFollow(&self) -> i8 {
      // Java: return this._ixchFollow;
      self._ixchFollow
    }

    // Java: void setIxchFollow(int ixchFollow)
    pub fn setIxchFollow(&mut self, ixch_follow: i32) {
      // Java: this._ixchFollow = (byte)ixchFollow;
      self._ixchFollow = ixch_follow as i8;
    }

    // Java: int getDxaIndentSav()
    pub fn getDxaIndentSav(&self) -> i32 {
      // Java: return this._dxaIndentSav;
      self._dxaIndentSav
    }

    // Java: void setDxaIndentSav(int dxaIndentSav)
    pub fn setDxaIndentSav(&mut self, dxa_indent_sav: i32) {
      // Java: this._dxaIndentSav = dxaIndentSav;
      self._dxaIndentSav = dxa_indent_sav;
    }

    // Java: short getCbGrpprlChpx()
    pub fn getCbGrpprlChpx(&self) -> i16 {
      // Java: return this._cbGrpprlChpx;
      self._cbGrpprlChpx
    }

    // Java: void setCbGrpprlChpx(short cbGrpprlChpx)
    pub fn setCbGrpprlChpx(&mut self, cb_grpprl_chpx: i16) {
      // Java: this._cbGrpprlChpx = cbGrpprlChpx;
      self._cbGrpprlChpx = cb_grpprl_chpx;
    }

    // Java: short getCbGrpprlPapx()
    pub fn getCbGrpprlPapx(&self) -> i16 {
      // Java: return this._cbGrpprlPapx;
      self._cbGrpprlPapx
    }

    // Java: void setCbGrpprlPapx(short cbGrpprlPapx)
    pub fn setCbGrpprlPapx(&mut self, cb_grpprl_papx: i16) {
      // Java: this._cbGrpprlPapx = cbGrpprlPapx;
      self._cbGrpprlPapx = cb_grpprl_papx;
    }

    // Java: byte[] getRgbxchNums()
    pub fn getRgbxchNums(&self) -> &[u8] {
      // Java: return this._rgbxchNums;
      &self._rgbxchNums
    }

    // Java: void setRgbxchNums(byte[] rgbxchNums)
    pub fn setRgbxchNums(&mut self, rgbxch_nums: &[u8]) {
      // Java: IOAssert.assertNotNull("rgbxchNums should not be null", rgbxchNums);
      // Java: if (rgbxchNums != null)
      {
         // Java: this._rgbxchNums = (byte[]) rgbxchNums.clone();
         self._rgbxchNums = rgbxch_nums.to_vec();
      }
    }

    // Java: String getNumberText()
    pub fn getNumberText(&self) -> Option<String> {
      // Java: return this._numberText != null ? new String(this._numberText) : null;
      if self._numberText.is_empty() { None } else { Some(self._numberText.iter().collect()) }
    }

    // Java: void setNumberText(String numberText)
    pub fn setNumberText(&mut self, number_text: Option<&str>) {
      // Java: this._numberText = numberText == null ? null : numberText.toCharArray();
      self._numberText = match number_text {
          Some(s) => s.chars().collect(),
          None => Vec::new(),
      };
    }

    // Java: byte[] getLevelPapProperties()
    pub fn getLevelPapProperties(&self) -> &[u8] {
      // Java: return this._grpprlPapx;
      &self._grpprlPapx
    }

    // Java: void setLevelPapProperties(byte[] grpprl)
    pub fn setLevelPapProperties(&mut self, grpprl: &[u8]) {
      // Java: IOAssert.assertNotNull("grpprl should not be null", grpprl);
      // Java: if (grpprl != null)
      {
         // Java: this._grpprlPapx = (byte[]) grpprl.clone();
         self._grpprlPapx = grpprl.to_vec();
      }
    }

    // Java: byte[] getLevelRunProperties()
    pub fn getLevelRunProperties(&self) -> &[u8] {
      // Java: return this._grpprlChpx;
      &self._grpprlChpx
    }

    // Java: void setLevelRunProperties(byte[] grpprl)
    pub fn setLevelRunProperties(&mut self, grpprl: &[u8]) {
      // Java: IOAssert.assertNotNull("grpprl should not be null", grpprl);
      // Java: if (grpprl != null)
      {
         // Java: this._grpprlChpx = (byte[]) grpprl.clone();
         self._grpprlChpx = grpprl.to_vec();
      }
    }

    // Java: byte getILvlRestartLim()
    pub fn getILvlRestartLim(&self) -> i8 {
      // Java: return this._ilvlRestartLim;
      self._ilvlRestartLim
    }

    // Java: void setILvlRestartLim(byte iLvlRestartLim)
    pub fn setILvlRestartLim(&mut self, i_lvl_restart_lim: i8) {
      // Java: this._ilvlRestartLim = iLvlRestartLim;
      self._ilvlRestartLim = i_lvl_restart_lim;
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

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return super.hashCode();
      0
    }

    // Java: boolean equals(Object obj)
    pub fn equals(&self, other: &ListLevel) -> bool {
      // Java: if (obj == null)
      {
         // Java: return false;
      }
      // Java: else
      {
         // Java: ListLevel lvl = (ListLevel)obj;
         // Java: return this._cbGrpprlChpx == lvl._cbGrpprlChpx && lvl._cbGrpprlPapx == this._cbGrpprlPapx && lvl._unused == this._unused && lvl._dxaIndentSav == this._dxaIndentSav && Arrays.equals(lvl._grpprlChpx, this._grpprlChpx) && Arrays.equals(lvl._grpprlPapx, this._grpprlPapx) && lvl._info == this._info && lvl._iStartAt == this._iStartAt && lvl._ixchFollow == this._ixchFollow && lvl._nfc == this._nfc && Arrays.equals(lvl._numberText, this._numberText) && Arrays.equals(lvl._rgbxchNums, this._rgbxchNums) && lvl._ilvlRestartLim == this._ilvlRestartLim && lvl._grfhic == this._grfhic;
         self._cbGrpprlChpx == other._cbGrpprlChpx &&
         self._cbGrpprlPapx == other._cbGrpprlPapx &&
         self._unused == other._unused &&
         self._dxaIndentSav == other._dxaIndentSav &&
         self._grpprlChpx == other._grpprlChpx &&
         self._grpprlPapx == other._grpprlPapx &&
         self._info == other._info &&
         self._iStartAt == other._iStartAt &&
         self._ixchFollow == other._ixchFollow &&
         self._nfc == other._nfc &&
         self._numberText == other._numberText &&
         self._rgbxchNums == other._rgbxchNums &&
         self._ilvlRestartLim == other._ilvlRestartLim &&
         self._grfhic == other._grfhic
      }
    }

    // Java: byte[] toByteArray()
    pub fn toByteArray(&self) -> Vec<u8> {
      // Java: byte[] buf = new byte[this.getSizeInBytes()];
      let mut buf = vec![0u8; self.getSizeInBytes() as usize];
      // Java: int offset = 0;
      let mut offset = 0usize;
      // Java: LittleEndian.putInt(buf, offset, this._iStartAt);
      le_put_int(&mut buf, offset, self._iStartAt);
      // Java: offset += 4;
      offset += 4;
      // Java: buf[offset++] = this._nfc;
      buf[offset] = self._nfc as u8; offset += 1;
      // Java: buf[offset++] = this._info;
      buf[offset] = self._info as u8; offset += 1;
      // Java: System.arraycopy(this._rgbxchNums, 0, buf, offset, 9);
      buf[offset..offset+9].copy_from_slice(&self._rgbxchNums[..9]);
      // Java: offset += 9;
      offset += 9;
      // Java: buf[offset++] = this._ixchFollow;
      buf[offset] = self._ixchFollow as u8; offset += 1;
      // Java: LittleEndian.putInt(buf, offset, this._dxaIndentSav);
      le_put_int(&mut buf, offset, self._dxaIndentSav);
      // Java: offset += 4;
      offset += 4;
      // Java: LittleEndian.putInt(buf, offset, this._unused);
      le_put_int(&mut buf, offset, self._unused);
      // Java: offset += 4;
      offset += 4;
      // Java: buf[offset++] = (byte)this._cbGrpprlChpx;
      buf[offset] = self._cbGrpprlChpx as u8; offset += 1;
      // Java: buf[offset++] = (byte)this._cbGrpprlPapx;
      buf[offset] = self._cbGrpprlPapx as u8; offset += 1;
      // Java: buf[offset++] = this._ilvlRestartLim;
      buf[offset] = self._ilvlRestartLim as u8; offset += 1;
      // Java: buf[offset++] = this._grfhic;
      buf[offset] = self._grfhic as u8; offset += 1;
      // Java: System.arraycopy(this._grpprlPapx, 0, buf, offset, this._cbGrpprlPapx);
      let papx_len = self._cbGrpprlPapx as usize;
      buf[offset..offset+papx_len].copy_from_slice(&self._grpprlPapx[..papx_len]);
      // Java: offset += this._cbGrpprlPapx;
      offset += papx_len;
      // Java: System.arraycopy(this._grpprlChpx, 0, buf, offset, this._cbGrpprlChpx);
      let chpx_len = self._cbGrpprlChpx as usize;
      buf[offset..offset+chpx_len].copy_from_slice(&self._grpprlChpx[..chpx_len]);
      // Java: offset += this._cbGrpprlChpx;
      offset += chpx_len;
      // Java: if (this._numberText == null)
        if self._numberText.is_empty()
      {
         // Java: LittleEndian.putUShort(buf, offset, 0);
         le_put_ushort(&mut buf, offset, 0);
      }
      // Java: else
        else
      {
         // Java: LittleEndian.putUShort(buf, offset, this._numberText.length);
         le_put_ushort(&mut buf, offset, self._numberText.len() as u16);
         // Java: offset += 2;
         offset += 2;
         // Java: int i = 0;
         // Java: for(int len = this._numberText.length; i < len; ++i)
            for i in 0..self._numberText.len()
         {
            // Java: LittleEndian.putUShort(buf, offset, this._numberText[i]);
            le_put_ushort(&mut buf, offset, self._numberText[i] as u16);
            // Java: offset += 2;
            offset += 2;
         }
      }
      // Java: return buf;
      buf
    }

    // Java: int getSizeInBytes()
    pub fn getSizeInBytes(&self) -> i32 {
      // Java: int result = 28 + this._cbGrpprlChpx + this._cbGrpprlPapx + 2;
      let mut result = 28 + self._cbGrpprlChpx as i32 + self._cbGrpprlPapx as i32 + 2;
      // Java: if (this._numberText != null)
        if !self._numberText.is_empty()
      {
         // Java: result += this._numberText.length * 2;
         result += self._numberText.len() as i32 * 2;
      }
      // Java: return result;
      result
    }

}
