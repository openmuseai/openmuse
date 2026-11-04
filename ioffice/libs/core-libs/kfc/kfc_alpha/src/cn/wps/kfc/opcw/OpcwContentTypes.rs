use std::{rc::Rc,cell::RefCell,io};
use crate::kfc::{ooxfr::{OxfRelations,OxfPptRelations},xpxml::writer::{XmlWriter::XmlWriter,r#impl::BufferXmlWriter::BufferXmlWriter}};
use super::{native::{SharedZip,ZipSink},StringMap::StringMap,OpcwRelationships::OpcwRelationships};
// java public final class OpcwContentTypes {
pub struct OpcwContentTypes{
    // java private ZipOutputStream mZipStream;
    mZipStream:SharedZip,
    // java private HashMap<String, String> mDefaultTypes;
    mDefaultTypes:StringMap<Option<String>>,
    // java private ArrayList<String> mOverrideContentTypes;
    mOverrideContentTypes:Vec<String>,
    // java private ArrayList<String> mOverridePartNames;
    mOverridePartNames:Vec<String>,
}
impl OpcwContentTypes{
    // java public OpcwContentTypes(ZipOutputStream zipStream) {
    pub fn OpcwContentTypes(zipStream:SharedZip)->Self{
        // java super(); this.mZipStream = zipStream; this.mDefaultTypes = new HashMap(); this.mOverrideContentTypes = new ArrayList(); this.mOverridePartNames = new ArrayList();
        Self{mZipStream:zipStream,mDefaultTypes:StringMap::default(),mOverrideContentTypes:vec![],mOverridePartNames:vec![]}
    }
    // java public void write(OpcwRelationships relses) throws IOException {
    pub fn write(&mut self,relses:Option<&OpcwRelationships>)->io::Result<()>{
        // java this.initPartData(relses);
        self.initPartData(relses)?;
        // java ZipEntry entry = new ZipEntry("[Content_Types].xml"); this.mZipStream.putNextEntry(entry);
        let entry=self.mZipStream.borrow().newEntry(Some("[Content_Types].xml".into()));self.mZipStream.borrow_mut().putNextEntry(entry.as_ref(),None,None)?;
        // java XmlWriter writer = new BufferXmlWriter(this.mZipStream); writer.startDocument(); writer.startElement("Types"); writer.addNS((String)null, "http://schemas.openxmlformats.org/package/2006/content-types");
        let mut writer=BufferXmlWriter::new(Box::new(ZipSink(self.mZipStream.clone())));writer.startDocument();writer.startElement("Types");writer.addNS(None,"http://schemas.openxmlformats.org/package/2006/content-types");
        // java writer.startElement("Default"); writer.addAttr("Extension", "rels"); writer.addAttr("ContentType", OxfRelations.RELS.getContentType()); writer.endElement("Default");
        writer.startElement("Default");writer.addAttr("Extension","rels");writer.addAttrNullable(Some("ContentType"),OxfRelations::RELS.getContentType().as_deref());writer.endElement("Default");
        // java writer.startElement("Default"); writer.addAttr("Extension", "xml"); writer.addAttr("ContentType", "application/xml"); writer.endElement("Default");
        writer.startElement("Default");writer.addAttr("Extension","xml");writer.addAttr("ContentType","application/xml");writer.endElement("Default");
        // java for(Map.Entry<String, String> type : this.mDefaultTypes.entrySet()) {
        for (key,value)in self.mDefaultTypes.entrySet(){
            // java writer.startElement("Default"); writer.addAttr("Extension", (String)type.getKey()); writer.addAttr("ContentType", (String)type.getValue()); writer.endElement("Default");
            writer.startElement("Default");writer.addAttr("Extension",key);writer.addAttrNullable(Some("ContentType"),value.as_deref());writer.endElement("Default");
        }
        // java int index = 0; for(int size = this.mOverrideContentTypes.size(); index < size; ++index) {
        let mut index=0;let size=self.mOverrideContentTypes.len();while index<size{
            // java writer.startElement("Override"); writer.addAttr("PartName", (String)this.mOverridePartNames.get(index)); writer.addAttr("ContentType", (String)this.mOverrideContentTypes.get(index)); writer.endElement("Override");
            writer.startElement("Override");writer.addAttr("PartName",&self.mOverridePartNames[index]);writer.addAttr("ContentType",&self.mOverrideContentTypes[index]);writer.endElement("Override");index+=1;
        }
        // java writer.endElement("Types"); writer.endDocument();
        writer.endElement("Types");writer.endDocument();Ok(())
    }
    // java private void initPartData(OpcwRelationships relses) throws IOException {
    fn initPartData(&mut self,relses:Option<&OpcwRelationships>)->io::Result<()>{
        // java if (relses != null) {
        if let Some(relses)=relses{
            // java for(OpcwRelationship rels : relses) { OpcwPackagePart part = rels.getPackagePart(); if (part != null) {
            for rels in relses.iterator(){let part=rels.borrow().getPackagePart();if let Some(part)=part{
                // java OpcRelation rel = part.getRelation(); String partName = rel.getPartName(part.getRelIndex());
                let rel=part.base().getRelation();let partName=rel.getPartName__index(part.base().getRelIndex());
                // java if (!partName.endsWith(".xml") && !partName.endsWith(".vml")) {
                if !partName.ends_with(".xml")&&!partName.ends_with(".vml"){
                    // java int index = partName.lastIndexOf("."); if (index != -1) { String suffix = partName.substring(index + 1); this.mDefaultTypes.put(suffix, rel.getContentType()); }
                    if let Some(index)=partName.rfind('.'){let suffix=partName[index+1..].to_string();self.mDefaultTypes.put(suffix,rel.getContentType());}
                // java } else {
                }else{
                    // java if (!rel.getContentType().equals(OxfPptRelations.ITEM.getContentType())) { this.mOverrideContentTypes.add(rel.getContentType()); this.mOverridePartNames.add(partName); }
                    if rel.getContentType().expect("NullPointerException")!=OxfPptRelations::ITEM().getContentType().unwrap(){self.mOverrideContentTypes.push(rel.getContentType().unwrap());self.mOverridePartNames.push(partName);}
                    // java this.initPartData(part.getRelationships());
                    self.initPartData(Some(&part.getRelationships().borrow()))?;
                }
            }}
        }Ok(())
    }
}
