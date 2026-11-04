#![allow(non_snake_case)]
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;
use super::DmlHandlerBase::DmlHandlerBase;
use super::QuadBezToBranchHandler::QuadBezToBranchHandler;
use super::MoveToBranchHandler::MoveToBranchHandler;
use super::LineToBranchHandler::LineToBranchHandler;
use super::ArcToBranchHandler::ArcToBranchHandler;
use super::CloseHandler::CloseHandler;
use super::CubicBeztoBranchHandler::CubicBeztoBranchHandler;

// Java: public class PathHandler2 extends DmlHandlerBase
pub struct PathHandler2 {
    parent: DmlHandlerBase,
    mQuadBezToBranchHandler: RefCell<Option<ElementHandlerRef>>,
    mMoveToBranchHandler: RefCell<Option<ElementHandlerRef>>,
    mLineToBranchHandler: RefCell<Option<ElementHandlerRef>>,
    mArcToBranchHandler: RefCell<Option<ElementHandlerRef>>,
    mCloseHandler: RefCell<Option<ElementHandlerRef>>,
    mCubicBeztoBranchHandler: RefCell<Option<ElementHandlerRef>>,
}
impl PathHandler2 {
    // Java: PathHandler2(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType)
    pub fn new(dmlImporter: DmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32) -> Self {
        Self { parent: DmlHandlerBase::new_typed(dmlImporter, part, subDocType),
            mQuadBezToBranchHandler: RefCell::new(None),
            mMoveToBranchHandler: RefCell::new(None),
            mLineToBranchHandler: RefCell::new(None),
            mArcToBranchHandler: RefCell::new(None),
            mCloseHandler: RefCell::new(None),
            mCubicBeztoBranchHandler: RefCell::new(None),
        }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
}
impl ElementHandlerSimple for PathHandler2 {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: importer.startPath2(attributes)
    fn onStart_with_id(&mut self, _: i32, _: &str, _: &str, _: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        self.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut()
            .startPath2(super::DmlHandlerBase::sax_attributes(attrs)); Ok(())
    }
    // Java: importer.endPath2()
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut().endPath2(); Ok(())
    }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: getElementHandler routes each geometry segment to a cached handler.
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        match id {
            -1513004949 => {
                let mut slot = self.mQuadBezToBranchHandler.borrow_mut();
                if slot.is_none() { *slot = Some(QuadBezToBranchHandler::new(self.parent.mTypedDmlImporter.as_ref()?.clone(),
                    self.parent.mTypedPart.clone(), self.parent.mSubDocType).as_handler_ref()); }
                slot.clone()
            }
            -1068263892 => {
                let mut slot = self.mMoveToBranchHandler.borrow_mut();
                if slot.is_none() { *slot = Some(MoveToBranchHandler::new(self.parent.mTypedDmlImporter.as_ref()?.clone(),
                    self.parent.mTypedPart.clone(), self.parent.mSubDocType).as_handler_ref()); }
                slot.clone()
            }
            3325853 => {
                let mut slot = self.mLineToBranchHandler.borrow_mut();
                if slot.is_none() { *slot = Some(LineToBranchHandler::new(self.parent.mTypedDmlImporter.as_ref()?.clone(),
                    self.parent.mTypedPart.clone(), self.parent.mSubDocType).as_handler_ref()); }
                slot.clone()
            }
            93075565 => {
                let mut slot = self.mArcToBranchHandler.borrow_mut();
                if slot.is_none() { *slot = Some(ArcToBranchHandler::new(self.parent.mTypedDmlImporter.as_ref()?.clone(),
                    self.parent.mTypedPart.clone(), self.parent.mSubDocType).as_handler_ref()); }
                slot.clone()
            }
            94756344 => {
                let mut slot = self.mCloseHandler.borrow_mut();
                if slot.is_none() { *slot = Some(CloseHandler::new(self.parent.mTypedDmlImporter.as_ref()?.clone(),
                    self.parent.mTypedPart.clone(), self.parent.mSubDocType).as_handler_ref()); }
                slot.clone()
            }
            1594918984 => {
                let mut slot = self.mCubicBeztoBranchHandler.borrow_mut();
                if slot.is_none() { *slot = Some(CubicBeztoBranchHandler::new(self.parent.mTypedDmlImporter.as_ref()?.clone(),
                    self.parent.mTypedPart.clone(), self.parent.mSubDocType).as_handler_ref()); }
                slot.clone()
            }
            _ => None,
        }
    }
}
