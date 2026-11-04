#![allow(non_snake_case)]
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::{IDmlCommonImporter::IDmlCommonImporter, IDmlImporter::DmlImporterRef};

// Java: public class GrpFillHandler extends XmlSimpleNodeElementHandler
pub struct GrpFillHandler {
    mDmlImporter: DmlImporterRef,
    // Java: private int mBelonger;
    mBelonger: i32,
}
impl GrpFillHandler {
    // Java: GrpFillHandler(IDmlCommonImporter importer, int belonger)
    pub fn new(importer: DmlImporterRef, belonger: i32) -> Self { Self { mDmlImporter: importer, mBelonger: belonger } }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
}
impl ElementHandlerSimple for GrpFillHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: this.mDmlImporter.startGrpFill(this.mBelonger, elementId, attributes)
    fn onStart_with_id(&mut self, id: i32, _: &str, _: &str, _: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        self.mDmlImporter.borrow_mut().startGrpFill(self.mBelonger, id, attrs);
        Ok(())
    }
    // Java: this.mDmlImporter.endGrpFill(this.mBelonger)
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.mDmlImporter.borrow_mut().endGrpFill(self.mBelonger);
        Ok(())
    }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java inherits the default child handler lookup, returning null.
    fn getElementHandler(&self, _: i32, _: &str) -> Option<ElementHandlerRef> { None }
}
