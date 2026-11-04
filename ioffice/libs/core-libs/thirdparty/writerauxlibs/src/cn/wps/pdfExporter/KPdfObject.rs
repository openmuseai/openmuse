// Java: import java.util.Vector
// (mapped to Vec<T>)

// Java: public class KPdfObject
pub struct KPdfObject {
}

// Java: public class KPdfObject
#[allow(non_snake_case)]
impl KPdfObject {
    // Java: KPdfObject()
    pub fn new() -> Self {
      // Java: super();
      Self {}
    }

    // Java: long getOutputSize(KPdfCatalog catalog, boolean indirect)
    pub fn getOutputSize(&self, _catalog: &KPdfObject, _indirect: bool) -> i64 {
      // Java: KStream stream = new KDynamicMemoryStream();
      // Java: this.emit(stream, catalog, indirect);
      // Java: return (long)stream.getLength();
      todo!("getOutputSize requires KPdfCatalog")
    }

    // Java: void emit(KStream stream, KPdfCatalog catalog, boolean indirect)
    pub fn emit(&self, _stream: &mut super::KStream::KStream, _catalog: &KPdfObject, _indirect: bool) {
      // Java: KPdfObject realObject = catalog.getSubstitute(this);
      // Java: realObject.emitObject(stream, catalog, indirect);
      todo!("emit requires KPdfCatalog")
    }

    // Java: void emitIndirectObject(KStream stream, KPdfCatalog catalog)
    pub fn emitIndirectObject(&self, _stream: &mut super::KStream::KStream, _catalog: &KPdfObject) {
      // Java: catalog.emitObjectNum(stream, this);
      // Java: stream.write(" obj\n");
      // Java: this.emit(stream, catalog, false);
      // Java: stream.write("\nendobj\n");
      todo!("emitIndirectObject requires KPdfCatalog")
    }

    // Java: long getIndirectOutputSize(KPdfCatalog catalog)
    pub fn getIndirectOutputSize(&self, _catalog: &KPdfObject) -> i64 {
      // Java: return (long)(catalog.getObjectNumSize(this) + " obj\n".length()) + this.getOutputSize(catalog, false) + (long)"\nendobj\n".length();
      todo!("getIndirectOutputSize requires KPdfCatalog")
    }

    // Java: void getResourcesHelper(Vector<KPdfObject> resources, Vector<KPdfObject> result)
    pub fn getResourcesHelper(&self, resources: &Option<Vec<KPdfObject>>, result: &mut Vec<KPdfObject>) {
      // Java: if (resources != null)
        if let Some(res) = resources
      {
         // Java: int count = resources.size();
         let count = res.len();
         // Java: for(int i = 0; i < count; ++i)
            for i in 0..count
         {
            // Java: KPdfObject resource = (KPdfObject)resources.get(i);
            let _resource = &res[i];
            // Java: result.add(resource);
            result.push(KPdfObject::new());
            // Java: resource.getResources(result);
         }
      }
    }

    // Java: void getResources(Vector<KPdfObject> resources)
    pub fn getResources(&self, _resources: &mut Vec<KPdfObject>) {
    }

    // Java: abstract void emitObject(KStream var1, KPdfCatalog var2, boolean var3)
    pub fn emitObject(&self, _var1: &mut super::KStream::KStream, _var2: &KPdfObject, _var3: bool) {
    }

}
