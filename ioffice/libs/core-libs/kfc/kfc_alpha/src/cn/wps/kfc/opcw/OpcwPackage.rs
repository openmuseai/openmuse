use std::{rc::Rc,cell::RefCell,io};
use super::{native::{SharedZip,RelationRef,PartRef,openZip},OpcwRelationships::OpcwRelationships,OpcwContentTypes::OpcwContentTypes};
// java public class OpcwPackage {
pub struct OpcwPackage{
    // java private ZipOutputStream mZipStream;
    mZipStream:Option<SharedZip>,
    // java protected OpcwRelationships mRelses;
    pub(crate) mRelses:Rc<RefCell<OpcwRelationships>>,
    // java private OpcwPackagePart mOfficeDocumentPart = null;
    mOfficeDocumentPart:Option<PartRef>,
}
impl OpcwPackage{
    // java public OpcwPackage(String path) throws IOException {
    pub fn OpcwPackage(path:&str)->io::Result<Self>{
        // java super(); this.mZipStream = new ZipOutputStream(new BufferedOutputStream(new FileOutputStream(path))); this.mRelses = new OpcwRelationships(this.mZipStream, "");
        let zip=openZip(path)?;let rels=OpcwRelationships::OpcwRelationships(Some(zip.clone()),String::new());Ok(Self{mZipStream:Some(zip),mRelses:Rc::new(RefCell::new(rels)),mOfficeDocumentPart:None})
    }
    // java public OpcwPackagePart createOfficeDocument(OpcRelation rel) {
    pub fn createOfficeDocument(&mut self,rel:RelationRef)->PartRef{
        // java if (this.mOfficeDocumentPart == null) { OpcwRelationship rels = this.mRelses.addRelationship(rel); this.mOfficeDocumentPart = rels.createPart(); }
        if self.mOfficeDocumentPart.is_none(){let rels=self.mRelses.borrow_mut().addRelationship(rel);self.mOfficeDocumentPart=Some(rels.borrow_mut().createPart());}
        // java return this.mOfficeDocumentPart;
        self.mOfficeDocumentPart.as_ref().unwrap().clone()
    }
    // java public OpcwRelationships getRelationships() { return this.mRelses; }
    pub fn getRelationships(&self)->Rc<RefCell<OpcwRelationships>>{self.mRelses.clone()}
    // java public void close() throws IOException {
    pub fn close(&mut self)->io::Result<()>{
        // java if (this.mZipStream != null) {
        if let Some(zip)=self.mZipStream.clone(){
            // java try { this.mRelses.write(); (new OpcwContentTypes(this.mZipStream)).write(this.mRelses); }
            let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{self.mRelses.borrow().write()?;OpcwContentTypes::OpcwContentTypes(zip.clone()).write(Some(&self.mRelses.borrow()))}));
            // java finally { this.mZipStream.flush(); FileUtil.closeQuietly(this.mZipStream); this.mZipStream = null; }
            zip.borrow_mut().flush()?;let _=zip.borrow_mut().close();self.mZipStream=None;
            // rust: resume the original Java checked/runtime exception after finally.
            match result{Ok(result)=>result,Err(error)=>std::panic::resume_unwind(error)}
        }else{Ok(())}
    }
}
