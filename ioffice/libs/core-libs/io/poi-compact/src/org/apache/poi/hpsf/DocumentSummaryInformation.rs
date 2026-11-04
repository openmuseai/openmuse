// Java: import cn.wps.kfc.api.map.ILongObjectMap
// Java: import org.apache.poi.hpsf.wellknown.PropertyIDMap
// Java: import org.apache.poi.hpsf.wellknown.SectionIDMap

use std::any::Any;
use super::SpecialPropertySet::SpecialPropertySet;
use super::PropertySet::{PropertySet, Section};
use super::MutablePropertySet::MutablePropertySet;

// Java: public class DocumentSummaryInformation extends SpecialPropertySet
pub struct DocumentSummaryInformation {
    pub(crate) base: SpecialPropertySet,
}

// Java: public class DocumentSummaryInformation extends SpecialPropertySet
impl DocumentSummaryInformation {
    // Java field: String DEFAULT_STREAM_NAME
    pub const DEFAULT_STREAM_NAME: &'static str = "\u{0005}DocumentSummaryInformation";

    // Java: abstract PropertyIDMap getPropertySetIDMap()
    // Java: return PropertyIDMap.getDocumentSummaryInformationProperties();

    // Java: DocumentSummaryInformation(PropertySet ps)
    pub fn new(ps: &PropertySet) -> Self {
        // Java: super(ps);
        let base = SpecialPropertySet::new(ps);
        // Java: if (!this.isDocumentSummaryInformation())
        // Java: throw new UnexpectedPropertySetTypeException("Not a " + this.getClass().getName());
        DocumentSummaryInformation { base }
    }

    pub fn new_from_property_set(ps: PropertySet) -> std::io::Result<Self> {
        Ok(Self::new(&ps))
    }

    pub fn new_from_mutable(ps: MutablePropertySet) -> Self {
        DocumentSummaryInformation {
            base: SpecialPropertySet::new_from_mutable(ps),
        }
    }

    // Java: String getCategory()
    pub fn getCategory(&self) -> Option<String> {
        // Java: return (String)this.getProperty(2);
        None
    }

    // Java: void setCategory(String category)
    pub fn setCategory(&mut self, _category: &str) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(2, (String)category);
    }

    // Java: void removeCategory()
    pub fn removeCategory(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(2L);
    }

    // Java: String getPresentationFormat()
    pub fn getPresentationFormat(&self) -> Option<String> {
        // Java: Object obj = this.getProperty(3);
        // Java: return null != obj && obj instanceof String ? (String)obj : null;
        None
    }

    // Java: void setPresentationFormat(String presentationFormat)
    pub fn setPresentationFormat(&mut self, _presentationFormat: &str) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(3, (String)presentationFormat);
    }

    // Java: void removePresentationFormat()
    pub fn removePresentationFormat(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(3L);
    }

    // Java: int getByteCount()
    pub fn getByteCount(&self) -> i32 {
        // Java: return this.getPropertyIntValue(4);
        self.base.getPropertyIntValue(4)
    }

    // Java: void setByteCount(int byteCount)
    pub fn setByteCount(&mut self, _byteCount: i32) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(4, byteCount);
    }

    // Java: void removeByteCount()
    pub fn removeByteCount(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(4L);
    }

    // Java: int getLineCount()
    pub fn getLineCount(&self) -> i32 {
        // Java: return this.getPropertyIntValue(5);
        self.base.getPropertyIntValue(5)
    }

    // Java: void setLineCount(int lineCount)
    pub fn setLineCount(&mut self, _lineCount: i32) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(5, lineCount);
    }

    // Java: void removeLineCount()
    pub fn removeLineCount(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(5L);
    }

    // Java: int getParaCount()
    pub fn getParaCount(&self) -> i32 {
        // Java: return this.getPropertyIntValue(6);
        self.base.getPropertyIntValue(6)
    }

    // Java: void setParaCount(int paraCount)
    pub fn setParaCount(&mut self, _paraCount: i32) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(6, paraCount);
    }

    // Java: void removeParaCount()
    pub fn removeParaCount(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(6L);
    }

    // Java: int getSlideCount()
    pub fn getSlideCount(&self) -> i32 {
        // Java: return this.getPropertyIntValue(7);
        self.base.getPropertyIntValue(7)
    }

    // Java: void setSlideCount(int slideCount)
    pub fn setSlideCount(&mut self, _slideCount: i32) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(7, slideCount);
    }

    // Java: void removeSlideCount()
    pub fn removeSlideCount(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(7L);
    }

    // Java: int getNoteCount()
    pub fn getNoteCount(&self) -> i32 {
        // Java: return this.getPropertyIntValue(8);
        self.base.getPropertyIntValue(8)
    }

    // Java: void setNoteCount(int noteCount)
    pub fn setNoteCount(&mut self, _noteCount: i32) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(8, noteCount);
    }

    // Java: void removeNoteCount()
    pub fn removeNoteCount(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(8L);
    }

    // Java: int getHiddenCount()
    pub fn getHiddenCount(&self) -> i32 {
        // Java: return this.getPropertyIntValue(9);
        self.base.getPropertyIntValue(9)
    }

    // Java: void setHiddenCount(int hiddenCount)
    pub fn setHiddenCount(&mut self, _hiddenCount: i32) {
        // Java: MutableSection s = (MutableSection)this.getSections().get(0);
        // Java: s.setProperty(9, hiddenCount);
    }

    // Java: void removeHiddenCount()
    pub fn removeHiddenCount(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(9L);
    }

    // Java: int getMMClipCount()
    pub fn getMMClipCount(&self) -> i32 {
        // Java: return this.getPropertyIntValue(10);
        self.base.getPropertyIntValue(10)
    }

    // Java: void setMMClipCount(int mmClipCount)
    pub fn setMMClipCount(&mut self, _mmClipCount: i32) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(10, mmClipCount);
    }

    // Java: void removeMMClipCount()
    pub fn removeMMClipCount(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(10L);
    }

    // Java: boolean getScale()
    pub fn getScale(&self) -> bool {
        // Java: return this.getPropertyBooleanValue(11);
        self.base.getPropertyBooleanValue(11)
    }

    // Java: void setScale(boolean scale)
    pub fn setScale(&mut self, _scale: bool) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(11, scale);
    }

    // Java: void removeScale()
    pub fn removeScale(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(11L);
    }

    // Java: byte[] getHeadingPair()
    pub fn getHeadingPair(&self) -> Option<Vec<u8>> {
        // Java: this.notYetImplemented("Reading byte arrays ");
        // Java: return (byte[])this.getProperty(12);
        None
    }

    // Java: void setHeadingPair(byte[] headingPair)
    pub fn setHeadingPair(&mut self, _headingPair: &[u8]) {
        // Java: this.notYetImplemented("Writing byte arrays ");
    }

    // Java: void removeHeadingPair()
    pub fn removeHeadingPair(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(12L);
    }

    // Java: byte[] getDocparts()
    pub fn getDocparts(&self) -> Option<Vec<u8>> {
        // Java: this.notYetImplemented("Reading byte arrays");
        // Java: return (byte[])this.getProperty(13);
        None
    }

    // Java: void setDocparts(byte[] docparts)
    pub fn setDocparts(&mut self, _docparts: &[u8]) {
        // Java: this.notYetImplemented("Writing byte arrays");
    }

    // Java: void removeDocparts()
    pub fn removeDocparts(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(13L);
    }

    // Java: String getManager()
    pub fn getManager(&self) -> Option<String> {
        // Java: return (String)this.getProperty(14);
        None
    }

    // Java: void setManager(String manager)
    pub fn setManager(&mut self, _manager: &str) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(14, (String)manager);
    }

    // Java: void removeManager()
    pub fn removeManager(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(14L);
    }

    // Java: String getCompany()
    pub fn getCompany(&self) -> Option<String> {
        // Java: return (String)this.getProperty(15);
        None
    }

    // Java: void setCompany(String company)
    pub fn setCompany(&mut self, _company: &str) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(15, (String)company);
    }

    // Java: void removeCompany()
    pub fn removeCompany(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(15L);
    }

    // Java: boolean getLinksDirty()
    pub fn getLinksDirty(&self) -> bool {
        // Java: return this.getPropertyBooleanValue(16);
        self.base.getPropertyBooleanValue(16)
    }

    // Java: void setLinksDirty(boolean linksDirty)
    pub fn setLinksDirty(&mut self, _linksDirty: bool) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(16, linksDirty);
    }

    // Java: void removeLinksDirty()
    pub fn removeLinksDirty(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(16L);
    }

    // Java: int getCchWithSpace()
    pub fn getCchWithSpace(&self) -> i32 {
        // Java: return this.getPropertyIntValue(17);
        self.base.getPropertyIntValue(17)
    }

    // Java: void setCchWithSpace(int cchWithspace)
    pub fn setCchWithSpace(&mut self, _cchWithspace: i32) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(17, cchWithspace);
    }

    // Java: void removeCchWithSpace()
    pub fn removeCchWithSpace(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(17L);
    }

    // Java: boolean getSharedDoc()
    pub fn getSharedDoc(&self) -> bool {
        // Java: return this.getPropertyBooleanValue(19);
        self.base.getPropertyBooleanValue(19)
    }

    // Java: void setSharedDoc(boolean sharedDoc)
    pub fn setSharedDoc(&mut self, _sharedDoc: bool) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(19, sharedDoc);
    }

    // Java: void removeSharedDoc()
    pub fn removeSharedDoc(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(19L);
    }

    // Java: boolean getHyperlinksChanged()
    pub fn getHyperlinksChanged(&self) -> bool {
        // Java: return this.getPropertyBooleanValue(22);
        self.base.getPropertyBooleanValue(22)
    }

    // Java: void setHyperlinksChanged(boolean hyperlinksChanged)
    pub fn setHyperlinksChanged(&mut self, _hyperlinksChanged: bool) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(22, hyperlinksChanged);
    }

    // Java: void removeHyperlinksChanged()
    pub fn removeHyperlinksChanged(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(22L);
    }

    // Java: int getVersion()
    pub fn getVersion(&self) -> i32 {
        // Java: return this.getPropertyIntValue(23);
        self.base.getPropertyIntValue(23)
    }

    // Java: void setVersion(int version)
    pub fn setVersion(&mut self, _version: i32) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(23, version);
    }

    // Java: void removeVersion()
    pub fn removeVersion(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(23L);
    }

    // Java: String getContentType()
    pub fn getContentType(&self) -> Option<String> {
        // Java: return (String)this.getProperty(26);
        None
    }

    // Java: void setContentType(String contentType)
    pub fn setContentType(&mut self, _contentType: &str) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(26, (String)contentType);
    }

    // Java: void removeContentType()
    pub fn removeContentType(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(26L);
    }

    // Java: String getContentStatus()
    pub fn getContentStatus(&self) -> Option<String> {
        // Java: return (String)this.getProperty(27);
        None
    }

    // Java: void setContentStatus(String contentStatus)
    pub fn setContentStatus(&mut self, _contentStatus: &str) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(27, (String)contentStatus);
    }

    // Java: void removeContentStatus()
    pub fn removeContentStatus(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(27L);
    }

    // Java: String getLanguage()
    pub fn getLanguage(&self) -> Option<String> {
        // Java: return (String)this.getProperty(28);
        None
    }

    // Java: void setLanguage(String language)
    pub fn setLanguage(&mut self, _language: &str) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(28, (String)language);
    }

    // Java: void removeLanguage()
    pub fn removeLanguage(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(28L);
    }

    // Java: String getDocVersion()
    pub fn getDocVersion(&self) -> Option<String> {
        // Java: return (String)this.getProperty(29);
        None
    }

    // Java: void setDocVersion(String docVersion)
    pub fn setDocVersion(&mut self, _docVersion: &str) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.setProperty(29, (String)docVersion);
    }

    // Java: void removeDocVersion()
    pub fn removeDocVersion(&mut self) {
        // Java: MutableSection s = (MutableSection)this.getFirstSection();
        // Java: s.removeProperty(29L);
    }

    // Java: CustomProperties getCustomProperties()
    pub fn getCustomProperties(&self) -> Option<Box<dyn Any>> {
        // Java: CustomProperties cps = null;
        // Java: if (this.getSectionCount() >= 2)
        if self.base.getSectionCount() >= 2 {
            // Java: cps = new CustomProperties();
            // Java: Section section = (Section)this.getSections().get(1);
            // Java: ILongObjectMap<String> dictionary = section.getDictionary();
            // Java: Property[] properties = section.getProperties();
            // Java: int propertyCount = 0;
            let mut _propertyCount = 0i32;
            // Java: int i = 0;
            let mut _i = 0i32;
            // Java: for(int len = properties.length; i < len; ++i)
            {
                // Java: Property p = properties[i];
                // Java: long id = p.getID();
                // Java: if (id != 0L && id != 1L)
                {
                    // Java: ++propertyCount;
                    _propertyCount += 1;
                    // Java: if (id > 16777216L)
                    {
                        // Java: long originId = id - 16777216L;
                        // Java: if (cps.containsKey(originId))
                        {
                            // Java: CustomProperty cp = (CustomProperty)cps.get(originId);
                            // Java: cp.setLinkTarget((String)properties[i].getValue());
                        }
                    }
                    // Java: else
                    {
                        // Java: CustomProperty cp = new CustomProperty(p, (String)dictionary.get(id));
                        // Java: cps.put(cp.getName(), cp);
                    }
                }
            }
            // Java: if (cps.size() != propertyCount)
            {
                // Java: cps.setPure(false);
            }
        }
        // Java: return cps;
        None
    }

    // Java: void setCustomProperties(CustomProperties customProperties)
    pub fn setCustomProperties(&mut self, _customProperties: Box<dyn Any>) {
        // Java: this.ensureSection2();
        self.ensureSection2();
        // Java: MutableSection section = (MutableSection)this.getSections().get(1);
        // Java: ILongObjectMap<String> dictionary = customProperties.getDictionary();
        // Java: section.clear();
        // Java: int cpCodepage = customProperties.getCodepage();
        let mut _cpCodepage: i32 = -1;
        // Java: if (cpCodepage < 0)
        if _cpCodepage < 0 {
            // Java: cpCodepage = section.getCodepage();
        }
        // Java: if (cpCodepage < 0)
        if _cpCodepage < 0 {
            // Java: cpCodepage = 936;
            _cpCodepage = 936;
        }
        // Java: customProperties.setCodepage(cpCodepage);
        // Java: section.setCodepage(cpCodepage);
        // Java: section.setDictionary(dictionary);
        // Java: for(Property p : customProperties.values())
        {
            // Java: section.setProperty(p);
            // Java: String linkTarget = ((CustomProperty)p).getLinkTarget();
            // Java: if (linkTarget != null)
            {
                // Java: MutableProperty cp = new MutableProperty();
                // Java: cp.setID(16777216L + p.getID());
                // Java: cp.setType(30L);
                // Java: cp.setValue(linkTarget);
                // Java: section.setProperty(cp);
            }
        }
    }

    // Java: void ensureSection2()
    fn ensureSection2(&mut self) {
        // Java: if (this.getSectionCount() < 2)
        if self.base.getSectionCount() < 2 {
            // Java: MutableSection s2 = new MutableSection();
            // Java: s2.setFormatID(SectionIDMap.DOCUMENT_SUMMARY_INFORMATION_ID[1]);
            // Java: this.addSection(s2);
        }
    }

    // Java: void removeCustomProperties()
    pub fn removeCustomProperties(&mut self) {
        // Java: if (this.getSectionCount() >= 2)
        if self.base.getSectionCount() >= 2 {
            // Java: this.getSections().remove(1);
        }
        // Java: else
        else {
            // Java: throw new HPSFRuntimeException("Illegal internal format of Document SummaryInformation stream: second section is missing.");
            panic!("Illegal internal format of Document SummaryInformation stream: second section is missing.");
        }
    }

    // Java: void notYetImplemented(String msg)
    fn notYetImplemented(&self, msg: &str) {
        // Java: throw new UnsupportedOperationException(msg + " is not yet implemented.");
        panic!("{} is not yet implemented.", msg);
    }
}

impl From<DocumentSummaryInformation> for PropertySet {
    fn from(dsi: DocumentSummaryInformation) -> Self {
        dsi.base.base.base
    }
}
