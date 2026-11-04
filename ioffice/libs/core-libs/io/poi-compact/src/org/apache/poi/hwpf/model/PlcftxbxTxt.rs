// Java: import cn.wps.base.log.IOLog
// Java: import java.io.IOException
// Java: import java.util.Map
// Java: import java.util.TreeMap
use std::collections::BTreeMap;
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use super::FTXBXS::FTXBXS;
use super::PlexOfCps::{PlexOfCps, GenericPropertyNode};

use std::io::Write;

fn le_put_int(buf: &mut [u8], offset: i32, value: i32) {
    let o = offset as usize;
    buf[o] = (value & 0xFF) as u8;
    buf[o+1] = ((value >> 8) & 0xFF) as u8;
    buf[o+2] = ((value >> 16) & 0xFF) as u8;
    buf[o+3] = ((value >> 24) & 0xFF) as u8;
}

// Java field: String TAG
pub(crate) const TAG: &str = "";

// Java: public class PlcftxbxTxt
pub struct PlcftxbxTxt {
    // Java field: TreeMap<Integer, FTXBXS> _txbxMap
    _txbxMap: BTreeMap<i32, FTXBXS>,
    // Java field: PlexOfCps _txbxPlex
    pub(crate) _txbxPlex: Option<PlexOfCps>,
}

// Java: public class PlcftxbxTxt
impl PlcftxbxTxt {
    // Java: PlcftxbxTxt()
    pub fn new() -> Self {
      // Java: super();
      PlcftxbxTxt { _txbxMap: BTreeMap::new(), _txbxPlex: None }
    }

    // Java: PlcftxbxTxt(DocumentInputStream tableStream, int offset, int size)
    pub fn new_from_stream(table_stream: DocumentInputStream, offset: i32, size: i32) -> Self {
      // Java: super();
      let mut this = PlcftxbxTxt { _txbxMap: BTreeMap::new(), _txbxPlex: None };
      // Java: this._txbxPlex = new PlexOfCps(tableStream, offset, size, 22);
      this._txbxPlex = Some(PlexOfCps::new_from_stream(table_stream, offset, size, 22));
      this
    }

    // Java: GenericPropertyNode getRawFtxbxByIndex(int index)
    pub fn getRawFtxbxByIndex(&mut self, index: i32) -> Option<GenericPropertyNode> {
      // Java: try
      {
         // Java: return this._txbxPlex.getProperty(index);
         if self._txbxPlex.is_some() {
             return Some(self._txbxPlex.as_mut().unwrap().getProperty(index));
         }
      }
      // Java: catch (IOException e)
      {
         // Java: IOLog.e(TAG, "IOException", e);
         // Java: return null;
      }
      None
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return this._txbxPlex != null ? this._txbxPlex.length() : this._txbxMap.size();
      if self._txbxPlex.is_some() {
          self._txbxPlex.as_ref().unwrap().length()
      } else {
          self._txbxMap.len() as i32
      }
    }

    // Java: void addFTXBX(FTXBXS ftxbx, int cp)
    pub fn addFTXBX(&mut self, ftxbx: FTXBXS, cp: i32) {
      // Java: this._txbxMap.put(cp, ftxbx);
      self._txbxMap.insert(cp, ftxbx);
    }

    // Java: void writeTo(HWPFOutputStream tableStream)
    pub fn writeTo(&self, table_stream: &mut HWPFOutputStream) {
      // Java: int count = this._txbxMap.size();
      let count = self._txbxMap.len() as i32;
      // Java: if (count > 0)
      if count > 0
      {
         // Java: byte[] buf = new byte[count * 4];
         let mut buf = vec![0u8; (count * 4) as usize];
         // Java: int i = 0;
         let mut i: i32 = 0;
         // Java: for(Map.Entry<Integer, FTXBXS> entry : this._txbxMap.entrySet())
         for (&cp, _) in &self._txbxMap
         {
            // Java: Integer cp = (Integer)entry.getKey();
            // Java: LittleEndian.putInt(buf, i * 4, cp);
            le_put_int(&mut buf, i * 4, cp);
            // Java: ++i;
            i += 1;
         }
         // Java: tableStream.write(buf);
         let _ = table_stream.write(&buf);
         // Java: i = 0;
         i = 0;
         // Java: for(Map.Entry<Integer, FTXBXS> entry : this._txbxMap.entrySet())
         for (_, ftxbxs) in &self._txbxMap
         {
            // Java: FTXBXS ftxbxs = (FTXBXS)entry.getValue();
            // Java: ++i;
            i += 1;
            // Java: if (i != count)
            if i != count
            {
               // Java: tableStream.write(ftxbxs.toByteArray());
               let _ = table_stream.write(&ftxbxs.toByteArray());
            }
         }
      }
    }

}
