// Java: import java.io.IOException
// Java: import java.util.TreeMap
use std::collections::BTreeMap;
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use super::FLD::FLD;
use super::PlexOfCps::{PlexOfCps, GenericPropertyNode};

use std::io::Write;

fn le_put_int(buf: &mut [u8], offset: i32, value: i32) {
    let o = offset as usize;
    buf[o] = (value & 0xFF) as u8;
    buf[o+1] = ((value >> 8) & 0xFF) as u8;
    buf[o+2] = ((value >> 16) & 0xFF) as u8;
    buf[o+3] = ((value >> 24) & 0xFF) as u8;
}

// Java: public class PlcffldMom
pub struct PlcffldMom {
    // Java field: TreeMap<Integer, FLD> _plcffldMom
    _plcffldMom: BTreeMap<i32, FLD>,
    // Java field: PlexOfCps _fldMomPlex
    pub(crate) _fldMomPlex: Option<PlexOfCps>,
}

// Java: public class PlcffldMom
impl PlcffldMom {
    // Java: PlcffldMom()
    pub fn new() -> Self {
      // Java: super();
      PlcffldMom { _plcffldMom: BTreeMap::new(), _fldMomPlex: None }
    }

    // Java: PlcffldMom(DocumentInputStream tableStream, int offset, int size)
    pub fn new_from_stream(table_stream: DocumentInputStream, offset: i32, size: i32) -> Self {
      // Java: super();
      let mut this = PlcffldMom { _plcffldMom: BTreeMap::new(), _fldMomPlex: None };
      // Java: this._fldMomPlex = new PlexOfCps(tableStream, offset, size, 2);
      this._fldMomPlex = Some(PlexOfCps::new_from_stream(table_stream, offset, size, 2));
      this
    }

    // Java: int getNodeCount()
    pub fn getNodeCount(&self) -> i32 {
      // Java: return this._fldMomPlex.length();
      self._fldMomPlex.as_ref().unwrap().length()
    }

    // Java: GenericPropertyNode getRawFLDByIndex(int index)
    pub fn getRawFLDByIndex(&mut self, index: i32) -> GenericPropertyNode {
      // Java: return this._fldMomPlex.getProperty(index);
      self._fldMomPlex.as_mut().unwrap().getProperty(index)
    }

    // Java: void addFLD(FLD fld, int cp)
    pub fn addFLD(&mut self, fld: FLD, cp: i32) {
      // Java: this._plcffldMom.put(cp, fld);
      self._plcffldMom.insert(cp, fld);
    }

    // Java: void writeTo(HWPFOutputStream tableStream)
    pub fn writeTo(&self, table_stream: &mut HWPFOutputStream) {
      // Java: int count = this._plcffldMom.size();
      let count = self._plcffldMom.len() as i32;
      // Java: if (count > 0)
      if count > 0
      {
         // Java: byte[] buf = new byte[count * 4];
         let mut buf = vec![0u8; (count * 4) as usize];
         // Java: int i = 0;
         let mut i: i32 = 0;
         // Java: for(Integer cp : this._plcffldMom.keySet())
         for (&cp, _) in &self._plcffldMom
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
         // Java: for(Integer cp : this._plcffldMom.keySet())
         for (_, fld) in &self._plcffldMom
         {
            // Java: FLD fld = (FLD)this._plcffldMom.get(cp);
            // Java: ++i;
            i += 1;
            // Java: if (i != count)
            if i != count
            {
               // Java: tableStream.write(fld.toByteArray());
               let _ = table_stream.write(&fld.toByteArray());
            }
         }
      }
    }

}
