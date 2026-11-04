use std::{rc::Rc,cell::RefCell,io};
use super::{native::{SharedZip,RelationRef,ZipSink,ZipEntry,IListener,DeflateDataCallBack},OpcwRelationships::OpcwRelationships};
// java public class OpcwPackagePart implements IPackagePart {
pub struct OpcwPackagePart{
    // java private ZipOutputStream mZipStream;
    mZipStream:Option<SharedZip>,
    // java private OpcRelation mRel;
    mRel:RelationRef,
    // java private int mRelIndex;
    mRelIndex:i32,
    // java private OpcwRelationships mRelses = null;
    mRelses:Option<Rc<RefCell<OpcwRelationships>>>,
    // java private ZipEntry mEntry = null;
    mEntry:Option<Box<dyn ZipEntry>>,
}
impl OpcwPackagePart{
    // java public OpcwPackagePart(ZipOutputStream zipStream, OpcRelation rel, int relIndex) {
    pub fn OpcwPackagePart(zipStream:Option<SharedZip>,rel:RelationRef,relIndex:i32)->Self{
        // java super(); this.mZipStream = zipStream; this.mRel = rel; this.mRelIndex = relIndex;
        Self{mZipStream:zipStream,mRel:rel,mRelIndex:relIndex,mRelses:None,mEntry:None}
    }
    // java public OutputStream getOutputStream() throws IOException { return this.getOutputStream((ZipEntry)null, (ZipOutputStream.IListener)null); }
    pub fn getOutputStream(&mut self)->io::Result<Option<ZipSink>>{self.getOutputStream__2(None,None)}
    // java public OutputStream getOutputStream(ZipEntry entry, ZipOutputStream.IListener listener) throws IOException { return this.getOutputStream(entry, listener, (ZipOutputStream.DeflateDataCallBack)null); }
    pub fn getOutputStream__2(&mut self,entry:Option<&dyn ZipEntry>,listener:Option<Box<dyn IListener>>)->io::Result<Option<ZipSink>>{self.getOutputStream__3(entry,listener,None)}
    // java public OutputStream getOutputStream(ZipEntry entry, ZipOutputStream.IListener listener, ZipOutputStream.DeflateDataCallBack deflateDataCallBack) throws IOException {
    pub fn getOutputStream__3(&mut self,entry:Option<&dyn ZipEntry>,listener:Option<Box<dyn IListener>>,deflateDataCallBack:Option<Box<dyn DeflateDataCallBack>>)->io::Result<Option<ZipSink>>{
        // java if (null == this.mZipStream) { return null; } else {
        if self.mZipStream.is_none(){Ok(None)}else{
            // java this.mEntry = null != entry ? (ZipEntry)entry.clone() : null; if (this.mEntry == null) { this.mEntry = new ZipEntry(); }
            self.mEntry=entry.map(|e|e.cloneEntry());if self.mEntry.is_none(){self.mEntry=Some(self.mZipStream.as_ref().unwrap().borrow().newEntry(None));}
            // java this.mEntry.setName(this.getPartName());
            let name=self.getPartName();self.mEntry.as_mut().unwrap().setName(Some(name));
            // java this.mZipStream.putNextEntry(this.mEntry, listener, deflateDataCallBack); return this.mZipStream;
            let zip=self.mZipStream.as_ref().unwrap();zip.borrow_mut().putNextEntry(self.mEntry.as_ref().unwrap().as_ref(),listener,deflateDataCallBack)?;Ok(Some(ZipSink(zip.clone())))
        }
    }
    // java public OpcwRelationships getRelationships() {
    pub fn getRelationships(&mut self)->Rc<RefCell<OpcwRelationships>>{
        // java if (this.mRelses == null) { this.mRelses = new OpcwRelationships(this.mZipStream, this.getPartName()); }
        if self.mRelses.is_none(){self.mRelses=Some(Rc::new(RefCell::new(OpcwRelationships::OpcwRelationships(self.mZipStream.clone(),self.getPartName()))));}
        // java return this.mRelses;
        self.mRelses.as_ref().unwrap().clone()
    }
    // java OpcRelation getRelation() { return this.mRel; }
    pub fn getRelation(&self)->RelationRef{self.mRel.clone()}
    // java int getRelIndex() { return this.mRelIndex; }
    pub fn getRelIndex(&self)->i32{self.mRelIndex}
    // java public String getPartName() {
    pub fn getPartName(&self)->String{
        // java String partName = this.mRel.getPartName(this.mRelIndex);
        let mut partName=self.mRel.getPartName__index(self.mRelIndex);
        // java if (partName.startsWith("/")) { partName = partName.substring(1); }
        if partName.starts_with('/'){partName=partName[1..].into();}
        // java return partName;
        partName
    }
}
