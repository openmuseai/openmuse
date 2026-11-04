#![allow(non_snake_case)]
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
use crate::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
use super::ThemeElementsHandler::ThemeElementsHandler;
// Java: public final class ThemeHandler extends XmlSimpleNodeElementHandler
pub struct ThemeHandler {
    mDocImporter: Rc<dyn IDocumentImporter>,
    // Java: private ThemeElementsHandler mThemeElementsHandler;
    mThemeElementsHandler: RefCell<Option<ElementHandlerRef>>,
}
impl ThemeHandler {
    // Java: ThemeHandler(IDocumentImporter importer, POIXMLDocumentPart packagePart)
    pub fn new(importer: Rc<dyn IDocumentImporter>, packagePart: Rc<POIXMLDocumentPart>) -> Self {
        // Java: super(importer);
        // Java: this.mDocImporter.setThemePackagePart(packagePart);
        importer.setThemePackagePart(&packagePart);
        Self { mDocImporter: importer, mThemeElementsHandler: RefCell::new(None) }
    }
    pub fn as_handler_ref(importer: Rc<dyn IDocumentImporter>, packagePart: Rc<POIXMLDocumentPart>) -> ElementHandlerRef {
        Rc::new(RefCell::new(Self::new(importer, packagePart)))
    }
    // Java: public ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, _: &str) -> Option<ElementHandlerRef> {
        // Java: switch (elementId)
        match elementId {
            // Java: case 2003091936: return this.getThemeElementsHandler();
            2003091936 => Some(self.getThemeElementsHandler()),
            // Java: default: return null;
            _ => None,
        }
    }
    // Java: private ElementHandlerSimple getThemeElementsHandler()
    fn getThemeElementsHandler(&self) -> ElementHandlerRef {
        // Java: if (null == this.mThemeElementsHandler)
        if self.mThemeElementsHandler.borrow().is_none() {
            // Java: this.mThemeElementsHandler = new ThemeElementsHandler(this.mDocImporter);
            *self.mThemeElementsHandler.borrow_mut() = Some(ThemeElementsHandler::as_handler_ref(self.mDocImporter.clone()));
        }
        // Java: return this.mThemeElementsHandler;
        self.mThemeElementsHandler.borrow().as_ref().unwrap().clone()
    }
    pub fn onStart(&self, _: i32, _: &str, _: &str, _: &str, _: &dyn Any) {}
}

impl ElementHandlerSimple for ThemeHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, ns: &str, local: &str, qualified: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        ThemeHandler::onStart(self, id, ns, local, qualified, attrs);
        Ok(())
    }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> { self.getElementHandler(id, ns) }
}
