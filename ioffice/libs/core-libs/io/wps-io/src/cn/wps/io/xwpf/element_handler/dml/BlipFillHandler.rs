#![allow(non_snake_case, dead_code)]
use std::{any::Any, cell::RefCell, rc::{Rc, Weak}};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;
use super::{ClrFromBranchHandler::ClrFromBranchHandler, DmlHandlerBase::DmlHandlerBase, DuotoneHandler::DuotoneHandler};

// Java: public class BlipFillHandler extends DmlHandlerBase
pub struct BlipFillHandler {
    parent: DmlHandlerBase,
    // Java field: DuotoneHandler mDuotoneHandler
    mDuotoneHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: ClrFromBranchHandler mClrFromBranchHandler
    mClrFromBranchHandler: RefCell<Option<ElementHandlerRef>>,
    mSelf: Weak<RefCell<BlipFillHandler>>,
}
impl BlipFillHandler {
    // Java: BlipFillHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType, int belonger)
    pub fn new(dmlImporter: DmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32, belonger: i32) -> Self {
        Self { parent: DmlHandlerBase::new_typed_with_belonger(dmlImporter, part, subDocType, belonger),
            mDuotoneHandler: RefCell::new(None), mClrFromBranchHandler: RefCell::new(None), mSelf: Weak::new() }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef {
        let rc = Rc::new(RefCell::new(self));
        rc.borrow_mut().mSelf = Rc::downgrade(&rc);
        rc
    }
}
impl ElementHandlerSimple for BlipFillHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, _: &str, _: &str, _: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        // Java: importer.startBlipFill(elementId, attributes, this.mPart, this.mBelonger);
        let part = self.parent.mTypedPart.as_ref().expect("mPart");
        self.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut()
            .startBlipFill(id, super::DmlHandlerBase::sax_attributes(attrs),
                super::DmlHandlerBase::document_part(part.as_ref()), self.parent.mBelonger); Ok(())
    }
    fn onEnd_with_id(&mut self, id: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Java: importer.endBlipFill(elementId, this.mBelonger);
        self.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut().endBlipFill(id, self.parent.mBelonger); Ok(())
    }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        // Java: nested blip, stretch and source nodes return this handler.
        match id {
            -1953564184 | -1881872635 | -729134585 | -139578499 | 107524 |
            3026673 | 3560110 | 280621785 | 536515761 | 808992313 =>
                self.mSelf.upgrade().map(|h| h as ElementHandlerRef),
            // Java: 867979379 -> ClrFromBranchHandler; 2011742320 -> DuotoneHandler.
            867979379 => {
                // Java: getClrFromBranchHandler() caches the child with this.mBelonger.
                let mut slot = self.mClrFromBranchHandler.borrow_mut();
                if slot.is_none() {
                    *slot = Some(ClrFromBranchHandler::new(
                        self.parent.mTypedDmlImporter.as_ref()?.clone(), self.parent.mBelonger,
                    ).as_handler_ref());
                }
                slot.clone()
            }
            2011742320 => {
                // Java: getDuotoneHandler() caches the child with this.mBelonger.
                let mut slot = self.mDuotoneHandler.borrow_mut();
                if slot.is_none() {
                    *slot = Some(DuotoneHandler::new(
                        self.parent.mTypedDmlImporter.as_ref()?.clone(), self.parent.mBelonger,
                    ).as_handler_ref());
                }
                slot.clone()
            }
            _ => None,
        }
    }
}
