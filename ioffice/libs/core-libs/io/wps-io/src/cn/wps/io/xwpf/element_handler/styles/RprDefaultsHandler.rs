#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.simple.ElementHandlerSimple
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.io.xwpf.element_handler.IPropHandler
// Java: import cn.wps.io.xwpf.element_handler.RprBaseHandler
use crate::io::xwpf::element_handler::RprBaseHandler::RprBaseHandler;
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
use crate::io::xwpf::element_handler::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;
// Java: import cn.wps.io.xwpf.element_handler.prop.PropBase
use crate::io::xwpf::element_handler::prop::Prop::Prop;
// Java: import cn.wps.moffice.property.MutablePropertySet
// Java: import org.xml.sax.Attributes

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

// Java: class RprDefaultsHandler extends XmlSimpleNodeElementHandler implements IPropHandler
pub struct RprDefaultsHandler {
    base: XmlSimpleNodeElementHandler,
    // Java field: RprBaseHandler mRprBaseHandler
    mRprBaseHandler: RefCell<Option<Rc<RefCell<RprBaseHandler>>>>,
    // Java field: PropBase mRPrBaseSet
    mRPrBaseSet: Rc<RefCell<Prop>>,
}

// Java: class RprDefaultsHandler extends XmlSimpleNodeElementHandler implements IPropHandler
impl RprDefaultsHandler {
    pub fn new_rc(importer: Rc<dyn IDocumentImporter>) -> Self {
        // Java: super(importer);
        RprDefaultsHandler {
            base: XmlSimpleNodeElementHandler::new_from_doc_rc(importer),
            mRprBaseHandler: RefCell::new(None),
            // Java: this.mRPrBaseSet = new PropBase();
            mRPrBaseSet: Rc::new(RefCell::new(Prop::new())),
        }
    }

    pub fn as_handler_ref(importer: Rc<dyn IDocumentImporter>) -> ElementHandlerRef {
        Rc::new(RefCell::new(Self::new_rc(importer)))
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, elementId: i32, namespaceURI: &str, localName: &str, qualifiedName: &str, _attributes: &dyn Any) {
        // empty in Java
        let _ = (elementId, namespaceURI, localName, qualifiedName);
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, _elementId: i32, _namespaceURI: &str) -> Option<ElementHandlerRef> {
        // Java: return this.getRprHandler();
        Some(self.getRprHandler() as ElementHandlerRef)
    }

    // Java: private ElementHandlerSimple getRprHandler()
    fn getRprHandler(&self) -> Rc<RefCell<RprBaseHandler>> {
        // Java: if (null == this.mRprBaseHandler)
        if self.mRprBaseHandler.borrow().is_none() {
            // Java: this.mRprBaseHandler = new RprBaseHandler(this.mDocImporter, this.mRPrBaseSet);
            let importer = self.base.mDocImporter.clone().expect("mDocImporter");
            *self.mRprBaseHandler.borrow_mut() = Some(Rc::new(RefCell::new(RprBaseHandler::new(
                importer,
                self.mRPrBaseSet.clone(),
            ))));
        }
        // Java: return this.mRprBaseHandler;
        self.mRprBaseHandler.borrow().clone().expect("rpr handler")
    }

    // Java: MutablePropertySet getProp()
    pub fn getProp(&mut self) -> Option<Box<dyn Any + Send + Sync>> {
        // Java: return this.mRPrBaseSet.isContainProp() ? this.mRPrBaseSet.getProp() : null;
        let mut prop = self.mRPrBaseSet.borrow_mut();
        if prop.isContainProp() {
            Some(Box::new(prop.getProp().clone_mps()))
        } else {
            None
        }
    }

    // Java: void clearProp()
    pub fn clearProp(&mut self) {
        // Java: this.mRPrBaseSet.clearProp();
        self.mRPrBaseSet.borrow_mut().clearProp();
    }
}

impl ElementHandlerSimple for RprDefaultsHandler {
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
        elementId: i32,
        namespaceURI: &str,
        localName: &str,
        qualifiedName: &str,
        attributes: &dyn Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        RprDefaultsHandler::onStart(self, elementId, namespaceURI, localName, qualifiedName, attributes);
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
        RprDefaultsHandler::getElementHandler(self, id, ns)
    }
}