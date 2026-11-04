#![allow(non_snake_case)]
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;
use super::{BgBranchClrHandler::BgBranchClrHandler, DmlHandlerBase::DmlHandlerBase, FgBranchClrHandler::FgBranchClrHandler};

// Java: public class PattFillHandler extends DmlHandlerBase
pub struct PattFillHandler {
    parent: DmlHandlerBase,
    mFgBranchClrHandler: RefCell<Option<ElementHandlerRef>>,
    mBgBranchClrHandler: RefCell<Option<ElementHandlerRef>>,
}
impl PattFillHandler {
    // Java: PattFillHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType, int belonger)
    pub fn new(dmlImporter: DmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32, belonger: i32) -> Self {
        Self { parent: DmlHandlerBase::new_typed_with_belonger(dmlImporter, part, subDocType, belonger),
            mFgBranchClrHandler: RefCell::new(None), mBgBranchClrHandler: RefCell::new(None) }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
}
impl ElementHandlerSimple for PattFillHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: importer.startPattFill(this.mBelonger, elementId, attributes)
    fn onStart_with_id(&mut self, id: i32, _: &str, _: &str, _: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        self.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut()
            .startPattFill(self.parent.mBelonger, id, super::DmlHandlerBase::sax_attributes(attrs));
        Ok(())
    }
    // Java: importer.endPattFill(this.mBelonger, elementId)
    fn onEnd_with_id(&mut self, id: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut()
            .endPattFill(self.parent.mBelonger, id);
        Ok(())
    }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        let importer = self.parent.mTypedDmlImporter.as_ref()?.clone();
        let part = self.parent.mTypedPart.clone();
        let sub_doc_type = self.parent.mSubDocType;
        let belonger = self.parent.mBelonger;
        match id {
            93641380 => { // Java: getBgBranchClrHandler()
                let mut slot = self.mBgBranchClrHandler.borrow_mut();
                if slot.is_none() { *slot = Some(BgBranchClrHandler::new(importer, part, sub_doc_type, belonger).as_handler_ref()); }
                slot.clone()
            }
            97335464 => { // Java: getFgBranchClrHandler()
                let mut slot = self.mFgBranchClrHandler.borrow_mut();
                if slot.is_none() { *slot = Some(FgBranchClrHandler::new(importer, part, sub_doc_type, belonger).as_handler_ref()); }
                slot.clone()
            }
            _ => None,
        }
    }
}
