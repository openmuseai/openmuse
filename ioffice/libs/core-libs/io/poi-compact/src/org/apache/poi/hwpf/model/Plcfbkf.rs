// Java: import cn.wps.base.assertion.IOAssert
// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use super::FBKF::FBKF;
use super::PlexOfCps::{PlexOfCps, GenericPropertyNode};

use std::io::Write;

fn le_put_int(buf: &mut [u8], offset: i32, value: i32) {
    let o = offset as usize;
    buf[o] = (value & 0xFF) as u8;
    buf[o+1] = ((value >> 8) & 0xFF) as u8;
    buf[o+2] = ((value >> 16) & 0xFF) as u8;
    buf[o+3] = ((value >> 24) & 0xFF) as u8;
}

// Java: public class Combination
pub struct Combination {
    // Java field: int cp
    pub(crate) cp: i32,
    // Java field: FBKF fbkf
    pub(crate) fbkf: FBKF,
}

// Java: public class Plcfbkf
pub struct Plcfbkf {
    // Java field: ArrayList<Combination> _list
    _list: Vec<Combination>,
    // Java field: PlexOfCps _bkfPlex
    pub(crate) _bkfPlex: Option<PlexOfCps>,
}

// Java: public class Plcfbkf
impl Plcfbkf {
    // Java: Plcfbkf()
    pub fn new() -> Self {
      // Java: super();
      Plcfbkf { _list: Vec::new(), _bkfPlex: None }
    }

    // Java: Plcfbkf(DocumentInputStream tableStream, int offset, int size)
    pub fn new_from_stream(table_stream: DocumentInputStream, offset: i32, size: i32) -> Self {
      // Java: super();
      let mut this = Plcfbkf { _list: Vec::new(), _bkfPlex: None };
      // Java: this._bkfPlex = new PlexOfCps(tableStream, offset, size, 4);
      this._bkfPlex = Some(PlexOfCps::new_from_stream(table_stream, offset, size, 4));
      this
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return this._bkfPlex != null ? this._bkfPlex.length() : this._list.size();
      if self._bkfPlex.is_some() {
          self._bkfPlex.as_ref().unwrap().length()
      } else {
          self._list.len() as i32
      }
    }

    // Java: GenericPropertyNode getRawBkfByIndex(int index)
    pub fn getRawBkfByIndex(&mut self, index: i32) -> GenericPropertyNode {
      // Java: return this._bkfPlex.getProperty(index);
      self._bkfPlex.as_mut().unwrap().getProperty(index)
    }

    // Java: void addCombination(FBKF fbkf, int cp)
    pub fn addCombination(&mut self, fbkf: FBKF, cp: i32) {
      // Java: Combination combination = new Combination();
      let mut combination = Combination { cp: 0, fbkf: FBKF::new() };
      // Java: combination.fbkf = fbkf;
      combination.fbkf = fbkf;
      // Java: combination.cp = cp;
      combination.cp = cp;
      // Java: this._list.add(combination);
      self._list.push(combination);
      // Java: Combination var4 = null;
    }

    // Java: Combination getLastCombination()
    pub fn getLastCombination(&self) -> &Combination {
      // Java: IOAssert.assertTrue("_list.size() > 0 should be true!", this._list.size() > 0);
      assert!(self._list.len() > 0, "_list.size() > 0 should be true!");
      // Java: return (Combination)this._list.get(this._list.size() - 1);
      &self._list[self._list.len() - 1]
    }

    // Java: void writeTo(HWPFOutputStream tableStream)
    pub fn writeTo(&self, table_stream: &mut HWPFOutputStream) {
      // Java: int count = this._list.size();
      let count = self._list.len() as i32;
      // Java: if (count > 0)
      if count > 0
      {
         // Java: int fbkfOffset = count * 4;
         let fbkf_offset = count * 4;
         // Java: byte[] buf = new byte[fbkfOffset + (count - 1) * 4];
         let mut buf = vec![0u8; (fbkf_offset + (count - 1) * 4) as usize];
         // Java: for(int i = 0; i < count; ++i)
         for i in 0..count
         {
            // Java: Combination combination = (Combination)this._list.get(i);
            let combination = &self._list[i as usize];
            // Java: LittleEndian.putInt(buf, i * 4, combination.cp);
            le_put_int(&mut buf, i * 4, combination.cp);
            // Java: if (i < count - 1)
            if i < count - 1
            {
               // Java: System.arraycopy(combination.fbkf.toByteArray(), 0, buf, fbkfOffset + i * 4, 4);
               let fbkf_bytes = combination.fbkf.toByteArray();
               let dst_offset = (fbkf_offset + i * 4) as usize;
               buf[dst_offset..dst_offset + 4].copy_from_slice(&fbkf_bytes[0..4]);
            }
         }
         // Java: tableStream.write(buf);
         let _ = table_stream.write(&buf);
      }
    }

}

// Java: public class Combination
impl Combination {
    // Java: Combination()
    pub fn new() -> Self {
         // Java: super();
         Combination { cp: 0, fbkf: FBKF::new() }
    }

}
