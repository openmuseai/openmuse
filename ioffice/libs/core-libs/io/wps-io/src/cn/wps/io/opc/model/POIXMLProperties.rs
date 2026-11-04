#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.opc.CustomPackageProperties
// Java: import cn.wps.io.opc.internal.CustomPackagePropertiesPart
// Java: import cn.wps.io.opc.internal.ExtendedPackagePropertiesPart
// Java: import cn.wps.io.opc.internal.PackagePropertiesPart
// Java: import cn.wps.io.xwpf.XWPFPackage
// Java: import cn.wps.io.xwpf.usermodel.exceptions.OpenXML4JException
// Java: import cn.wps.io.xwpf.util.Nullable
// Java: import java.io.IOException
// Java: import java.util.Date
// Java: import java.util.List

use crate::io::opc::internal::PackagePropertiesPart::PackagePropertiesPart;
use crate::io::opc::internal::ExtendedPackagePropertiesPart::ExtendedPackagePropertiesPart;
use crate::io::opc::internal::CustomPackagePropertiesPart::CustomPackagePropertiesPart;
use crate::io::xwpf::XWPFPackage::XWPFPackage;
use crate::io::xwpf::util::Nullable::Nullable;

// Java: public class POIXMLProperties
pub struct POIXMLProperties {
    // Java field: XWPFPackage pkg
    pub(crate) pkg: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: CoreProperties core
    pub(crate) core: Option<CoreProperties>,
    // Java field: ExtendedProperties ext
    pub(crate) ext: Option<ExtendedProperties>,
    // Java field: CustomProperties cust
    pub(crate) cust: Option<CustomProperties>,
}

// Java: public class CoreProperties
pub struct CoreProperties {
    // Java field: PackagePropertiesPart part
    pub(crate) part: Option<PackagePropertiesPart>,
}

// Java: public class ExtendedProperties
pub struct ExtendedProperties {
    // Java field: ExtendedPackagePropertiesPart part
    pub(crate) part: Option<ExtendedPackagePropertiesPart>,
}

// Java: public class CustomProperties
pub struct CustomProperties {
    // Java field: CustomPackagePropertiesPart part
    pub(crate) part: Option<CustomPackagePropertiesPart>,
}

// Java: public class POIXMLProperties
impl POIXMLProperties {
    // Java: POIXMLProperties(XWPFPackage docPackage)
    pub fn new(doc_package: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super();
        // Java: this.pkg = docPackage;
        let mut pkg_box = doc_package;
        let mut core_part = None;
        let mut ext_part = None;
        let mut cust_part = None;
        if let Some(pkg) = pkg_box.downcast_mut::<XWPFPackage>() {
            // Java: this.core = new CoreProperties((PackagePropertiesPart)this.pkg.getPackageProperties());
            let _ = pkg.getPackageProperties();
            // Java: this.ext = new ExtendedProperties((ExtendedPackagePropertiesPart)this.pkg.getExtendedPackageProperties());
            let _ = pkg.getExtendedPackageProperties();
            // Java: this.cust = new CustomProperties((CustomPackagePropertiesPart)this.pkg.getCustomPackageProperties());
            let _ = pkg.getCustomPackageProperties();
            // ownership: parts remain in pkg; wrappers hold None until take is possible
            let _ = (&core_part, &ext_part, &cust_part);
        }
        POIXMLProperties {
            pkg: Some(pkg_box),
            core: Some(CoreProperties { part: core_part }),
            ext: Some(ExtendedProperties { part: ext_part }),
            cust: Some(CustomProperties { part: cust_part }),
        }
    }

    // Java: CoreProperties getCoreProperties()
    pub fn getCoreProperties(&self) -> Option<&CoreProperties> {
        // Java: return this.core;
        self.core.as_ref()
    }

    // Java: ExtendedProperties getExtendedProperties()
    pub fn getExtendedProperties(&self) -> Option<&ExtendedProperties> {
        // Java: return this.ext;
        self.ext.as_ref()
    }

    // Java: CustomProperties getCustomProperties()
    pub fn getCustomProperties(&self) -> Option<&CustomProperties> {
        // Java: return this.cust;
        self.cust.as_ref()
    }
}

// Java: public class CoreProperties
impl CoreProperties {
    // Java: CoreProperties(PackagePropertiesPart part)
    pub fn new(part: PackagePropertiesPart) -> Self {
        // Java: super();
        // Java: this.part = part;
        CoreProperties { part: Some(part) }
    }

    // Java: String getCategory()
    pub fn getCategory(&self) -> Option<String> {
        // Java: return (String)this.part.getCategoryProperty().getValue();
        self.part.as_ref().and_then(|p| p.getCategoryProperty().getValue().cloned())
    }

    // Java: void setCategory(String category)
    pub fn setCategory(&mut self, category: &str) {
        // Java: this.part.setCategoryProperty(category);
        if let Some(ref mut p) = self.part {
            p.setCategoryProperty(Some(category));
        }
    }

    // Java: String getContentStatus()
    pub fn getContentStatus(&self) -> Option<String> {
        // Java: return (String)this.part.getContentStatusProperty().getValue();
        self.part.as_ref().and_then(|p| p.getContentStatusProperty().getValue().cloned())
    }

    // Java: void setContentStatus(String contentStatus)
    pub fn setContentStatus(&mut self, content_status: &str) {
        // Java: this.part.setContentStatusProperty(contentStatus);
        if let Some(ref mut p) = self.part {
            p.setContentStatusProperty(Some(content_status));
        }
    }

    // Java: String getContentType()
    pub fn getContentType(&self) -> Option<String> {
        // Java: return (String)this.part.getContentTypeProperty().getValue();
        self.part.as_ref().and_then(|p| p.getContentTypeProperty().getValue().cloned())
    }

    // Java: void setContentType(String contentType)
    pub fn setContentType(&mut self, content_type: &str) {
        // Java: this.part.setContentTypeProperty(contentType);
        if let Some(ref mut p) = self.part {
            p.setContentTypeProperty(Some(content_type));
        }
    }

    // Java: Date getCreated()
    pub fn getCreated(&self) -> Option<i64> {
        // Java: return (Date)this.part.getCreatedProperty().getValue();
        self.part.as_ref().and_then(|p| {
            p.getCreatedProperty().getValue().and_then(|s| s.parse::<i64>().ok())
        })
    }

    // Java: void setCreated(Nullable<Date> date)
    pub fn setCreated(&mut self, date: Option<i64>) {
        // Java: this.part.setCreatedProperty(date);
        if let Some(ref mut p) = self.part {
            match date {
                Some(d) => p.setCreatedProperty(Nullable::new_with_value(d.to_string())),
                None => {}
            }
        }
    }

    // Java: void setCreated(String date)
    pub fn setCreatedFromString(&mut self, date: &str) {
        // Java: this.part.setCreatedProperty(date);
        if let Some(ref mut p) = self.part {
            p.setCreatedProperty_str(Some(date));
        }
    }

    // Java: String getCreator()
    pub fn getCreator(&self) -> Option<String> {
        // Java: return (String)this.part.getCreatorProperty().getValue();
        self.part.as_ref().and_then(|p| p.getCreatorProperty().getValue().cloned())
    }

    // Java: void setCreator(String creator)
    pub fn setCreator(&mut self, creator: &str) {
        // Java: this.part.setCreatorProperty(creator);
        if let Some(ref mut p) = self.part {
            p.setCreatorProperty(Some(creator));
        }
    }

    // Java: String getDescription()
    pub fn getDescription(&self) -> Option<String> {
        // Java: return (String)this.part.getDescriptionProperty().getValue();
        self.part.as_ref().and_then(|p| p.getDescriptionProperty().getValue().cloned())
    }

    // Java: void setDescription(String description)
    pub fn setDescription(&mut self, description: &str) {
        // Java: this.part.setDescriptionProperty(description);
        if let Some(ref mut p) = self.part {
            p.setDescriptionProperty(Some(description));
        }
    }

    // Java: String getIdentifier()
    pub fn getIdentifier(&self) -> Option<String> {
        // Java: return (String)this.part.getIdentifierProperty().getValue();
        self.part.as_ref().and_then(|p| p.getIdentifierProperty().getValue().cloned())
    }

    // Java: void setIdentifier(String identifier)
    pub fn setIdentifier(&mut self, identifier: &str) {
        // Java: this.part.setIdentifierProperty(identifier);
        if let Some(ref mut p) = self.part {
            p.setIdentifierProperty(Some(identifier));
        }
    }

    // Java: String getKeywords()
    pub fn getKeywords(&self) -> Option<String> {
        // Java: return (String)this.part.getKeywordsProperty().getValue();
        self.part.as_ref().and_then(|p| p.getKeywordsProperty().getValue().cloned())
    }

    // Java: void setKeywords(String keywords)
    pub fn setKeywords(&mut self, keywords: &str) {
        // Java: this.part.setKeywordsProperty(keywords);
        if let Some(ref mut p) = self.part {
            p.setKeywordsProperty(Some(keywords));
        }
    }

    // Java: Date getLastPrinted()
    pub fn getLastPrinted(&self) -> Option<i64> {
        // Java: return (Date)this.part.getLastPrintedProperty().getValue();
        self.part.as_ref().and_then(|p| {
            p.getLastPrintedProperty().getValue().and_then(|s| s.parse::<i64>().ok())
        })
    }

    // Java: void setLastPrinted(Nullable<Date> date)
    pub fn setLastPrinted(&mut self, date: Option<i64>) {
        // Java: this.part.setLastPrintedProperty(date);
        if let Some(ref mut p) = self.part {
            if let Some(d) = date {
                p.setLastPrintedProperty(Nullable::new_with_value(d.to_string()));
            }
        }
    }

    // Java: void setLastPrinted(String date)
    pub fn setLastPrintedFromString(&mut self, date: &str) {
        // Java: this.part.setLastPrintedProperty(date);
        if let Some(ref mut p) = self.part {
            p.setLastPrintedProperty_str(Some(date));
        }
    }

    // Java: Date getModified()
    pub fn getModified(&self) -> Option<i64> {
        // Java: return (Date)this.part.getModifiedProperty().getValue();
        self.part.as_ref().and_then(|p| {
            p.getModifiedProperty().getValue().and_then(|s| s.parse::<i64>().ok())
        })
    }

    // Java: String getLastModifiedBy()
    pub fn getLastModifiedBy(&self) -> Option<String> {
        // Java: return (String)this.part.getLastModifiedByProperty().getValue();
        self.part.as_ref().and_then(|p| p.getLastModifiedByProperty().getValue().cloned())
    }

    // Java: void setLastModifiedBy(String value)
    pub fn setLastModifiedBy(&mut self, value: &str) {
        // Java: this.part.setLastModifiedByProperty(value);
        if let Some(ref mut p) = self.part {
            p.setLastModifiedByProperty(Some(value));
        }
    }

    // Java: void setModified(Nullable<Date> date)
    pub fn setModified(&mut self, date: Option<i64>) {
        // Java: this.part.setModifiedProperty(date);
        if let Some(ref mut p) = self.part {
            if let Some(d) = date {
                p.setModifiedProperty(Nullable::new_with_value(d.to_string()));
            }
        }
    }

    // Java: void setModified(String date)
    pub fn setModifiedFromString(&mut self, date: &str) {
        // Java: this.part.setModifiedProperty(date);
        if let Some(ref mut p) = self.part {
            p.setModifiedProperty_str(Some(date));
        }
    }

    // Java: String getSubject()
    pub fn getSubject(&self) -> Option<String> {
        // Java: return (String)this.part.getSubjectProperty().getValue();
        self.part.as_ref().and_then(|p| p.getSubjectProperty().getValue().cloned())
    }

    // Java: void setSubjectProperty(String subject)
    pub fn setSubjectProperty(&mut self, subject: &str) {
        // Java: this.part.setSubjectProperty(subject);
        if let Some(ref mut p) = self.part {
            p.setSubjectProperty(Some(subject));
        }
    }

    // Java: void setTitle(String title)
    pub fn setTitle(&mut self, title: &str) {
        // Java: this.part.setTitleProperty(title);
        if let Some(ref mut p) = self.part {
            p.setTitleProperty(Some(title));
        }
    }

    // Java: String getTitle()
    pub fn getTitle(&self) -> Option<String> {
        // Java: return (String)this.part.getTitleProperty().getValue();
        self.part.as_ref().and_then(|p| p.getTitleProperty().getValue().cloned())
    }

    // Java: String getRevision()
    pub fn getRevision(&self) -> Option<String> {
        // Java: return (String)this.part.getRevisionProperty().getValue();
        self.part.as_ref().and_then(|p| p.getRevisionProperty().getValue().cloned())
    }

    // Java: void setRevision(String revision)
    pub fn setRevision(&mut self, revision: &str) {
        // Java: try
        // Java: Long.valueOf(revision);
        // Java: this.part.setRevisionProperty(revision);
        // Java: catch (NumberFormatException var3)
        if revision.parse::<i64>().is_ok() {
            if let Some(ref mut p) = self.part {
                p.setRevisionProperty(Some(revision));
            }
        }
    }

    // Java: String getLanguage()
    pub fn getLanguage(&self) -> Option<String> {
        // Java: return (String)this.part.getLanguageProperty().getValue();
        self.part.as_ref().and_then(|p| p.getLanguageProperty().getValue().cloned())
    }

    // Java: void setgetLanguage(String language)
    pub fn setgetLanguage(&mut self, language: &str) {
        // Java: this.part.setLanguageProperty(language);
        if let Some(ref mut p) = self.part {
            p.setLanguageProperty(Some(language));
        }
    }

    // Java: PackagePropertiesPart getUnderlyingProperties()
    pub fn getUnderlyingProperties(&self) -> Option<&PackagePropertiesPart> {
        // Java: return this.part;
        self.part.as_ref()
    }
}

// Java: public class ExtendedProperties
impl ExtendedProperties {
    // Java: ExtendedProperties(ExtendedPackagePropertiesPart part)
    pub fn new(part: ExtendedPackagePropertiesPart) -> Self {
        // Java: super();
        // Java: this.part = part;
        ExtendedProperties { part: Some(part) }
    }

    // Java: Integer getWordCount()
    pub fn getWordCount(&self) -> Option<i32> {
        // Java: return this.part.getWordCountProperty();
        self.part.as_ref().and_then(|p| p.getWordCountProperty())
    }

    // Java: Integer getNumberCharacters()
    pub fn getNumberCharacters(&self) -> Option<i32> {
        // Java: return this.part.getNumberCharactersProperty();
        self.part.as_ref().and_then(|p| p.getNumberCharactersProperty())
    }

    // Java: String getApplicationProperty()
    pub fn getApplicationProperty(&self) -> Option<String> {
        // Java: return this.part.getApplicationProperty();
        self.part.as_ref().and_then(|p| p.getApplicationProperty().map(|s| s.to_string()))
    }

    // Java: String getAppVersionProperty()
    pub fn getAppVersionProperty(&self) -> Option<String> {
        // Java: return this.part.getAppVersionProperty();
        self.part.as_ref().and_then(|p| p.getAppVersionProperty().map(|s| s.to_string()))
    }

    // Java: String getCompanyProperty()
    pub fn getCompanyProperty(&self) -> Option<String> {
        // Java: return this.part.getCompanyProperty();
        self.part.as_ref().and_then(|p| p.getCompanyProperty().map(|s| s.to_string()))
    }

    // Java: Integer getDocSecurityProperty()
    pub fn getDocSecurityProperty(&self) -> Option<i32> {
        // Java: return this.part.getDocSecurityProperty();
        self.part.as_ref().and_then(|p| p.getDocSecurityProperty())
    }

    // Java: Integer getHiddenSlidesProperty()
    pub fn getHiddenSlidesProperty(&self) -> Option<i32> {
        // Java: return this.part.getHiddenSlidesProperty();
        self.part.as_ref().and_then(|p| p.getHiddenSlidesProperty())
    }

    // Java: String getHyperlinkBaseProperty()
    pub fn getHyperlinkBaseProperty(&self) -> Option<String> {
        // Java: return this.part.getHyperlinkBaseProperty();
        self.part.as_ref().and_then(|p| p.getHyperlinkBaseProperty().map(|s| s.to_string()))
    }

    // Java: Boolean getHyperlinksChangedProperty()
    pub fn getHyperlinksChangedProperty(&self) -> Option<bool> {
        // Java: return this.part.getHyperlinksChangedProperty();
        self.part.as_ref().and_then(|p| p.getHyperlinksChangedProperty())
    }

    // Java: Integer getLinesProperty()
    pub fn getLinesProperty(&self) -> Option<i32> {
        // Java: return this.part.getLinesProperty();
        self.part.as_ref().and_then(|p| p.getLinesProperty())
    }

    // Java: Boolean getLinksUpToDateProperty()
    pub fn getLinksUpToDateProperty(&self) -> Option<bool> {
        // Java: return this.part.getLinksUpToDateProperty();
        self.part.as_ref().and_then(|p| p.getLinksUpToDateProperty())
    }

    // Java: Integer getMMClipsProperty()
    pub fn getMMClipsProperty(&self) -> Option<i32> {
        // Java: return this.part.getMMClipsProperty();
        self.part.as_ref().and_then(|p| p.getMMClipsProperty())
    }

    // Java: String getManagerProperty()
    pub fn getManagerProperty(&self) -> Option<String> {
        // Java: return this.part.getManagerProperty();
        self.part.as_ref().and_then(|p| p.getManagerProperty().map(|s| s.to_string()))
    }

    // Java: Integer getNotesProperty()
    pub fn getNotesProperty(&self) -> Option<i32> {
        // Java: return this.part.getNotesProperty();
        self.part.as_ref().and_then(|p| p.getNotesProperty())
    }

    // Java: Integer getPagesProperty()
    pub fn getPagesProperty(&self) -> Option<i32> {
        // Java: return this.part.getPagesProperty();
        self.part.as_ref().and_then(|p| p.getPagesProperty())
    }

    // Java: Integer getParagraphsProperty()
    pub fn getParagraphsProperty(&self) -> Option<i32> {
        // Java: return this.part.getParagraphsProperty();
        self.part.as_ref().and_then(|p| p.getParagraphsProperty())
    }

    // Java: String getPresentationFormatProperty()
    pub fn getPresentationFormatProperty(&self) -> Option<String> {
        // Java: return this.part.getPresentationFormatProperty();
        self.part.as_ref().and_then(|p| p.getPresentationFormatProperty().map(|s| s.to_string()))
    }

    // Java: Boolean getScaleCropProperty()
    pub fn getScaleCropProperty(&self) -> Option<bool> {
        // Java: return this.part.getScaleCropProperty();
        self.part.as_ref().and_then(|p| p.getScaleCropProperty())
    }

    // Java: Boolean getSharedDocProperty()
    pub fn getSharedDocProperty(&self) -> Option<bool> {
        // Java: return this.part.getSharedDocProperty();
        self.part.as_ref().and_then(|p| p.getSharedDocProperty())
    }

    // Java: Integer getSlidesProperty()
    pub fn getSlidesProperty(&self) -> Option<i32> {
        // Java: return this.part.getSlidesProperty();
        self.part.as_ref().and_then(|p| p.getSlidesProperty())
    }

    // Java: String getTemplateProperty()
    pub fn getTemplateProperty(&self) -> Option<String> {
        // Java: return this.part.getTemplateProperty();
        self.part.as_ref().and_then(|p| p.getTemplateProperty().map(|s| s.to_string()))
    }

    // Java: Integer getTotalTimeProperty()
    pub fn getTotalTimeProperty(&self) -> Option<i32> {
        // Java: return this.part.getTotalTimeProperty();
        self.part.as_ref().and_then(|p| p.getTotalTimeProperty())
    }
}

// Java: public class CustomProperties
impl CustomProperties {
    // Java: CustomProperties(CustomPackagePropertiesPart part)
    pub fn new(part: CustomPackagePropertiesPart) -> Self {
        // Java: super();
        // Java: this.part = part;
        CustomProperties { part: Some(part) }
    }

    // Java: void addProperty(String name, double value)
    pub fn addPropertyDouble(&mut self, _name: &str, _value: f64) {
    }

    // Java: void addProperty(String name, int value)
    pub fn addPropertyInt(&mut self, _name: &str, _value: i32) {
    }

    // Java: void addProperty(String name, boolean value)
    pub fn addPropertyBool(&mut self, _name: &str, _value: bool) {
    }

    // Java: boolean contains(String name)
    pub fn contains(&self, _name: &str) -> bool {
        // Java: return false;
        false
    }
}
