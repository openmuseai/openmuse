#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

use crate::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;

// Java: public class XWPFEndnote extends POIXMLDocumentPart
pub struct XWPFEndnote {
    pub(crate) base: POIXMLDocumentPart,
    // Java field: EndnotesHandler mEndnotesHandler
    pub(crate) mEndnotesHandler: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: boolean mIsParse
    pub(crate) mIsParse: bool,
}

impl XWPFEndnote {
    // Java: XWPFEndnote(POIXMLDocumentPart parent, PackagePart part, PackageRelationship rel)
    pub fn new_with_parts(_parent: Box<dyn std::any::Any + Send + Sync>, _part: Box<dyn std::any::Any + Send + Sync>, _rel: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super(parent, part, rel);
        todo!()
    }

    // Java: XWPFEndnote()
    pub fn new() -> Self {
        // Java: super();
        XWPFEndnote {
            base: POIXMLDocumentPart::new(),
            mEndnotesHandler: None,
            mIsParse: false,
        }
    }

    // Java: static XWPFEndnote startPart(XWPFDocument doc)
    pub fn startPart(_doc: &dyn std::any::Any) -> Self {
        // Java: IOAssert.assertNotNull("doc should not be null.", doc);
        // Java: POIXMLDocumentPart part = POIXMLDocumentPart.startPart(doc, XWPFRelation.ENDNOTE, XWPFFactory.getInstance());
        // Java: return (XWPFEndnote)part;
        todo!()
    }

    // Java: void setDocumentImporter(IDocumentImporter importer)
    pub fn setDocumentImporter(&mut self, importer: Box<dyn std::any::Any + Send + Sync>) {
        // Java: this.mEndnotesHandler = new EndnotesHandler(importer, this);
        todo!()
    }

    // Java: void parse()
    pub fn parse(&mut self) {
        // Java: if (!this.mIsParse)
        if !self.mIsParse {
            // Java: this.mIsParse = true;
            self.mIsParse = true;
            // Java: XmlReader xmlReader = new XmlReader();
            // Java: xmlReader.parse(this.getPackagePart().getInputStream(), this.mEndnotesHandler, "utf-8", -1, 0);
            todo!()
        }
    }

    // Java: boolean isParse()
    pub fn isParse(&self) -> bool {
        // Java: return this.mIsParse;
        self.mIsParse
    }
}
