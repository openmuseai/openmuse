use crate::kfc::{opcw::{OpcwPackagePart::OpcwPackagePart,native::{SharedZip,RelationRef}},ooxfr::OxfRelations,xpxml::writer::{XmlWriter::XmlWriter,r#impl::BufferXmlWriter::BufferXmlWriter}};
// java public final class OxfwAppPart extends OpcwPackagePart {
pub struct OxfwAppPart{
    pub(crate) base:OpcwPackagePart,
    // java private String mAppName = "WPS Office";
    mAppName:Option<String>,
    // java private String mAppVersion = null;
    mAppVersion:Option<String>,
    // java private int mDocSecurity = -1;
    mDocSecurity:i32,
    // java private String mManager = null;
    mManager:Option<String>,
    // java private String mCompany = null;
    mCompany:Option<String>,
    // java private String mHyperlinkBase = null;
    mHyperlinkBase:Option<String>,
    // java private boolean bScale = false;
    bScale:bool,
    // java private boolean bLinkDirty = false;
    bLinkDirty:bool,
}
impl OxfwAppPart{
    // java public OxfwAppPart(ZipOutputStream zipStream) { super(zipStream, OxfRelations.APP, -1); }
    pub fn OxfwAppPart(zipStream:Option<SharedZip>)->Self{Self{base:OpcwPackagePart::OpcwPackagePart(zipStream,RelationRef::Static(&*OxfRelations::APP),-1),mAppName:Some("WPS Office".into()),mAppVersion:None,mDocSecurity:-1,mManager:None,mCompany:None,mHyperlinkBase:None,bScale:false,bLinkDirty:false}}
    // java public void setAppName(String appName) { this.mAppName = appName; }
    pub fn setAppName(&mut self,appName:Option<String>){self.mAppName=appName;}
    // java public void setAppVersion(String appVersion) { this.mAppVersion = appVersion; }
    pub fn setAppVersion(&mut self,appVersion:Option<String>){self.mAppVersion=appVersion;}
    // java public void setDocSecurity(int security) { this.mDocSecurity = security; }
    pub fn setDocSecurity(&mut self,security:i32){self.mDocSecurity=security;}
    // java public void setHyperlinkBase(String hyperlink) { this.mHyperlinkBase = hyperlink; }
    pub fn setHyperlinkBase(&mut self,hyperlink:Option<String>){self.mHyperlinkBase=hyperlink;}
    // java public void setCompany(String company) { this.mCompany = company; }
    pub fn setCompany(&mut self,company:Option<String>){self.mCompany=company;}
    // java public void setManager(String manager) { this.mManager = manager; }
    pub fn setManager(&mut self,manager:Option<String>){self.mManager=manager;}
    // java public void setScale(boolean b) { this.bScale = b; }
    pub fn setScale(&mut self,b:bool){self.bScale=b;}
    // java public void setLinkDirty(boolean b) { this.bLinkDirty = b; }
    pub fn setLinkDirty(&mut self,b:bool){self.bLinkDirty=b;}
    // java void write() throws IOException {
    pub fn write(&mut self)->std::io::Result<()>{
        // java OutputStream stream = super.getOutputStream(); if (null != stream) {
        if let Some(stream)=self.base.getOutputStream()?{
            // java XmlWriter writer = new BufferXmlWriter(stream); writer.startDocument(); writer.startElement("Properties"); writer.addNS((String)null, "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties");
            let mut writer=BufferXmlWriter::new(Box::new(stream));writer.startDocument();writer.startElement("Properties");writer.addNS(None,"http://schemas.openxmlformats.org/officeDocument/2006/extended-properties");
            // java if (this.mAppName != null && this.mAppName.length() > 0) { writer.startElement("Application"); writer.addText(this.mAppName); writer.endElement("Application"); }
            if let Some(v)=self.mAppName.as_ref().filter(|v|!v.is_empty()){writer.startElement("Application");writer.addText(v);writer.endElement("Application");}
            // java if (this.mDocSecurity != -1) { writer.startElement("DocSecurity"); writer.addText(this.mDocSecurity); writer.endElement("DocSecurity"); }
            if self.mDocSecurity!=-1{writer.startElement("DocSecurity");writer.addText__i32(self.mDocSecurity);writer.endElement("DocSecurity");}
            // java writer.startElement("ScaleCrop"); writer.addText(this.bScale); writer.endElement("ScaleCrop");
            writer.startElement("ScaleCrop");writer.addText__bool(self.bScale);writer.endElement("ScaleCrop");
            // java if (this.mManager != null && this.mManager.length() > 0) { writer.startElement("Manager"); writer.addText(this.mManager); writer.endElement("Manager"); }
            if let Some(v)=self.mManager.as_ref().filter(|v|!v.is_empty()){writer.startElement("Manager");writer.addText(v);writer.endElement("Manager");}
            // java if (this.mCompany != null && this.mCompany.length() > 0) { writer.startElement("Company"); writer.addText(this.mCompany); writer.endElement("Company"); }
            if let Some(v)=self.mCompany.as_ref().filter(|v|!v.is_empty()){writer.startElement("Company");writer.addText(v);writer.endElement("Company");}
            // java writer.startElement("LinksUpToDate"); writer.addText(this.bLinkDirty); writer.endElement("LinksUpToDate");
            writer.startElement("LinksUpToDate");writer.addText__bool(self.bLinkDirty);writer.endElement("LinksUpToDate");
            // java if (this.mHyperlinkBase != null && this.mHyperlinkBase.length() > 0) { writer.startElement("HyperlinkBase"); writer.addText(this.mHyperlinkBase); writer.endElement("HyperlinkBase"); }
            if let Some(v)=self.mHyperlinkBase.as_ref().filter(|v|!v.is_empty()){writer.startElement("HyperlinkBase");writer.addText(v);writer.endElement("HyperlinkBase");}
            // java if (this.mAppVersion != null && this.mAppVersion.length() > 0) { writer.startElement("AppVersion"); writer.addText(this.mAppVersion); writer.endElement("AppVersion"); }
            if let Some(v)=self.mAppVersion.as_ref().filter(|v|!v.is_empty()){writer.startElement("AppVersion");writer.addText(v);writer.endElement("AppVersion");}
            // java writer.endElement("Properties"); writer.endDocument();
            writer.endElement("Properties");writer.endDocument();
        }Ok(())
    }
}
impl std::ops::Deref for OxfwAppPart{type Target=OpcwPackagePart;fn deref(&self)->&Self::Target{&self.base}}
impl std::ops::DerefMut for OxfwAppPart{fn deref_mut(&mut self)->&mut Self::Target{&mut self.base}}
