use crate::kfc::{opcr::{OpcrPackagePart::OpcrPackagePart,ZipFile::{ZipFile,ZipEntry}},xpxml::{hlxml::HLXmlReader::HLXmlReader,reader::{ElementStrHandler::ElementStrHandler,DefaultElementStrHandler::Subclass}}};
use std::ops::{Deref,DerefMut};
// java public final class OxfrAppPart extends OpcrPackagePart {
pub struct OxfrAppPart{
    base:OpcrPackagePart,
    // java private String mAppName = null;
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
impl OxfrAppPart{
    // java public OxfrAppPart(ZipFile zipFile, ZipEntry entry) { super(zipFile, entry); }
    pub fn OxfrAppPart(zipFile:&ZipFile,entry:&ZipEntry)->Self{Self{base:OpcrPackagePart::OpcrPackagePart(zipFile,entry),mAppName:None,mAppVersion:None,mDocSecurity:-1,mManager:None,mCompany:None,mHyperlinkBase:None,bScale:false,bLinkDirty:false}}
    // java public void parse() throws IOException {
    pub fn parse(&mut self)->std::io::Result<()>{
        // java InputStream stream = this.getInputStream();
        let stream=self.base.getInputStream_nullable(false)?;
        // java if (stream != null) { HLXmlReader.parser(stream, (ElementStrHandler)(new PropertiesHandler())); }
        if let Some(mut stream)=stream{
            // Native outer-this binding: parser_str_handler owns and drops every
            // handler synchronously, including unwind. No handler escapes parse,
            // and the exclusive part borrow is suspended until parsing returns.
            HLXmlReader::parser_str_handler(&mut stream,Box::new(PropertiesHandler{outer:self as *mut Self}))?;
        }Ok(())
    }
    // java public String getAppName() { return this.mAppName; }
    pub fn getAppName(&self)->Option<String>{self.mAppName.clone()}
    // java public String getAppVersion() { return this.mAppVersion; }
    pub fn getAppVersion(&self)->Option<String>{self.mAppVersion.clone()}
    // java public int getDocSecurity() { return this.mDocSecurity; }
    pub fn getDocSecurity(&self)->i32{self.mDocSecurity}
    // java public String getHyperlinkBase() { return this.mHyperlinkBase; }
    pub fn getHyperlinkBase(&self)->Option<String>{self.mHyperlinkBase.clone()}
    // java public String getManager() { return this.mManager; }
    pub fn getManager(&self)->Option<String>{self.mManager.clone()}
    // java public String getCompany() { return this.mCompany; }
    pub fn getCompany(&self)->Option<String>{self.mCompany.clone()}
    // java public boolean getScale() { return this.bScale; }
    pub fn getScale(&self)->bool{self.bScale}
    // java public boolean getLinkDirty() { return this.bLinkDirty; }
    pub fn getLinkDirty(&self)->bool{self.bLinkDirty}
}
// java private class PropertiesHandler extends DefaultElementStrHandler {
struct PropertiesHandler{outer:*mut OxfrAppPart}
impl Subclass for PropertiesHandler{
    // java public ElementStrHandler enterSubElement(String subLabel) {
    fn enter_sub_element(&mut self,subLabel:&str)->Option<Box<dyn ElementStrHandler>>{
        // java if (subLabel.equals("Application")) { return OxfrAppPart.this.new ApplicationHandler(); }
        if subLabel=="Application"{Some(Box::new(ApplicationHandler{outer:self.outer}))}
        // java else if (subLabel.equals("AppVersion")) { return OxfrAppPart.this.new AppVersionHandler(); }
        else if subLabel=="AppVersion"{Some(Box::new(AppVersionHandler{outer:self.outer}))}
        // java else if (subLabel.equals("HyperlinkBase")) { return OxfrAppPart.this.new HyperlinkBaseHandler(); }
        else if subLabel=="HyperlinkBase"{Some(Box::new(HyperlinkBaseHandler{outer:self.outer}))}
        // java else if (subLabel.equals("Manager")) { return OxfrAppPart.this.new ManagerHandler(); }
        else if subLabel=="Manager"{Some(Box::new(ManagerHandler{outer:self.outer}))}
        // java else if (subLabel.equals("Company")) { return OxfrAppPart.this.new CompanyHandler(); }
        else if subLabel=="Company"{Some(Box::new(CompanyHandler{outer:self.outer}))}
        // java else if (subLabel.equals("ScaleCrop")) { return OxfrAppPart.this.new ScaleCropHandler(); }
        else if subLabel=="ScaleCrop"{Some(Box::new(ScaleCropHandler{outer:self.outer}))}
        // java else if (subLabel.equals("LinksUpToDate")) { return OxfrAppPart.this.new LinkUpToDateHandler(); }
        else if subLabel=="LinksUpToDate"{Some(Box::new(LinkUpToDateHandler{outer:self.outer}))}
        // java else { return subLabel.equals("DocSecurity") ? OxfrAppPart.this.new DocSecurityHandler() : null; }
        else if subLabel=="DocSecurity"{Some(Box::new(DocSecurityHandler{outer:self.outer}))}else{None}
    }
}
// The private handler pointers are scoped outer-this references as documented in
// parse. Each callback creates the only active mutable reborrow of that part.
// java private class ApplicationHandler extends DefaultElementStrHandler {
struct ApplicationHandler{outer:*mut OxfrAppPart}
impl Subclass for ApplicationHandler{
    // java public void characters(String chars) { OxfrAppPart.this.mAppName = chars; }
    fn characters(&mut self,chars:&str){unsafe{(*self.outer).mAppName=Some(chars.into());}}
}
// java private class AppVersionHandler extends DefaultElementStrHandler {
struct AppVersionHandler{outer:*mut OxfrAppPart}
impl Subclass for AppVersionHandler{
    // java public void characters(String chars) { OxfrAppPart.this.mAppVersion = chars; }
    fn characters(&mut self,chars:&str){unsafe{(*self.outer).mAppVersion=Some(chars.into());}}
}
// java private class HyperlinkBaseHandler extends DefaultElementStrHandler {
struct HyperlinkBaseHandler{outer:*mut OxfrAppPart}
impl Subclass for HyperlinkBaseHandler{
    // java public void characters(String chars) { OxfrAppPart.this.mHyperlinkBase = chars; }
    fn characters(&mut self,chars:&str){unsafe{(*self.outer).mHyperlinkBase=Some(chars.into());}}
}
// java private class ManagerHandler extends DefaultElementStrHandler {
struct ManagerHandler{outer:*mut OxfrAppPart}
impl Subclass for ManagerHandler{
    // java public void characters(String chars) { OxfrAppPart.this.mManager = chars; }
    fn characters(&mut self,chars:&str){unsafe{(*self.outer).mManager=Some(chars.into());}}
}
// java private class CompanyHandler extends DefaultElementStrHandler {
struct CompanyHandler{outer:*mut OxfrAppPart}
impl Subclass for CompanyHandler{
    // java public void characters(String chars) { OxfrAppPart.this.mCompany = chars; }
    fn characters(&mut self,chars:&str){unsafe{(*self.outer).mCompany=Some(chars.into());}}
}
// java private class ScaleCropHandler extends DefaultElementStrHandler {
struct ScaleCropHandler{outer:*mut OxfrAppPart}
impl Subclass for ScaleCropHandler{
    // java public void characters(String chars) { OxfrAppPart.this.bScale = Boolean.parseBoolean(chars); }
    fn characters(&mut self,chars:&str){unsafe{(*self.outer).bScale=chars.eq_ignore_ascii_case("true");}}
}
// java private class LinkUpToDateHandler extends DefaultElementStrHandler {
struct LinkUpToDateHandler{outer:*mut OxfrAppPart}
impl Subclass for LinkUpToDateHandler{
    // java public void characters(String chars) { OxfrAppPart.this.bLinkDirty = Boolean.parseBoolean(chars); }
    fn characters(&mut self,chars:&str){unsafe{(*self.outer).bLinkDirty=chars.eq_ignore_ascii_case("true");}}
}
// java private class DocSecurityHandler extends DefaultElementStrHandler {
struct DocSecurityHandler{outer:*mut OxfrAppPart}
impl Subclass for DocSecurityHandler{
    // java public void characters(String chars) { OxfrAppPart.this.mDocSecurity = Integer.parseInt(chars); }
    fn characters(&mut self,chars:&str){let n=crate::native_runtime::DecimalInteger::parseInt(&chars.encode_utf16().collect::<Vec<_>>()).expect("NumberFormatException");unsafe{(*self.outer).mDocSecurity=n;}}
}
impl Deref for OxfrAppPart{type Target=OpcrPackagePart;fn deref(&self)->&Self::Target{&self.base}}
impl DerefMut for OxfrAppPart{fn deref_mut(&mut self)->&mut Self::Target{&mut self.base}}
