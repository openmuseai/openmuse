#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IVmlImporter
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException

// Java: public class Xml03BackgroundHandler extends XmlSimpleNodeElementHandler
pub struct Xml03BackgroundHandler {
    // Java field: boolean isBgColor
    isBgColor: bool,
}

// Java: public class Xml03BackgroundHandler extends XmlSimpleNodeElementHandler
impl Xml03BackgroundHandler {
    // Java: Xml03BackgroundHandler(IVmlImporter importer)
    pub fn new(_importer: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super(importer);
        Xml03BackgroundHandler {
            isBgColor: false,
        }
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&mut self, _element_id: i32, _namespace_uri: &str, _local_name: &str, _qualified_name: &str, _attributes: &dyn std::any::Any) {
        // Java: if (this.isBgColor = attributes.getValue("bgcolor") != null)
        // Java: this.mVmlImporter.onImportWBackgroundStart((POIXMLDocumentPart)null, elementId, attributes);
        // Java: else
        // Java: this.mVmlImporter.onImportVBackgroundStart(elementId, namespaceURI, attributes);
        todo!()
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd(&self, _element_id: i32, _namespace_uri: &str) {
        // Java: this.mVmlImporter.onImportBackgroundEnd(elementId, namespaceURI);
        todo!()
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, element_id: i32, _namespace_uri: &str) -> Option<Box<dyn std::any::Any + Send + Sync>> {
        // Java: switch (elementId)
        match element_id {
            // Java: case 3143043:
            3143043 => {
                // Java: return this;
                todo!()
            }
            // Java: default:
            _ => {
                // Java: IOAssert.fail("it should not reach here");
                // Java: return null;
                None
            }
        }
    }
}
