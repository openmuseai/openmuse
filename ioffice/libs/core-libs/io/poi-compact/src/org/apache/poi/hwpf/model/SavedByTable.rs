// Java: import java.io.IOException
// Java: import java.util.Arrays
// Java: import java.util.Collections
// Java: import java.util.List
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian
// Java: import org.apache.poi.util.StringUtil

use super::SavedByEntry::SavedByEntry;
use std::io::Write;

fn le_put_short(buf: &mut [u8], offset: i32, value: i16) {
    let o = offset as usize;
    let v = value as u16;
    buf[o] = (v & 0xFF) as u8;
    buf[o + 1] = (v >> 8) as u8;
}

fn le_put_int(buf: &mut [u8], offset: i32, value: i32) {
    let o = offset as usize;
    buf[o] = (value & 0xFF) as u8;
    buf[o+1] = ((value >> 8) & 0xFF) as u8;
    buf[o+2] = ((value >> 16) & 0xFF) as u8;
    buf[o+3] = ((value >> 24) & 0xFF) as u8;
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

// Java: public class SavedByTable
pub struct SavedByTable {
    // Java field: short unknownValue
    unknownValue: i16,
    // Java field: SavedByEntry[] entries
    entries: Option<Vec<SavedByEntry>>,
}

// Java: public class SavedByTable
impl SavedByTable {
    // Java: SavedByTable(DocumentInputStream tableStream, int offset, int size)
    pub fn new_from_stream(table_stream: &mut DocumentInputStream, mut offset: i32, _size: i32) -> Self {
      // Java: super();
      let mut this = SavedByTable { unknownValue: -1, entries: None };
      // Java: tableStream.seek((long)offset);
      table_stream.seek(offset as i64);
      // Java: this.unknownValue = tableStream.readShort();
      this.unknownValue = table_stream.readShort();
      // Java: offset += 2;
      offset += 2;
      // Java: int numEntries = tableStream.readInt() / 2;
      let num_entries = table_stream.readInt() / 2;
      // Java: offset += 4;
      offset += 4;
      // Java: this.entries = new SavedByEntry[numEntries];
      this.entries = Some(Vec::with_capacity(num_entries as usize));
      // Java: for(int i = 0; i < numEntries; ++i)
      for _i in 0..num_entries
      {
         // Java: int len = tableStream.readUShort();
         let mut len = table_stream.readUShort() as i32;
         // Java: offset += 2;
         offset += 2;
         // Java: byte[] buf = new byte[len * 2];
         let mut buf = vec![0u8; (len * 2) as usize];
         // Java: tableStream.read(buf, 0, len * 2);
         table_stream.read_off(&mut buf, 0, len * 2);
         // Java: String userName = StringUtil.getFromUnicodeLE(buf, 0, len);
         let user_name = get_from_unicode_le(&buf, 0, len);
         // Java: offset += len * 2;
         offset += len * 2;
         // Java: len = tableStream.readUShort();
         len = table_stream.readUShort() as i32;
         // Java: offset += 2;
         offset += 2;
         // Java: buf = new byte[len * 2];
         let mut buf = vec![0u8; (len * 2) as usize];
         // Java: tableStream.read(buf, 0, len * 2);
         table_stream.read_off(&mut buf, 0, len * 2);
         // Java: String saveLocation = StringUtil.getFromUnicodeLE(buf, 0, len);
         let save_location = get_from_unicode_le(&buf, 0, len);
         // Java: offset += len * 2;
         offset += len * 2;
         // Java: this.entries[i] = new SavedByEntry(userName, saveLocation);
         this.entries.as_mut().unwrap().push(SavedByEntry::new(user_name, save_location));
      }
      this
    }

    // Java: List getEntries()
    pub fn getEntries(&self) -> &[SavedByEntry] {
      // Java: return Collections.unmodifiableList(Arrays.asList(this.entries));
      self.entries.as_ref().map_or(&[] as &[SavedByEntry], |v| v.as_slice())
    }

    // Java: void writeTo(HWPFOutputStream tableStream)
    pub fn writeTo(&self, table_stream: &mut HWPFOutputStream) {
      // Java: byte[] header = new byte[6];
      let mut header = vec![0u8; 6];
      // Java: LittleEndian.putShort(header, 0, this.unknownValue);
      le_put_short(&mut header, 0, self.unknownValue);
      // Java: LittleEndian.putInt(header, 2, this.entries.length * 2);
      let entries_len = self.entries.as_ref().map_or(0, |e| e.len()) as i32;
      le_put_int(&mut header, 2, entries_len * 2);
      // Java: tableStream.write(header);
      let _ = table_stream.write(&header);
      // Java: for(int i = 0; i < this.entries.length; ++i)
      if let Some(ref entries) = self.entries { for i in 0..entries.len()
      {
         // Java: this.writeStringValue(tableStream, this.entries[i].getUserName());
         self.writeStringValue(table_stream, entries[i].getUserName());
         // Java: this.writeStringValue(tableStream, this.entries[i].getSaveLocation());
         self.writeStringValue(table_stream, entries[i].getSaveLocation());
      }}
    }

    // Java: void writeStringValue(HWPFOutputStream tableStream, String value)
    fn writeStringValue(&self, table_stream: &mut HWPFOutputStream, value: &str) {
      // Java: byte[] buf = new byte[value.length() * 2 + 2];
      let value_len = value.encode_utf16().count();
      let mut buf = vec![0u8; value_len * 2 + 2];
      // Java: LittleEndian.putShort(buf, 0, (short)value.length());
      le_put_short(&mut buf, 0, value_len as i16);
      // Java: StringUtil.putUnicodeLE(value, buf, 2);
      put_unicode_le(value, &mut buf, 2);
      // Java: tableStream.write(buf);
      let _ = table_stream.write(&buf);
    }

}
