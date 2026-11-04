#![allow(non_snake_case)]
use std::{any::Any, cell::RefCell, rc::{Rc, Weak}};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::{IDmlCommonImporter::IDmlCommonImporter, IDmlImporter::DmlImporterRef};
use super::{DmlHandlerBase::DmlHandlerBase, GradFillHandler::GradFillHandler,
    PattFillHandler::PattFillHandler, SolidFillBranchHandler::SolidFillBranchHandler};

// Java: public class LnBranchHandler extends DmlHandlerBase
pub struct LnBranchHandler {
    parent: DmlHandlerBase,
    mPattFillHandler: RefCell<Option<ElementHandlerRef>>,
    mSolidFillHandler: RefCell<Option<ElementHandlerRef>>,
    mGradFillHandler: RefCell<Option<ElementHandlerRef>>,
    mSelf: Weak<RefCell<LnBranchHandler>>,
}
impl LnBranchHandler {
    // Java: LnBranchHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType)
    pub fn new(dmlImporter: DmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32) -> Self {
        Self { parent: DmlHandlerBase::new_typed(dmlImporter, part, subDocType),
            mPattFillHandler: RefCell::new(None), mSolidFillHandler: RefCell::new(None),
            mGradFillHandler: RefCell::new(None), mSelf: Weak::new() }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef {
        let handler = Rc::new(RefCell::new(self));
        handler.borrow_mut().mSelf = Rc::downgrade(&handler);
        handler
    }
}
impl ElementHandlerSimple for LnBranchHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: importer.startLn(elementId, attributes)
    fn onStart_with_id(&mut self, id: i32, _: &str, _: &str, _: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        self.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut().startLn(id, attrs); Ok(())
    }
    // Java: importer.endLn(elementId)
    fn onEnd_with_id(&mut self, id: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut().endLn(id); Ok(())
    }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        let importer = self.parent.mTypedDmlImporter.as_ref()?.clone();
        match id {
            -1353661746 => { // Java: getSolidHandler(), belonger 3458
                let mut slot = self.mSolidFillHandler.borrow_mut();
                if slot.is_none() { *slot = Some(SolidFillBranchHandler::new(importer, 3458).as_handler_ref()); }
                slot.clone()
            }
            88612081 => { // Java: getGradFillHandler(), belonger 3458
                let mut slot = self.mGradFillHandler.borrow_mut();
                if slot.is_none() { *slot = Some(GradFillHandler::new(importer, 3458).as_handler_ref()); }
                slot.clone()
            }
            1245072724 => { // Java: getPattFillHandler(), belonger 3458
                let mut slot = self.mPattFillHandler.borrow_mut();
                if slot.is_none() { *slot = Some(PattFillHandler::new(importer,
                    self.parent.mTypedPart.clone(), self.parent.mSubDocType, 3458).as_handler_ref()); }
                slot.clone()
            }
            // Java: default: return this;
            _ => self.mSelf.upgrade().map(|h| h as ElementHandlerRef),
        }
    }
}
