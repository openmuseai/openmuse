#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

use crate::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;

// Java: public class XWPFFootnote extends POIXMLDocumentPart
pub struct XWPFFootnote {
    pub(crate) base: POIXMLDocumentPart,
    // Java field: FootnotesHandler mFootnotesHandler
    pub(crate) mFootnotesHandler: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: boolean mIsParse
    pub(crate) mIsParse: bool,
}

impl XWPFFootnote {
    // Java: XWPFFootnote(POIXMLDocumentPart parent, PackagePart part, PackageRelationship rel)
    pub fn new_with_parts(_parent: Box<dyn std::any::Any + Send + Sync>, _part: Box<dyn std::any::Any + Send + Sync>, _rel: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super(parent, part, rel);
        todo!()
    }

    // Java: XWPFFootnote()
    pub fn new() -> Self {
        // Java: super();
        XWPFFootnote {
            base: POIXMLDocumentPart::new(),
            mFootnotesHandler: None,
            mIsParse: false,
        }
    }

    // Java: static XWPFFootnote startPart(XWPFDocument doc)
    pub fn startPart(_doc: &dyn std::any::Any) -> Self {
        // Java: IOAssert.assertNotNull("doc should not be null.", doc);
        // Java: POIXMLDocumentPart part = POIXMLDocumentPart.startPart(doc, XWPFRelation.FOOTNOTE, XWPFFactory.getInstance());
        // Java: return (XWPFFootnote)part;
        todo!()
    }

    // Java: void setDocumentImporter(IDocumentImporter importer)
    pub fn setDocumentImporter(&mut self, importer: Box<dyn std::any::Any + Send + Sync>) {
        // Java: this.mFootnotesHandler = new FootnotesHandler(importer, this);
        todo!()
    }

    // Java: void parse()
    pub fn parse(&mut self) {
        // Java: if (!this.mIsParse)
        if !self.mIsParse {
            // Java: this.mIsParse = true;
            self.mIsParse = true;
            // Java: XmlReader xmlReader = new XmlReader();
            // Java: xmlReader.parse(this.getPackagePart().getInputStream(), this.mFootnotesHandler, "utf-8", -1, 0);
            todo!()
        }
    }

    // Java: boolean isParse()
    pub fn isParse(&self) -> bool {
        // Java: return this.mIsParse;
        self.mIsParse
    }
}
