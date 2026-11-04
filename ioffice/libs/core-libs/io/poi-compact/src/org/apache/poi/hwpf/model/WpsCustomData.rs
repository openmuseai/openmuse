// Java: import java.io.IOException
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use super::DocFields::DocFields;
use std::io::Write;

fn le_put_int(buf: &mut [u8], offset: i32, value: i32) {
    let o = offset as usize;
    let bytes = value.to_le_bytes();
    buf[o] = bytes[0];
    buf[o + 1] = bytes[1];
    buf[o + 2] = bytes[2];
    buf[o + 3] = bytes[3];
}

// Java: public class FCLCB
pub struct FCLCB {
    // Java field: int _fc
    pub(crate) _fc: i32,
    // Java field: int _lcb
    pub(crate) _lcb: i32,
}

// Java: public class WpsCustomData
pub struct WpsCustomData {
    // Java field: DocumentInputStream stream
    pub(crate) stream: Option<DocumentInputStream>,
    // Java field: int _nCount
    pub(crate) _nCount: i32,
    // Java field: FCLCB[] _fclcb
    pub(crate) _fclcb: Option<Vec<FCLCB>>,
    // Java field: DocFields docFields
    pub(crate) docFields: Option<DocFields>,
}

// Java: public class WpsCustomData
impl WpsCustomData {
    // Java field: int INDEX_DOCFIELD
    pub const INDEX_DOCFIELD: i32 = 0;
    // Java field: int MAX
    pub const MAX: i32 = 1;

    // Java: WpsCustomData()
    pub fn new() -> Self {
      // Java: super();
      WpsCustomData {
          stream: None,
          _nCount: 0,
          _fclcb: None,
          docFields: None,
      }
    }

    // Java: WpsCustomData(DocumentInputStream stream)
    pub fn new_from_stream(mut stream: DocumentInputStream) -> Self {
      // Java: super();
      // Java: this.stream = stream;
      // Java: stream.seek(0L);
      stream.seek(0);
      // Java: this._nCount = stream.readInt();
      let _nCount = stream.readInt();
      // Java: this._fclcb = new FCLCB[this._nCount];
      let mut _fclcb: Vec<FCLCB> = Vec::with_capacity(_nCount as usize);
      // Java: for(int i = 0; i < this._nCount; ++i)
      for _i in 0.._nCount {
         // Java: this._fclcb[i] = new FCLCB(stream);
         _fclcb.push(FCLCB::new_from_stream(&mut stream));
      }
      WpsCustomData {
          stream: Some(stream),
          _nCount,
          _fclcb: Some(_fclcb),
          docFields: None,
      }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this._nCount;
      self._nCount
    }

    // Java: boolean hasDocFields()
    pub fn hasDocFields(&self) -> bool {
      // Java: FCLCB fclcb = this.getFCLCB(0);
      let fclcb = self.getFCLCB(0);
      // Java: return null != fclcb && fclcb._lcb > 0;
      match fclcb {
          Some(f) => f._lcb > 0,
          None => false,
      }
    }

    // Java: DocFields getDocFields()
    pub fn getDocFields(&mut self) -> Option<&DocFields> {
      // Java: if (null == this.docFields)
      if self.docFields.is_none() {
         // Java: FCLCB fclcb = this.getFCLCB(0);
         let fc;
         let lcb;
         if let Some(ref fclcb_vec) = self._fclcb {
             if !fclcb_vec.is_empty() {
                 fc = fclcb_vec[0]._fc;
                 lcb = fclcb_vec[0]._lcb;
             } else {
                 return None;
             }
         } else {
             return None;
         }
         // Java: this.docFields = new DocFields(this.stream, fclcb._fc, fclcb._lcb);
         if let Some(ref mut stream) = self.stream {
             self.docFields = Some(DocFields::new_from_stream(stream, fc, lcb));
         }
      }
      // Java: return this.docFields;
      self.docFields.as_ref()
    }

    // Java: FCLCB getFCLCB(int index)
    pub fn getFCLCB(&self, index: i32) -> Option<&FCLCB> {
      // Java: return this._fclcb != null && index >= 0 && index <= this._fclcb.length ? this._fclcb[index] : null;
      if let Some(ref fclcb) = self._fclcb {
          if index >= 0 && (index as usize) < fclcb.len() {
              return Some(&fclcb[index as usize]);
          }
      }
      None
    }

    // Java: void setDocFields(DocFields docFields, int fc, int lcb)
    pub fn setDocFields(&mut self, doc_fields: DocFields, mut fc: i32, lcb: i32) {
      // Java: if (null == this._fclcb)
      if self._fclcb.is_none() {
         // Java: this._nCount = 1;
         self._nCount = 1;
         // Java: this._fclcb = new FCLCB[this._nCount];
         self._fclcb = Some(Vec::with_capacity(self._nCount as usize));
      }
      // Java: this.docFields = docFields;
      self.docFields = Some(doc_fields);
      // Java: fc += 4 + 8 * this._nCount;
      fc += 4 + 8 * self._nCount;
      // Java: this._fclcb[0] = new FCLCB(fc, lcb);
      if let Some(ref mut fclcb) = self._fclcb {
          if fclcb.is_empty() {
              fclcb.push(FCLCB::new(fc, lcb));
          } else {
              fclcb[0] = FCLCB::new(fc, lcb);
          }
      }
    }

    // Java: void writeTo(HWPFOutputStream stream)
    pub fn writeTo(&self, stream: &mut HWPFOutputStream) {
      // Java: byte[] buf = new byte[4];
      let mut buf = [0u8; 4];
      // Java: LittleEndian.putInt(buf, 0, this._nCount);
      le_put_int(&mut buf, 0, self._nCount);
      // Java: stream.write(buf);
      stream.write_all(&buf).ok();
      // Java: if (this._nCount > 0)
      if self._nCount > 0 {
         // Java: for(int i = 0; i < this._nCount; ++i)
         if let Some(ref fclcb) = self._fclcb {
             for i in 0..self._nCount as usize {
                // Java: this._fclcb[i].writeTo(stream);
                fclcb[i].writeTo(stream);
             }
         }
         // Java: if (null != this.docFields)
         if let Some(ref doc_fields) = self.docFields {
            // Java: this.docFields.writeTo(stream);
            doc_fields.writeTo(stream);
         }
      }
    }

}

// Java: public class FCLCB
impl FCLCB {
    // Java: FCLCB(int _fc, int _lcb)
    pub fn new(fc: i32, lcb: i32) -> Self {
         // Java: super();
         // Java: this._fc = _fc;
         // Java: this._lcb = _lcb;
         FCLCB { _fc: fc, _lcb: lcb }
    }

    // Java: FCLCB(DocumentInputStream stream)
    pub fn new_from_stream(stream: &mut DocumentInputStream) -> Self {
         // Java: super();
         // Java: this._fc = stream.readInt();
         let _fc = stream.readInt();
         // Java: this._lcb = stream.readInt();
         let _lcb = stream.readInt();
         FCLCB { _fc, _lcb }
    }

    // Java: void writeTo(HWPFOutputStream stream)
    pub fn writeTo(&self, stream: &mut HWPFOutputStream) {
         // Java: int offset = 0;
         let mut offset: i32 = 0;
         // Java: byte[] buf = new byte[8];
         let mut buf = [0u8; 8];
         // Java: LittleEndian.putInt(buf, offset, this._fc);
         le_put_int(&mut buf, offset, self._fc);
         // Java: offset += 4;
         offset += 4;
         // Java: LittleEndian.putInt(buf, offset, this._lcb);
         le_put_int(&mut buf, offset, self._lcb);
         // Java: stream.write(buf);
         stream.write_all(&buf).ok();
    }

}
