use std::{rc::Rc,cell::RefCell,io};
use crate::kfc::{ooxfr::OxfRelations,ooxfw::{OxfwAppPart::OxfwAppPart,OxfwCorePart::OxfwCorePart,OxfwCustomPart::OxfwCustomPart},opcr::{OpcTargetMode::OpcTargetMode,OpcUtil::OpcUtil},xpxml::writer::XmlWriter::XmlWriter};
use super::{OpcwPackagePart::OpcwPackagePart,native::{SharedZip,RelationRef,PartRef}};
// java public final class OpcwRelationship {
pub struct OpcwRelationship{
    // java private ZipOutputStream mZipStream;
    mZipStream:Option<SharedZip>,
    // java private OpcRelation mRel;
    mRel:RelationRef,
    // java private int mRelIndex;
    mRelIndex:i32,
    // java private String mRId;
    mRId:String,
    // java private String mTarget;
    mTarget:Option<String>,
    // java private OpcTargetMode mTargetMode;
    mTargetMode:Option<OpcTargetMode>,
    // java private OpcwPackagePart mPart;
    mPart:Option<PartRef>,
}
impl OpcwRelationship{
    // java OpcwRelationship(ZipOutputStream zipStream, OpcRelation rel, int relIndex, String rid) {
    pub fn OpcwRelationship(zipStream:Option<SharedZip>,rel:RelationRef,relIndex:i32,rid:String)->Self{
        // java super(); this.mRelIndex = -1; this.mTarget = null; this.mPart = null; this.mZipStream = zipStream; this.mRel = rel; this.mRelIndex = relIndex; this.mRId = rid;
        Self{mZipStream:zipStream,mRel:rel,mRelIndex:relIndex,mRId:rid,mTarget:None,mTargetMode:None,mPart:None}
    }
    // java OpcwRelationship(ZipOutputStream zipStream, OpcRelation rel, String target, OpcTargetMode targetMode, String rid) { this(zipStream, rel, target, targetMode, rid, true); }
    pub fn OpcwRelationship__2(zipStream:Option<SharedZip>,rel:RelationRef,target:Option<String>,targetMode:Option<OpcTargetMode>,rid:String)->Self{Self::OpcwRelationship__3(zipStream,rel,target,targetMode,rid,true)}
    // java OpcwRelationship(ZipOutputStream zipStream, OpcRelation rel, String target, OpcTargetMode targetMode, String rid, boolean targetEncode) {
    pub fn OpcwRelationship__3(zipStream:Option<SharedZip>,rel:RelationRef,target:Option<String>,targetMode:Option<OpcTargetMode>,rid:String,targetEncode:bool)->Self{
        // java super(); this.mRelIndex = -1; this.mTarget = null; this.mPart = null; this.mZipStream = zipStream; this.mRel = rel;
        let mut value=Self{mZipStream:zipStream,mRel:rel,mRelIndex:-1,mRId:rid,mTarget:None,mTargetMode:None,mPart:None};
        // java if (targetEncode) { this.mTarget = this.encodeURL(target); } else { this.mTarget = target; }
        value.mTarget=if targetEncode{Self::encodeURL(target)}else{target};
        // java this.mTargetMode = targetMode; this.mRId = rid;
        value.mTargetMode=targetMode;value
    }
    // java OpcwRelationship(ZipOutputStream zipStream, OpcRelation rel, String target, OpcTargetMode targetMode, String rid, int idx) {
    pub fn OpcwRelationship__4(zipStream:Option<SharedZip>,rel:RelationRef,target:Option<String>,targetMode:Option<OpcTargetMode>,rid:String,idx:i32)->Self{
        // java super(); this.mRelIndex = -1; this.mTarget = null; this.mPart = null; this.mZipStream = zipStream; this.mRel = rel; this.mTarget = this.encodeURL(target); this.mTargetMode = targetMode; this.mRId = rid; this.mRelIndex = idx;
        Self{mZipStream:zipStream,mRel:rel,mRelIndex:idx,mRId:rid,mTarget:Self::encodeURL(target),mTargetMode:targetMode,mPart:None}
    }
    // java public OpcwRelationship(ZipOutputStream zipStream, OpcRelation rel, OpcTargetMode targetMode, String rid, int idx) {
    pub fn OpcwRelationship__5(zipStream:Option<SharedZip>,rel:RelationRef,targetMode:Option<OpcTargetMode>,rid:String,idx:i32)->Self{
        // java super(); this.mRelIndex = -1; this.mTarget = null; this.mPart = null; this.mZipStream = zipStream; this.mRel = rel; this.mTargetMode = targetMode; this.mRId = rid; this.mRelIndex = idx;
        Self{mZipStream:zipStream,mRel:rel,mRelIndex:idx,mRId:rid,mTarget:None,mTargetMode:targetMode,mPart:None}
    }
    // java public OpcwPackagePart getPackagePart() { return this.mPart; }
    pub fn getPackagePart(&self)->Option<PartRef>{self.mPart.clone()}
    // java public OpcwPackagePart createPart() {
    pub fn createPart(&mut self)->PartRef{
        // java if (this.mPart == null) {
        if self.mPart.is_none(){
            // java if (this.mRel == OxfRelations.APP) { this.mPart = new OxfwAppPart(this.mZipStream); }
            self.mPart=Some(if std::ptr::eq(&*self.mRel,&*OxfRelations::APP){PartRef::App(Rc::new(RefCell::new(OxfwAppPart::OxfwAppPart(self.mZipStream.clone()))))}
            // java else if (this.mRel == OxfRelations.CORE) { this.mPart = new OxfwCorePart(this.mZipStream); }
            else if std::ptr::eq(&*self.mRel,&*OxfRelations::CORE){PartRef::Core(Rc::new(RefCell::new(OxfwCorePart::OxfwCorePart(self.mZipStream.clone()))))}
            // java else if (this.mRel == OxfRelations.CUSTOM) { this.mPart = new OxfwCustomPart(this.mZipStream); }
            else if std::ptr::eq(&*self.mRel,&*OxfRelations::CUSTOM){PartRef::Custom(Rc::new(RefCell::new(OxfwCustomPart::OxfwCustomPart(self.mZipStream.clone()))))}
            // java else { this.mPart = new OpcwPackagePart(this.mZipStream, this.mRel, this.mRelIndex); }
            else{PartRef::Base(Rc::new(RefCell::new(OpcwPackagePart::OpcwPackagePart(self.mZipStream.clone(),self.mRel.clone(),self.mRelIndex))))});
        }
        // java return this.mPart;
        self.mPart.as_ref().unwrap().clone()
    }
    // java public String getRId() { return this.mRId; }
    pub fn getRId(&self)->String{self.mRId.clone()}
    // java public int getRelIndex() { return this.mRelIndex; }
    pub fn getRelIndex(&self)->i32{self.mRelIndex}
    // java public OpcRelation getOxfRelation() { return this.mRel; }
    pub fn getOxfRelation(&self)->RelationRef{self.mRel.clone()}
    // java void write(XmlWriter writer, String parentPartName) throws IOException {
    pub fn write(&self,writer:&mut dyn XmlWriter,parentPartName:&str)->io::Result<()>{
        // java writer.startElement("Relationship"); writer.addAttr("Id", this.mRId); writer.addAttr("Type", this.mRel.getRelsType());
        writer.startElement("Relationship");writer.addAttr("Id",&self.mRId);writer.addAttrNullable(Some("Type"),self.mRel.getRelsType().as_deref());
        // java if (this.mTarget == null) {
        if self.mTarget.is_none(){
            // java String relsTarget = OpcUtil.getRelsTarget(parentPartName, this.mRel.getPartName(this.mRelIndex)); writer.addAttr("Target", relsTarget);
            let relsTarget=OpcUtil::getRelsTarget(Some(parentPartName),Some(&self.mRel.getPartName__index(self.mRelIndex)));writer.addAttr("Target",&relsTarget);
        // java } else { writer.addAttr("Target", this.mTarget); if (this.mTargetMode == OpcTargetMode.EXTERNAL) { writer.addAttr("TargetMode", "External"); } }
        }else{writer.addAttr("Target",self.mTarget.as_ref().unwrap());if self.mTargetMode==Some(OpcTargetMode::EXTERNAL){writer.addAttr("TargetMode","External");}}
        // java writer.endElement("Relationship");
        writer.endElement("Relationship");Ok(())
    }
    // java private String encodeURL(String url) {
    fn encodeURL(mut url:Option<String>)->Option<String>{
        // java if (url != null) {
        if let Some(url)=url.as_mut(){
            // java if (url.contains("%")) { url = url.replaceAll("\\%", "%25"); }
            if url.contains('%'){*url=url.replace('%',"%25");}
            // java if (url.contains(" ")) { url = url.replaceAll("\\ ", "%20"); }
            if url.contains(' '){*url=url.replace(' ',"%20");}
            // java if (url.contains("[")) { url = url.replaceAll("\\[", "%5b"); }
            if url.contains('['){*url=url.replace('[',"%5b");}
            // java if (url.contains("]")) { url = url.replaceAll("\\]", "%5d"); }
            if url.contains(']'){*url=url.replace(']',"%5d");}
            // java if (url.contains("#")) { url = url.replaceAll("\\#", "%23"); }
            if url.contains('#'){*url=url.replace('#',"%23");}
            // java if (url.contains("{")) { url = url.replaceAll("\\{", "%7b"); }
            if url.contains('{'){*url=url.replace('{',"%7b");}
            // java if (url.contains("}")) { url = url.replaceAll("\\}", "%7d"); }
            if url.contains('}'){*url=url.replace('}',"%7d");}
            // java if (url.contains("`")) { url = url.replaceAll("\\`", "%60"); }
            if url.contains('`'){*url=url.replace('`',"%60");}
            // java if (url.contains("^")) { url = url.replaceAll("\\^", "%5e"); }
            if url.contains('^'){*url=url.replace('^',"%5e");}
        }
        // java return url;
        url
    }
}
