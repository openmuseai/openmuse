#![allow(non_snake_case)]
use std::{any::Any, cell::RefCell, rc::{Rc, Weak}};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;

// Java: public class ClrFromBranchHandler extends XmlSimpleNodeElementHandler
pub struct ClrFromBranchHandler {
    mDmlImporter: DmlImporterRef,
    // Java: int mBelonger;
    mBelonger: i32,
    mSelf: Weak<RefCell<ClrFromBranchHandler>>,
}

impl ClrFromBranchHandler {
    // Java: ClrFromBranchHandler(IDmlImporter dmlImporter, int belonger)
    pub fn new(dmlImporter: DmlImporterRef, belonger: i32) -> Self {
        Self { mDmlImporter: dmlImporter, mBelonger: belonger, mSelf: Weak::new() }
    }

    pub fn as_handler_ref(self) -> ElementHandlerRef {
        let handler = Rc::new(RefCell::new(self));
        handler.borrow_mut().mSelf = Rc::downgrade(&handler);
        handler
    }
}

impl ElementHandlerSimple for ClrFromBranchHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: importer.startClrFrom(elementId, attributes, this.mBelonger);
    fn onStart_with_id(&mut self, id: i32, _: &str, _: &str, _: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        self.mDmlImporter.borrow_mut().startClrFrom(id, super::DmlHandlerBase::sax_attributes(attrs), self.mBelonger);
        Ok(())
    }
    // Java: importer.endClrFrom(elementId, this.mBelonger);
    fn onEnd_with_id(&mut self, id: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.mDmlImporter.borrow_mut().endClrFrom(id, self.mBelonger);
        Ok(())
    }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: getElementHandler(int elementId, String namespaceURI) { return this; }
    fn getElementHandler(&self, _: i32, _: &str) -> Option<ElementHandlerRef> {
        self.mSelf.upgrade().map(|h| h as ElementHandlerRef)
    }
}
