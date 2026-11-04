#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.io.simple.ElementHandlerSimple
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
use crate::io::xwpf::element_handler::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;
// Java: import cn.wps.moffice.property.MutablePropertySet
// Java: import org.xml.sax.SAXException

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use super::PprDefaultsHandler::PprDefaultsHandler;
use super::RprDefaultsHandler::RprDefaultsHandler;

// Java: class DocDefaultsHandler extends XmlSimpleNodeElementHandler
pub struct DocDefaultsHandler {
    base: XmlSimpleNodeElementHandler,
    // Java field: RprDefaultsHandler mRprDefaultsHandler
    mRprDefaultsHandler: RefCell<Option<Rc<RefCell<RprDefaultsHandler>>>>,
    // Java field: PprDefaultsHandler mPprDefaultsHandler
    mPprDefaultsHandler: RefCell<Option<Rc<RefCell<PprDefaultsHandler>>>>,
}

// Java: class DocDefaultsHandler extends XmlSimpleNodeElementHandler
impl DocDefaultsHandler {
    pub fn new_rc(importer: Rc<dyn IDocumentImporter>) -> Self {
        // Java: super(importer);
        DocDefaultsHandler {
            base: XmlSimpleNodeElementHandler::new_from_doc_rc(importer),
            mRprDefaultsHandler: RefCell::new(None),
            mPprDefaultsHandler: RefCell::new(None),
        }
    }

    pub fn as_handler_ref(importer: Rc<dyn IDocumentImporter>) -> ElementHandlerRef {
        Rc::new(RefCell::new(Self::new_rc(importer)))
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, namespaceURI: &str) -> Option<ElementHandlerRef> {
        // Java: switch (elementId)
        match elementId {
            // Java: case 531200397:
            531200397 => {
                // Java: return this.getRprDefaultsHandler();
                Some(self.getRprDefaultsHandler() as ElementHandlerRef)
            }
            // Java: case 924227407:
            924227407 => {
                // Java: return this.getPprDefaultsHandler();
                Some(self.getPprDefaultsHandler() as ElementHandlerRef)
            }
            // Java: default:
            _ => {
                // Java: IOAssert.fail("it should not reach here");
                // rust: no-stmt（IOAssert.fail 语法在 Rust 侧无对应语句，性能路径不炸）
                let _ = namespaceURI;
                None
            }
        }
    }

    // Java: private ElementHandlerSimple getPprDefaultsHandler()
    fn getPprDefaultsHandler(&self) -> Rc<RefCell<PprDefaultsHandler>> {
        // Java: if (null == this.mPprDefaultsHandler)
        if self.mPprDefaultsHandler.borrow().is_none() {
            // Java: this.mPprDefaultsHandler = new PprDefaultsHandler(this.mDocImporter);
            let importer = self.base.mDocImporter.clone().expect("mDocImporter");
            *self.mPprDefaultsHandler.borrow_mut() = Some(Rc::new(RefCell::new(PprDefaultsHandler::new_rc(importer))));
        }
        // Java: return this.mPprDefaultsHandler;
        self.mPprDefaultsHandler.borrow().clone().expect("ppr defaults handler")
    }

    // Java: private ElementHandlerSimple getRprDefaultsHandler()
    fn getRprDefaultsHandler(&self) -> Rc<RefCell<RprDefaultsHandler>> {
        // Java: if (null == this.mRprDefaultsHandler)
        if self.mRprDefaultsHandler.borrow().is_none() {
            // Java: this.mRprDefaultsHandler = new RprDefaultsHandler(this.mDocImporter);
            let importer = self.base.mDocImporter.clone().expect("mDocImporter");
            *self.mRprDefaultsHandler.borrow_mut() = Some(Rc::new(RefCell::new(RprDefaultsHandler::new_rc(importer))));
        }
        // Java: return this.mRprDefaultsHandler;
        self.mRprDefaultsHandler.borrow().clone().expect("rpr defaults handler")
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd(&self, elementId: i32, namespaceURI: &str) {
        // Java: MutablePropertySet rPrSet = null;
        let mut rPrSet: Option<Box<dyn Any + Send + Sync>> = None;
        // Java: if (null != this.mRprDefaultsHandler)
        if let Some(ref h) = *self.mRprDefaultsHandler.borrow() {
            // Java: rPrSet = this.mRprDefaultsHandler.getProp();
            rPrSet = h.borrow_mut().getProp();
            // Java: this.mRprDefaultsHandler.clearProp();
            h.borrow_mut().clearProp();
        }
        // Java: MutablePropertySet pPrSet = null;
        let mut pPrSet: Option<Box<dyn Any + Send + Sync>> = None;
        // Java: if (null != this.mPprDefaultsHandler)
        if let Some(ref h) = *self.mPprDefaultsHandler.borrow() {
            // Java: pPrSet = this.mPprDefaultsHandler.getProp();
            pPrSet = h.borrow_mut().getProp();
            // Java: this.mPprDefaultsHandler.clearProp();
            h.borrow_mut().clearProp();
        }
        // Java: if (null != rPrSet || null != pPrSet)
        if rPrSet.is_some() || pPrSet.is_some() {
            // Java: this.mDocImporter.onImportStyleDocDefaults(rPrSet, pPrSet);
            if let Some(ref importer) = self.base.mDocImporter {
                let rpr: &dyn Any = match &rPrSet { Some(b) => &**b, None => &NO_PROP };
                let ppr: &dyn Any = match &pPrSet { Some(b) => &**b, None => &NO_PROP };
                importer.onImportStyleDocDefaults(rpr, ppr);
            }
        }
        let _ = (elementId, namespaceURI);
    }
}

/// Java null 的哨兵（null PropertySet → &NO_PROP as &dyn Any）
static NO_PROP: () = ();

impl ElementHandlerSimple for DocDefaultsHandler {
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
        elementId: i32,
        namespaceURI: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        DocDefaultsHandler::onEnd(self, elementId, namespaceURI);
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
        DocDefaultsHandler::getElementHandler(self, id, ns)
    }
}