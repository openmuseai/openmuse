// Java: import cn.wps.base.assertion.IOAssert
// Java: import java.io.IOException
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

// Java: public class SttbListNames
pub struct SttbListNames {
    // Java field: short _fExtend
    _fExtend: i16,
    // Java field: short _cData
    pub(crate) _cData: i16,
    // Java field: short _cbExtra
    _cbExtra: i16,
    // Java field: String[] _names
    pub(crate) _names: Option<Vec<String>>,
}

// Java: public class SttbListNames
impl SttbListNames {
    // Java: SttbListNames(DocumentInputStream tableStream, int offset, int size)
    pub fn new_from_stream(table_stream: &mut DocumentInputStream, mut offset: i32, size: i32) -> Self {
      // Java: super();
      let mut this = SttbListNames { _fExtend: -1, _cData: 0, _cbExtra: 0, _names: None };
      // Java: if (size != 0)
      if size != 0
      {
         // Java: tableStream.seek((long)offset);
         table_stream.seek(offset as i64);
         // Java: this._fExtend = tableStream.readShort();
         this._fExtend = table_stream.readShort();
         // Java: offset += 2;
         offset += 2;
         // Java: this._cData = tableStream.readShort();
         this._cData = table_stream.readShort();
         // Java: offset += 2;
         offset += 2;
         // Java: this._cbExtra = tableStream.readShort();
         this._cbExtra = table_stream.readShort();
         // Java: offset += 2;
         offset += 2;
         // Java: this._names = new String[this._cData];
         this._names = Some(vec![String::new(); this._cData as usize]);
         // Java: int nameSize = size - 6;
         let name_size = size - 6;
         // Java: this.initialNames(tableStream, offset, nameSize);
         this.initialNames(table_stream, offset, name_size);
      }
      this
    }

    // Java: SttbListNames()
    pub fn new() -> Self {
      // Java: super();
      SttbListNames { _fExtend: -1, _cData: 0, _cbExtra: 0, _names: None }
    }

    // Java: void setEntryCount(int count)
    pub fn setEntryCount(&mut self, count: i32) {
      // Java: this._cData = (short)count;
      self._cData = count as i16;
      // Java: this._names = new String[this._cData];
      self._names = Some(vec![String::new(); count as usize]);
      // Java: for(int i = 0; i < count; ++i)
      for i in 0..count as usize
      {
         // Java: this._names[i] = "";
         self._names.as_mut().unwrap()[i] = String::new();
      }
    }

    // Java: int getEntryCount()
    pub fn getEntryCount(&self) -> i32 {
      // Java: return this._cData;
      self._cData as i32
    }

    // Java: void initialNames(DocumentInputStream tableStream, int offset, int size)
    fn initialNames(&mut self, table_stream: &mut DocumentInputStream, mut offset: i32, _size: i32) {
      // Java: for(int i = 0; i < this._cData; ++i)
      for i in 0..self._cData as usize
      {
         // Java: tableStream.seek((long)offset);
         table_stream.seek(offset as i64);
         // Java: short nameLength = tableStream.readShort();
         let name_length = table_stream.readShort();
         // Java: offset += 2;
         offset += 2;
         // Java: char[] name = new char[nameLength];
         let mut name_chars: Vec<u16> = Vec::with_capacity(name_length as usize);
         // Java: for(int j = 0; j < nameLength; ++j)
         for _j in 0..name_length
         {
            // Java: name[j] = (char)tableStream.readShort();
            name_chars.push(table_stream.readShort() as u16);
            // Java: offset += 2;
            offset += 2;
         }
         // Java: String listName = new String(name);
         let list_name = String::from_utf16_lossy(&name_chars);
         // Java: this._names[i] = listName;
         self._names.as_mut().unwrap()[i] = list_name;
      }
    }

    // Java: String[] getNames()
    pub fn getNames(&self) -> &[String] {
      // Java: return this._names;
      self._names.as_ref().map_or(&[] as &[String], |v| v.as_slice())
    }

    // Java: void writeTo(HWPFOutputStream out)
    pub fn writeTo(&self, out: &mut HWPFOutputStream) {
      // Java: if (this._cData > 0 && null != this._names)
      if self._cData > 0 && self._names.is_some()
      {
         // Java: int offset = 0;
         let mut offset: i32 = 0;
         // Java: byte[] header = new byte[6];
         let mut header = vec![0u8; 6];
         // Java: LittleEndian.putShort(header, offset, this._fExtend);
         le_put_short(&mut header, offset, self._fExtend);
         // Java: offset += 2;
         offset += 2;
         // Java: LittleEndian.putShort(header, offset, this._cData);
         le_put_short(&mut header, offset, self._cData);
         // Java: offset += 2;
         offset += 2;
         // Java: LittleEndian.putShort(header, offset, this._cbExtra);
         le_put_short(&mut header, offset, self._cbExtra);
         // Java: offset += 2;
         offset += 2;
         // Java: out.write(header);
         let _ = out.write(&header);
         // Java: IOAssert.assertTrue("names.length should be equal to cData!", this._names.length == this._cData);
         assert!(self._names.as_ref().unwrap().len() == self._cData as usize, "names.length should be equal to cData!");
         // Java: byte[] buf = new byte[2];
         let mut buf = vec![0u8; 2];
         // Java: int i = 0;
         let mut i: i32 = 0;
         // Java: for(int len = this._names.length; i < len; ++i)
         let names = self._names.as_ref().unwrap();
         let len = names.len() as i32;
         while i < len
         {
            // Java: String name = this._names[i];
            let name = &names[i as usize];
            // Java: int nameLen = 0;
            let mut name_len: i32 = 0;
            // Java: if (null != name)
            if !name.is_empty() || true
            {
               // Java: nameLen = name.length();
               name_len = name.encode_utf16().count() as i32;
            }
            // Java: LittleEndian.putShort(buf, 0, (short)nameLen);
            le_put_short(&mut buf, 0, name_len as i16);
            // Java: out.write(buf);
            let _ = out.write(&buf);
            // Java: for(int j = 0; j < nameLen; ++j)
            for (j, ch) in name.encode_utf16().enumerate()
            {
               // Java: LittleEndian.putShort(buf, 0, (short)name.charAt(j));
               le_put_short(&mut buf, 0, ch as i16);
               // Java: out.write(buf);
               let _ = out.write(&buf);
            }
            i += 1;
         }
      }
    }

    // Java: void setName(int i, String name)
    pub fn setName(&mut self, i: i32, name: String) {
      // Java: this._names[i] = name;
      if let Some(ref mut names) = self._names {
          names[i as usize] = name;
      }
    }

}
