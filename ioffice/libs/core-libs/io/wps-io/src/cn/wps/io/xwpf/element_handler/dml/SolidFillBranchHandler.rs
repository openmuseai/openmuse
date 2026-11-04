#![allow(non_snake_case)]
use std::{any::Any, cell::RefCell, rc::{Rc, Weak}};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::{IDmlCommonImporter::IDmlCommonImporter, IDmlImporter::DmlImporterRef};

// Java: public class SolidFillBranchHandler extends XmlSimpleNodeElementHandler
pub struct SolidFillBranchHandler {
    mDmlImporter: DmlImporterRef,
    // Java: private int mBelonger;
    mBelonger: i32,
    mSelf: Weak<RefCell<SolidFillBranchHandler>>,
}
impl SolidFillBranchHandler {
    // Java: SolidFillBranchHandler(IDmlCommonImporter dmlImporter, int belonger)
    pub fn new(dmlImporter: DmlImporterRef, belonger: i32) -> Self {
        Self { mDmlImporter: dmlImporter, mBelonger: belonger, mSelf: Weak::new() }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef {
        let handler = Rc::new(RefCell::new(self));
        handler.borrow_mut().mSelf = Rc::downgrade(&handler);
        handler
    }
}
impl ElementHandlerSimple for SolidFillBranchHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: this.mDmlImporter.startSolidFill(this.mBelonger, elementId, attributes)
    fn onStart_with_id(&mut self, id: i32, _: &str, _: &str, _: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        self.mDmlImporter.borrow_mut().startSolidFill(self.mBelonger, id, attrs);
        Ok(())
    }
    // Java: this.mDmlImporter.endSolidFill(this.mBelonger, elementId)
    fn onEnd_with_id(&mut self, id: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.mDmlImporter.borrow_mut().endSolidFill(self.mBelonger, id);
        Ok(())
    }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: return this;
    fn getElementHandler(&self, _: i32, _: &str) -> Option<ElementHandlerRef> {
        self.mSelf.upgrade().map(|h| h as ElementHandlerRef)
    }
}
