#![allow(non_snake_case, dead_code)]
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;
use super::{BlipFillHandler::BlipFillHandler, GraphicDataChildHandler::GraphicDataChildHandler};

// Java: public class PicHandler extends GraphicDataChildHandler
pub struct PicHandler {
    parent: GraphicDataChildHandler,
}
impl PicHandler {
    // Java: PicHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType, int belonger)
    pub fn new(dmlImporter: DmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32, belonger: i32) -> Self {
        Self { parent: GraphicDataChildHandler::new_typed_with_belonger(dmlImporter, part, subDocType, belonger) }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
    // Java: protected ElementHandlerSimple getBlipFillHandler()
    fn getBlipFillHandler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.parent.mTypedBlipFillHandler.borrow_mut();
        if slot.is_none() { *slot = Some(BlipFillHandler::new(
            self.parent.parent.mTypedDmlImporter.as_ref()?.clone(), self.parent.parent.mTypedPart.clone(),
            self.parent.parent.mSubDocType, 110986).as_handler_ref()); }
        slot.clone()
    }
}
impl ElementHandlerSimple for PicHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        let importer = self.parent.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter");
        // Java: super.onStart(..., false); importer.startPic(elementId);
        self.parent.onStart_typed(false); importer.borrow_mut().startPic(id); Ok(())
    }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Java: super.onEnd(..., false);
        self.parent.onEnd_typed(false); Ok(())
    }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        // Java: GraphicDataChildHandler case -825444332 -> getBlipFillHandler()
        match id {
            -825444332 => self.getBlipFillHandler(),
            // Java: GraphicDataChildHandler case 3536191 -> getSPPRHandler().
            3536191 => self.parent.getSPPRHandler_typed(),
            _ => None,
        }
    }
}
