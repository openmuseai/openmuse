// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian
// Java: import org.apache.poi.util.StringUtil

use super::Xst::Xst;
use std::io::Write;

fn le_put_short(buf: &mut [u8], offset: i32, value: i16) {
    let o = offset as usize;
    let v = value as u16;
    buf[o] = (v & 0xFF) as u8;
    buf[o + 1] = (v >> 8) as u8;
}

fn get_from_unicode_le(buf: &[u8], offset: i32, len: i32) -> String {
    let o = offset as usize;
    let l = len as usize;
    let chars: Vec<u16> = buf[o..o + l * 2]
        .chunks(2)
        .map(|c| u16::from_le_bytes([c[0], if c.len() > 1 { c[1] } else { 0 }]))
        .collect();
    String::from_utf16_lossy(&chars)
}

fn put_unicode_le(s: &str, buf: &mut [u8], offset: i32) {
    let o = offset as usize;
    for (i, ch) in s.encode_utf16().enumerate() {
        let bytes = ch.to_le_bytes();
        let pos = o + i * 2;
        if pos + 1 < buf.len() {
            buf[pos] = bytes[0];
            buf[pos + 1] = bytes[1];
        }
    }
}

// Java: public class StwUser
pub struct StwUser {
    // Java field: DocumentInputStream _stream
    _stream: Option<DocumentInputStream>,
    // Java field: short _fExtend
    _fExtend: i16,
    // Java field: short _nCount
    pub(crate) _nCount: i16,
    // Java field: short _cbExtra
    _cbExtra: i16,
    // Java field: int numberOfBytes
    numberOfBytes: i32,
    // Java field: ArrayList<Integer> keyLength
    pub(crate) keyLength: Vec<i32>,
    // Java field: ArrayList<String> keyMap
    pub(crate) keyMap: Vec<String>,
    // Java field: ArrayList<byte[]> extraMap
    pub(crate) extraMap: Vec<Vec<u8>>,
    // Java field: ArrayList<Integer> valueLength
    pub(crate) valueLength: Vec<i32>,
    // Java field: ArrayList<String> valueMap
    pub(crate) valueMap: Vec<String>,
}

// Java: public class StwUser
impl StwUser {
    // Java field: int EXTENDED_CHARACTER_BYTES
    pub const EXTENDED_CHARACTER_BYTES: i32 = 2;
    // Java field: short CBEXTRA
    pub const CBEXTRA: i16 = 4;

    // Java: StwUser()
    pub fn new() -> Self {
      // Java: super();
      StwUser { _stream: None, _fExtend: -1, _nCount: 0, _cbExtra: 0, numberOfBytes: 0, keyLength: Vec::new(), keyMap: Vec::new(), extraMap: Vec::new(), valueLength: Vec::new(), valueMap: Vec::new() }
    }

    // Java: StwUser(DocumentInputStream stream, int offset, int size)
    pub fn new_from_stream(mut stream: DocumentInputStream, mut offset: i32, _size: i32) -> Self {
      // Java: super();
      let mut this = StwUser { _stream: None, _fExtend: -1, _nCount: 0, _cbExtra: 0, numberOfBytes: 0, keyLength: Vec::new(), keyMap: Vec::new(), extraMap: Vec::new(), valueLength: Vec::new(), valueMap: Vec::new() };
      // Java: this._stream = stream;
      // Java: this._stream.seek((long)offset);
      stream.seek(offset as i64);
      // Java: this._fExtend = this._stream.readShort();
      this._fExtend = stream.readShort();
      // Java: offset += 2;
      offset += 2;
      // Java: this._nCount = this._stream.readShort();
      this._nCount = stream.readShort();
      // Java: offset += 2;
      offset += 2;
      // Java: this._cbExtra = this._stream.readShort();
      this._cbExtra = stream.readShort();
      // Java: offset += 2;
      offset += 2;
      // Java: int cchData = 0;
      let mut cch_data: i32;
      // Java: String data = null;
      // Java: byte[] ExtraData = new byte[this._cbExtra];
      // Java: for(int i = 0; i < this._nCount; ++i)
      for _i in 0..this._nCount
      {
         // Java: cchData = this._stream.readShort();
         cch_data = stream.readShort() as i32;
         // Java: offset += 2;
         offset += 2;
         // Java: this.keyLength.add(cchData);
         this.keyLength.push(cch_data);
         // Java: int length = cchData * 2;
         let length = cch_data * 2;
         // Java: byte[] strData = new byte[length];
         let mut str_data = vec![0u8; length as usize];
         // Java: this._stream.read(strData);
         stream.read_off(&mut str_data, 0, length);
         // Java: offset += length;
         offset += length;
         // Java: data = StringUtil.getFromUnicodeLE(strData, 0, cchData);
         let data = get_from_unicode_le(&str_data, 0, cch_data);
         // Java: this.keyMap.add(data);
         this.keyMap.push(data);
         // Java: String var17 = null;
         // Java: this._stream.read(ExtraData);
         let mut extra_data = vec![0u8; this._cbExtra as usize];
         stream.read_off(&mut extra_data, 0, this._cbExtra as i32);
         // Java: offset += this._cbExtra;
         offset += this._cbExtra as i32;
         // Java: this.extraMap.add(ExtraData);
         this.extraMap.push(extra_data);
      }
      // Java: for(int i = 0; i < this._nCount; ++i)
      for _i in 0..this._nCount
      {
         // Java: Xst mXst = new Xst(this._stream, offset);
         let m_xst = Xst::new_from_stream(&mut stream, offset);
         // Java: this.valueMap.add(mXst.getString());
         this.valueMap.push(m_xst.getString().to_string());
         // Java: this.valueLength.add(mXst.getNumberOfChars());
         this.valueLength.push(m_xst.getNumberOfChars());
         // Java: offset += mXst.getNumberOfChars() * 2 + 2;
         offset += m_xst.getNumberOfChars() * 2 + 2;
      }
      this._stream = Some(stream);
      this
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return this._nCount;
      self._nCount as i32
    }

    // Java: void setSize(short count)
    pub fn setSize(&mut self, count: i16) {
      // Java: this._nCount = count;
      self._nCount = count;
    }

    // Java: ArrayList<String> getKeyMap()
    pub fn getKeyMap(&self) -> &Vec<String> {
      // Java: return this.keyMap;
      &self.keyMap
    }

    // Java: ArrayList<String> getValueMap()
    pub fn getValueMap(&self) -> &Vec<String> {
      // Java: return this.valueMap;
      &self.valueMap
    }

    // Java: ArrayList<Integer> getKeyLengthMap()
    pub fn getKeyLengthMap(&self) -> &Vec<i32> {
      // Java: return this.keyLength;
      &self.keyLength
    }

    // Java: ArrayList<byte[]> getExtraMap()
    pub fn getExtraMap(&self) -> &Vec<Vec<u8>> {
      // Java: return this.extraMap;
      &self.extraMap
    }

    // Java: ArrayList<Integer> getValueLengthMap()
    pub fn getValueLengthMap(&self) -> &Vec<i32> {
      // Java: return this.valueLength;
      &self.valueLength
    }

    // Java: void writeTo(HWPFOutputStream out)
    pub fn writeTo(&self, out: &mut HWPFOutputStream) {
      // Java: if (this._nCount > 0)
      if self._nCount > 0
      {
         // Java: int offset = 0;
         let mut offset: i32 = 0;
         // Java: byte[] buf = new byte[6];
         let mut buf = vec![0u8; 6];
         // Java: LittleEndian.putShort(buf, offset, this._fExtend);
         le_put_short(&mut buf, offset, self._fExtend);
         // Java: offset += 2;
         offset += 2;
         // Java: LittleEndian.putShort(buf, offset, this._nCount);
         le_put_short(&mut buf, offset, self._nCount);
         // Java: offset += 2;
         offset += 2;
         // Java: LittleEndian.putShort(buf, offset, (short)4);
         le_put_short(&mut buf, offset, 4);
         // Java: offset += 2;
         offset += 2;
         // Java: out.write(buf);
         let _ = out.write(&buf);
         // Java: offset = 0;
         offset = 0;
         // Java: buf = new byte[this.numberOfBytes];
         let mut buf = vec![0u8; self.numberOfBytes as usize];
         // Java: for(int i = 0; i < this._nCount; ++i)
         for i in 0..self._nCount as usize
         {
            // Java: int key_length = (Integer)this.keyLength.get(i);
            let key_length = self.keyLength[i];
            // Java: LittleEndian.putShort(buf, offset, (short)key_length);
            le_put_short(&mut buf, offset, key_length as i16);
            // Java: offset += 2;
            offset += 2;
            // Java: StringUtil.putUnicodeLE((String)this.keyMap.get(i), buf, offset);
            put_unicode_le(&self.keyMap[i], &mut buf, offset);
            // Java: offset += ((String)this.keyMap.get(i)).length() * 2;
            offset += (self.keyMap[i].encode_utf16().count() as i32) * 2;
            // Java: System.arraycopy(this.extraMap.get(i), 0, buf, offset, ((byte[])this.extraMap.get(i)).length);
            let extra = &self.extraMap[i];
            let dst_offset = offset as usize;
            buf[dst_offset..dst_offset + extra.len()].copy_from_slice(extra);
            // Java: offset += ((byte[])this.extraMap.get(i)).length;
            offset += extra.len() as i32;
         }
         // Java: for(int i = 0; i < this._nCount; ++i)
         for i in 0..self._nCount as usize
         {
            // Java: int value_length = (Integer)this.valueLength.get(i);
            let value_length = self.valueLength[i];
            // Java: LittleEndian.putShort(buf, offset, (short)value_length);
            le_put_short(&mut buf, offset, value_length as i16);
            // Java: offset += 2;
            offset += 2;
            // Java: StringUtil.putUnicodeLE((String)this.valueMap.get(i), buf, offset);
            put_unicode_le(&self.valueMap[i], &mut buf, offset);
            // Java: offset += ((String)this.valueMap.get(i)).length() * 2;
            offset += (self.valueMap[i].encode_utf16().count() as i32) * 2;
         }
         // Java: out.write(buf);
         let _ = out.write(&buf);
      }
    }

    // Java: void setNumofBytes(int size)
    pub fn setNumofBytes(&mut self, size: i32) {
      // Java: this.numberOfBytes = size;
      self.numberOfBytes = size;
    }

}
