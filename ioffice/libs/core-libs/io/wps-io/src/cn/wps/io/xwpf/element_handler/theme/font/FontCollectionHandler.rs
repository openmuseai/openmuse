#![allow(non_snake_case)]
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
use super::CSFontHandler::CSFontHandler;
use super::EAFontHandler::EAFontHandler;
use super::FontHandler::FontHandler;
use super::LatinFontHandler::LatinFontHandler;
// Java: public class FontCollectionHandler extends XmlSimpleNodeElementHandler
pub struct FontCollectionHandler {
    mDocImporter: Rc<dyn IDocumentImporter>,
    // Java: private FontHandler mFontHandler;
    mFontHandler: RefCell<Option<ElementHandlerRef>>,
}
impl FontCollectionHandler {
    // Java: FontCollectionHandler(IDocumentImporter importer)
    pub fn new(importer: Rc<dyn IDocumentImporter>) -> Self { Self { mDocImporter: importer, mFontHandler: RefCell::new(None) } }
    pub fn as_handler_ref(importer: Rc<dyn IDocumentImporter>) -> ElementHandlerRef { Rc::new(RefCell::new(Self::new(importer))) }
    // Java: public ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, _: &str) -> Option<ElementHandlerRef> {
        // Java: switch (elementId)
        match elementId {
            // Java: case 3184: return new CSFontHandler(this.mDocImporter);
            3184 => Some(CSFontHandler::as_handler_ref(self.mDocImporter.clone())),
            // Java: case 3228: return new EAFontHandler(this.mDocImporter);
            3228 => Some(EAFontHandler::as_handler_ref(self.mDocImporter.clone())),
            // Java: case 3148879: return this.getFontHandler();
            3148879 => Some(self.getFontHandler()),
            // Java: case 102744836: return new LatinFontHandler(this.mDocImporter);
            102744836 => Some(LatinFontHandler::as_handler_ref(self.mDocImporter.clone())),
            // Java: default: return null;
            _ => None,
        }
    }
    // Java: private ElementHandlerSimple getFontHandler()
    fn getFontHandler(&self) -> ElementHandlerRef {
        // Java: if (null == this.mFontHandler) this.mFontHandler = new FontHandler(this.mDocImporter);
        if self.mFontHandler.borrow().is_none() {
            *self.mFontHandler.borrow_mut() = Some(FontHandler::as_handler_ref(self.mDocImporter.clone()));
        }
        // Java: return this.mFontHandler;
        self.mFontHandler.borrow().as_ref().unwrap().clone()
    }
    pub fn onStart(&self, _: i32, _: &str, _: &str, _: &str, _: &dyn Any) {}
}

impl ElementHandlerSimple for FontCollectionHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, ns: &str, local: &str, qualified: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        FontCollectionHandler::onStart(self, id, ns, local, qualified, attrs);
        Ok(())
    }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> { self.getElementHandler(id, ns) }
}
