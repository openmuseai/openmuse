// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use super::DocField::DocField;
use std::io::Write;

fn le_put_int(buf: &mut [u8], offset: i32, value: i32) {
    let o = offset as usize;
    let bytes = value.to_le_bytes();
    buf[o] = bytes[0];
    buf[o + 1] = bytes[1];
    buf[o + 2] = bytes[2];
    buf[o + 3] = bytes[3];
}

// Java: public class DocFields
pub struct DocFields {
    // Java field: int _nCount
    pub(crate) _nCount: i32,
    // Java field: ArrayList<DocField> _list
    _list: Option<Vec<DocField>>,
}

// Java: public class DocFields
impl DocFields {
    // Java: DocFields()
    pub fn new() -> Self {
      // Java: super();
      // Java: this._list = new ArrayList();
      DocFields {
          _nCount: 0,
          _list: Some(Vec::new()),
      }
    }

    // Java: DocFields(DocumentInputStream stream, int fc, int lcb)
    pub fn new_from_stream(stream: &mut DocumentInputStream, fc: i32, lcb: i32) -> Self {
      // Java: super();
      // Java: stream.seek((long)fc);
      stream.seek(fc as i64);
      // Java: this._nCount = stream.readInt();
      let _nCount = stream.readInt();
      // Java: this._list = new ArrayList(this._nCount);
      let mut _list: Vec<DocField> = Vec::with_capacity(_nCount as usize);
      // Java: for(int i = 0; i < this._nCount; ++i)
      for _i in 0.._nCount {
         // Java: this._list.add(new DocField(stream));
         _list.push(DocField::new_from_stream(stream));
      }
      DocFields {
          _nCount,
          _list: Some(_list),
      }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this._nCount;
      self._nCount
    }

    // Java: DocField get(int index)
    pub fn get(&self, index: i32) -> Option<&DocField> {
      // Java: return (DocField)this._list.get(index);
      self._list.as_ref().and_then(|list| list.get(index as usize))
    }

    // Java: void setSize(int count)
    pub fn setSize(&mut self, count: i32) {
      // Java: this._nCount = count;
      self._nCount = count;
    }

    // Java: void add(DocField docField)
    pub fn add(&mut self, doc_field: DocField) {
      // Java: this._list.add(docField);
      if let Some(ref mut list) = self._list {
          list.push(doc_field);
      }
    }

    // Java: void writeTo(HWPFOutputStream stream)
    pub fn writeTo(&self, stream: &mut HWPFOutputStream) {
      // Java: if (this._nCount > 0)
      if self._nCount > 0 {
         // Java: byte[] buf = new byte[4];
         let mut buf = [0u8; 4];
         // Java: LittleEndian.putInt(buf, 0, this._nCount);
         le_put_int(&mut buf, 0, self._nCount);
         // Java: stream.write(buf);
         stream.write_all(&buf).ok();
         // Java: for(int i = 0; i < this._nCount; ++i)
         if let Some(ref list) = self._list {
             for i in 0..self._nCount as usize {
                // Java: DocField docField = (DocField)this._list.get(i);
                let doc_field = &list[i];
                // Java: docField.writeTo(stream);
                doc_field.writeTo(stream);
             }
         }
      }
    }

}
