use std::{rc::Rc,cell::RefCell,io};
use crate::kfc::{opcr::{OpcTargetMode::OpcTargetMode,OpcUtil::OpcUtil},xpxml::writer::{XmlWriter::XmlWriter,r#impl::BufferXmlWriter::BufferXmlWriter}};
use super::{OpcwRelationship::OpcwRelationship,native::{SharedZip,RelationRef,ZipSink}};
// java public final class OpcwRelationships implements Iterable<OpcwRelationship> {
pub struct OpcwRelationships{
    // java private ArrayList<OpcwRelationship> mRelses = new ArrayList();
    mRelses:Vec<Rc<RefCell<OpcwRelationship>>>,
    // java private ZipOutputStream mZipStream;
    mZipStream:Option<SharedZip>,
    // java private String mParentPartName;
    mParentPartName:String,
}
impl OpcwRelationships{
    // java public OpcwRelationships(ZipOutputStream zipStream, String parentPartName) { super(); this.mZipStream = zipStream; this.mParentPartName = parentPartName; }
    pub fn OpcwRelationships(zipStream:Option<SharedZip>,parentPartName:String)->Self{Self{mRelses:vec![],mZipStream:zipStream,mParentPartName:parentPartName}}
    // java public OpcwRelationship addRelationship(OpcRelation rel) { return this.addRelationship(rel, -1); }
    pub fn addRelationship(&mut self,rel:RelationRef)->Rc<RefCell<OpcwRelationship>>{self.addRelationship__2(rel,-1)}
    // java public OpcwRelationship addRelationship(OpcRelation rel, int relIndex) {
    pub fn addRelationship__2(&mut self,rel:RelationRef,relIndex:i32)->Rc<RefCell<OpcwRelationship>>{
        // java String rid = "rId" + Integer.toString(this.mRelses.size() + 1); OpcwRelationship rels = new OpcwRelationship(this.mZipStream, rel, relIndex, rid); this.mRelses.add(rels); return rels;
        let rid=format!("rId{}",self.mRelses.len()+1);let rels=Rc::new(RefCell::new(OpcwRelationship::OpcwRelationship(self.mZipStream.clone(),rel,relIndex,rid)));self.mRelses.push(rels.clone());rels
    }
    // java public OpcwRelationship addRelationship(OpcRelation rel, int relIdx, int rIdx) {
    pub fn addRelationship__3(&mut self,rel:RelationRef,relIdx:i32,rIdx:i32)->Rc<RefCell<OpcwRelationship>>{
        // java String rid = "rId" + Integer.toString(rIdx); OpcwRelationship rels = new OpcwRelationship(this.mZipStream, rel, relIdx, rid); this.mRelses.add(rels); return rels;
        let rid=format!("rId{rIdx}");let rels=Rc::new(RefCell::new(OpcwRelationship::OpcwRelationship(self.mZipStream.clone(),rel,relIdx,rid)));self.mRelses.push(rels.clone());rels
    }
    // java public OpcwRelationship getRelationship(OpcRelation rel, int relIndex) {
    pub fn getRelationship(&self,rel:&RelationRef,relIndex:i32)->Option<Rc<RefCell<OpcwRelationship>>>{
        // java for(OpcwRelationship rels : this.mRelses) { if (rels.getRelIndex() == relIndex && rels.getOxfRelation() == rel) { return rels; } }
        for rels in &self.mRelses{let value=rels.borrow();if value.getRelIndex()==relIndex&&std::ptr::eq(&*value.getOxfRelation(),&**rel){return Some(rels.clone());}}
        // java return null;
        None
    }
    // java public OpcwRelationship addRelationship(OpcRelation rel, String userDefinedTarget, OpcTargetMode targetMode) {
    pub fn addRelationship__4(&mut self,rel:RelationRef,userDefinedTarget:Option<String>,targetMode:Option<OpcTargetMode>)->Rc<RefCell<OpcwRelationship>>{
        // java String rid = "rId" + Integer.toString(this.mRelses.size() + 1); OpcwRelationship rels = new OpcwRelationship(this.mZipStream, rel, userDefinedTarget, targetMode, rid); this.mRelses.add(rels); return rels;
        let rid=format!("rId{}",self.mRelses.len()+1);let rels=Rc::new(RefCell::new(OpcwRelationship::OpcwRelationship__2(self.mZipStream.clone(),rel,userDefinedTarget,targetMode,rid)));self.mRelses.push(rels.clone());rels
    }
    // java public OpcwRelationship addRelationship(OpcRelation rel, String target, OpcTargetMode targetMode, boolean targetEncode) {
    pub fn addRelationship__5(&mut self,rel:RelationRef,target:Option<String>,targetMode:Option<OpcTargetMode>,targetEncode:bool)->Rc<RefCell<OpcwRelationship>>{
        // java String rid = "rId" + Integer.toString(this.mRelses.size() + 1); OpcwRelationship rels = new OpcwRelationship(this.mZipStream, rel, target, targetMode, rid, targetEncode); this.mRelses.add(rels); return rels;
        let rid=format!("rId{}",self.mRelses.len()+1);let rels=Rc::new(RefCell::new(OpcwRelationship::OpcwRelationship__3(self.mZipStream.clone(),rel,target,targetMode,rid,targetEncode)));self.mRelses.push(rels.clone());rels
    }
    // java public OpcwRelationship addRelationship(OpcRelation rel, String trarget, OpcTargetMode mode, String rid, int idx) {
    pub fn addRelationship__6(&mut self,rel:RelationRef,trarget:Option<String>,mode:Option<OpcTargetMode>,rid:String,idx:i32)->Rc<RefCell<OpcwRelationship>>{
        // java OpcwRelationship rels = new OpcwRelationship(this.mZipStream, rel, trarget, mode, rid, idx); this.mRelses.add(rels); return rels;
        let rels=Rc::new(RefCell::new(OpcwRelationship::OpcwRelationship__4(self.mZipStream.clone(),rel,trarget,mode,rid,idx)));self.mRelses.push(rels.clone());rels
    }
    // java public OpcwRelationship addRelationship(OpcRelation rel, OpcTargetMode mode, String rid, int idx) {
    pub fn addRelationship__7(&mut self,rel:RelationRef,mode:Option<OpcTargetMode>,rid:String,idx:i32)->Rc<RefCell<OpcwRelationship>>{
        // java OpcwRelationship rels = new OpcwRelationship(this.mZipStream, rel, mode, rid, idx); this.mRelses.add(rels); return rels;
        let rels=Rc::new(RefCell::new(OpcwRelationship::OpcwRelationship__5(self.mZipStream.clone(),rel,mode,rid,idx)));self.mRelses.push(rels.clone());rels
    }
    // java public OpcwRelationship getRelationship(String rid) {
    pub fn getRelationship__2(&self,rid:&str)->Option<Rc<RefCell<OpcwRelationship>>>{
        // java for(OpcwRelationship rels : this.mRelses) { if (rels.getRId().equals(rid)) { return rels; } } return null;
        for rels in &self.mRelses{if rels.borrow().getRId()==rid{return Some(rels.clone());}}None
    }
    // java public Iterator<OpcwRelationship> iterator() { return this.mRelses.iterator(); }
    pub fn iterator(&self)->impl Iterator<Item=&Rc<RefCell<OpcwRelationship>>>{self.mRelses.iter()}
    // java public int size() { return this.mRelses.size(); }
    pub fn size(&self)->i32{self.mRelses.len()as i32}
    // java public void write() throws IOException {
    pub fn write(&self)->io::Result<()>{
        // java if (this.mRelses.size() != 0) {
        if !self.mRelses.is_empty(){
            // java this.writeImpl();
            self.writeImpl()?;
            // java for(OpcwRelationship rels : this.mRelses) { OpcwPackagePart part = rels.getPackagePart(); if (part != null) { part.getRelationships().write(); } }
            for rels in &self.mRelses{let part=rels.borrow().getPackagePart();if let Some(part)=part{part.getRelationships().borrow().write()?;}}
        }Ok(())
    }
    // java private void writeImpl() throws IOException {
    fn writeImpl(&self)->io::Result<()>{
        // java ZipEntry entry = new ZipEntry(OpcUtil.getRelsesName(this.mParentPartName)); this.mZipStream.putNextEntry(entry);
        let zip=self.mZipStream.as_ref().expect("NullPointerException");let entry=zip.borrow().newEntry(Some(OpcUtil::getRelsesName(&self.mParentPartName)));zip.borrow_mut().putNextEntry(entry.as_ref(),None,None)?;
        // java XmlWriter writer = new BufferXmlWriter(this.mZipStream); writer.startDocument(); writer.startElement("Relationships"); writer.addNS((String)null, "http://schemas.openxmlformats.org/package/2006/relationships");
        let mut writer=BufferXmlWriter::new(Box::new(ZipSink(zip.clone())));writer.startDocument();writer.startElement("Relationships");writer.addNS(None,"http://schemas.openxmlformats.org/package/2006/relationships");
        // java for(OpcwRelationship rels : this.mRelses) { rels.write(writer, this.mParentPartName); }
        for rels in &self.mRelses{rels.borrow().write(&mut writer,&self.mParentPartName)?;}
        // java writer.endElement("Relationships"); writer.endDocument();
        writer.endElement("Relationships");writer.endDocument();Ok(())
    }
    // java public OpcwRelationship addRelationship(OpcRelation rel, int relIndex, String rID) {
    pub fn addRelationship__8(&mut self,rel:RelationRef,relIndex:i32,rID:String)->Rc<RefCell<OpcwRelationship>>{
        // java OpcwRelationship rels = new OpcwRelationship(this.mZipStream, rel, relIndex, rID); this.mRelses.add(rels); return rels;
        let rels=Rc::new(RefCell::new(OpcwRelationship::OpcwRelationship(self.mZipStream.clone(),rel,relIndex,rID)));self.mRelses.push(rels.clone());rels
    }
}
