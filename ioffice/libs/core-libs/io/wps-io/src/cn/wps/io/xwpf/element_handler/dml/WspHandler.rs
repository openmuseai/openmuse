#![allow(non_snake_case, dead_code)]
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;
use super::{BlipFillHandler::BlipFillHandler, DmlHandlerBase::sax_attributes, GraphicDataChildHandler::GraphicDataChildHandler, TxbxHandler::TxbxHandler};

// Java: public class WspHandler extends GraphicDataChildHandler
pub struct WspHandler {
    parent: GraphicDataChildHandler,
    // Java field: TxbxHandler mTxbxHandler
    mTxbxHandler: RefCell<Option<ElementHandlerRef>>,
    // Java: protected BlipFillHandler mBlipFillHandler, cached by GraphicDataChildHandler.
}
impl WspHandler {
    // Java: WspHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType, int belonger)
    pub fn new(dmlImporter: DmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32, belonger: i32) -> Self {
        Self { parent: GraphicDataChildHandler::new_typed_with_belonger(dmlImporter, part, subDocType, belonger),
            mTxbxHandler: RefCell::new(None) }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
    pub fn setRootBelonger(&mut self, rootBelonger: i32) { self.parent.parent.mRootBelonger = rootBelonger; }
    // Java: protected ElementHandlerSimple getBlipFillHandler()
    fn getBlipFillHandler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.parent.mTypedBlipFillHandler.borrow_mut();
        if slot.is_none() { *slot = Some(BlipFillHandler::new(
            self.parent.parent.mTypedDmlImporter.as_ref()?.clone(), self.parent.parent.mTypedPart.clone(),
            self.parent.parent.mSubDocType, 118036).as_handler_ref()); }
        slot.clone()
    }
    // Java: private TxbxHandler getTXBXHandler()
    fn getTXBXHandler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mTxbxHandler.borrow_mut();
        if slot.is_none() { *slot = Some(TxbxHandler::new(self.parent.parent.mTypedDmlImporter.as_ref()?.clone(),
            self.parent.parent.mTypedPart.clone(), self.parent.parent.mSubDocType).as_handler_ref()); }
        slot.clone()
    }
}
impl ElementHandlerSimple for WspHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, _: i32, _: &str, _: &str, _: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        // Java: this.onStart(..., false); importer.startWSP(attributes);
        self.parent.onStart_typed(false);
        self.parent.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut().startWSP(sax_attributes(attrs));
        Ok(())
    }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Java: this.onEnd(..., false);
        self.parent.onEnd_typed(false); Ok(())
    }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        match id {
            // Java: case 3574234: return this.getTXBXHandler();
            3574234 => self.getTXBXHandler(),
            // Java: GraphicDataChildHandler.getElementHandler case blipFill / spPr.
            -825444332 => self.getBlipFillHandler(),
            3536191 => self.parent.getSPPRHandler_typed(),
            _ => None,
        }
    }
}
