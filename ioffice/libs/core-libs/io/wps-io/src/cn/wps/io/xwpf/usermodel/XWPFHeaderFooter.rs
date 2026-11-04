#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

use crate::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;

// Java: public static enum HdrFtrType
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HdrFtrType {
    // Java: EVEN
    EVEN,
    // Java: ODD
    ODD,
    // Java: FIRST
    FIRST,
}

// Java: public abstract class XWPFHeaderFooter extends POIXMLDocumentPart
pub struct XWPFHeaderFooter {
    pub(crate) base: POIXMLDocumentPart,
    // Java field: HdrFtrType mType
    pub(crate) mType: Option<HdrFtrType>,
    // Java field: IDocumentImporter mImporter
    pub(crate) mImporter: Option<Box<dyn std::any::Any + Send + Sync>>,
}

impl XWPFHeaderFooter {
    // Java: XWPFHeaderFooter()
    pub fn new() -> Self {
        // Java: super();
        XWPFHeaderFooter {
            base: POIXMLDocumentPart::new(),
            mType: None,
            mImporter: None,
        }
    }

    // Java: void setDocumentImporter(IDocumentImporter importer)
    pub fn setDocumentImporter(&mut self, importer: Box<dyn std::any::Any + Send + Sync>) {
        // Java: this.mImporter = importer;
        self.mImporter = Some(importer);
    }

    // Java: void setType(HdrFtrType type)
    pub fn setType(&mut self, t: HdrFtrType) {
        // Java: this.mType = type;
        self.mType = Some(t);
    }

    // Java: HdrFtrType getType()
    pub fn getType(&self) -> Option<HdrFtrType> {
        // Java: return this.mType;
        self.mType
    }

    // Java: XWPFHeaderFooter(POIXMLDocumentPart parent, PackagePart part, PackageRelationship rel)
    pub fn new_with_parts(_parent: Box<dyn std::any::Any + Send + Sync>, _part: Box<dyn std::any::Any + Send + Sync>, _rel: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super(parent, part, rel);
        todo!()
    }
}
