// Java: import cn.wps.base.assertion.IOAssert
// Java: import java.io.IOException
// Java: import java.io.UnsupportedEncodingException
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use std::io::Write;

fn le_put_short(buf: &mut [u8], offset: i32, value: i16) {
    let o = offset as usize;
    let v = value as u16;
    buf[o] = (v & 0xFF) as u8;
    buf[o + 1] = (v >> 8) as u8;
}

// Java: public class SttbfAssoc
pub struct SttbfAssoc {
    // Java field: short _fExtend
    _fExtend: i16,
    // Java field: short _cData
    pub(crate) _cData: i16,
    // Java field: short _cbExtra
    _cbExtra: i16,
    // Java field: short _cchData0
    _cchData0: i16,
    // Java field: HashMap<Integer, byte[]> sttbs
    sttbs: HashMap<i32, Option<Vec<u8>>>,
    // Java field: byte[] dataArray
    dataArray: Option<Vec<u8>>,
    // Java field: int _ibstAssocFileNext
    _ibstAssocFileNext: i32,
    // Java field: int _ibstAssocDot
    _ibstAssocDot: i32,
    // Java field: int _ibstAssocTitle
    _ibstAssocTitle: i32,
    // Java field: int _ibstAssocSubject
    _ibstAssocSubject: i32,
    // Java field: int _ibstAssocKeyWords
    _ibstAssocKeyWords: i32,
    // Java field: int _ibstAssocComments
    _ibstAssocComments: i32,
    // Java field: int _ibstAssocAuthor
    _ibstAssocAuthor: i32,
    // Java field: int _ibstAssocLastRevBy
    _ibstAssocLastRevBy: i32,
    // Java field: int _ibstAssocDataDoc
    _ibstAssocDataDoc: i32,
    // Java field: int _ibstAssocHeaderDoc
    _ibstAssocHeaderDoc: i32,
    // Java field: int _ibstAssocCriteria1
    _ibstAssocCriteria1: i32,
    // Java field: int _ibstAssocCriteria2
    _ibstAssocCriteria2: i32,
    // Java field: int _ibstAssocCriteria3
    _ibstAssocCriteria3: i32,
    // Java field: int _ibstAssocCriteria4
    _ibstAssocCriteria4: i32,
    // Java field: int _ibstAssocCriteria5
    _ibstAssocCriteria5: i32,
    // Java field: int _ibstAssocCriteria6
    _ibstAssocCriteria6: i32,
    // Java field: int _ibstAssocCriteria7
    _ibstAssocCriteria7: i32,
    // Java field: int _ibstAssocMax
    _ibstAssocMax: i32,
}

// Java: public class SttbfAssoc
impl SttbfAssoc {
    // Java: SttbfAssoc()
    pub fn new() -> Self {
      // Java: super();
      SttbfAssoc {
          _fExtend: -1, _cData: 0, _cbExtra: 0, _cchData0: 0,
          sttbs: HashMap::new(), dataArray: None,
          _ibstAssocFileNext: 0, _ibstAssocDot: 1, _ibstAssocTitle: 2,
          _ibstAssocSubject: 3, _ibstAssocKeyWords: 4, _ibstAssocComments: 5,
          _ibstAssocAuthor: 6, _ibstAssocLastRevBy: 7, _ibstAssocDataDoc: 8,
          _ibstAssocHeaderDoc: 9, _ibstAssocCriteria1: 10, _ibstAssocCriteria2: 11,
          _ibstAssocCriteria3: 12, _ibstAssocCriteria4: 13, _ibstAssocCriteria5: 14,
          _ibstAssocCriteria6: 15, _ibstAssocCriteria7: 16, _ibstAssocMax: 17,
      }
    }

    // Java: SttbfAssoc(DocumentInputStream buf, int offset, int size)
    pub fn new_from_stream(buf: &mut DocumentInputStream, mut offset: i32, _size: i32) -> Self {
      // Java: super();
      let mut this = Self::new();
      // Java: buf.seek((long)offset);
      buf.seek(offset as i64);
      // Java: this._fExtend = buf.readShort();
      this._fExtend = buf.readShort();
      // Java: offset += 2;
      offset += 2;
      // Java: this._cData = buf.readShort();
      this._cData = buf.readShort();
      // Java: offset += 2;
      offset += 2;
      // Java: this._cbExtra = buf.readShort();
      this._cbExtra = buf.readShort();
      // Java: offset += 2;
      offset += 2;
      // Java: for(int i = 0; i < this._cData; ++i)
      for i in 0..this._cData as i32
      {
         // Java: this._cchData0 = buf.readShort();
         this._cchData0 = buf.readShort();
         // Java: offset += 2;
         offset += 2;
         // Java: if (this._cchData0 > 0)
         if this._cchData0 > 0
         {
            // Java: int len = this._cchData0 * 2;
            let len = (this._cchData0 as i32) * 2;
            // Java: this.dataArray = new byte[len];
            let mut data_array = vec![0u8; len as usize];
            // Java: buf.read(this.dataArray, 0, len);
            buf.read_off(&mut data_array, 0, len);
            // Java: offset += len;
            offset += len;
            // Java: this.sttbs.put(i, this.dataArray);
            this.sttbs.insert(i, Some(data_array));
         }
         // Java: else
         else
         {
            // Java: this.sttbs.put(i, null);
            this.sttbs.insert(i, None);
         }
      }
      this
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return this._cData;
      self._cData as i32
    }

    // Java: void setSize(int count)
    pub fn setSize(&mut self, count: i32) {
      // Java: this._cData = (short)count;
      self._cData = count as i16;
    }

    // Java: String getTemplate()
    pub fn getTemplate(&self) -> Option<String> {
      // Java: return this.getString(this._ibstAssocDot);
      self.getString(self._ibstAssocDot)
    }

    // Java: void setTemplate(String template)
    pub fn setTemplate(&mut self, template: &str) {
      // Java: IOAssert.assertNotNull("template should not be null!", template);
      // Java: this.setData(this._ibstAssocDot, template);
      self.setData(self._ibstAssocDot, template);
    }

    // Java: String getWriteReservation()
    pub fn getWriteReservation(&self) -> Option<String> {
      // Java: return this.getString(this._ibstAssocMax);
      self.getString(self._ibstAssocMax)
    }

    // Java: void setWriteReservation(String writeReservation)
    pub fn setWriteReservation(&mut self, write_reservation: &str) {
      // Java: IOAssert.assertNotNull("writeReservation should not be null!", writeReservation);
      // Java: this.setData(this._ibstAssocMax, writeReservation);
      self.setData(self._ibstAssocMax, write_reservation);
    }

    // Java: void setData(int index, String data)
    pub fn setData(&mut self, index: i32, data: &str) {
      // Java: if (null != data)
      if !data.is_empty() || true
      {
         // Java: try
         {
            // Java: this.sttbs.put(index, data.getBytes("UTF-16LE"));
            let bytes: Vec<u8> = data.encode_utf16().flat_map(|c| c.to_le_bytes()).collect();
            self.sttbs.insert(index, Some(bytes));
         }
         // Java: catch (UnsupportedEncodingException var4)
         {
            // Java: IOAssert.fail("get unicode byte fail!");
         }
      }
    }

    // Java: String getString(int index)
    fn getString(&self, index: i32) -> Option<String> {
      // Java: if (index >= 0 && index <= this.sttbs.size())
      if index >= 0 && (index as usize) <= self.sttbs.len()
      {
         // Java: try
         {
            // Java: byte[] data = (byte[])this.sttbs.get(index);
            let data = self.sttbs.get(&index);
            // Java: if (null != data)
            if let Some(Some(bytes)) = data
            {
               // Java: return new String(data, "UTF-16LE");
               let chars: Vec<u16> = bytes.chunks(2)
                   .map(|c| u16::from_le_bytes([c[0], if c.len() > 1 { c[1] } else { 0 }]))
                   .collect();
               return Some(String::from_utf16_lossy(&chars));
            }
         }
         // Java: catch (UnsupportedEncodingException var3)
         {
            // Java: IOAssert.fail("get unicode byte fail!");
         }
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         // Java: return null;
         return None;
      }
    }

    // Java: void writeTo(HWPFOutputStream out)
    pub fn writeTo(&self, out: &mut HWPFOutputStream) {
      // Java: if (this._cData > 0)
      if self._cData > 0
      {
         // Java: int offset = 0;
         let mut offset: i32 = 0;
         // Java: byte[] buf = new byte[6];
         let mut buf = vec![0u8; 6];
         // Java: LittleEndian.putShort(buf, offset, this._fExtend);
         le_put_short(&mut buf, offset, self._fExtend);
         // Java: offset += 2;
         offset += 2;
         // Java: LittleEndian.putShort(buf, offset, this._cData);
         le_put_short(&mut buf, offset, self._cData);
         // Java: offset += 2;
         offset += 2;
         // Java: LittleEndian.putShort(buf, offset, this._cbExtra);
         le_put_short(&mut buf, offset, self._cbExtra);
         // Java: offset += 2;
         offset += 2;
         // Java: out.write(buf);
         let _ = out.write(&buf);
         // Java: buf = new byte[2];
         let mut buf = vec![0u8; 2];
         // Java: for(int i = 0; i < this._cData; ++i)
         for i in 0..self._cData as i32
         {
            // Java: byte[] dataArray = (byte[])this.sttbs.get(i);
            let data_array = self.sttbs.get(&i).and_then(|opt| opt.as_ref());
            // Java: if (null == dataArray)
            if data_array.is_none()
            {
               // Java: LittleEndian.putShort(buf, 0, (short)0);
               le_put_short(&mut buf, 0, 0);
               // Java: out.write(buf);
               let _ = out.write(&buf);
            }
            // Java: else
            else
            {
               // Java: int len = dataArray.length / 2;
               let data = data_array.unwrap();
               let len = (data.len() / 2) as i16;
               // Java: LittleEndian.putShort(buf, 0, (short)len);
               le_put_short(&mut buf, 0, len);
               // Java: out.write(buf);
               let _ = out.write(&buf);
               // Java: out.write(dataArray);
               let _ = out.write(data);
            }
         }
      }
    }

}
