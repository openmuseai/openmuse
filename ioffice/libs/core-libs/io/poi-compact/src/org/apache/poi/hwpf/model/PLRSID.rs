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

fn le_put_int_0(buf: &mut [u8], value: i32) {
    buf[0] = (value & 0xFF) as u8;
    buf[1] = ((value >> 8) & 0xFF) as u8;
    buf[2] = ((value >> 16) & 0xFF) as u8;
    buf[3] = ((value >> 24) & 0xFF) as u8;
}

// Java: public class PLRSID
pub struct PLRSID {
    // Java field: int cRsid
    pub(crate) cRsid: i32,
    // Java field: int cbRsidInFile
    cbRsidInFile: i32,
    // Java field: int cbHeadExtraInFile
    cbHeadExtraInFile: i32,
    // Java field: int reserved1
    reserved1: i32,
    // Java field: int reserved2
    reserved2: i32,
    // Java field: int reserved3
    reserved3: i32,
    // Java field: int[] rgrsid
    rgrsid: Option<Vec<i32>>,
}

// Java: public class PLRSID
impl PLRSID {
    // Java: PLRSID(DocumentInputStream tableStream, int offset, int size)
    pub fn new_from_stream(table_stream: &mut DocumentInputStream, offset: i32, size: i32) -> Self {
      // Java: super();
      let mut this = PLRSID {
          cRsid: 0,
          cbRsidInFile: 4,
          cbHeadExtraInFile: 8,
          reserved1: 229,
          reserved2: 0,
          reserved3: 0,
          rgrsid: None,
      };
      // Java: if (size != 0)
      if size != 0
      {
         // Java: tableStream.seek((long)offset);
         table_stream.seek(offset as i64);
         // Java: this.cRsid = tableStream.readInt();
         this.cRsid = table_stream.readInt();
         // Java: offset += 4;
         let mut _offset = offset + 4;
         // Java: this.cbRsidInFile = tableStream.readInt();
         this.cbRsidInFile = table_stream.readInt();
         // Java: offset += 4;
         _offset += 4;
         // Java: this.cbHeadExtraInFile = tableStream.readInt();
         this.cbHeadExtraInFile = table_stream.readInt();
         // Java: offset += 4;
         _offset += 4;
         // Java: this.reserved1 = tableStream.readInt();
         this.reserved1 = table_stream.readInt();
         // Java: offset += 4;
         _offset += 4;
         // Java: this.reserved2 = tableStream.readInt();
         this.reserved2 = table_stream.readInt();
         // Java: offset += 4;
         _offset += 4;
         // Java: this.reserved3 = tableStream.readInt();
         this.reserved3 = table_stream.readInt();
         // Java: offset += 4;
         _offset += 4;
         // Java: this.rgrsid = new int[this.cRsid];
         let mut rgrsid = vec![0i32; this.cRsid as usize];
         // Java: for(int i = 0; i < this.cRsid; ++i)
         for i in 0..this.cRsid as usize
         {
            // Java: this.rgrsid[i] = tableStream.readInt();
            rgrsid[i] = table_stream.readInt();
            // Java: offset += 4;
            _offset += 4;
         }
         this.rgrsid = Some(rgrsid);
      }
      this
    }

    // Java: PLRSID(int cRsid)
    pub fn new(c_rsid: i32) -> Self {
      // Java: super();
      let mut this = PLRSID {
          cRsid: 0,
          cbRsidInFile: 4,
          cbHeadExtraInFile: 8,
          reserved1: 229,
          reserved2: 0,
          reserved3: 0,
          rgrsid: None,
      };
      // Java: this.cRsid = cRsid;
      this.cRsid = c_rsid;
      // Java: this.rgrsid = new int[cRsid];
      this.rgrsid = Some(vec![0i32; c_rsid as usize]);
      this
    }

    // Java: Integer getRsid(int index)
    pub fn getRsid(&self, index: i32) -> Option<i32> {
      // Java: Integer rsid = null;
      let mut rsid: Option<i32> = None;
      // Java: if (index >= 0 && index < this.rgrsid.length)
      if let Some(ref arr) = self.rgrsid {
          if index >= 0 && (index as usize) < arr.len()
          {
             // Java: rsid = this.rgrsid[index];
             rsid = Some(arr[index as usize]);
          }
      }
      // Java: return rsid;
      rsid
    }

    // Java: void setRgrsid(ArrayList<Integer> rsidList)
    pub fn setRgrsid(&mut self, rsid_list: &Vec<i32>) {
      // Java: this.cRsid = rsidList.size();
      self.cRsid = rsid_list.len() as i32;
      // Java: this.rgrsid = new int[this.cRsid];
      let mut rgrsid = vec![0i32; self.cRsid as usize];
      // Java: for(int i = 0; i < this.cRsid; ++i)
      for i in 0..self.cRsid as usize
      {
         // Java: this.rgrsid[i] = (Integer)rsidList.get(i);
         rgrsid[i] = rsid_list[i];
      }
      self.rgrsid = Some(rgrsid);
      // Java: this.reserved3 = this.cRsid - 1;
      self.reserved3 = self.cRsid - 1;
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return this.cRsid;
      self.cRsid
    }

    // Java: void writeTo(HWPFOutputStream tableStream)
    pub fn writeTo(&self, table_stream: &mut HWPFOutputStream) {
      // Java: int offset = 0;
      let mut offset: i32 = 0;
      // Java: byte[] header = new byte[24];
      let mut header = [0u8; 24];
      // Java: LittleEndian.putInt(header, offset, this.cRsid);
      le_put_int(&mut header, offset, self.cRsid);
      // Java: offset += 4;
      offset += 4;
      // Java: LittleEndian.putInt(header, offset, this.cbRsidInFile);
      le_put_int(&mut header, offset, self.cbRsidInFile);
      // Java: offset += 4;
      offset += 4;
      // Java: LittleEndian.putInt(header, offset, this.cbHeadExtraInFile);
      le_put_int(&mut header, offset, self.cbHeadExtraInFile);
      // Java: offset += 4;
      offset += 4;
      // Java: LittleEndian.putInt(header, offset, this.reserved1);
      le_put_int(&mut header, offset, self.reserved1);
      // Java: offset += 4;
      offset += 4;
      // Java: LittleEndian.putInt(header, offset, this.reserved2);
      le_put_int(&mut header, offset, self.reserved2);
      // Java: offset += 4;
      offset += 4;
      // Java: LittleEndian.putInt(header, offset, this.reserved3);
      le_put_int(&mut header, offset, self.reserved3);
      // Java: offset += 4;
      offset += 4;
      // Java: tableStream.write(header);
      let _ = table_stream.write(&header);
      // Java: for(int rsid : this.rgrsid)
      if let Some(ref arr) = self.rgrsid {
          for &rsid in arr.iter()
          {
             // Java: byte[] buf = new byte[4];
             let mut buf = [0u8; 4];
             // Java: LittleEndian.putInt(buf, rsid);
             le_put_int_0(&mut buf, rsid);
             // Java: tableStream.write(buf);
             let _ = table_stream.write(&buf);
          }
      }
    }

}
