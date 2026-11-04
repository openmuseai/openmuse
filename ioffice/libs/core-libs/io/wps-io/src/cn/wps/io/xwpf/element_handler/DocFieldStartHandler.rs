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

// Java: public class DocFieldStartHandler extends XmlSimpleNodeElementHandler
pub struct DocFieldStartHandler {
    base: XmlSimpleNodeElementHandler,
    // Java field: IDocumentImporter mDocumentImporter
    pub(crate) mDocumentImporter: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: SubDocType mSubDocType
    pub(crate) mSubDocType: i32,
}

// Java: public class DocFieldStartHandler extends XmlSimpleNodeElementHandler
impl DocFieldStartHandler {
    // Java: DocFieldStartHandler(IDocumentImporter iDocumentImporter, SubDocType subDocType)
    pub fn new(iDocumentImporter: Box<dyn std::any::Any + Send + Sync>, subDocType: i32) -> Self {
        // Java: super();
        // Java: this.mDocumentImporter = iDocumentImporter;
        // Java: this.mSubDocType = subDocType;
        DocFieldStartHandler {
            base: XmlSimpleNodeElementHandler::new(),
            mDocumentImporter: Some(iDocumentImporter),
            mSubDocType: subDocType,
        }
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, elementId: i32, namespaceURI: &str, localName: &str, qualifiedName: &str, attributes: &dyn std::any::Any) {
        // Java: this.mDocumentImporter.onImportDocFieldStartEnd(elementId, attributes, this.mSubDocType);
        // TODO: call mDocumentImporter.onImportDocFieldStartEnd
    }
}
