#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
use crate::io::xwpf::element_handler::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException

// Java field: String ATTR_CLASS_ID
const ATTR_CLASS_ID: &str = "classid";
// Java field: String ATTR_PERSISTSTORAGE
const ATTR_PERSISTSTORAGE: &str = "persistence";

// Java: public class OCX
pub struct OCX {
    // Java field: String clsId
    pub(crate) clsId: Option<String>,
    // Java field: String persistence
    pub(crate) persistence: Option<String>,
}

impl OCX {
    // Java: OCX(String clsId, String persistence)
    pub fn new(clsId: Option<String>, persistence: Option<String>) -> Self {
        // Java: super();
        // Java: this.clsId = clsId;
        // Java: this.persistence = persistence;
        OCX { clsId, persistence }
    }

    // Java: String getClsId()
    pub fn getClsId(&self) -> Option<&str> {
        // Java: return this.clsId;
        self.clsId.as_deref()
    }

    // Java: String getPersistence()
    pub fn getPersistence(&self) -> Option<&str> {
        // Java: return this.persistence;
        self.persistence.as_deref()
    }
}

// Java: public class ActiveXHandler extends XmlSimpleNodeElementHandler
pub struct ActiveXHandler {
    base: XmlSimpleNodeElementHandler,
    // Java field: String mClassId
    pub(crate) mClassId: Option<String>,
    // Java field: String mPersistence
    pub(crate) mPersistence: Option<String>,
}

// Java: public class ActiveXHandler extends XmlSimpleNodeElementHandler
impl ActiveXHandler {
    // Java: ActiveXHandler()
    pub fn new() -> Self {
        // Java: super();
        ActiveXHandler {
            base: XmlSimpleNodeElementHandler::new(),
            mClassId: None,
            mPersistence: None,
        }
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&mut self, elementId: i32, namespaceURI: &str, localName: &str, qualifiedName: &str, attributes: &dyn std::any::Any) {
        // Java: super.onStart(elementId, namespaceURI, localName, qualifiedName, attributes);
        // Java: this.mClassId = attributes.getValue("classid");
        self.mClassId = None; // TODO: extract from attributes
        // Java: this.mPersistence = attributes.getValue("persistence");
        self.mPersistence = None; // TODO: extract from attributes
    }

    // Java: OCX getOCX()
    pub fn getOCX(&self) -> OCX {
        // Java: return new OCX(this.mClassId, this.mPersistence);
        OCX::new(self.mClassId.clone(), self.mPersistence.clone())
    }
}
