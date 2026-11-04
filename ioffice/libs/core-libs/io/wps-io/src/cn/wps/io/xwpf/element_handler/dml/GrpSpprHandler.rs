#![allow(non_snake_case, dead_code)]
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;
use super::{DmlHandlerBase::sax_attributes, SpprHandler::SpprHandler};

// Java: public class GrpSpprHandler extends SpprHandler
pub struct GrpSpprHandler {
    parent: SpprHandler,
    mDmlImporter: DmlImporterRef,
}
impl GrpSpprHandler {
    // Java: GrpSpprHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType)
    pub fn new(dmlImporter: DmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32) -> Self {
        Self { parent: SpprHandler::new_typed_group(dmlImporter.clone(), part, subDocType), mDmlImporter: dmlImporter }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
    // Java: protected boolean isGroup() { return true; }
    pub fn isGroup(&self) -> bool { self.parent.isGroup() }
}
impl ElementHandlerSimple for GrpSpprHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, _: &str, _: &str, _: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        // Java: importer.startGrpSppr(elementId, attrs);
        self.mDmlImporter.borrow_mut().startGrpSppr(id, sax_attributes(attrs)); Ok(())
    }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Java: importer.endGrpSppr();
        self.mDmlImporter.borrow_mut().endGrpSppr(); Ok(())
    }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> {
        ElementHandlerSimple::getElementHandler(&self.parent, id, ns)
    }
}
