#![allow(non_snake_case)]
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: public class FontHandler extends XmlSimpleNodeElementHandler
pub struct FontHandler { mDocImporter: Rc<dyn IDocumentImporter> }
impl FontHandler {
    // Java: FontHandler(IDocumentImporter importer)
    pub fn new(importer: Rc<dyn IDocumentImporter>) -> Self { Self { mDocImporter: importer } }
    pub fn as_handler_ref(importer: Rc<dyn IDocumentImporter>) -> ElementHandlerRef { Rc::new(RefCell::new(Self::new(importer))) }
    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, _: i32, _: &str, _: &str, _: &str, attributes: &dyn Any) {
        // Java: this.mDocImporter.onImportThemeFontStart(attributes);
        self.mDocImporter.onImportThemeFontStart(attributes);
    }
    // Java: XmlSimpleNodeElementHandler.getElementHandler returns null.
    pub fn getElementHandler(&self, _: i32, _: &str) -> Option<ElementHandlerRef> { None }
}

impl ElementHandlerSimple for FontHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, ns: &str, local: &str, qualified: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        FontHandler::onStart(self, id, ns, local, qualified, attrs);
        Ok(())
    }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> { self.getElementHandler(id, ns) }
}
