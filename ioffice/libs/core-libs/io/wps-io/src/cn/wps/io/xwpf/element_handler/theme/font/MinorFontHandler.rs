#![allow(non_snake_case)]
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
use super::FontCollectionHandler::FontCollectionHandler;
// Java: public class MinorFontHandler extends FontCollectionHandler
pub struct MinorFontHandler { base: FontCollectionHandler, mDocImporter: Rc<dyn IDocumentImporter> }
impl MinorFontHandler {
    // Java: MinorFontHandler(IDocumentImporter importer)
    pub fn new(importer: Rc<dyn IDocumentImporter>) -> Self { Self { base: FontCollectionHandler::new(importer.clone()), mDocImporter: importer } }
    pub fn as_handler_ref(importer: Rc<dyn IDocumentImporter>) -> ElementHandlerRef { Rc::new(RefCell::new(Self::new(importer))) }
    // Java: public void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, _: i32, _: &str, _: &str, _: &str, _: &dyn Any) {
        // Java: this.mDocImporter.onImportMinorFontStart();
        self.mDocImporter.onImportMinorFontStart();
    }
    // Java: inherited FontCollectionHandler.getElementHandler.
    pub fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> { self.base.getElementHandler(id, ns) }
}

impl ElementHandlerSimple for MinorFontHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, ns: &str, local: &str, qualified: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        MinorFontHandler::onStart(self, id, ns, local, qualified, attrs);
        Ok(())
    }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> { self.getElementHandler(id, ns) }
}
