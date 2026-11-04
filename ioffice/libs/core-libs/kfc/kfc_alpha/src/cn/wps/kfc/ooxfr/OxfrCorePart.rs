use crate::{kfc::{opcr::{OpcrPackagePart::OpcrPackagePart,OpcUtil::OpcUtil,ZipFile::{ZipFile,ZipEntry}},xpxml::{hlxml::HLXmlReader::HLXmlReader,reader::{ElementStrHandler::ElementStrHandler,DefaultElementStrHandler::Subclass}}},native_runtime::Date};
use std::{sync::Arc,ops::{Deref,DerefMut}};
// java public final class OxfrCorePart extends OpcrPackagePart {
pub struct OxfrCorePart {
    base:OpcrPackagePart,
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
    mCreated:Option<Arc<Date>>,
    // java private Date mModified = null;
    mModified:Option<Arc<Date>>,
    // java private String mCategory = null;
    mCategory:Option<String>,
    // java private String mContentStatus = null;
    mContentStatus:Option<String>,
}
impl OxfrCorePart {
    // java public OxfrCorePart(ZipFile zipFile, ZipEntry entry) { super(zipFile, entry); }
    pub fn OxfrCorePart(zipFile:&ZipFile,entry:&ZipEntry)->Self{Self{base:OpcrPackagePart::OpcrPackagePart(zipFile,entry),mTitle:None,mSubject:None,mCreator:None,mKeywords:None,mDescription:None,mLastModifiedBy:None,mCreated:None,mModified:None,mCategory:None,mContentStatus:None}}
    // java public void parse() throws IOException {
    pub fn parse(&mut self)->std::io::Result<()> {
        // java InputStream stream = this.getInputStream();
        let stream=self.base.getInputStream_nullable(false)?;
        // java if (stream != null) { HLXmlReader.parser(stream, (ElementStrHandler)(new CorePropertiesHandler())); }
        if let Some(mut stream)=stream {
            // Native outer-this binding: synchronous parser owns/drops all private
            // handlers before return/unwind, while this exclusive borrow is suspended.
            HLXmlReader::parser_str_handler(&mut stream,Box::new(CorePropertiesHandler{outer:self as *mut Self}))?;
        }Ok(())
    }
    // java public String getCreator() { return this.mCreator; }
    pub fn getCreator(&self)->Option<String>{self.mCreator.clone()}
    // java public Date getCreated() { return this.mCreated; }
    pub fn getCreated(&self)->Option<Arc<Date>>{self.mCreated.clone()}
    // java public String getLastModifiedBy() { return this.mLastModifiedBy; }
    pub fn getLastModifiedBy(&self)->Option<String>{self.mLastModifiedBy.clone()}
    // java public Date getModified() { return this.mModified; }
    pub fn getModified(&self)->Option<Arc<Date>>{self.mModified.clone()}
    // java public String getTitle() { return this.mTitle; }
    pub fn getTitle(&self)->Option<String>{self.mTitle.clone()}
    // java public String getKeywords() { return this.mKeywords; }
    pub fn getKeywords(&self)->Option<String>{self.mKeywords.clone()}
    // java public String getCategory() { return this.mCategory; }
    pub fn getCategory(&self)->Option<String>{self.mCategory.clone()}
    // java public String getDescription() { return this.mDescription; }
    pub fn getDescription(&self)->Option<String>{self.mDescription.clone()}
    // java public String getContentStatus() { return this.mContentStatus; }
    pub fn getContentStatus(&self)->Option<String>{self.mContentStatus.clone()}
    // java public String getSubject() { return this.mSubject; }
    pub fn getSubject(&self)->Option<String>{self.mSubject.clone()}
}
// java private class CorePropertiesHandler extends DefaultElementStrHandler {
struct CorePropertiesHandler{outer:*mut OxfrCorePart}
impl Subclass for CorePropertiesHandler {
    // java public ElementStrHandler enterSubElement(String subLabel) {
    fn enter_sub_element(&mut self,subLabel:&str)->Option<Box<dyn ElementStrHandler>> {
        // java if (subLabel.equals("dc:creator")) { return OxfrCorePart.this.new CreatorHandler(); }
        if subLabel=="dc:creator" {Some(Box::new(CreatorHandler{outer:self.outer}))}
        // java else if (subLabel.equals("cp:lastModifiedBy")) { return OxfrCorePart.this.new LastModifiedByHandler(); }
        else if subLabel=="cp:lastModifiedBy" {Some(Box::new(LastModifiedByHandler{outer:self.outer}))}
        // java else if (subLabel.equals("dcterms:created")) { return OxfrCorePart.this.new CreatedHandler(); }
        else if subLabel=="dcterms:created" {Some(Box::new(CreatedHandler{outer:self.outer}))}
        // java else if (subLabel.equals("dcterms:modified")) { return OxfrCorePart.this.new ModifiedHandler(); }
        else if subLabel=="dcterms:modified" {Some(Box::new(ModifiedHandler{outer:self.outer}))}
        // java else if (subLabel.equals("dc:title")) { return OxfrCorePart.this.new TitleHandler(); }
        else if subLabel=="dc:title" {Some(Box::new(TitleHandler{outer:self.outer}))}
        // java else if (subLabel.equals("cp:keywords")) { return OxfrCorePart.this.new KeywordsHandler(); }
        else if subLabel=="cp:keywords" {Some(Box::new(KeywordsHandler{outer:self.outer}))}
        // java else if (subLabel.equals("cp:category")) { return OxfrCorePart.this.new CategoryHandler(); }
        else if subLabel=="cp:category" {Some(Box::new(CategoryHandler{outer:self.outer}))}
        // java else if (subLabel.equals("dc:description")) { return OxfrCorePart.this.new DescriptionHandler(); }
        else if subLabel=="dc:description" {Some(Box::new(DescriptionHandler{outer:self.outer}))}
        // java else if (subLabel.equals("cp:contentStatus")) { return OxfrCorePart.this.new ContentStatusHandler(); }
        else if subLabel=="cp:contentStatus" {Some(Box::new(ContentStatusHandler{outer:self.outer}))}
        // java else { return subLabel.equals("dc:subject") ? OxfrCorePart.this.new SubjectHandler() : null; }
        else if subLabel=="dc:subject" {Some(Box::new(SubjectHandler{outer:self.outer}))}else{None}
    }
}
// Each private outer pointer is valid only for the synchronous parse call above;
// callbacks create the sole active reborrow and do not expose/store references.
// java private class CreatorHandler extends DefaultElementStrHandler {
struct CreatorHandler{outer:*mut OxfrCorePart}
impl Subclass for CreatorHandler {
    // java public void characters(String chars) { OxfrCorePart.this.mCreator = chars; }
    fn characters(&mut self,chars:&str){unsafe{(*self.outer).mCreator=Some(chars.into());}}
}
// java private class LastModifiedByHandler extends DefaultElementStrHandler {
struct LastModifiedByHandler{outer:*mut OxfrCorePart}
impl Subclass for LastModifiedByHandler {
    // java public void characters(String chars) { OxfrCorePart.this.mLastModifiedBy = chars; }
    fn characters(&mut self,chars:&str){unsafe{(*self.outer).mLastModifiedBy=Some(chars.into());}}
}
// java private class CreatedHandler extends DefaultElementStrHandler {
struct CreatedHandler{outer:*mut OxfrCorePart}
impl Subclass for CreatedHandler {
    // java public void characters(String chars) {
    fn characters(&mut self,chars:&str){unsafe{
        // java OxfrCorePart.this.mCreated = OpcUtil.getDate(chars);
        (*self.outer).mCreated=OpcUtil::getDate(chars).map(|t|Arc::new(Date::fromTime(t)));
        // java if (OxfrCorePart.this.mCreated != null && OxfrCorePart.this.mCreated.getTime() < 0L) {
        if let Some(date)=&(*self.outer).mCreated {if date.getTime()<0 {
            // java OxfrCorePart.this.mCreated.setTime(0L);
            date.setTime(0);
        }}
    }}
}
// java private class ModifiedHandler extends DefaultElementStrHandler {
struct ModifiedHandler{outer:*mut OxfrCorePart}
impl Subclass for ModifiedHandler {
    // java public void characters(String chars) {
    fn characters(&mut self,chars:&str){unsafe{
        // java OxfrCorePart.this.mModified = OpcUtil.getDate(chars);
        (*self.outer).mModified=OpcUtil::getDate(chars).map(|t|Arc::new(Date::fromTime(t)));
        // java if (OxfrCorePart.this.mModified != null && OxfrCorePart.this.mModified.getTime() < 0L) {
        if let Some(date)=&(*self.outer).mModified {if date.getTime()<0 {
            // java OxfrCorePart.this.mModified.setTime(0L);
            date.setTime(0);
        }}
    }}
}
// java private class TitleHandler extends DefaultElementStrHandler {
struct TitleHandler{outer:*mut OxfrCorePart}
impl Subclass for TitleHandler {
    // java public void characters(String chars) { OxfrCorePart.this.mTitle = chars; }
    fn characters(&mut self,chars:&str){unsafe{(*self.outer).mTitle=Some(chars.into());}}
}
// java private class KeywordsHandler extends DefaultElementStrHandler {
struct KeywordsHandler{outer:*mut OxfrCorePart}
impl Subclass for KeywordsHandler {
    // java public void characters(String chars) { OxfrCorePart.this.mKeywords = chars; }
    fn characters(&mut self,chars:&str){unsafe{(*self.outer).mKeywords=Some(chars.into());}}
}
// java private class CategoryHandler extends DefaultElementStrHandler {
struct CategoryHandler{outer:*mut OxfrCorePart}
impl Subclass for CategoryHandler {
    // java public void characters(String chars) { OxfrCorePart.this.mCategory = chars; }
    fn characters(&mut self,chars:&str){unsafe{(*self.outer).mCategory=Some(chars.into());}}
}
// java private class DescriptionHandler extends DefaultElementStrHandler {
struct DescriptionHandler{outer:*mut OxfrCorePart}
impl Subclass for DescriptionHandler {
    // java public void characters(String chars) { OxfrCorePart.this.mDescription = chars; }
    fn characters(&mut self,chars:&str){unsafe{(*self.outer).mDescription=Some(chars.into());}}
}
// java private class ContentStatusHandler extends DefaultElementStrHandler {
struct ContentStatusHandler{outer:*mut OxfrCorePart}
impl Subclass for ContentStatusHandler {
    // java public void characters(String chars) { OxfrCorePart.this.mContentStatus = chars; }
    fn characters(&mut self,chars:&str){unsafe{(*self.outer).mContentStatus=Some(chars.into());}}
}
// java private class SubjectHandler extends DefaultElementStrHandler {
struct SubjectHandler{outer:*mut OxfrCorePart}
impl Subclass for SubjectHandler {
    // java public void characters(String chars) { OxfrCorePart.this.mSubject = chars; }
    fn characters(&mut self,chars:&str){unsafe{(*self.outer).mSubject=Some(chars.into());}}
}
impl Deref for OxfrCorePart{type Target=OpcrPackagePart;fn deref(&self)->&Self::Target{&self.base}}
impl DerefMut for OxfrCorePart{fn deref_mut(&mut self)->&mut Self::Target{&mut self.base}}
