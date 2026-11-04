#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException
use super::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;

// Java: public class MoveRangeStartHandler extends XmlSimpleNodeElementHandler
pub struct MoveRangeStartHandler {
    base: XmlSimpleNodeElementHandler,
}

// Java: public class MoveRangeStartHandler extends XmlSimpleNodeElementHandler
impl MoveRangeStartHandler {
    // Java: MoveRangeStartHandler(IDocumentImporter importer)
    pub fn new(_importer: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super(importer);
        MoveRangeStartHandler {
            base: XmlSimpleNodeElementHandler::new(),
        }
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, elementId: i32, namespaceURI: &str, localName: &str, qualifiedName: &str, attributes: &dyn std::any::Any) {
        // Java: this.mDocImporter.onImportMoveRangeStart(elementId, attributes);
        // TODO: call importer.onImportMoveRangeStart
    }
}
