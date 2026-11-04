#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

use crate::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
use crate::io::opc::PackagePart::PackagePart;
use crate::io::opc::PackageRelationship::PackageRelationship;
use crate::io::opt::XmlReader::XmlReader;

// Java: public class XWPFComment extends POIXMLDocumentPart
pub struct XWPFComment {
    pub(crate) base: POIXMLDocumentPart,
    // Java field: CommentsHandler mCommentsHandler
    pub(crate) mCommentsHandler: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: boolean mIsParse
    pub(crate) mIsParse: bool,
}

impl XWPFComment {
    // Java: XWPFComment(POIXMLDocumentPart parent, PackagePart part, PackageRelationship rel)
    // rust: ★M-STAGE1 P1-e2 —— 此前是 `todo!()` 桩，且形参被擦除成
    //   `Box<dyn Any + Send + Sync>`（旧约定），既无法绑定真实 part、也无法被调用方使用。
    //   现按 Java 真实签名落地：`super(parent, part, rel)` ⇒ 基类存下 `packagePart`/`packageRel`。
    pub fn new_with_parts(
        parent: Option<&POIXMLDocumentPart>,
        part: Option<Box<PackagePart>>,
        rel: Option<Box<PackageRelationship>>,
    ) -> Self {
        // Java: super(parent, part, rel);
        XWPFComment {
            base: POIXMLDocumentPart::POIXMLDocumentPart__parent(parent, part, rel),
            mCommentsHandler: None,
            mIsParse: false,
        }
    }

    // Java: XWPFComment()
    pub fn new() -> Self {
        // Java: super();
        XWPFComment {
            base: POIXMLDocumentPart::new(),
            mCommentsHandler: None,
            mIsParse: false,
        }
    }

    // Java: boolean isParse()
    pub fn isParse(&self) -> bool {
        // Java: return this.mIsParse;
        self.mIsParse
    }

    // Java: static XWPFComment startPart(XWPFDocument doc)
    pub fn startPart(_doc: &dyn std::any::Any) -> Self {
        // Java: IOAssert.assertNotNull("doc should not be null.", doc);
        // Java: POIXMLDocumentPart part = POIXMLDocumentPart.startPart(doc, XWPFRelation.COMMENT, XWPFFactory.getInstance());
        // Java: IOAssert.assertTrue("part should be instanceof XWPFComment", part instanceof XWPFComment);
        // Java: return (XWPFComment)part;
        todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: void setDocumentImporter(IDocumentImporter importer)
    pub fn setDocumentImporter(&mut self, importer: Box<dyn std::any::Any + Send + Sync>) {
        // Java: this.mCommentsHandler = new CommentsHandler(importer, this);
        todo!()
    }

    // Java: void parse()
    pub fn parse(&mut self) {
        // Java: if (!this.mIsParse)
        if !self.mIsParse {
            // Java: this.mIsParse = true;
            self.mIsParse = true;
            // Java: try {
            // Java: XmlReader xmlReader = new XmlReader();
            let xmlReader = XmlReader::new();
            // Java: xmlReader.parse(this.getPackagePart().getInputStream(), this.mCommentsHandler, "utf-8", -1, 0);
            // Java: } catch (IOException e) {
            // Java: IOLog.e(TAG, "IOException: ", e);
            // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e)) {
            // Java: throw new POIXMLException(e);
            todo!("DEFERRED_NON_FUNCTION")
        }
    }
}
