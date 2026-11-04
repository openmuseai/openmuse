#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.dom.Document
// Java: import cn.wps.io.dom.Element
// Java: import cn.wps.io.dom.Namespace
// Java: import cn.wps.io.dom.tree.DefaultDocument
// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.internal.PackagePropertiesPart
// Java: import cn.wps.io.opc.internal.PartMarshaller
// Java: import cn.wps.io.xwpf.usermodel.exceptions.OpenXML4JException
// Java: import java.io.OutputStream

use crate::io::opc::PackagePart::PackagePart;
use crate::io::opc::internal::PackagePropertiesPart::PackagePropertiesPart;

// Java field: Namespace namespaceDC
// Java field: Namespace namespaceCoreProperties
// Java field: Namespace namespaceDcTerms
// Java field: Namespace namespaceXSI

// Java field: String KEYWORD_CATEGORY
pub const KEYWORD_CATEGORY: &str = "category";
// Java field: String KEYWORD_CONTENT_STATUS
pub const KEYWORD_CONTENT_STATUS: &str = "contentStatus";
// Java field: String KEYWORD_CONTENT_TYPE
pub const KEYWORD_CONTENT_TYPE: &str = "contentType";
// Java field: String KEYWORD_CREATED
pub const KEYWORD_CREATED: &str = "created";
// Java field: String KEYWORD_CREATOR
pub const KEYWORD_CREATOR: &str = "creator";
// Java field: String KEYWORD_DESCRIPTION
pub const KEYWORD_DESCRIPTION: &str = "description";
// Java field: String KEYWORD_IDENTIFIER
pub const KEYWORD_IDENTIFIER: &str = "identifier";
// Java field: String KEYWORD_KEYWORDS
pub const KEYWORD_KEYWORDS: &str = "keywords";
// Java field: String KEYWORD_LANGUAGE
pub const KEYWORD_LANGUAGE: &str = "language";
// Java field: String KEYWORD_LAST_MODIFIED_BY
pub const KEYWORD_LAST_MODIFIED_BY: &str = "lastModifiedBy";
// Java field: String KEYWORD_LAST_PRINTED
pub const KEYWORD_LAST_PRINTED: &str = "lastPrinted";
// Java field: String KEYWORD_MODIFIED
pub const KEYWORD_MODIFIED: &str = "modified";
// Java field: String KEYWORD_REVISION
pub const KEYWORD_REVISION: &str = "revision";
// Java field: String KEYWORD_SUBJECT
pub const KEYWORD_SUBJECT: &str = "subject";
// Java field: String KEYWORD_TITLE
pub const KEYWORD_TITLE: &str = "title";
// Java field: String KEYWORD_VERSION
pub const KEYWORD_VERSION: &str = "version";

// Java: public class PackagePropertiesMarshaller implements PartMarshaller
pub struct PackagePropertiesMarshaller {
    // Java field: PackagePropertiesPart propsPart
    propsPart: Option<PackagePropertiesPart>,
    // Java field: Document xmlDoc
    xmlDoc: Option<Box<dyn std::any::Any + Send + Sync>>,
}

// Java: public class PackagePropertiesMarshaller implements PartMarshaller
impl PackagePropertiesMarshaller {
    // Java: PackagePropertiesMarshaller()
    pub fn new() -> Self {
        // Java: super();
        PackagePropertiesMarshaller {
            propsPart: None,
            xmlDoc: None,
        }
    }

    // Java: boolean marshall(PackagePart part, OutputStream out)
    pub fn marshall(&mut self, _part: &PackagePart, _out: &mut dyn std::io::Write) -> bool {
        // Java: if (!(part instanceof PackagePropertiesPart))
        // Java: throw new IllegalArgumentException("'part' must be a PackagePropertiesPart instance.");
        // Java: else
        // Java: this.propsPart = (PackagePropertiesPart)part;
        // Java: this.xmlDoc = new DefaultDocument();
        // Java: Element rootElem = this.xmlDoc.addElement(namespaceCoreProperties.getURI(), namespaceCoreProperties.getPrefix(), "coreProperties");
        // Java: rootElem.addNamespace("cp", "http://schemas.openxmlformats.org/package/2006/metadata/core-properties");
        // Java: rootElem.addNamespace("dc", "http://purl.org/dc/elements/1.1/");
        // Java: rootElem.addNamespace("dcterms", "http://purl.org/dc/terms/");
        // Java: rootElem.addNamespace("xsi", "http://www.w3.org/2001/XMLSchema-instance");
        // Java: this.addCategory();
        // Java: this.addContentStatus();
        // Java: this.addContentType();
        // Java: this.addCreated();
        // Java: this.addCreator();
        // Java: this.addDescription();
        // Java: this.addIdentifier();
        // Java: this.addKeywords();
        // Java: this.addLanguage();
        // Java: this.addLastModifiedBy();
        // Java: this.addLastPrinted();
        // Java: this.addModified();
        // Java: this.addRevision();
        // Java: this.addSubject();
        // Java: this.addTitle();
        // Java: this.addVersion();
        // Java: return true;
        todo!()
    }

    // Java: void finish()
    pub fn finish(&self) {
        // Java: if (null != this.xmlDoc)
        // Java: Element root = this.xmlDoc.getRootElement();
        // Java: if (null != root)
        // Java: root.detachRecursively();
        todo!()
    }

    // Java: void addCategory()
    pub fn addCategory(&self) {
        // Java: if (this.propsPart.getCategoryProperty().hasValue())
        // Java: Element elem = this.xmlDoc.getRootElement().element("category", namespaceCoreProperties.getURI());
        // Java: if (elem == null)
        // Java: elem = this.xmlDoc.getRootElement().addElement(namespaceCoreProperties.getURI(), namespaceCoreProperties.getPrefix(), "category");
        // Java: else
        // Java: elem.clearContent();
        // Java: elem.addText((String)this.propsPart.getCategoryProperty().getValue());
        todo!()
    }

    // Java: void addContentStatus()
    pub fn addContentStatus(&self) {
        // Java: if (this.propsPart.getContentStatusProperty().hasValue())
        // Java: Element elem = this.xmlDoc.getRootElement().element("contentStatus", namespaceCoreProperties.getURI());
        // Java: if (elem == null)
        // Java: elem = this.xmlDoc.getRootElement().addElement(namespaceCoreProperties.getURI(), namespaceCoreProperties.getPrefix(), "contentStatus");
        // Java: else
        // Java: elem.clearContent();
        // Java: elem.addText((String)this.propsPart.getContentStatusProperty().getValue());
        todo!()
    }

    // Java: void addContentType()
    pub fn addContentType(&self) {
        // Java: if (this.propsPart.getContentTypeProperty().hasValue())
        // Java: Element elem = this.xmlDoc.getRootElement().element("contentType", namespaceCoreProperties.getURI());
        // Java: if (elem == null)
        // Java: elem = this.xmlDoc.getRootElement().addElement(namespaceCoreProperties.getURI(), namespaceCoreProperties.getPrefix(), "contentType");
        // Java: else
        // Java: elem.clearContent();
        // Java: elem.addText((String)this.propsPart.getContentTypeProperty().getValue());
        todo!()
    }

    // Java: void addCreated()
    pub fn addCreated(&self) {
        // Java: if (this.propsPart.getCreatedProperty().hasValue())
        // Java: Element elem = this.xmlDoc.getRootElement().element("created", namespaceDcTerms.getURI());
        // Java: if (elem == null)
        // Java: elem = this.xmlDoc.getRootElement().addElement(namespaceDcTerms.getURI(), namespaceDcTerms.getPrefix(), "created");
        // Java: else
        // Java: elem.clearContent();
        // Java: elem.addAttribute("type", namespaceXSI.getPrefix(), namespaceXSI.getURI(), "dcterms:W3CDTF");
        // Java: elem.addText(this.propsPart.getCreatedPropertyString());
        todo!()
    }

    // Java: void addCreator()
    pub fn addCreator(&self) {
        // Java: if (this.propsPart.getCreatorProperty().hasValue())
        // Java: Element elem = this.xmlDoc.getRootElement().element("creator", namespaceDC.getURI());
        // Java: if (elem == null)
        // Java: elem = this.xmlDoc.getRootElement().addElement(namespaceDC.getURI(), namespaceDC.getPrefix(), "creator");
        // Java: else
        // Java: elem.clearContent();
        // Java: elem.addText((String)this.propsPart.getCreatorProperty().getValue());
        todo!()
    }

    // Java: void addDescription()
    pub fn addDescription(&self) {
        // Java: if (this.propsPart.getDescriptionProperty().hasValue())
        // Java: Element elem = this.xmlDoc.getRootElement().element("description", namespaceDC.getURI());
        // Java: if (elem == null)
        // Java: elem = this.xmlDoc.getRootElement().addElement(namespaceDC.getURI(), namespaceDC.getPrefix(), "description");
        // Java: else
        // Java: elem.clearContent();
        // Java: elem.addText((String)this.propsPart.getDescriptionProperty().getValue());
        todo!()
    }

    // Java: void addIdentifier()
    pub fn addIdentifier(&self) {
        // Java: if (this.propsPart.getIdentifierProperty().hasValue())
        // Java: Element elem = this.xmlDoc.getRootElement().element("identifier", namespaceDC.getURI());
        // Java: if (elem == null)
        // Java: elem = this.xmlDoc.getRootElement().addElement(namespaceDC.getURI(), namespaceDC.getPrefix(), "identifier");
        // Java: else
        // Java: elem.clearContent();
        // Java: elem.addText((String)this.propsPart.getIdentifierProperty().getValue());
        todo!()
    }

    // Java: void addKeywords()
    pub fn addKeywords(&self) {
        // Java: if (this.propsPart.getKeywordsProperty().hasValue())
        // Java: Element elem = this.xmlDoc.getRootElement().element("keywords", namespaceCoreProperties.getURI());
        // Java: if (elem == null)
        // Java: elem = this.xmlDoc.getRootElement().addElement(namespaceCoreProperties.getURI(), namespaceCoreProperties.getPrefix(), "keywords");
        // Java: else
        // Java: elem.clearContent();
        // Java: elem.addText((String)this.propsPart.getKeywordsProperty().getValue());
        todo!()
    }

    // Java: void addLanguage()
    pub fn addLanguage(&self) {
        // Java: if (this.propsPart.getLanguageProperty().hasValue())
        // Java: Element elem = this.xmlDoc.getRootElement().element("language", namespaceDC.getURI());
        // Java: if (elem == null)
        // Java: elem = this.xmlDoc.getRootElement().addElement(namespaceDC.getURI(), namespaceDC.getPrefix(), "language");
        // Java: else
        // Java: elem.clearContent();
        // Java: elem.addText((String)this.propsPart.getLanguageProperty().getValue());
        todo!()
    }

    // Java: void addLastModifiedBy()
    pub fn addLastModifiedBy(&self) {
        // Java: if (this.propsPart.getLastModifiedByProperty().hasValue())
        // Java: Element elem = this.xmlDoc.getRootElement().element("lastModifiedBy", namespaceCoreProperties.getURI());
        // Java: if (elem == null)
        // Java: elem = this.xmlDoc.getRootElement().addElement(namespaceCoreProperties.getURI(), namespaceCoreProperties.getPrefix(), "lastModifiedBy");
        // Java: else
        // Java: elem.clearContent();
        // Java: elem.addText((String)this.propsPart.getLastModifiedByProperty().getValue());
        todo!()
    }

    // Java: void addLastPrinted()
    pub fn addLastPrinted(&self) {
        // Java: if (this.propsPart.getLastPrintedProperty().hasValue())
        // Java: Element elem = this.xmlDoc.getRootElement().element("lastPrinted", namespaceCoreProperties.getURI());
        // Java: if (elem == null)
        // Java: elem = this.xmlDoc.getRootElement().addElement(namespaceCoreProperties.getURI(), namespaceCoreProperties.getPrefix(), "lastPrinted");
        // Java: else
        // Java: elem.clearContent();
        // Java: elem.addText(this.propsPart.getLastPrintedPropertyString());
        todo!()
    }

    // Java: void addModified()
    pub fn addModified(&self) {
        // Java: if (this.propsPart.getModifiedProperty().hasValue())
        // Java: Element elem = this.xmlDoc.getRootElement().element("modified", namespaceDcTerms.getURI());
        // Java: if (elem == null)
        // Java: elem = this.xmlDoc.getRootElement().addElement(namespaceDcTerms.getURI(), namespaceDcTerms.getPrefix(), "modified");
        // Java: else
        // Java: elem.clearContent();
        // Java: elem.addAttribute("type", namespaceXSI.getPrefix(), namespaceXSI.getURI(), "dcterms:W3CDTF");
        // Java: elem.addText(this.propsPart.getModifiedPropertyString());
        todo!()
    }

    // Java: void addRevision()
    pub fn addRevision(&self) {
        // Java: if (this.propsPart.getRevisionProperty().hasValue())
        // Java: Element elem = this.xmlDoc.getRootElement().element("revision", namespaceCoreProperties.getURI());
        // Java: if (elem == null)
        // Java: elem = this.xmlDoc.getRootElement().addElement(namespaceCoreProperties.getURI(), namespaceCoreProperties.getPrefix(), "revision");
        // Java: else
        // Java: elem.clearContent();
        // Java: elem.addText((String)this.propsPart.getRevisionProperty().getValue());
        todo!()
    }

    // Java: void addSubject()
    pub fn addSubject(&self) {
        // Java: if (this.propsPart.getSubjectProperty().hasValue())
        // Java: Element elem = this.xmlDoc.getRootElement().element("subject", namespaceDC.getURI());
        // Java: if (elem == null)
        // Java: elem = this.xmlDoc.getRootElement().addElement(namespaceDC.getURI(), namespaceDC.getPrefix(), "subject");
        // Java: else
        // Java: elem.clearContent();
        // Java: elem.addText((String)this.propsPart.getSubjectProperty().getValue());
        todo!()
    }

    // Java: void addTitle()
    pub fn addTitle(&self) {
        // Java: if (this.propsPart.getTitleProperty().hasValue())
        // Java: Element elem = this.xmlDoc.getRootElement().element("title", namespaceDC.getURI());
        // Java: if (elem == null)
        // Java: elem = this.xmlDoc.getRootElement().addElement(namespaceDC.getURI(), namespaceDC.getPrefix(), "title");
        // Java: else
        // Java: elem.clearContent();
        // Java: elem.addText((String)this.propsPart.getTitleProperty().getValue());
        todo!()
    }

    // Java: void addVersion()
    pub fn addVersion(&self) {
        // Java: if (this.propsPart.getVersionProperty().hasValue())
        // Java: Element elem = this.xmlDoc.getRootElement().element("version", namespaceCoreProperties.getURI());
        // Java: if (elem == null)
        // Java: elem = this.xmlDoc.getRootElement().addElement(namespaceCoreProperties.getURI(), namespaceCoreProperties.getPrefix(), "version");
        // Java: else
        // Java: elem.clearContent();
        // Java: elem.addText((String)this.propsPart.getVersionProperty().getValue());
        todo!()
    }
}
