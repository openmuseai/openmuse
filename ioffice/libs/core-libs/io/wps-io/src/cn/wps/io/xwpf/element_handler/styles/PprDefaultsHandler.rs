#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.simple.ElementHandlerSimple
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.io.xwpf.element_handler.IPropHandler
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
use crate::io::xwpf::element_handler::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;
// Java: import cn.wps.io.xwpf.element_handler.prop.NormalPropBaseHandler
use crate::io::xwpf::element_handler::prop::NormalPropBaseHandler::NormalPropBaseHandler;
// Java: import cn.wps.io.xwpf.element_handler.prop.PropHandlerHelper
// Java: import cn.wps.moffice.property.MutablePropertySet

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

// Java: class PprDefaultsHandler extends XmlSimpleNodeElementHandler implements IPropHandler
pub struct PprDefaultsHandler {
    base: XmlSimpleNodeElementHandler,
    // Java field: NormalPropBaseHandler mPprGeneralHandler
    mPprGeneralHandler: RefCell<Option<Rc<RefCell<NormalPropBaseHandler>>>>,
}

// Java: class PprDefaultsHandler extends XmlSimpleNodeElementHandler implements IPropHandler
impl PprDefaultsHandler {
    pub fn new_rc(importer: Rc<dyn IDocumentImporter>) -> Self {
        // Java: super(importer);
        PprDefaultsHandler {
            base: XmlSimpleNodeElementHandler::new_from_doc_rc(importer),
            mPprGeneralHandler: RefCell::new(None),
        }
    }

    pub fn as_handler_ref(importer: Rc<dyn IDocumentImporter>) -> ElementHandlerRef {
        Rc::new(RefCell::new(Self::new_rc(importer)))
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, _elementId: i32, _namespaceURI: &str) -> Option<ElementHandlerRef> {
        // Java: return this.getPprHandler();
        Some(self.getPprHandler() as ElementHandlerRef)
    }

    // Java: private ElementHandlerSimple getPprHandler()
    fn getPprHandler(&self) -> Rc<RefCell<NormalPropBaseHandler>> {
        // Java: if (null == this.mPprGeneralHandler)
        if self.mPprGeneralHandler.borrow().is_none() {
            // Java: this.mPprGeneralHandler = new NormalPropBaseHandler(this.mDocImporter, PropHandlerHelper.PprGeneralBaseHandlerHelper.geHandlerHelper());
            let importer = self.base.mDocImporter.clone().expect("mDocImporter");
            *self.mPprGeneralHandler.borrow_mut() = Some(Rc::new(RefCell::new(NormalPropBaseHandler::new_ppr_general_rc(importer))));
        }
        // Java: return this.mPprGeneralHandler;
        self.mPprGeneralHandler.borrow().clone().expect("ppr handler")
    }

    // Java: MutablePropertySet getProp()
    pub fn getProp(&mut self) -> Option<Box<dyn Any + Send + Sync>> {
        // Java: return null == this.mPprGeneralHandler ? null : this.mPprGeneralHandler.getProp();
        match self.mPprGeneralHandler.borrow().as_ref() {
            Some(h) => h.borrow_mut().getProp(),
            None => None,
        }
    }

    // Java: void clearProp()
    pub fn clearProp(&mut self) {
        // Java: if (null != this.mPprGeneralHandler)
        if let Some(ref h) = *self.mPprGeneralHandler.borrow() {
            // Java: this.mPprGeneralHandler.clearProp();
            h.borrow_mut().clearProp();
        }
    }
}

impl ElementHandlerSimple for PprDefaultsHandler {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn filterIndex(&self) -> i32 {
        0
    }
    fn onStart(
        &mut self,
        _a: &str,
        _b: &str,
        _c: &str,
        _d: &dyn Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onEnd(
        &mut self,
        _a: &str,
        _b: &str,
        _c: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onStart_with_id(
        &mut self,
        _id: i32,
        _ns: &str,
        _local: &str,
        _q: &str,
        _attrs: &dyn Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onEnd_with_id(
        &mut self,
        _id: i32,
        _ns: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onCharacters(
        &mut self,
        _b: &[char],
        _s: i32,
        _l: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onCharacters_str(&mut self, _s: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> {
        PprDefaultsHandler::getElementHandler(self, id, ns)
    }
}
