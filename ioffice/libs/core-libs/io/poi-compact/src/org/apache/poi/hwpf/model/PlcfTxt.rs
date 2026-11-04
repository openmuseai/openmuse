// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use std::io::Write;

fn le_put_int(buf: &mut [u8], offset: i32, value: i32) {
    let o = offset as usize;
    buf[o] = (value & 0xFF) as u8;
    buf[o+1] = ((value >> 8) & 0xFF) as u8;
    buf[o+2] = ((value >> 16) & 0xFF) as u8;
    buf[o+3] = ((value >> 24) & 0xFF) as u8;
}

// Java: public class PlcfTxt
pub struct PlcfTxt {
    // Java field: ArrayList<Integer> cpList
    pub(crate) cpList: Vec<i32>,
}

// Java: public class PlcfTxt
impl PlcfTxt {
    // Java: PlcfTxt()
    pub fn new() -> Self {
      // Java: super();
      PlcfTxt { cpList: Vec::new() }
    }

    // Java: PlcfTxt(DocumentInputStream tableStream, int offset, int size)
    pub fn new_from_stream(table_stream: &mut DocumentInputStream, offset: i32, size: i32) -> Self {
      // Java: super();
      let mut this = PlcfTxt { cpList: Vec::new() };
      // Java: if (size > 0)
      if size > 0
      {
         // Java: tableStream.seek((long)offset);
         table_stream.seek(offset as i64);
         // Java: int count = size / 4;
         let count = size / 4;
         // Java: for(int i = 0; i < count; ++i)
         for _i in 0..count
         {
            // Java: this.cpList.add(tableStream.readInt());
            this.cpList.push(table_stream.readInt());
            // Java: offset += 4;
         }
      }
      this
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return this.cpList.size();
      self.cpList.len() as i32
    }

    // Java: int getCp(int index)
    pub fn getCp(&self, index: i32) -> i32 {
      // Java: return index >= 0 && index < this.cpList.size() ? (Integer)this.cpList.get(index) : -1;
      if index >= 0 && (index as usize) < self.cpList.len() {
          self.cpList[index as usize]
      } else {
          -1
      }
    }

    // Java: void addCp(Integer cp)
    pub fn addCp(&mut self, cp: i32) {
      // Java: this.cpList.add(cp);
      self.cpList.push(cp);
    }

    // Java: void writeTo(HWPFOutputStream tableStream)
    pub fn writeTo(&self, table_stream: &mut HWPFOutputStream) {
      // Java: if (this.getSize() > 0)
      if self.getSize() > 0
      {
         // Java: int offset = 0;
         let mut offset: i32 = 0;
         // Java: int count = this.getSize();
         let count = self.getSize();
         // Java: byte[] buf = new byte[count * 4];
         let mut buf = vec![0u8; (count * 4) as usize];
         // Java: for(int i = 0; i < count; ++i)
         for i in 0..count
         {
            // Java: LittleEndian.putInt(buf, offset, (Integer)this.cpList.get(i));
            le_put_int(&mut buf, offset, self.cpList[i as usize]);
            // Java: offset += 4;
            offset += 4;
         }
         // Java: tableStream.write(buf);
         let _ = table_stream.write(&buf);
      }
    }

}
