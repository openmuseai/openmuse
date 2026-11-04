// Java: import cn.wps.base.log.IOLog
// Java: import java.io.IOException
// Java: import java.util.Map
// Java: import java.util.TreeMap
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use std::collections::BTreeMap;
use std::io::Write;
use super::PlexOfCps::{PlexOfCps, GenericPropertyNode};
use super::FTBKD::FTBKD;

fn le_put_int(buf: &mut [u8], offset: i32, value: i32) {
    let o = offset as usize;
    buf[o] = (value & 0xFF) as u8;
    buf[o+1] = ((value >> 8) & 0xFF) as u8;
    buf[o+2] = ((value >> 16) & 0xFF) as u8;
    buf[o+3] = ((value >> 24) & 0xFF) as u8;
}

// Java field: String TAG
pub const TAG: &str = "";

// Java: public class PlcfTxbxBkd
pub struct PlcfTxbxBkd {
    // Java field: TreeMap<Integer, FTBKD> _ftbkdMap
    _ftbkdMap: BTreeMap<i32, FTBKD>,
    // Java field: PlexOfCps _ftbkdPlex
    pub(crate) _ftbkdPlex: Option<PlexOfCps>,
}

// Java: public class PlcfTxbxBkd
impl PlcfTxbxBkd {
    // Java: PlcfTxbxBkd()
    pub fn new() -> Self {
      // Java: super();
      PlcfTxbxBkd {
          _ftbkdMap: BTreeMap::new(),
          _ftbkdPlex: None,
      }
    }

    // Java: PlcfTxbxBkd(DocumentInputStream tableStream, int fcPlcftbkd, int lcbPlcftbkd)
    pub fn new_from_stream(table_stream: DocumentInputStream, fc_plcftbkd: i32, lcb_plcftbkd: i32) -> Self {
      // Java: super();
      let mut this = PlcfTxbxBkd {
          _ftbkdMap: BTreeMap::new(),
          _ftbkdPlex: None,
      };
      // Java: this._ftbkdPlex = new PlexOfCps(tableStream, fcPlcftbkd, lcbPlcftbkd, 6);
      this._ftbkdPlex = Some(PlexOfCps::new_from_stream(table_stream, fc_plcftbkd, lcb_plcftbkd, 6));
      this
    }

    // Java: GenericPropertyNode getRawFtbkdByIndex(int index)
    pub fn getRawFtbkdByIndex(&mut self, index: i32) -> Option<GenericPropertyNode> {
      // Java: try
      {
         // Java: return this._ftbkdPlex.getProperty(index);
         if let Some(ref mut plex) = self._ftbkdPlex {
             return Some(plex.getProperty(index));
         }
         return None;
      }
      // Java: catch (IOException e)
      // Java: IOLog.e(TAG, "IOException", e);
      // Java: return null;
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return this._ftbkdPlex.length();
      self._ftbkdPlex.as_ref().map_or(0, |p| p.length())
    }

    // Java: void addFTBKD(FTBKD ftbkd, int cp)
    pub fn addFTBKD(&mut self, ftbkd: FTBKD, cp: i32) {
      // Java: this._ftbkdMap.put(cp, ftbkd);
      self._ftbkdMap.insert(cp, ftbkd);
    }

    // Java: void writeTo(HWPFOutputStream tableStream)
    pub fn writeTo(&self, table_stream: &mut HWPFOutputStream) {
      // Java: int count = this._ftbkdMap.size();
      let count = self._ftbkdMap.len();
      // Java: if (count > 0)
      if count > 0
      {
         // Java: byte[] buf = new byte[count * 4];
         let mut buf = vec![0u8; count * 4];
         // Java: int i = 0;
         let mut i: i32 = 0;
         // Java: for(Map.Entry<Integer, FTBKD> entyr : this._ftbkdMap.entrySet())
         for (cp, _ftbkd) in &self._ftbkdMap
         {
            // Java: Integer cp = (Integer)entyr.getKey();
            // Java: LittleEndian.putInt(buf, i * 4, cp);
            le_put_int(&mut buf, i * 4, *cp);
            // Java: ++i;
            i += 1;
         }
         // Java: tableStream.write(buf);
         let _ = table_stream.write(&buf);
         // Java: i = 0;
         i = 0;
         // Java: for(Map.Entry<Integer, FTBKD> entry : this._ftbkdMap.entrySet())
         for (_cp, ftbkd) in &self._ftbkdMap
         {
            // Java: FTBKD ftbkd = (FTBKD)entry.getValue();
            // Java: ++i;
            i += 1;
            // Java: if (i != count)
            if i != count as i32
            {
               // Java: tableStream.write(ftbkd.toByteArray());
               let _ = table_stream.write(&ftbkd.toByteArray());
            }
         }
      }
    }

}
