// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import java.util.Arrays
// Java: import java.util.Collections
// Java: import java.util.List
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian
// Java: import org.apache.poi.util.StringUtil

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

// Java: public class RevisionMarkAuthorTable
pub struct RevisionMarkAuthorTable {
    // Java field: short fExtend
    fExtend: i16,
    // Java field: short cData
    pub(crate) cData: i16,
    // Java field: short cbExtra
    cbExtra: i16,
    // Java field: String[] entries
    entries: Option<Vec<String>>,
}

// Java: public class RevisionMarkAuthorTable
impl RevisionMarkAuthorTable {
    // Java: RevisionMarkAuthorTable(DocumentInputStream tableStream, int offset, int size)
    pub fn new_from_stream(table_stream: &mut DocumentInputStream, mut offset: i32, size: i32) -> Self {
      // Java: super();
      let mut this = RevisionMarkAuthorTable { fExtend: -1, cData: 0, cbExtra: 0, entries: None };
      // Java: if (size != 0)
      if size != 0
      {
         // Java: tableStream.seek((long)offset);
         table_stream.seek(offset as i64);
         // Java: this.fExtend = tableStream.readShort();
         this.fExtend = table_stream.readShort();
         // Java: if (this.fExtend != '\uffff')
         if this.fExtend != -1
         {
         }
         // Java: offset += 2;
         offset += 2;
         // Java: this.cData = tableStream.readShort();
         this.cData = table_stream.readShort();
         // Java: offset += 2;
         offset += 2;
         // Java: this.cbExtra = tableStream.readShort();
         this.cbExtra = table_stream.readShort();
         // Java: if (this.cbExtra != 0)
         if this.cbExtra != 0
         {
         }
         // Java: offset += 2;
         offset += 2;
         // Java: this.entries = new String[this.cData];
         this.entries = Some(vec![String::new(); this.cData as usize]);
         // Java: for(int i = 0; i < this.cData; ++i)
         for i in 0..this.cData as usize
         {
            // Java: int len = tableStream.readShort();
            let len = table_stream.readShort() as i32;
            // Java: offset += 2;
            offset += 2;
            // Java: byte[] buf = new byte[len * 2];
            let mut buf = vec![0u8; (len * 2) as usize];
            // Java: tableStream.read(buf, 0, len * 2);
            table_stream.read_off(&mut buf, 0, len * 2);
            // Java: String name = StringUtil.getFromUnicodeLE(buf, 0, len);
            let name = get_from_unicode_le(&buf, 0, len);
            // Java: offset += len * 2;
            offset += len * 2;
            // Java: this.entries[i] = name;
            this.entries.as_mut().unwrap()[i] = name;
         }
      }
      this
    }

    // Java: RevisionMarkAuthorTable(int cData)
    pub fn new(c_data: i32) -> Self {
      // Java: super();
      let mut this = RevisionMarkAuthorTable { fExtend: -1, cData: 0, cbExtra: 0, entries: None };
      // Java: this.cData = (short)cData;
      this.cData = c_data as i16;
      // Java: this.entries = new String[cData];
      this.entries = Some(vec![String::new(); c_data as usize]);
      this
    }

    // Java: List<String> getEntries()
    pub fn getEntries(&self) -> &[String] {
      // Java: return Collections.unmodifiableList(Arrays.asList(this.entries));
      self.entries.as_ref().map_or(&[] as &[String], |v| v.as_slice())
    }

    // Java: String getAuthor(int index)
    pub fn getAuthor(&self, index: i32) -> Option<&String> {
      // Java: String auth = null;
      let mut auth: Option<&String> = None;
      // Java: if (index >= 0 && index < this.entries.length)
      if index >= 0 && self.entries.as_ref().map_or(false, |e| (index as usize) < e.len())
      {
         // Java: auth = this.entries[index];
         auth = self.entries.as_ref().map(|e| &e[index as usize]);
      }
      // Java: return auth;
      auth
    }

    // Java: void setAuthorList(ArrayList<String> authorList)
    pub fn setAuthorList(&mut self, author_list: &[String]) {
      // Java: for(int i = 0; i < this.cData; ++i)
      for i in 0..self.cData as usize
      {
         // Java: this.entries[i] = (String)authorList.get(i);
         if let Some(ref mut entries) = self.entries {
             entries[i] = author_list[i].clone();
         }
      }
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return this.cData;
      self.cData as i32
    }

    // Java: void setSize(int size)
    pub fn setSize(&mut self, size: i32) {
      // Java: this.cData = (short)size;
      self.cData = size as i16;
    }

    // Java: void writeTo(HWPFOutputStream tableStream)
    pub fn writeTo(&self, table_stream: &mut HWPFOutputStream) {
      // Java: byte[] header = new byte[6];
      let mut header = vec![0u8; 6];
      // Java: LittleEndian.putShort(header, 0, this.fExtend);
      le_put_short(&mut header, 0, self.fExtend);
      // Java: LittleEndian.putShort(header, 2, this.cData);
      le_put_short(&mut header, 2, self.cData);
      // Java: LittleEndian.putShort(header, 4, this.cbExtra);
      le_put_short(&mut header, 4, self.cbExtra);
      // Java: tableStream.write(header);
      let _ = table_stream.write(&header);
      // Java: for(String name : this.entries)
      if let Some(ref entries) = self.entries { for name in entries
      {
         // Java: byte[] buf = new byte[name.length() * 2 + 2];
         let name_len = name.encode_utf16().count();
         let mut buf = vec![0u8; name_len * 2 + 2];
         // Java: LittleEndian.putShort(buf, 0, (short)name.length());
         le_put_short(&mut buf, 0, name_len as i16);
         // Java: StringUtil.putUnicodeLE(name, buf, 2);
         put_unicode_le(name, &mut buf, 2);
         // Java: tableStream.write(buf);
         let _ = table_stream.write(&buf);
      }}
    }

}
