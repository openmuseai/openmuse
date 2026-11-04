#![allow(non_snake_case)]
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
use crate::io::xwpf::element_handler::theme::font::FontSchemeHandler::FontSchemeHandler;
// Java: public class ThemeElementsHandler extends XmlSimpleNodeElementHandler
pub struct ThemeElementsHandler { mDocImporter: Rc<dyn IDocumentImporter> }
impl ThemeElementsHandler {
    // Java: ThemeElementsHandler(IDocumentImporter importer)
    pub fn new(importer: Rc<dyn IDocumentImporter>) -> Self { Self { mDocImporter: importer } }
    pub fn as_handler_ref(importer: Rc<dyn IDocumentImporter>) -> ElementHandlerRef { Rc::new(RefCell::new(Self::new(importer))) }
    // Java: public ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, _: &str) -> Option<ElementHandlerRef> {
        // Java: switch (elementId)
        match elementId {
            // Java: case -850823468: return new FontSchemeHandler(this.mDocImporter);
            -850823468 => Some(FontSchemeHandler::as_handler_ref(self.mDocImporter.clone())),
            // Java: case -641904494: return this.getFmtSchemeHandler();
            // Java: case 1262641198: return this.getClrSchemeHandler();
            // Rust: fmt/color branches remain unimplemented; never fabricate font callbacks.
            _ => None,
        }
    }
    pub fn onStart(&self, _: i32, _: &str, _: &str, _: &str, _: &dyn Any) {}
}

impl ElementHandlerSimple for ThemeElementsHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, ns: &str, local: &str, qualified: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        ThemeElementsHandler::onStart(self, id, ns, local, qualified, attrs);
        Ok(())
    }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> { self.getElementHandler(id, ns) }
}
