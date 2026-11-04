#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
use crate::io::xwpf::element_handler::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException

// Java field: String RELID
const RELID: &str = "relId";

// Java: public class DataModelExtHandler extends XmlSimpleNodeElementHandler
pub struct DataModelExtHandler {
    base: XmlSimpleNodeElementHandler,
    // Java field: String mDrawingRelId
    pub(crate) mDrawingRelId: Option<String>,
}

// Java: public class DataModelExtHandler extends XmlSimpleNodeElementHandler
impl DataModelExtHandler {
    // Java: DataModelExtHandler()
    pub fn new() -> Self {
        // Java: super();
        DataModelExtHandler {
            base: XmlSimpleNodeElementHandler::new(),
            mDrawingRelId: None,
        }
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&mut self, elementId: i32, namespaceURI: &str, localName: &str, qualifiedName: &str, attributes: &dyn std::any::Any) {
        // Java: String relId = attributes.getValue("relId");
        let relId: Option<String> = None; // TODO: extract from attributes
        // Java: if (null != relId)
        if let Some(ref id) = relId {
            // Java: this.mDrawingRelId = relId;
            self.mDrawingRelId = relId.clone();
        }
    }

    // Java: String getDrawingRelId()
    pub fn getDrawingRelId(&self) -> Option<&str> {
        // Java: return this.mDrawingRelId;
        self.mDrawingRelId.as_deref()
    }
}
