#![allow(non_snake_case, dead_code)]
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;
use super::{ContentPartHandler::ContentPartHandler, GraphicDataChildHandler::GraphicDataChildHandler,
    PicHandler::PicHandler, WgpHandler::WgpHandler, WspHandler::WspHandler};

// Java: public class WPCHandler extends GraphicDataChildHandler
pub struct WPCHandler {
    parent: GraphicDataChildHandler,
    mWspHandler: RefCell<Option<ElementHandlerRef>>,
    mContentPartHandler: RefCell<Option<ElementHandlerRef>>,
    mWgpHandler: RefCell<Option<ElementHandlerRef>>,
    mPicHandler: RefCell<Option<ElementHandlerRef>>,
    mFirstChildIsEnd: RefCell<bool>,
    mTableLayer: i32,
}
impl WPCHandler {
    // Java: WPCHandler(POIXMLDocumentPart part, IDmlImporter dmlImporter, SubDocType subDocType, int tableLayer)
    pub fn new(part: Option<Rc<dyn Any>>, dmlImporter: DmlImporterRef, subDocType: i32, tableLayer: i32) -> Self {
        Self { parent: GraphicDataChildHandler::new_typed_with_belonger(dmlImporter, part, subDocType, 0),
            mWspHandler: RefCell::new(None), mContentPartHandler: RefCell::new(None),
            mWgpHandler: RefCell::new(None), mPicHandler: RefCell::new(None),
            mFirstChildIsEnd: RefCell::new(false), mTableLayer: tableLayer }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
    // Java: private void endFirstChild()
    fn endFirstChild(&self) {
        if !*self.mFirstChildIsEnd.borrow() {
            self.parent.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut().onCanvasEnd();
        }
        *self.mFirstChildIsEnd.borrow_mut() = true;
    }
}
impl ElementHandlerSimple for WPCHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { 117930 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, _: i32, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        // Java: mFirstChildIsEnd = false; importer.startWPC();
        *self.mFirstChildIsEnd.borrow_mut() = false;
        self.parent.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut().startWPC(); Ok(())
    }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Java: endFirstChild(); this.onEnd(..., true);
        self.endFirstChild(); self.parent.onEnd_typed(true); Ok(())
    }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        match id {
            -389273588 => {
                self.endFirstChild();
                let mut slot = self.mContentPartHandler.borrow_mut();
                if slot.is_none() { *slot = Some(ContentPartHandler::new_typed(self.parent.parent.mTypedPart.clone(),
                    self.parent.parent.mTypedDmlImporter.as_ref()?.clone(), self.parent.parent.mSubDocType,
                    self.filterIndex()).as_handler_ref()); }
                slot.clone()
            }
            110986 => {
                self.endFirstChild();
                let mut slot = self.mPicHandler.borrow_mut();
                if slot.is_none() { *slot = Some(PicHandler::new(self.parent.parent.mTypedDmlImporter.as_ref()?.clone(),
                    self.parent.parent.mTypedPart.clone(), self.parent.parent.mSubDocType,
                    self.filterIndex()).as_handler_ref()); }
                slot.clone()
            }
            117664 => {
                self.endFirstChild();
                let mut slot = self.mWgpHandler.borrow_mut();
                if slot.is_none() { *slot = Some(WgpHandler::new(self.parent.parent.mTypedDmlImporter.as_ref()?.clone(),
                    self.parent.parent.mTypedPart.clone(), self.parent.parent.mSubDocType,
                    self.filterIndex(), self.mTableLayer).as_handler_ref()); }
                slot.clone()
            }
            118036 => {
                self.endFirstChild();
                let mut slot = self.mWspHandler.borrow_mut();
                if slot.is_none() { *slot = Some(WspHandler::new(self.parent.parent.mTypedDmlImporter.as_ref()?.clone(),
                    self.parent.parent.mTypedPart.clone(), self.parent.parent.mSubDocType,
                    self.filterIndex()).as_handler_ref()); }
                slot.clone()
            }
            _ => None,
        }
    }
}
