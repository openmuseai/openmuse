#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
use crate::io::xwpf::element_handler::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;
// Java: import org.xml.sax.Attributes

// Java: public class FontChildHandler extends XmlSimpleNodeElementHandler
pub struct FontChildHandler {
    base: XmlSimpleNodeElementHandler,
}

// Java: public class FontChildHandler extends XmlSimpleNodeElementHandler
impl FontChildHandler {
    // Java: FontChildHandler(IDocumentImporter importer)
    pub fn new(_importer: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super(importer);
        FontChildHandler {
            base: XmlSimpleNodeElementHandler::new(),
        }
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, elementId: i32, namespaceURI: &str, localName: &str, qualifiedName: &str, attributes: &dyn std::any::Any) {
        // Java: this.mDocImporter.onImportFontChild(elementId, attributes);
        // TODO: call importer.onImportFontChild
    }
}
