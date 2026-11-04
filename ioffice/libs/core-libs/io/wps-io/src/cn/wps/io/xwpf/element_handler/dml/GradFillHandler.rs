#![allow(non_snake_case)]
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::{IDmlCommonImporter::IDmlCommonImporter, IDmlImporter::DmlImporterRef};
use super::{GsLstHandler::GsLstHandler, LinHandler::LinHandler, PathBranchHandler::PathBranchHandler, TileRectHandler::TileRectHandler};

// Java: public class GradFillHandler extends XmlSimpleNodeElementHandler
pub struct GradFillHandler {
    mDmlImporter: DmlImporterRef,
    mBelonger: i32,
    mGsLstHandler: RefCell<Option<ElementHandlerRef>>,
    mLinHandler: RefCell<Option<ElementHandlerRef>>,
    mPathBranchHandler: RefCell<Option<ElementHandlerRef>>,
    mTileRectHandler: RefCell<Option<ElementHandlerRef>>,
}
impl GradFillHandler {
    // Java: GradFillHandler(IDmlCommonImporter dmlImporter, int belonger)
    pub fn new(dmlImporter: DmlImporterRef, belonger: i32) -> Self {
        Self { mDmlImporter: dmlImporter, mBelonger: belonger,
            mGsLstHandler: RefCell::new(None), mLinHandler: RefCell::new(None),
            mPathBranchHandler: RefCell::new(None), mTileRectHandler: RefCell::new(None) }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
}
impl ElementHandlerSimple for GradFillHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: this.mDmlImporter.startGradFill(this.mBelonger, elementId, attributes)
    fn onStart_with_id(&mut self, id: i32, _: &str, _: &str, _: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        self.mDmlImporter.borrow_mut().startGradFill(self.mBelonger, id, attrs); Ok(())
    }
    // Java: this.mDmlImporter.endGradFill(this.mBelonger)
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.mDmlImporter.borrow_mut().endGradFill(self.mBelonger); Ok(())
    }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        match id {
            -2106058318 => { // Java: getTileRectHandler()
                let mut slot = self.mTileRectHandler.borrow_mut();
                if slot.is_none() { *slot = Some(TileRectHandler::new(self.mDmlImporter.clone(), self.mBelonger).as_handler_ref()); }
                slot.clone()
            }
            107153 => { // Java: getLinHandler()
                let mut slot = self.mLinHandler.borrow_mut();
                if slot.is_none() { *slot = Some(LinHandler::new(self.mDmlImporter.clone(), self.mBelonger).as_handler_ref()); }
                slot.clone()
            }
            3433509 => { // Java: getPathBranchHandler()
                let mut slot = self.mPathBranchHandler.borrow_mut();
                if slot.is_none() { *slot = Some(PathBranchHandler::new(self.mDmlImporter.clone(), self.mBelonger).as_handler_ref()); }
                slot.clone()
            }
            98625345 => { // Java: getGsLstHandler()
                let mut slot = self.mGsLstHandler.borrow_mut();
                if slot.is_none() { *slot = Some(GsLstHandler::new(self.mDmlImporter.clone(), self.mBelonger).as_handler_ref()); }
                slot.clone()
            }
            _ => None,
        }
    }
}
