use super::KPdfObject::KPdfObject;
use super::KStream::KStream;

// Java: public class KPdfScalar extends KPdfObject
pub struct KPdfScalar {
    // Java field: float fValue
    pub(crate) fValue: f32,
}

// Java: public class KPdfScalar extends KPdfObject
#[allow(non_snake_case)]
impl KPdfScalar {
    // Java: KPdfScalar(float val)
    pub fn new(val: f32) -> Self {
      // Java: super();
      // Java: this.fValue = val;
      Self { fValue: val }
    }

    // Java: void emitObject(KStream stream, KPdfCatalog catalog, boolean indirect)
    pub fn emitObject(&self, stream: &mut KStream, _catalog: &KPdfObject, indirect: bool) {
      // Java: if (indirect)
      if indirect {
         // Java: this.emitIndirectObject(stream, catalog);
         todo!("emitIndirectObject requires KPdfCatalog")
      }
      // Java: else
      {
         // Java: this.append(this.fValue, stream);
         self.append(self.fValue, stream);
      }
    }

    // Java: void append(float val, KStream stream)
    pub fn append(&self, val: f32, stream: &mut KStream) {
      // Java: stream.write(KPdfUtils.toString(val));
      stream.write_float(val);
    }

}
