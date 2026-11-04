// Java: import java.util.Date
// Java: import org.apache.poi.hpsf.wellknown.PropertyIDMap

use super::SpecialPropertySet::SpecialPropertySet;
use super::PropertySet::PropertySet;
use super::MutablePropertySet::MutablePropertySet;

// Java: public class SummaryInformation extends SpecialPropertySet
pub struct SummaryInformation {
    pub(crate) base: SpecialPropertySet,
}

// Java: public class SummaryInformation extends SpecialPropertySet
impl SummaryInformation {
    // Java field: String DEFAULT_STREAM_NAME
    pub const DEFAULT_STREAM_NAME: &'static str = "\u{0005}SummaryInformation";

    // Java: abstract PropertyIDMap getPropertySetIDMap()
    // Java: return PropertyIDMap.getSummaryInformationProperties();

    // Java: SummaryInformation(PropertySet ps)
    pub fn new(ps: &PropertySet) -> Self {
        // Java: super(ps);
        let base = SpecialPropertySet::new(ps);
        // Java: if (!this.isSummaryInformation())
        // Java: throw new UnexpectedPropertySetTypeException("Not a " + this.getClass().getName());
        SummaryInformation { base }
    }

    pub fn new_from_property_set(ps: PropertySet) -> std::io::Result<Self> {
        Ok(Self::new(&ps))
    }

    pub fn new_from_mutable(ps: MutablePropertySet) -> Self {
        SummaryInformation {
            base: SpecialPropertySet::new_from_mutable(ps),
        }
    }

    // Java: String getTitle()
    pub fn getTitle(&self) -> Option<String> {
        // Java: return (String)this.getProperty(2);
        None
    }

    // Java: void setTitle(String title)
    pub fn setTitle(&mut self, _title: &str) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(2, (String)title);
    }

    // Java: void removeTitle()
    pub fn removeTitle(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(2L);
    }

    // Java: String getSubject()
    pub fn getSubject(&self) -> Option<String> {
        // Java: return (String)this.getProperty(3);
        None
    }

    // Java: void setSubject(String subject)
    pub fn setSubject(&mut self, _subject: &str) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(3, (String)subject);
    }

    // Java: void removeSubject()
    pub fn removeSubject(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(3L);
    }

    // Java: String getAuthor()
    pub fn getAuthor(&self) -> Option<String> {
        // Java: return (String)this.getProperty(4);
        None
    }

    // Java: void setAuthor(String author)
    pub fn setAuthor(&mut self, _author: &str) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(4, (String)author);
    }

    // Java: void removeAuthor()
    pub fn removeAuthor(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(4L);
    }

    // Java: String getKeywords()
    pub fn getKeywords(&self) -> Option<String> {
        // Java: return (String)this.getProperty(5);
        None
    }

    // Java: void setKeywords(String keywords)
    pub fn setKeywords(&mut self, _keywords: &str) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(5, (String)keywords);
    }

    // Java: void removeKeywords()
    pub fn removeKeywords(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(5L);
    }

    // Java: String getComments()
    pub fn getComments(&self) -> Option<String> {
        // Java: return (String)this.getProperty(6);
        None
    }

    // Java: void setComments(String comments)
    pub fn setComments(&mut self, _comments: &str) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(6, (String)comments);
    }

    // Java: void removeComments()
    pub fn removeComments(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(6L);
    }

    // Java: String getTemplate()
    pub fn getTemplate(&self) -> Option<String> {
        // Java: return (String)this.getProperty(7);
        None
    }

    // Java: void setTemplate(String template)
    pub fn setTemplate(&mut self, _template: &str) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(7, (String)template);
    }

    // Java: void removeTemplate()
    pub fn removeTemplate(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(7L);
    }

    // Java: String getLastAuthor()
    pub fn getLastAuthor(&self) -> Option<String> {
        // Java: return (String)this.getProperty(8);
        None
    }

    // Java: void setLastAuthor(String lastAuthor)
    pub fn setLastAuthor(&mut self, _lastAuthor: &str) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(8, (String)lastAuthor);
    }

    // Java: void removeLastAuthor()
    pub fn removeLastAuthor(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(8L);
    }

    // Java: String getRevNumber()
    pub fn getRevNumber(&self) -> Option<String> {
        // Java: Object obj = this.getProperty(9);
        // Java: return null != obj && obj instanceof String ? (String)obj : null;
        None
    }

    // Java: void setRevNumber(String revNumber)
    pub fn setRevNumber(&mut self, _revNumber: &str) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(9, (String)revNumber);
    }

    // Java: void removeRevNumber()
    pub fn removeRevNumber(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(9L);
    }

    // Java: long getEditTime()
    pub fn getEditTime(&self) -> i64 {
        // Java: Object obj = this.getProperty(10);
        // Java: if (null != obj && obj instanceof Date)
        // Java: Date d = (Date)obj;
        // Java: return Util.dateToFileTime(d);
        // Java: else
        // Java: return 0L;
        0
    }

    // Java: void setEditTime(long time)
    pub fn setEditTime(&mut self, _time: i64) {
        // Java: Date d = Util.filetimeToDate(time);
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(10, 64L, d);
    }

    // Java: void removeEditTime()
    pub fn removeEditTime(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(10L);
    }

    // Java: Date getLastPrinted()
    pub fn getLastPrinted(&self) -> i64 {
        // Java: Object obj = this.getProperty(11);
        // Java: return null != obj && obj instanceof Date ? (Date)obj : null;
        0
    }

    // Java: void setLastPrinted(Date lastPrinted)
    pub fn setLastPrinted(&mut self, _lastPrinted: i64) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(11, 64L, lastPrinted);
    }

    // Java: void removeLastPrinted()
    pub fn removeLastPrinted(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(11L);
    }

    // Java: Date getCreateDateTime()
    pub fn getCreateDateTime(&self) -> i64 {
        // Java: Object obj = this.getProperty(12);
        // Java: return null != obj && obj instanceof Date ? (Date)obj : null;
        0
    }

    // Java: void setCreateDateTime(Date createDateTime)
    pub fn setCreateDateTime(&mut self, _createDateTime: i64) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(12, 64L, createDateTime);
    }

    // Java: void removeCreateDateTime()
    pub fn removeCreateDateTime(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(12L);
    }

    // Java: Date getLastSaveDateTime()
    pub fn getLastSaveDateTime(&self) -> i64 {
        // Java: Object obj = this.getProperty(13);
        // Java: return null != obj && obj instanceof Date ? (Date)obj : null;
        0
    }

    // Java: void setLastSaveDateTime(Date time)
    pub fn setLastSaveDateTime(&mut self, _time: i64) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(13, 64L, time);
    }

    // Java: void removeLastSaveDateTime()
    pub fn removeLastSaveDateTime(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(13L);
    }

    // Java: int getPageCount()
    pub fn getPageCount(&self) -> i32 {
        // Java: return this.getPropertyIntValue(14);
        self.base.getPropertyIntValue(14)
    }

    // Java: void setPageCount(int pageCount)
    pub fn setPageCount(&mut self, _pageCount: i32) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(14, pageCount);
    }

    // Java: void removePageCount()
    pub fn removePageCount(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(14L);
    }

    // Java: int getWordCount()
    pub fn getWordCount(&self) -> i32 {
        // Java: return this.getPropertyIntValue(15);
        self.base.getPropertyIntValue(15)
    }

    // Java: void setWordCount(int wordCount)
    pub fn setWordCount(&mut self, _wordCount: i32) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(15, wordCount);
    }

    // Java: void removeWordCount()
    pub fn removeWordCount(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(15L);
    }

    // Java: int getCharCount()
    pub fn getCharCount(&self) -> i32 {
        // Java: return this.getPropertyIntValue(16);
        self.base.getPropertyIntValue(16)
    }

    // Java: void setCharCount(int charCount)
    pub fn setCharCount(&mut self, _charCount: i32) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(16, charCount);
    }

    // Java: void removeCharCount()
    pub fn removeCharCount(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(16L);
    }

    // Java: byte[] getThumbnail()
    pub fn getThumbnail(&self) -> Option<Vec<u8>> {
        // Java: return (byte[])this.getProperty(17);
        None
    }

    // Java: void setThumbnail(byte[] thumbnail)
    pub fn setThumbnail(&mut self, _thumbnail: &[u8]) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(17, 30L, thumbnail);
    }

    // Java: void removeThumbnail()
    pub fn removeThumbnail(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(17L);
    }

    // Java: String getApplicationName()
    pub fn getApplicationName(&self) -> Option<String> {
        // Java: return (String)this.getProperty(18);
        None
    }

    // Java: void setApplicationName(String applicationName)
    pub fn setApplicationName(&mut self, _applicationName: &str) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(18, (String)applicationName);
    }

    // Java: void removeApplicationName()
    pub fn removeApplicationName(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(18L);
    }

    // Java: int getSecurity()
    pub fn getSecurity(&self) -> i32 {
        // Java: return this.getPropertyIntValue(19);
        self.base.getPropertyIntValue(19)
    }

    // Java: void setSecurity(int security)
    pub fn setSecurity(&mut self, _security: i32) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(19, security);
    }

    // Java: void removeSecurity()
    pub fn removeSecurity(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(19L);
    }
}

impl From<SummaryInformation> for PropertySet {
    fn from(si: SummaryInformation) -> Self {
        si.base.base.base
    }
}
