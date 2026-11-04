use super::KPdfObject::KPdfObject;

// Java: public class KPdfObjectRef extends KPdfObject
pub struct KPdfObjectRef {
    // Java field: KPdfObject fObject
    fObject: KPdfObject,
}

// Java: public class KPdfObjectRef extends KPdfObject
#[allow(non_snake_case)]
impl KPdfObjectRef {
    // Java: KPdfObjectRef(KPdfObject object)
    pub fn new(object: KPdfObject) -> Self {
      // Java: super();
      // Java: this.fObject = object;
      Self { fObject: object }
    }

    // Java: void emitObject(KStream stream, KPdfCatalog catalog, boolean indirect)
    pub fn emitObject(&self, _stream: &mut super::KStream::KStream, _catalog: &KPdfObject, _indirect: bool) {
      // Java: catalog.emitObjectNum(stream, this.fObject);
      // Java: stream.write(" R");
      todo!("emitObject requires KPdfCatalog")
    }

    // Java: long getOutputSize(KPdfCatalog catalog, boolean indirect)
    pub fn getOutputSize(&self, _catalog: &KPdfObject, _indirect: bool) -> i64 {
      // Java: return (long)(catalog.getObjectNumSize(this.fObject) + 2);
      todo!("getOutputSize requires KPdfCatalog")
    }

}
