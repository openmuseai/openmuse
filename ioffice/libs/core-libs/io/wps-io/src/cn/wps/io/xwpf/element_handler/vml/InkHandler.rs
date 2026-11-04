#![allow(non_snake_case)]

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IVmlImporter::VmlImporterRef;

// Java: public class InkHandler extends XmlSimpleNodeElementHandler
pub struct InkHandler {
    // Java: inherited IVmlImporter mVmlImporter
    mVmlImporter: VmlImporterRef,
}

impl InkHandler {
    // Java: InkHandler(IVmlImporter importer)
    pub fn new(importer: VmlImporterRef) -> Self { Self { mVmlImporter: importer } }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart_with_id(&self, _id: i32, _uri: &str, _local: &str, _qualified: &str, attributes: &dyn Any) {
        // Java: this.mVmlImporter.onImportInk(attributes);
        self.mVmlImporter.borrow_mut().onImportInk(attributes);
    }
}

impl ElementHandlerSimple for InkHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, uri: &str, local: &str, qualified: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { InkHandler::onStart_with_id(self, id, uri, local, qualified, attrs); Ok(()) }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, _: i32, _: &str) -> Option<ElementHandlerRef> { None }
}
