#![allow(non_snake_case)]
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;
use super::GsBranchHandler::GsBranchHandler;

// Java: public class GsLstHandler extends XmlSimpleNodeElementHandler
pub struct GsLstHandler {
    mDmlImporter: DmlImporterRef,
    mBelonger: i32,
    mGsBranchHandler: RefCell<Option<ElementHandlerRef>>,
}
impl GsLstHandler {
    // Java: GsLstHandler(IDmlCommonImporter dmlImporter, int belonger)
    pub fn new(dmlImporter: DmlImporterRef, belonger: i32) -> Self {
        Self { mDmlImporter: dmlImporter, mBelonger: belonger, mGsBranchHandler: RefCell::new(None) }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
}
impl ElementHandlerSimple for GsLstHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: onStart/onEnd are empty.
    fn onStart_with_id(&mut self, _: i32, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        if id != 3308 { return None; }
        // Java: getGsBranchHandler() caches the child.
        let mut slot = self.mGsBranchHandler.borrow_mut();
        if slot.is_none() { *slot = Some(GsBranchHandler::new(self.mDmlImporter.clone(), self.mBelonger).as_handler_ref()); }
        slot.clone()
    }
}
