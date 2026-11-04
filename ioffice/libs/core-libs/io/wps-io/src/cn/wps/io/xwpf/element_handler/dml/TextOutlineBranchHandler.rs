#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IDmlCommonImporter
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
// Java: import org.xml.sax.Attributes

use crate::io::xwpf::IDmlCommonImporter::IDmlCommonImporter;
use crate::io::xwpf::element_handler::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;
use super::SolidFillBranchHandler::SolidFillBranchHandler;
use super::GradFillHandler::GradFillHandler;
use std::cell::RefCell;

// Java: public class TextOutlineBranchHandler extends XmlSimpleNodeElementHandler
pub struct TextOutlineBranchHandler {
    base: XmlSimpleNodeElementHandler,
    // Java field: SolidFillBranchHandler mSolidFillHandler
    pub(crate) mSolidFillHandler: RefCell<Option<SolidFillBranchHandler>>,
    // Java field: GradFillHandler mGradFillHandler
    pub(crate) mGradFillHandler: RefCell<Option<GradFillHandler>>,
    mDmlImporter: RefCell<Option<Box<dyn IDmlCommonImporter>>>,
}

// Java: public class TextOutlineBranchHandler extends XmlSimpleNodeElementHandler
impl TextOutlineBranchHandler {
    // Java: TextOutlineBranchHandler(IDmlCommonImporter dmlImporter)
    pub fn new(dml_importer: Box<dyn IDmlCommonImporter>) -> Self {
        // Java: super(dmlImporter);
        Self {
            base: XmlSimpleNodeElementHandler::new(),
            mSolidFillHandler: RefCell::new(None),
            mGradFillHandler: RefCell::new(None),
            mDmlImporter: RefCell::new(Some(dml_importer)),
        }
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, elementId: i32, namespaceURI: &str, localName: &str, qualifiedName: &str, attributes: &dyn std::any::Any) {
        // Java: this.mDmlImporter.startLn(elementId, attributes);
        if let Some(ref mut importer) = *self.mDmlImporter.borrow_mut() {
            importer.startLn(elementId, attributes);
        }
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd(&self, elementId: i32, namespaceURI: &str) {
        // Java: this.mDmlImporter.endLn(elementId);
        if let Some(ref mut importer) = *self.mDmlImporter.borrow_mut() {
            importer.endLn(elementId);
        }
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, namespaceURI: &str) -> Option<Box<dyn std::any::Any + Send + Sync>> {
        // Java: switch (elementId)
        match elementId {
            // Java: case -1353661746:
            -1353661746_i32 => {
                // Java: return this.getSolidHandler();
                None
            }
            // Java: case 88612081:
            88612081 => {
                // Java: return this.getGradFillHandler();
                None
            }
            // Java: default:
            _ => {
                // Java: return this;
                None
            }
        }
    }

    // Java: protected ElementHandlerSimple getGradFillHandler()
    pub(crate) fn getGradFillHandler(&self) {
        // Java: if (null == this.mGradFillHandler)
        // Java: this.mGradFillHandler = new GradFillHandler(this.mDmlImporter, 14);
        // Java: return this.mGradFillHandler;
    }

    // Java: protected ElementHandlerSimple getSolidHandler()
    pub(crate) fn getSolidHandler(&self) {
        // Java: if (null == this.mSolidFillHandler)
        // Java: this.mSolidFillHandler = new SolidFillBranchHandler(this.mDmlImporter, 14);
        // Java: return this.mSolidFillHandler;
    }

}
