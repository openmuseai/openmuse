#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.simple.ElementHandlerSimple
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
// Java: import org.xml.sax.SAXException

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use super::DocDefaultsHandler::DocDefaultsHandler;
use super::StyleHandler::StyleHandler;

// Java: public class StylesHandler extends XmlSimpleNodeElementHandler
pub struct StylesHandler {
    // Java field: StyleHandler mStyleHandler
    pub(crate) mStyleHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: DocDefaultsHandler mDocDefaultsHandler
    pub(crate) mDocDefaultsHandler: RefCell<Option<ElementHandlerRef>>,
    mDocImporter: Option<Rc<dyn IDocumentImporter>>,
}

// Java: public class StylesHandler extends XmlSimpleNodeElementHandler
impl StylesHandler {
    // Java: StylesHandler(IDocumentImporter importer)
    pub fn new(importer: Rc<dyn IDocumentImporter>) -> Self {
        // Java: super(importer);
        StylesHandler {
            mStyleHandler: RefCell::new(None),
            mDocDefaultsHandler: RefCell::new(None),
            mDocImporter: Some(importer),
        }
    }

    pub fn as_handler_ref(importer: Rc<dyn IDocumentImporter>) -> ElementHandlerRef {
        Rc::new(RefCell::new(Self::new(importer)))
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, namespaceURI: &str) -> Option<ElementHandlerRef> {
        // Java: switch (elementId)
        match elementId {
            // Java: case -1780079702:
            -1780079702 => {
                // Java: return this.getDocDefaultsHandler();
                Some(self.getDocDefaultsHandler())
            }
            // Java: case 109780401:
            109780401 => {
                // Java: return this.getStyleHandler();
                Some(self.getStyleHandler())
            }
            // Java: default:
            _ => {
                // Java: return null;
                None
            }
        }
    }

    // Java: private ElementHandlerSimple getDocDefaultsHandler()
    fn getDocDefaultsHandler(&self) -> ElementHandlerRef {
        // Java: if (null == this.mDocDefaultsHandler)
        if self.mDocDefaultsHandler.borrow().is_none() {
            // Java: this.mDocDefaultsHandler = new DocDefaultsHandler(this.mDocImporter);
            let importer = self.mDocImporter.clone().expect("mDocImporter");
            let child: ElementHandlerRef = Rc::new(RefCell::new(DocDefaultsHandler::new_rc(importer)));
            *self.mDocDefaultsHandler.borrow_mut() = Some(child);
        }
        // Java: return this.mDocDefaultsHandler;
        self.mDocDefaultsHandler.borrow().clone().expect("doc defaults handler")
    }

    // Java: private ElementHandlerSimple getStyleHandler()
    fn getStyleHandler(&self) -> ElementHandlerRef {
        // Java: if (null == this.mStyleHandler)
        if self.mStyleHandler.borrow().is_none() {
            // Java: this.mStyleHandler = new StyleHandler(this.mDocImporter);
            let importer = self.mDocImporter.clone().expect("mDocImporter");
            let child: ElementHandlerRef = Rc::new(RefCell::new(StyleHandler::new_rc(importer)));
            *self.mStyleHandler.borrow_mut() = Some(child);
        }
        // Java: return this.mStyleHandler;
        self.mStyleHandler.borrow().clone().expect("style handler")
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return -891774750;
        -891774750
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd(&self, elementId: i32, namespaceURI: &str) {
        // Java: this.mDocImporter.onImportStylesEnd();
        if let Some(ref importer) = self.mDocImporter {
            importer.onImportStylesEnd();
        }
        let _ = (elementId, namespaceURI);
    }
}

impl ElementHandlerSimple for StylesHandler {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn filterIndex(&self) -> i32 {
        StylesHandler::filterIndex(self)
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
        StylesHandler::onEnd(self, elementId, namespaceURI);
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
        StylesHandler::getElementHandler(self, id, ns)
    }
}