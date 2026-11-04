// Java: import cn.wps.base.assertion.IOAssert
// Java: import java.io.IOException
// Java: import java.util.Arrays
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.BitField
// Java: import org.apache.poi.util.BitFieldFactory
// Java: import org.apache.poi.util.LittleEndian

struct BitField { _mask: i32, _shift_count: i32 }
impl BitField {
    fn new(mask: i32) -> Self {
        let mut count = 0; let mut bp = mask;
        if mask != 0 { while (bp & 1) == 0 { count += 1; bp >>= 1; } }
        BitField { _mask: mask, _shift_count: count }
    }
    fn getValue(&self, holder: i32) -> i32 { ((holder & self._mask) as u32 >> self._shift_count) as i32 }
    fn setValue(&self, holder: i32, value: i32) -> i32 { (holder & !self._mask) | ((value << self._shift_count) & self._mask) }
}

fn le_get_short(buf: &[u8], offset: usize) -> i16 {
    ((buf[offset] as i16) & 0xFF) | (((buf[offset+1] as i16) & 0xFF) << 8)
}
fn le_put_short(buf: &mut [u8], offset: usize, value: i16) {
    buf[offset] = (value & 0xFF) as u8;
    buf[offset+1] = ((value >> 8) & 0xFF) as u8;
}

// Java field: BitField _prq
fn bf_prq() -> BitField { BitField::new(3) }
// Java field: BitField _fTrueType
fn bf_fTrueType() -> BitField { BitField::new(4) }
// Java field: BitField _ff
fn bf_ff() -> BitField { BitField::new(112) }

// Java: public class Ffn
pub struct Ffn {
    // Java field: int _cbFfnM1
    pub(crate) _cbFfnM1: i32,
    // Java field: byte _info
    _info: i8,
    // Java field: short _wWeight
    pub(crate) _wWeight: i16,
    // Java field: short _chs
    pub(crate) _chs: i16,
    // Java field: byte _ixchSzAlt
    pub(crate) _ixchSzAlt: i8,
    // Java field: byte[] _panose
    pub(crate) _panose: Vec<u8>,
    // Java field: byte[] _fontSig
    pub(crate) _fontSig: Vec<u8>,
    // Java field: char[] _xszFfn
    _xszFfn: Vec<u16>,
    // Java field: int _xszFfnLength
    _xszFfnLength: i32,
}

// Java: public class Ffn
impl Ffn {
    // Java: Ffn()
    pub fn new() -> Self {
      // Java: super();
      // Java: this._xszFfn = "\u0000\u0000".toCharArray();
      let xszFfn: Vec<u16> = vec![0, 0];
      // Java: this._xszFfnLength = this._xszFfn.length;
      let xszFfnLength = xszFfn.len() as i32;
      // Java: this._ixchSzAlt = 1;
      let ixchSzAlt: i8 = 1;
      // Java: this._cbFfnM1 = 39 + this._xszFfnLength * 2;
      let cbFfnM1 = 39 + xszFfnLength * 2;
      Ffn {
          _cbFfnM1: cbFfnM1, _info: 0, _wWeight: 0, _chs: 0,
          _ixchSzAlt: ixchSzAlt, _panose: vec![0u8; 10], _fontSig: vec![0u8; 24],
          _xszFfn: xszFfn, _xszFfnLength: xszFfnLength,
      }
    }

    // Java: Ffn(byte[] buf, int offset)
    pub fn new_from_bytes(buf: &[u8], offset: usize) -> Self {
      // Java: super();
      // Java: int offsetTmp = offset;
      let offset_tmp = offset;
      // Java: this._cbFfnM1 = LittleEndian.getUnsignedByte(buf, offset);
      let cb_ffn_m1 = buf[offset] as i32;
      // Java: ++offset;
      let mut offset = offset + 1;
      // Java: this._info = buf[offset];
      let info = buf[offset] as i8;
      // Java: ++offset;
      offset += 1;
      // Java: this._wWeight = LittleEndian.getShort(buf, offset);
      let w_weight = le_get_short(buf, offset);
      // Java: offset += 2;
      offset += 2;
      // Java: this._chs = (short)Math.abs(buf[offset]);
      let chs = (buf[offset] as i8).unsigned_abs() as i16;
      // Java: ++offset;
      offset += 1;
      // Java: this._ixchSzAlt = buf[offset];
      let ixch_sz_alt = buf[offset] as i8;
      // Java: ++offset;
      offset += 1;
      // Java: System.arraycopy(buf, offset, this._panose, 0, this._panose.length);
      let mut panose = vec![0u8; 10];
      panose.copy_from_slice(&buf[offset..offset+10]);
      // Java: offset += this._panose.length;
      offset += 10;
      // Java: System.arraycopy(buf, offset, this._fontSig, 0, this._fontSig.length);
      let mut font_sig = vec![0u8; 24];
      font_sig.copy_from_slice(&buf[offset..offset+24]);
      // Java: offset += this._fontSig.length;
      offset += 24;
      // Java: offsetTmp = offset - offsetTmp;
      let offset_tmp = offset - offset_tmp;
      // Java: this._xszFfnLength = (this.getSize() - offsetTmp) / 2;
      let xsz_ffn_length = ((cb_ffn_m1 + 1) - offset_tmp as i32) / 2;
      // Java: this._xszFfn = new char[this._xszFfnLength];
      let mut xsz_ffn = vec![0u16; xsz_ffn_length as usize];
      // Java: for(int i = 0; i < this._xszFfnLength; ++i)
      for i in 0..xsz_ffn_length as usize {
         // Java: this._xszFfn[i] = (char)LittleEndian.getShort(buf, offset);
         xsz_ffn[i] = le_get_short(buf, offset) as u16;
         // Java: offset += 2;
         offset += 2;
      }
      Ffn {
          _cbFfnM1: cb_ffn_m1, _info: info, _wWeight: w_weight, _chs: chs,
          _ixchSzAlt: ixch_sz_alt, _panose: panose, _fontSig: font_sig,
          _xszFfn: xsz_ffn, _xszFfnLength: xsz_ffn_length,
      }
    }

    // Java: Ffn(DocumentInputStream stream, int offset)
    pub fn new_from_stream(stream: &mut DocumentInputStream, offset: i32) -> Self {
      // Java: super();
      // Java: int offsetTmp = offset;
      let offset_tmp = offset;
      // Java: stream.seek((long)offset);
      stream.seek(offset as i64);
      // Java: this._cbFfnM1 = stream.readUByte();
      let cb_ffn_m1 = stream.readUByte() as i32;
      // Java: ++offset;
      let mut offset = offset + 1;
      // Java: this._info = stream.readByte();
      let info = stream.readByte();
      // Java: ++offset;
      offset += 1;
      // Java: this._wWeight = stream.readShort();
      let w_weight = stream.readShort();
      // Java: offset += 2;
      offset += 2;
      // Java: this._chs = (short)Math.abs(stream.readByte());
      let chs = stream.readByte().unsigned_abs() as i16;
      // Java: ++offset;
      offset += 1;
      // Java: this._ixchSzAlt = stream.readByte();
      let ixch_sz_alt = stream.readByte();
      // Java: ++offset;
      offset += 1;
      // Java: stream.read(this._panose, 0, this._panose.length);
      let mut panose = vec![0u8; 10];
      stream.read(&mut panose);
      // Java: offset += this._panose.length;
      offset += 10;
      // Java: stream.read(this._fontSig, 0, this._fontSig.length);
      let mut font_sig = vec![0u8; 24];
      stream.read(&mut font_sig);
      // Java: offset += this._fontSig.length;
      offset += 24;
      // Java: offsetTmp = offset - offsetTmp;
      let offset_tmp = offset - offset_tmp;
      // Java: this._xszFfnLength = (this.getSize() - offsetTmp) / 2;
      let mut xsz_ffn_length = ((cb_ffn_m1 + 1) - offset_tmp) / 2;
      // Java: this._xszFfnLength = this._xszFfnLength < 0 ? 0 : this._xszFfnLength;
      if xsz_ffn_length < 0 { xsz_ffn_length = 0; }
      // Java: this._xszFfn = new char[this._xszFfnLength];
      let mut xsz_ffn = vec![0u16; xsz_ffn_length as usize];
      // Java: for(int i = 0; i < this._xszFfnLength; ++i)
      for i in 0..xsz_ffn_length as usize {
         // Java: this._xszFfn[i] = (char)stream.readShort();
         xsz_ffn[i] = stream.readShort() as u16;
         // Java: offset += 2;
         offset += 2;
      }
      Ffn {
          _cbFfnM1: cb_ffn_m1, _info: info, _wWeight: w_weight, _chs: chs,
          _ixchSzAlt: ixch_sz_alt, _panose: panose, _fontSig: font_sig,
          _xszFfn: xsz_ffn, _xszFfnLength: xsz_ffn_length,
      }
    }

    // Java: int get_cbFfnM1()
    pub fn get_cbFfnM1(&self) -> i32 {
      // Java: return this._cbFfnM1;
      self._cbFfnM1
    }

    // Java: void set_prq(int value)
    pub fn set_prq(&mut self, value: i32) {
      // Java: this._info = (byte)_prq.setValue(this._info, value);
      self._info = bf_prq().setValue(self._info as i32, value) as i8;
    }

    // Java: byte get_prq()
    pub fn get_prq(&self) -> i8 {
      // Java: return (byte)_prq.getValue(this._info);
      bf_prq().getValue(self._info as i32) as i8
    }

    // Java: void set_fTrueType(boolean value)
    pub fn set_fTrueType(&mut self, value: bool) {
      // Java: this._info = (byte)_fTrueType.setValue(this._info, value ? 1 : 0);
      self._info = bf_fTrueType().setValue(self._info as i32, if value { 1 } else { 0 }) as i8;
    }

    // Java: boolean isTrueType()
    pub fn isTrueType(&self) -> bool {
      // Java: return 1 == _fTrueType.getValue(this._info);
      1 == bf_fTrueType().getValue(self._info as i32)
    }

    // Java: void set_ff(int value)
    pub fn set_ff(&mut self, value: i32) {
      // Java: this._info = (byte)_ff.setValue(this._info, value);
      self._info = bf_ff().setValue(self._info as i32, value) as i8;
    }

    // Java: byte get_ff()
    pub fn get_ff(&self) -> i8 {
      // Java: return (byte)_ff.getValue(this._info);
      bf_ff().getValue(self._info as i32) as i8
    }

    // Java: void setWeight(short value)
    pub fn setWeight(&mut self, value: i16) {
      // Java: this._wWeight = value;
      self._wWeight = value;
    }

    // Java: short getWeight()
    pub fn getWeight(&self) -> i16 {
      // Java: return this._wWeight;
      self._wWeight
    }

    // Java: void setChs(short chs)
    pub fn setChs(&mut self, chs: i16) {
      // Java: this._chs = chs;
      self._chs = chs;
    }

    // Java: short getChs()
    pub fn getChs(&self) -> i16 {
      // Java: return this._chs;
      self._chs
    }

    // Java: void setPanose(byte[] panose)
    pub fn setPanose(&mut self, panose: Vec<u8>) {
      // Java: IOAssert.assertNotNull("panose should not be null!", panose);
      // Java: this._panose = panose;
      self._panose = panose;
    }

    // Java: byte[] getPanose()
    pub fn getPanose(&self) -> &[u8] {
      // Java: return this._panose;
      &self._panose
    }

    // Java: void setFontSig(byte[] fontSig)
    pub fn setFontSig(&mut self, font_sig: Vec<u8>) {
      // Java: IOAssert.assertNotNull("fontSig should not be null!", fontSig);
      // Java: this._fontSig = fontSig;
      self._fontSig = font_sig;
    }

    // Java: byte[] getFontSig()
    pub fn getFontSig(&self) -> &[u8] {
      // Java: return this._fontSig;
      &self._fontSig
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return this._cbFfnM1 + 1;
      self._cbFfnM1 + 1
    }

    // Java: String getMainFontName()
    pub fn getMainFontName(&self) -> String {
      // Java: int index;
      let mut index = 0;
      // Java: for(index = 0; index < this._xszFfnLength && this._xszFfn[index] != 0; ++index)
      while index < self._xszFfnLength as usize && self._xszFfn[index] != 0 {
          index += 1;
      }
      // Java: return new String(this._xszFfn, 0, index);
      String::from_utf16_lossy(&self._xszFfn[0..index])
    }

    // Java: void setMainFontName(String mainFontName)
    pub fn setMainFontName(&mut self, main_font_name: &str) {
      // Java: this._cbFfnM1 -= this._xszFfnLength * 2;
      self._cbFfnM1 -= self._xszFfnLength * 2;
      // Java: String data = mainFontName + '\u0000' + new String(this._xszFfn, this._ixchSzAlt, this._xszFfn.length - this._ixchSzAlt) + '\u0000';
      let alt_start = self._ixchSzAlt as usize;
      let alt_part: String = String::from_utf16_lossy(&self._xszFfn[alt_start..]);
      let data: Vec<u16> = main_font_name.encode_utf16()
          .chain(std::iter::once(0u16))
          .chain(alt_part.encode_utf16())
          .chain(std::iter::once(0u16))
          .collect();
      // Java: this._ixchSzAlt = (byte)(mainFontName.length() + 1);
      self._ixchSzAlt = (main_font_name.chars().count() + 1) as i8;
      // Java: this._xszFfn = data.toCharArray();
      self._xszFfn = data;
      // Java: this._xszFfnLength = this._xszFfn.length;
      self._xszFfnLength = self._xszFfn.len() as i32;
      // Java: this._cbFfnM1 += this._xszFfnLength * 2;
      self._cbFfnM1 += self._xszFfnLength * 2;
    }

    // Java: String getAltFontName()
    pub fn getAltFontName(&self) -> String {
      // Java: int index;
      let mut index = self._ixchSzAlt as usize;
      // Java: for(index = this._ixchSzAlt; index < this._xszFfnLength && this._xszFfn[index] != 0; ++index)
      while index < self._xszFfnLength as usize && self._xszFfn[index] != 0 {
          index += 1;
      }
      // Java: return this._ixchSzAlt >= this._xszFfn.length ? "" : new String(this._xszFfn, this._ixchSzAlt, index - this._ixchSzAlt);
      if self._ixchSzAlt as usize >= self._xszFfn.len() {
          String::new()
      } else {
          String::from_utf16_lossy(&self._xszFfn[self._ixchSzAlt as usize..index])
      }
    }

    // Java: void setAltFontName(String altFontName)
    pub fn setAltFontName(&mut self, alt_font_name: &str) {
      // Java: this._cbFfnM1 -= this._xszFfnLength * 2;
      self._cbFfnM1 -= self._xszFfnLength * 2;
      // Java: String data = new String(this._xszFfn, 0, this._ixchSzAlt) + '\u0000' + altFontName;
      let main_part: Vec<u16> = self._xszFfn[0..self._ixchSzAlt as usize].to_vec();
      let data: Vec<u16> = main_part.into_iter()
          .chain(std::iter::once(0u16))
          .chain(alt_font_name.encode_utf16())
          .collect();
      // Java: ++this._ixchSzAlt;
      self._ixchSzAlt += 1;
      // Java: this._xszFfn = data.toCharArray();
      self._xszFfn = data;
      // Java: this._xszFfnLength = this._xszFfn.length + 1;
      self._xszFfnLength = self._xszFfn.len() as i32 + 1;
      // Java: this._cbFfnM1 += this._xszFfnLength * 2;
      self._cbFfnM1 += self._xszFfnLength * 2;
    }

    // Java: byte[] toByteArray()
    pub fn toByteArray(&self) -> Vec<u8> {
      // Java: int offset = 0;
      let mut offset: usize = 0;
      // Java: byte[] buf = new byte[this.getSize()];
      let mut buf = vec![0u8; self.getSize() as usize];
      // Java: buf[offset] = (byte)this._cbFfnM1;
      buf[offset] = self._cbFfnM1 as u8;
      // Java: ++offset;
      offset += 1;
      // Java: buf[offset] = this._info;
      buf[offset] = self._info as u8;
      // Java: ++offset;
      offset += 1;
      // Java: LittleEndian.putShort(buf, offset, this._wWeight);
      le_put_short(&mut buf, offset, self._wWeight);
      // Java: offset += 2;
      offset += 2;
      // Java: buf[offset] = (byte)this._chs;
      buf[offset] = self._chs as u8;
      // Java: ++offset;
      offset += 1;
      // Java: buf[offset] = this._ixchSzAlt;
      buf[offset] = self._ixchSzAlt as u8;
      // Java: ++offset;
      offset += 1;
      // Java: System.arraycopy(this._panose, 0, buf, offset, this._panose.length);
      buf[offset..offset+self._panose.len()].copy_from_slice(&self._panose);
      // Java: offset += this._panose.length;
      offset += self._panose.len();
      // Java: System.arraycopy(this._fontSig, 0, buf, offset, this._fontSig.length);
      buf[offset..offset+self._fontSig.len()].copy_from_slice(&self._fontSig);
      // Java: offset += this._fontSig.length;
      offset += self._fontSig.len();
      // Java: for(int i = 0; i < this._xszFfn.length; ++i)
      for i in 0..self._xszFfn.len() {
         // Java: LittleEndian.putShort(buf, offset, (short)this._xszFfn[i]);
         le_put_short(&mut buf, offset, self._xszFfn[i] as i16);
         // Java: offset += 2;
         offset += 2;
      }
      // Java: return buf;
      buf
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return super.hashCode();
      0
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Ffn) -> bool {
      // Java: if (o == null)
      // Java: boolean retVal = true;
      let mut ret_val = true;
      // Java: if (((Ffn)o).get_cbFfnM1() == this._cbFfnM1)
      if o.get_cbFfnM1() == self._cbFfnM1 {
          // Java: if (((Ffn)o)._info == this._info)
          if o._info == self._info {
              // Java: if (((Ffn)o)._wWeight == this._wWeight)
              if o._wWeight == self._wWeight {
                  // Java: if (((Ffn)o)._chs == this._chs)
                  if o._chs == self._chs {
                      // Java: if (((Ffn)o)._ixchSzAlt == this._ixchSzAlt)
                      if o._ixchSzAlt == self._ixchSzAlt {
                          // Java: if (Arrays.equals(((Ffn)o)._panose, this._panose))
                          if o._panose == self._panose {
                              // Java: if (Arrays.equals(((Ffn)o)._fontSig, this._fontSig))
                              if o._fontSig == self._fontSig {
                                  // Java: if (!Arrays.equals(((Ffn)o)._xszFfn, this._xszFfn))
                                  if o._xszFfn != self._xszFfn {
                                      // Java: retVal = false;
                                      ret_val = false;
                                  }
                              }
                              // Java: else
                              else { // Java: retVal = false;
                                  ret_val = false; }
                          }
                          // Java: else
                          else { // Java: retVal = false;
                              ret_val = false; }
                      }
                      // Java: else
                      else { // Java: retVal = false;
                          ret_val = false; }
                  }
                  // Java: else
                  else { // Java: retVal = false;
                      ret_val = false; }
              }
              // Java: else
              else { // Java: retVal = false;
                  ret_val = false; }
          }
          // Java: else
          else { // Java: retVal = false;
              ret_val = false; }
      }
      // Java: else
      else { // Java: retVal = false;
          ret_val = false; }
      // Java: return retVal;
      ret_val
    }

}
