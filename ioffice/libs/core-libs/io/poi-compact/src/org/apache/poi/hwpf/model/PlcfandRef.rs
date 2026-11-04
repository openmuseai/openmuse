// Java: import java.io.IOException
// Java: import java.util.LinkedHashMap
use std::collections::BTreeMap;
// Java: import java.util.Map
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use super::ATRDPre::ATRDPre;
use super::PlexOfCps::PlexOfCps;

use std::io::Write;

fn le_put_int(buf: &mut [u8], offset: i32, value: i32) {
    let o = offset as usize;
    buf[o] = (value & 0xFF) as u8;
    buf[o+1] = ((value >> 8) & 0xFF) as u8;
    buf[o+2] = ((value >> 16) & 0xFF) as u8;
    buf[o+3] = ((value >> 24) & 0xFF) as u8;
}

// Java: public class PlcfandRef
pub struct PlcfandRef {
    // Java field: LinkedHashMap<Integer, ATRDPre> _atrdPreMap
    pub(crate) _atrdPreMap: Vec<(i32, ATRDPre)>,
}

// Java: public class PlcfandRef
impl PlcfandRef {
    // Java: PlcfandRef()
    pub fn new() -> Self {
      // Java: super();
      PlcfandRef { _atrdPreMap: Vec::new() }
    }

    // Java: PlcfandRef(DocumentInputStream tableStream, int offset, int size)
    pub fn new_from_stream(table_stream: DocumentInputStream, offset: i32, size: i32) -> Self {
      // Java: super();
      let mut this = PlcfandRef { _atrdPreMap: Vec::new() };
      // Java: if (size != 0)
      if size != 0
      {
         // Java: PlexOfCps plex = new PlexOfCps(tableStream, offset, size, 30);
         let mut plex = PlexOfCps::new_from_stream(table_stream, offset, size, 30);
         // Java: int count = plex.length();
         let count = plex.length();
         // Java: for(int i = 0; i < count; ++i)
         for i in 0..count
         {
            // Java: GenericPropertyNode property = plex.getProperty(i);
            let property = plex.getProperty(i);
            // Java: ATRDPre atrdPre = new ATRDPre(property.getBytes(), property.getOffset());
            let atrd_pre = ATRDPre::new_from_buf(property.getBytes(), property.getOffset());
            // Java: this._atrdPreMap.put(property.getStart(), atrdPre);
            this._atrdPreMap.push((property.getStart(), atrd_pre));
         }
      }
      this
    }

    // Java: public LinkedHashMap<Integer, ATRDPre> getATRDPreMap()
    pub fn getATRDPreMap(&self) -> &Vec<(i32, ATRDPre)> {
      // Java: return this._atrdPreMap;
      &self._atrdPreMap
    }

    // Java: ATRDPre getATRDPreFromCp(int cp)
    pub fn getATRDPreFromCp(&self, cp: i32) -> Option<&ATRDPre> {
      // Java: return (ATRDPre)this._atrdPreMap.get(cp);
      self._atrdPreMap.iter().find(|(k, _)| *k == cp).map(|(_, v)| v)
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return this._atrdPreMap.size();
      self._atrdPreMap.len() as i32
    }

    // Java: void addATRDPre(ATRDPre atrdPre, int cp)
    pub fn addATRDPre(&mut self, atrd_pre: ATRDPre, cp: i32) {
      // Java: this._atrdPreMap.put(cp, atrdPre);
      self._atrdPreMap.push((cp, atrd_pre));
    }

    // Java: void writeTo(HWPFOutputStream tableStream)
    pub fn writeTo(&self, table_stream: &mut HWPFOutputStream) {
      // Java: if (this.getSize() > 0)
      if self.getSize() > 0
      {
         // Java: int i = 0;
         let mut i: i32 = 0;
         // Java: int count = this.getSize();
         let count = self.getSize();
         // Java: byte[] buf = new byte[count * 4];
         let mut buf = vec![0u8; (count * 4) as usize];
         // Java: for(Map.Entry<Integer, ATRDPre> entry : this._atrdPreMap.entrySet())
         for entry in &self._atrdPreMap
         {
            // Java: Integer cp = (Integer)entry.getKey();
            let cp = entry.0;
            // Java: LittleEndian.putInt(buf, i * 4, cp);
            le_put_int(&mut buf, i * 4, cp);
            // Java: ++i;
            i += 1;
         }
         // Java: tableStream.write(buf);
         let _ = table_stream.write(&buf);
         // Java: i = 1;
         i = 1;
         // Java: for(Map.Entry<Integer, ATRDPre> entry : this._atrdPreMap.entrySet())
         for entry in &self._atrdPreMap
         {
            // Java: ATRDPre atrdPre = (ATRDPre)entry.getValue();
            let atrd_pre = &entry.1;
            // Java: if (i < count)
            if i < count
            {
               // Java: tableStream.write(atrdPre.toByteArray());
               let _ = table_stream.write(&atrd_pre.toByteArray());
            }
            // Java: ++i;
            i += 1;
         }
      }
    }

}
