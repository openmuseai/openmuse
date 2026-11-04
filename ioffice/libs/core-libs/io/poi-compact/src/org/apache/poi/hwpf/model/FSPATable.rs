// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.log.IOLog
// Java: import java.io.IOException
// Java: import java.util.TreeMap
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use super::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian
use super::PlexOfCps::{PlexOfCps, GenericPropertyNode};
use super::FSPA::FSPA;
use std::collections::BTreeMap;
use std::io::Write;

fn le_put_int(data: &mut [u8], offset: i32, value: i32) {
    let o = offset as usize;
    data[o] = value as u8;
    data[o + 1] = (value >> 8) as u8;
    data[o + 2] = (value >> 16) as u8;
    data[o + 3] = (value >> 24) as u8;
}

// Java field: String TAG

// Java: public class FSPATable
pub struct FSPATable {
    // Java field: int indexOfPlex
    indexOfPlex: i32,
    // Java field: TreeMap<Integer, FSPA> _fspaMap
    pub(crate) _fspaMap: BTreeMap<i32, FSPA>,
    // Java field: PlexOfCps _fspaPlex
    pub(crate) _fspaPlex: Option<PlexOfCps>,
}

// Java: public class FSPATable
impl FSPATable {
    // Java: FSPATable()
    pub fn new() -> Self {
      // Java: super();
      FSPATable {
          indexOfPlex: 0,
          _fspaMap: BTreeMap::new(),
          _fspaPlex: None,
      }
    }

    // Java: FSPATable(DocumentInputStream tableStream, int fcPlcspa, int lcbPlcspa)
    pub fn new_from_stream(table_stream: DocumentInputStream, fc_plcspa: i32, lcb_plcspa: i32) -> Self {
      // Java: super();
      // Java: IOAssert.assertTrue("fcPlcspa > 0 should be true!", fcPlcspa > 0);
      assert!(fc_plcspa > 0, "fcPlcspa > 0 should be true!");
      // Java: IOAssert.assertTrue("lcbPlcspa > 0 should be true!", lcbPlcspa > 0);
      assert!(lcb_plcspa > 0, "lcbPlcspa > 0 should be true!");
      // Java: this._fspaPlex = new PlexOfCps(tableStream, fcPlcspa, lcbPlcspa, 26);
      let plex = PlexOfCps::new_from_stream(table_stream, fc_plcspa, lcb_plcspa, 26);
      FSPATable {
          indexOfPlex: 0,
          _fspaMap: BTreeMap::new(),
          _fspaPlex: Some(plex),
      }
    }

    // Java: GenericPropertyNode getRawFspaFromCp(int cp)
    pub fn getRawFspaFromCp(&mut self, cp: i32) -> Option<GenericPropertyNode> {
      // Java: int startIndex = this.indexOfPlex;
      let start_index = self.indexOfPlex;
      // Java: GenericPropertyNode node = null;
      let mut node: Option<GenericPropertyNode> = None;
      // Java: try
      {
         // Java: int i = this.indexOfPlex;
         let mut i = self.indexOfPlex;
         // Java: for(int len = this.getNodeCount(); i < len; ++i)
         let len = self.getNodeCount();
         while i < len
         {
            // Java: node = this._fspaPlex.getProperty(i);
            if let Some(ref mut plex) = self._fspaPlex {
                let n = plex.getProperty(i);
                // Java: if (node.getStart() == cp)
                if n.getStart() == cp
                {
                   // Java: this.indexOfPlex = i;
                   self.indexOfPlex = i;
                   node = Some(n);
                   // Java: break;
                   break;
                }
                node = Some(n);
            }
            i += 1;
         }
      }
      // Java: catch (IOException e)
      {
         // Java: IOLog.e(TAG, "IOException", e);
      }
      // Java: if (null != node && node.getStart() != cp)
      if node.as_ref().map_or(false, |n| n.getStart() != cp)
      {
         // Java: node = null;
         node = None;
         // Java: this.indexOfPlex = startIndex;
         self.indexOfPlex = start_index;
      }
      // Java: return node;
      node
    }

    // Java: int getNodeCount()
    pub fn getNodeCount(&self) -> i32 {
      // Java: if (this._fspaPlex != null)
      if self._fspaPlex.is_some()
      {
         // Java: return this._fspaPlex.length();
         return self._fspaPlex.as_ref().unwrap().length();
      }
      // Java: else
      else
      {
         // Java: return this._fspaMap != null ? this._fspaMap.size() : -1;
         return self._fspaMap.len() as i32;
      }
    }

    // Java: void AddFSPA(FSPA fspa, int cp)
    pub fn AddFSPA(&mut self, fspa: FSPA, cp: i32) {
      // Java: this._fspaMap.put(cp, fspa);
      self._fspaMap.insert(cp, fspa);
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: StringBuffer buf = new StringBuffer();
      let mut buf = String::new();
      // Java: buf.append("[FPSA PLC size=").append(this._fspaMap.size()).append("]\n");
      buf.push_str(&format!("[FPSA PLC size={}]\n", self._fspaMap.len()));
      // Java: for(Integer i : this._fspaMap.keySet())
      for (i, fspa) in &self._fspaMap
      {
         // Java: FSPA fspa = (FSPA)this._fspaMap.get(i);
         // Java: buf.append("  [FC: ").append(i.toString()).append("] ");
         buf.push_str(&format!("  [FC: {}] ", i));
         // Java: buf.append(fspa.toString());
         buf.push_str(&fspa.toString());
         // Java: buf.append("\n");
         buf.push('\n');
      }
      // Java: buf.append("[/FSPA PLC]");
      buf.push_str("[/FSPA PLC]");
      // Java: return buf.toString();
      buf
    }

    // Java: void writeTo(HWPFOutputStream tableStream)
    pub fn writeTo(&self, table_stream: &mut HWPFOutputStream) {
      // Java: int count = this._fspaMap.size();
      let count = self._fspaMap.len();
      // Java: if (count > 0)
      if count > 0
      {
         // Java: byte[] buf = new byte[count * 4];
         let mut buf = vec![0u8; count * 4];
         // Java: int i = 0;
         let mut i: i32 = 0;
         // Java: for(Integer cp : this._fspaMap.keySet())
         for (&cp, _) in &self._fspaMap
         {
            // Java: LittleEndian.putInt(buf, i * 4, cp);
            le_put_int(&mut buf, i * 4, cp);
            // Java: ++i;
            i += 1;
         }
         // Java: tableStream.write(buf);
         let _ = table_stream.write(&buf);
         // Java: i = 0;
         i = 0;
         // Java: for(Integer cp : this._fspaMap.keySet())
         for (_, fspa) in &self._fspaMap
         {
            // Java: FSPA fspa = (FSPA)this._fspaMap.get(cp);
            // Java: ++i;
            i += 1;
            // Java: if (i != count)
            if i as usize != count
            {
               // Java: tableStream.write(fspa.toByteArray());
               let _ = table_stream.write(&fspa.toByteArray());
            }
         }
      }
    }

}
