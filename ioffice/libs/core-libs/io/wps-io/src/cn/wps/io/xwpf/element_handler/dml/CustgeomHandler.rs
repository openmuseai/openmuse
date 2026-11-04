#![allow(non_snake_case)]
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;
use super::{AvlstBranchHandler::AvlstBranchHandler, CxnlstHandler::CxnlstHandler,
    DmlHandlerBase::DmlHandlerBase, GdlstBranchHandler::GdlstBranchHandler,
    PathlstHandler::PathlstHandler};

// Java: public class CustgeomHandler extends DmlHandlerBase
pub struct CustgeomHandler {
    parent: DmlHandlerBase,
    mPathlstHandler: RefCell<Option<ElementHandlerRef>>,
    mAvlstHandler: RefCell<Option<ElementHandlerRef>>,
    mGdlstBranchHandler: RefCell<Option<ElementHandlerRef>>,
    mCxnlstHandler: RefCell<Option<ElementHandlerRef>>,
}
impl CustgeomHandler {
    // Java: CustgeomHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType)
    pub fn new(dmlImporter: DmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32) -> Self {
        Self { parent: DmlHandlerBase::new_typed(dmlImporter, part, subDocType),
            mPathlstHandler: RefCell::new(None), mAvlstHandler: RefCell::new(None),
            mGdlstBranchHandler: RefCell::new(None), mCxnlstHandler: RefCell::new(None) }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
}
impl ElementHandlerSimple for CustgeomHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: onStart is empty.
    fn onStart_with_id(&mut self, _: i32, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: importer.endCustgeom()
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut().endCustgeom(); Ok(())
    }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        let importer = self.parent.mTypedDmlImporter.as_ref()?.clone();
        let part = self.parent.mTypedPart.clone();
        let sub_doc_type = self.parent.mSubDocType;
        match id {
            -1346505100 => { // Java: getCxnlstHandler()
                let mut slot = self.mCxnlstHandler.borrow_mut();
                if slot.is_none() { *slot = Some(CxnlstHandler::new(importer, part, sub_doc_type).as_handler_ref()); }
                slot.clone()
            }
            -791471768 => { // Java: getPathlstHandler()
                let mut slot = self.mPathlstHandler.borrow_mut();
                if slot.is_none() { *slot = Some(PathlstHandler::new(importer, part, sub_doc_type).as_handler_ref()); }
                slot.clone()
            }
            93173592 => { // Java: getAvlstHandler(), belonger -875836993
                let mut slot = self.mAvlstHandler.borrow_mut();
                if slot.is_none() { *slot = Some(AvlstBranchHandler::new(importer, part, sub_doc_type, -875836993).as_handler_ref()); }
                slot.clone()
            }
            98178480 => { // Java: getGdlstBranchHandler(), belonger 604976175
                let mut slot = self.mGdlstBranchHandler.borrow_mut();
                if slot.is_none() { *slot = Some(GdlstBranchHandler::new(importer, part, sub_doc_type, 604976175).as_handler_ref()); }
                slot.clone()
            }
            _ => None,
        }
    }
}
