#![allow(non_snake_case)]

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IVmlImporter::VmlImporterRef;
use super::NormalShapeHandler::NormalShapeHandler;

// Java: public class ShapeHandler extends NormalShapeHandler
pub struct ShapeHandler {
    parent: NormalShapeHandler,
}

impl ShapeHandler {
    // Java: ShapeHandler(POIXMLDocumentPart part, IVmlImporter importer, SubDocType subDocType)
    pub fn new(part: Option<Rc<dyn Any>>, importer: VmlImporterRef, subDocType: i32) -> Self {
        // Java: super(part, importer, subDocType);
        Self { parent: NormalShapeHandler::new(part, importer, subDocType) }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, id: i32, uri: &str) -> Option<ElementHandlerRef> {
        // Java: case -1: case 1678632267: return null; default: return super.getElementHandler(...);
        match id { -1 | 1678632267 => None, _ => self.parent.getElementHandler(id, uri) }
    }
}

impl ElementHandlerSimple for ShapeHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, uri: &str, local: &str, qualified: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { self.parent.onStart_with_id(id, uri, local, qualified, attrs); Ok(()) }
    fn onEnd_with_id(&mut self, id: i32, uri: &str) -> Result<(), Box<dyn std::error::Error>> { self.parent.onEnd_with_id(id, uri); Ok(()) }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, uri: &str) -> Option<ElementHandlerRef> { ShapeHandler::getElementHandler(self, id, uri) }
}
