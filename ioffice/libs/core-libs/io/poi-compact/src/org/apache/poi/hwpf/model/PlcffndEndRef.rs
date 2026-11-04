// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.kfc.api.map.IIntObjectMap
// Java: import cn.wps.kfc.impl.map.WIntObjectHashMap
// Java: import java.io.IOException
// Java: import java.util.Arrays
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use super::PlexOfCps::{PlexOfCps, GenericPropertyNode};
use std::collections::BTreeMap;
use std::io::Write;

fn le_put_int(buf: &mut [u8], offset: i32, value: i32) {
    let o = offset as usize;
    buf[o] = (value & 0xFF) as u8;
    buf[o+1] = ((value >> 8) & 0xFF) as u8;
    buf[o+2] = ((value >> 16) & 0xFF) as u8;
    buf[o+3] = ((value >> 24) & 0xFF) as u8;
}

fn le_put_short(buf: &mut [u8], offset: i32, value: i16) {
    let o = offset as usize;
    let v = value as u16;
    buf[o] = (v & 0xFF) as u8;
    buf[o + 1] = (v >> 8) as u8;
}

// Java field: int ELEMENT_SIZE
pub(crate) const ELEMENT_SIZE: i32 = 2;

// Java: public class PlcffndEndRef
pub struct PlcffndEndRef {
    // Java field: IIntObjectMap<Integer> _refMap
    _refMap: BTreeMap<i32, i32>,
    // Java field: PlexOfCps _refPlex
    pub(crate) _refPlex: Option<PlexOfCps>,
}

// Java: public class PlcffndEndRef
impl PlcffndEndRef {
    // Java: PlcffndEndRef()
    pub fn new() -> Self {
      // Java: super();
      PlcffndEndRef { _refMap: BTreeMap::new(), _refPlex: None }
    }

    // Java: PlcffndEndRef(DocumentInputStream tableStream, int offset, int size)
    pub fn new_from_stream(table_stream: DocumentInputStream, offset: i32, size: i32) -> Self {
      // Java: super();
      let mut this = PlcffndEndRef { _refMap: BTreeMap::new(), _refPlex: None };
      // Java: IOAssert.assertNotNull("tableStream should not be null", tableStream);
      // Java: if (size > 0)
      if size > 0
      {
         // Java: this._refPlex = new PlexOfCps(tableStream, offset, size, 2);
         this._refPlex = Some(PlexOfCps::new_from_stream(table_stream, offset, size, 2));
      }
      this
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return this._refPlex != null ? this._refPlex.length() : this._refMap.size();
      if self._refPlex.is_some() {
          self._refPlex.as_ref().unwrap().length()
      } else {
          self._refMap.len() as i32
      }
    }

    // Java: GenericPropertyNode getRawFndRefByIndex(int index)
    pub fn getRawFndRefByIndex(&mut self, index: i32) -> GenericPropertyNode {
      // Java: IOAssert.assertNotNull("_refPlex should not be null", this._refPlex);
      assert!(self._refPlex.is_some(), "_refPlex should not be null");
      // Java: IOAssert.assertTrue("_refPlex.length() > index should be true", this._refPlex.length() > index);
      assert!(self._refPlex.as_ref().unwrap().length() > index, "_refPlex.length() > index should be true");
      // Java: return this._refPlex.getProperty(index);
      self._refPlex.as_mut().unwrap().getProperty(index)
    }

    // Java: void addRef(int cp, int numStart)
    pub fn addRef(&mut self, cp: i32, num_start: i32) {
      // Java: this._refMap.put(cp, numStart);
      self._refMap.insert(cp, num_start);
    }

    // Java: void writeTo(HWPFOutputStream tableStream)
    pub fn writeTo(&self, table_stream: &mut HWPFOutputStream) {
      // Java: IOAssert.assertNotNull("tableStream should not be null", tableStream);
      // Java: int count = this._refMap.size();
      let count = self._refMap.len() as i32;
      // Java: int[] cps = this._refMap.keys();
      let cps: Vec<i32> = self._refMap.keys().copied().collect();
      // Java: Arrays.sort(cps);
      // Java: if (count > 0)
      if count > 0
      {
         // Java: int numStartOffset = 4 * count;
         let num_start_offset = 4 * count;
         // Java: byte[] buf = new byte[numStartOffset + 2 * (count - 1)];
         let mut buf = vec![0u8; (num_start_offset + 2 * (count - 1)) as usize];
         // Java: for(int i = 0; i < count; ++i)
         for i in 0..count
         {
            // Java: int key = cps[i];
            let key = cps[i as usize];
            // Java: short value = ((Integer)this._refMap.get(key)).shortValue();
            let value = *self._refMap.get(&key).unwrap() as i16;
            // Java: LittleEndian.putInt(buf, i * 4, key);
            le_put_int(&mut buf, i * 4, key);
            // Java: if (i < count - 1)
            if i < count - 1
            {
               // Java: LittleEndian.putShort(buf, numStartOffset + 2 * i, value);
               le_put_short(&mut buf, num_start_offset + 2 * i, value);
            }
         }
         // Java: tableStream.write(buf);
         let _ = table_stream.write(&buf);
      }
    }

}
