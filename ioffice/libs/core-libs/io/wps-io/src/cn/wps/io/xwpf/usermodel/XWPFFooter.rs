#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

use super::XWPFHeaderFooter::XWPFHeaderFooter;
use crate::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;

// Java: public class XWPFFooter extends XWPFHeaderFooter
pub struct XWPFFooter {
    pub(crate) base: XWPFHeaderFooter,
    // Java field: FooterDocumentHandler mFooterDocumentHandler
    pub(crate) mFooterDocumentHandler: Option<Box<dyn std::any::Any + Send + Sync>>,
}

impl XWPFFooter {
    // Java: XWPFFooter()
    pub fn new() -> Self {
        // Java: super();
        XWPFFooter {
            base: XWPFHeaderFooter::new(),
            mFooterDocumentHandler: None,
        }
    }

    // Java: XWPFFooter(POIXMLDocumentPart parent, PackagePart part, PackageRelationship rel)
    pub fn new_with_parts(_parent: Box<dyn std::any::Any + Send + Sync>, _part: Box<dyn std::any::Any + Send + Sync>, _rel: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super(parent, part, rel);
        todo!()
    }

    // Java: void parse()
    pub fn parse(&mut self) {
        // Java: if (null == this.mFooterDocumentHandler)
        if self.mFooterDocumentHandler.is_none() {
            // Java: this.mFooterDocumentHandler = new FooterDocumentHandler(this.mImporter, this, this.mType);
            todo!()
        }
        // Java: XmlReader xmlReader = new XmlReader();
        // Java: xmlReader.parse(this.getPackagePart().getInputStream(), this.mFooterDocumentHandler, "utf-8", -1, 0);
        todo!()
    }
}
