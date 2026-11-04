use super::KPdfObject::KPdfObject;
use super::KStream::KStream;

// Java: public class KPdfInt extends KPdfObject
pub struct KPdfInt {
    // Java field: int fValue
    pub(crate) fValue: i32,
}

// Java: public class KPdfInt extends KPdfObject
#[allow(non_snake_case)]
impl KPdfInt {
    // Java: KPdfInt(int val)
    pub fn new(val: i32) -> Self {
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
         // Java: stream.write(this.fValue);
         stream.write_int(self.fValue);
      }
    }

}
