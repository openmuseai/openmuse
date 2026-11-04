#![allow(non_snake_case, dead_code)]
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;
use super::{ContentPartHandler::ContentPartHandler, GrpSpprHandler::GrpSpprHandler,
    GraphicDataChildHandler::GraphicDataChildHandler, PicHandler::PicHandler, WspHandler::WspHandler};

// Java: public class WgpHandler extends GraphicDataChildHandler
pub struct WgpHandler {
    parent: GraphicDataChildHandler,
    mWspHandler: RefCell<Option<ElementHandlerRef>>,
    mContentPartHandler: RefCell<Option<ElementHandlerRef>>,
    mGrpSpHandler: RefCell<Option<ElementHandlerRef>>,
    mPicHandler: RefCell<Option<ElementHandlerRef>>,
    mGrpSpprHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: int mTableLayer
    mTableLayer: i32,
}
impl WgpHandler {
    // Java: WgpHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType, int belonger, int tableLayer)
    pub fn new(dmlImporter: DmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32, belonger: i32, tableLayer: i32) -> Self {
        Self { parent: GraphicDataChildHandler::new_typed_with_belonger(dmlImporter, part, subDocType, belonger),
            mWspHandler: RefCell::new(None), mContentPartHandler: RefCell::new(None),
            mGrpSpHandler: RefCell::new(None), mPicHandler: RefCell::new(None),
            mGrpSpprHandler: RefCell::new(None), mTableLayer: tableLayer }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
    pub fn setRootBelonger(&mut self, rootBelonger: i32) { self.parent.parent.mRootBelonger = rootBelonger; }
}
impl ElementHandlerSimple for WgpHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { 117664 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, _: i32, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        // Java: this.onStart(..., true);
        self.parent.onStart_typed(true); Ok(())
    }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Java: this.onEnd(..., true);
        self.parent.onEnd_typed(true); Ok(())
    }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        match id {
            // Java: case -389273588: return getContentPartHandler();
            -389273588 => {
                let mut slot = self.mContentPartHandler.borrow_mut();
                if slot.is_none() { *slot = Some(ContentPartHandler::new_typed(
                    self.parent.parent.mTypedPart.clone(), self.parent.parent.mTypedDmlImporter.as_ref()?.clone(),
                    self.parent.parent.mSubDocType, self.filterIndex()).as_handler_ref()); }
                slot.clone()
            }
            // Java: case 3677 / 118036: return getWspHandler();
            3677 | 118036 => {
                let mut slot = self.mWspHandler.borrow_mut();
                if slot.is_none() {
                    let mut child = WspHandler::new(self.parent.parent.mTypedDmlImporter.as_ref()?.clone(),
                        self.parent.parent.mTypedPart.clone(), self.parent.parent.mSubDocType, self.filterIndex());
                    child.setRootBelonger(self.parent.parent.mRootBelonger);
                    *slot = Some(child.as_handler_ref());
                }
                slot.clone()
            }
            // Java: case 110986: return getPicHandler();
            110986 => {
                let mut slot = self.mPicHandler.borrow_mut();
                if slot.is_none() { *slot = Some(PicHandler::new(self.parent.parent.mTypedDmlImporter.as_ref()?.clone(),
                    self.parent.parent.mTypedPart.clone(), self.parent.parent.mSubDocType, self.filterIndex()).as_handler_ref()); }
                slot.clone()
            }
            // Java: case 98629154: return getGrpSpHandler();
            98629154 => {
                let mut slot = self.mGrpSpHandler.borrow_mut();
                if slot.is_none() {
                    let mut child = WgpHandler::new(self.parent.parent.mTypedDmlImporter.as_ref()?.clone(),
                        self.parent.parent.mTypedPart.clone(), self.parent.parent.mSubDocType, self.filterIndex(), self.mTableLayer);
                    child.setRootBelonger(self.parent.parent.mRootBelonger);
                    *slot = Some(child.as_handler_ref());
                }
                slot.clone()
            }
            // Java: case 293339076: return getGrpSpprHandler();
            293339076 => {
                let mut slot = self.mGrpSpprHandler.borrow_mut();
                if slot.is_none() { *slot = Some(GrpSpprHandler::new(
                    self.parent.parent.mTypedDmlImporter.as_ref()?.clone(),
                    self.parent.parent.mTypedPart.clone(), self.parent.parent.mSubDocType).as_handler_ref()); }
                slot.clone()
            }
            _ => None,
        }
    }
}
