#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

// Java: import cn.wps.io.xwpf.IDmlCommonImporter
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
// Java: import org.xml.sax.Attributes

use crate::io::xwpf::IDmlCommonImporter::IDmlCommonImporter;
use crate::io::xwpf::element_handler::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;
use std::cell::RefCell;

// Java: public class NoFillHandler extends XmlSimpleNodeElementHandler
pub struct NoFillHandler {
    base: XmlSimpleNodeElementHandler,
    // Java field: int mBelonger
    mBelonger: i32,
    mDmlImporter: RefCell<Option<Box<dyn IDmlCommonImporter>>>,
}

// Java: public class NoFillHandler extends XmlSimpleNodeElementHandler
impl NoFillHandler {
    // Java: NoFillHandler(IDmlCommonImporter dmlImporter, int belonger)
    pub fn new(dml_importer: Box<dyn IDmlCommonImporter>, belonger: i32) -> Self {
        // Java: super(dmlImporter);
        // Java: this.mBelonger = belonger;
        Self {
            base: XmlSimpleNodeElementHandler::new(),
            mBelonger: belonger,
            mDmlImporter: RefCell::new(Some(dml_importer)),
        }
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, elementId: i32, namespaceURI: &str, localName: &str, qualifiedName: &str, attributes: &dyn std::any::Any) {
        // Java: this.mDmlImporter.startNoFill(this.mBelonger);
        if let Some(ref mut importer) = *self.mDmlImporter.borrow_mut() {
            importer.startNoFill(self.mBelonger);
        }
    }

}
