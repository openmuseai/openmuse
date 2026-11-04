#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException
use super::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;

// Java: public class CommentRangeHandler extends XmlSimpleNodeElementHandler
pub struct CommentRangeHandler {
    base: XmlSimpleNodeElementHandler,
    // Java field: SubDocType mSubDocType
    pub(crate) mSubDocType: i32,
}

// Java: public class CommentRangeHandler extends XmlSimpleNodeElementHandler
impl CommentRangeHandler {
    // Java: CommentRangeHandler(IDocumentImporter importer, SubDocType subDocType)
    pub fn new(_importer: Box<dyn std::any::Any + Send + Sync>, subDocType: i32) -> Self {
        // Java: super(importer);
        // Java: this.mSubDocType = subDocType;
        CommentRangeHandler {
            base: XmlSimpleNodeElementHandler::new(),
            mSubDocType: subDocType,
        }
    }

    // Java: void setSubDocType(SubDocType mSubDocType)
    pub fn setSubDocType(&mut self, mSubDocType: i32) {
        // Java: this.mSubDocType = mSubDocType;
        self.mSubDocType = mSubDocType;
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, elementId: i32, namespaceURI: &str, localName: &str, qualifiedName: &str, attributes: &dyn std::any::Any) {
        // Java: this.mDocImporter.onImportCommentRange(elementId, attributes, this.mSubDocType);
        // TODO: call importer.onImportCommentRange
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 0;
        0
    }
}
