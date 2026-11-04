use crate::kfc::{opcw::{OpcwPackagePart::OpcwPackagePart,native::{SharedZip,RelationRef}},opcr::OpcUtil::OpcUtil,ooxfr::OxfRelations,xpxml::writer::{XmlWriter::XmlWriter,r#impl::BufferXmlWriter::BufferXmlWriter}};
// java public final class OxfwCorePart extends OpcwPackagePart {
pub struct OxfwCorePart{
    pub(crate) base:OpcwPackagePart,
    // java private String mTitle = null;
    mTitle:Option<String>,
    // java private String mSubject = null;
    mSubject:Option<String>,
    // java private String mCreator = null;
    mCreator:Option<String>,
    // java private String mKeywords = null;
    mKeywords:Option<String>,
    // java private String mDescription = null;
    mDescription:Option<String>,
    // java private String mLastModifiedBy = null;
    mLastModifiedBy:Option<String>,
    // java private Date mCreated = null;
    mCreated:Option<std::sync::Arc<crate::native_runtime::Date>>,
    // java private Date mModified = null;
    mModified:Option<std::sync::Arc<crate::native_runtime::Date>>,
    // java private String mCategory = null;
    mCategory:Option<String>,
    // java private String mContentStatus = null;
    mContentStatus:Option<String>,
}
impl OxfwCorePart{
    // java public OxfwCorePart(ZipOutputStream zipStream) { super(zipStream, OxfRelations.CORE, -1); }
    pub fn OxfwCorePart(zipStream:Option<SharedZip>)->Self{Self{base:OpcwPackagePart::OpcwPackagePart(zipStream,RelationRef::Static(&*OxfRelations::CORE),-1),mTitle:None,mSubject:None,mCreator:None,mKeywords:None,mDescription:None,mLastModifiedBy:None,mCreated:None,mModified:None,mCategory:None,mContentStatus:None}}
    // java public void setCreator(String creator) { this.mCreator = creator; }
    pub fn setCreator(&mut self,creator:Option<String>){self.mCreator=creator;}
    // java public void setCreated(Date created) { this.mCreated = created; }
    pub fn setCreated(&mut self,created:impl crate::native_runtime::JavaDateArg){self.mCreated=created.date();}
    // java public void setLastModifiedBy(String modify) { this.mLastModifiedBy = modify; }
    pub fn setLastModifiedBy(&mut self,modify:Option<String>){self.mLastModifiedBy=modify;}
    // java public void setModified(Date modified) { this.mModified = modified; }
    pub fn setModified(&mut self,modified:impl crate::native_runtime::JavaDateArg){self.mModified=modified.date();}
    // java public void setTitle(String title) { this.mTitle = title; }
    pub fn setTitle(&mut self,title:Option<String>){self.mTitle=title;}
    // java public void setKeywords(String keywords) { this.mKeywords = keywords; }
    pub fn setKeywords(&mut self,keywords:Option<String>){self.mKeywords=keywords;}
    // java public void setCategory(String category) { this.mCategory = category; }
    pub fn setCategory(&mut self,category:Option<String>){self.mCategory=category;}
    // java public void setDescription(String description) { this.mDescription = description; }
    pub fn setDescription(&mut self,description:Option<String>){self.mDescription=description;}
    // java public void setContentStatus(String status) { this.mContentStatus = status; }
    pub fn setContentStatus(&mut self,status:Option<String>){self.mContentStatus=status;}
    // java public void setSubject(String subject) { this.mSubject = subject; }
    pub fn setSubject(&mut self,subject:Option<String>){self.mSubject=subject;}
    // java void write() throws IOException {
    pub fn write(&mut self)->std::io::Result<()>{
        // java OutputStream stream = super.getOutputStream(); if (null != stream) {
        if let Some(stream)=self.base.getOutputStream()?{
            // java XmlWriter writer = new BufferXmlWriter(stream); writer.startDocument(); writer.startElement("cp", "coreProperties"); writer.addNS("cp", "http://schemas.openxmlformats.org/package/2006/metadata/core-properties");
            let mut writer=BufferXmlWriter::new(Box::new(stream));writer.startDocument();writer.startElement__ns(Some("cp"),"coreProperties");writer.addNS(Some("cp"),"http://schemas.openxmlformats.org/package/2006/metadata/core-properties");
            // java if (this.hasDcNamespace()) { writer.addNS("dc", "http://purl.org/dc/elements/1.1/"); }
            if self.hasDcNamespace(){writer.addNS(Some("dc"),"http://purl.org/dc/elements/1.1/");}
            // java if (this.mCreated != null || this.mModified != null) { writer.addNS("dcterms", "http://purl.org/dc/terms/"); writer.addNS("xsi", "http://www.w3.org/2001/XMLSchema-instance"); }
            if self.mCreated.is_some()||self.mModified.is_some(){writer.addNS(Some("dcterms"),"http://purl.org/dc/terms/");writer.addNS(Some("xsi"),"http://www.w3.org/2001/XMLSchema-instance");}
            // java if (this.mTitle != null && this.mTitle.length() > 0) { writer.startElement("dc", "title"); writer.addText(this.mTitle); writer.endElement("dc", "title"); }
            if let Some(v)=self.mTitle.as_ref().filter(|v|!v.is_empty()){writer.startElement__ns(Some("dc"),"title");writer.addText(v);writer.endElement__ns(Some("dc"),"title");}
            // java if (this.mSubject != null && this.mSubject.length() > 0) { writer.startElement("dc", "subject"); writer.addText(this.mSubject); writer.endElement("dc", "subject"); }
            if let Some(v)=self.mSubject.as_ref().filter(|v|!v.is_empty()){writer.startElement__ns(Some("dc"),"subject");writer.addText(v);writer.endElement__ns(Some("dc"),"subject");}
            // java if (this.mCreator != null && this.mCreator.length() > 0) { writer.startElement("dc", "creator"); writer.addText(this.mCreator); writer.endElement("dc", "creator"); }
            if let Some(v)=self.mCreator.as_ref().filter(|v|!v.is_empty()){writer.startElement__ns(Some("dc"),"creator");writer.addText(v);writer.endElement__ns(Some("dc"),"creator");}
            // java if (this.mKeywords != null && this.mKeywords.length() > 0) { writer.startElement("cp", "keywords"); writer.addText(this.mKeywords); writer.endElement("cp", "keywords"); }
            if let Some(v)=self.mKeywords.as_ref().filter(|v|!v.is_empty()){writer.startElement__ns(Some("cp"),"keywords");writer.addText(v);writer.endElement__ns(Some("cp"),"keywords");}
            // java if (this.mDescription != null && this.mDescription.length() > 0) { writer.startElement("dc", "description"); writer.addText(this.mDescription); writer.endElement("dc", "description"); }
            if let Some(v)=self.mDescription.as_ref().filter(|v|!v.is_empty()){writer.startElement__ns(Some("dc"),"description");writer.addText(v);writer.endElement__ns(Some("dc"),"description");}
            // java if (this.mLastModifiedBy != null && this.mLastModifiedBy.length() > 0) { writer.startElement("cp", "lastModifiedBy"); writer.addText(this.mLastModifiedBy); writer.endElement("cp", "lastModifiedBy"); }
            if let Some(v)=self.mLastModifiedBy.as_ref().filter(|v|!v.is_empty()){writer.startElement__ns(Some("cp"),"lastModifiedBy");writer.addText(v);writer.endElement__ns(Some("cp"),"lastModifiedBy");}
            // java if (this.mCreated != null) { writer.startElement("dcterms", "created"); writer.addAttr("xsi", "type", "dcterms:W3CDTF"); writer.addText(OpcUtil.getDateString(this.mCreated)); writer.endElement("dcterms", "created"); }
            if let Some(v)=&self.mCreated{writer.startElement__ns(Some("dcterms"),"created");writer.addAttr__ns(Some("xsi"),"type","dcterms:W3CDTF");writer.addText(&OpcUtil::getDateString(v.getTime()));writer.endElement__ns(Some("dcterms"),"created");}
            // java if (this.mModified != null) { writer.startElement("dcterms", "modified"); writer.addAttr("xsi", "type", "dcterms:W3CDTF"); writer.addText(OpcUtil.getDateString(this.mModified)); writer.endElement("dcterms", "modified"); }
            if let Some(v)=&self.mModified{writer.startElement__ns(Some("dcterms"),"modified");writer.addAttr__ns(Some("xsi"),"type","dcterms:W3CDTF");writer.addText(&OpcUtil::getDateString(v.getTime()));writer.endElement__ns(Some("dcterms"),"modified");}
            // java if (this.mCategory != null && this.mCategory.length() > 0) { writer.startElement("cp", "category"); writer.addText(this.mCategory); writer.endElement("cp", "category"); }
            if let Some(v)=self.mCategory.as_ref().filter(|v|!v.is_empty()){writer.startElement__ns(Some("cp"),"category");writer.addText(v);writer.endElement__ns(Some("cp"),"category");}
            // java if (this.mContentStatus != null && this.mContentStatus.length() > 0) { writer.startElement("cp", "contentStatus"); writer.addText(this.mContentStatus); writer.endElement("cp", "contentStatus"); }
            if let Some(v)=self.mContentStatus.as_ref().filter(|v|!v.is_empty()){writer.startElement__ns(Some("cp"),"contentStatus");writer.addText(v);writer.endElement__ns(Some("cp"),"contentStatus");}
            // java writer.endElement("cp", "coreProperties"); writer.endDocument();
            writer.endElement__ns(Some("cp"),"coreProperties");writer.endDocument();
        }Ok(())
    }
    // java private boolean hasDcNamespace() {
    fn hasDcNamespace(&self)->bool{
        // java if (this.mCreator != null && this.mCreator.length() > 0) { return true; } else if (this.mTitle != null && this.mTitle.length() > 0) { return true; } else { return this.mDescription != null && this.mDescription.length() > 0; }
        if self.mCreator.as_ref().is_some_and(|v|!v.is_empty()){true}else if self.mTitle.as_ref().is_some_and(|v|!v.is_empty()){true}else{self.mDescription.as_ref().is_some_and(|v|!v.is_empty())}
    }
}
impl std::ops::Deref for OxfwCorePart{type Target=OpcwPackagePart;fn deref(&self)->&Self::Target{&self.base}}
impl std::ops::DerefMut for OxfwCorePart{fn deref_mut(&mut self)->&mut Self::Target{&mut self.base}}
