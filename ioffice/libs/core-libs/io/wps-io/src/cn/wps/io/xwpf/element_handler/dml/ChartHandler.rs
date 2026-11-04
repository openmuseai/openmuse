#![allow(non_snake_case)]
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;
use super::GraphicDataChildHandler::GraphicDataChildHandler;
use super::BlipFillHandler::BlipFillHandler;

// Java: public class ChartHandler extends GraphicDataChildHandler
pub struct ChartHandler { parent: GraphicDataChildHandler }

impl ChartHandler {
    // Java: ChartHandler(POIXMLDocumentPart part, IDmlImporter dmlImporter, SubDocType subDocType, int belonger)
    pub fn new(part: Option<Rc<dyn Any>>, dmlImporter: DmlImporterRef, subDocType: i32, belonger: i32) -> Self {
        Self { parent: GraphicDataChildHandler::new_typed_with_belonger(dmlImporter, part, subDocType, belonger) }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }

    // Java: protected ElementHandlerSimple getBlipFillHandler()
    fn getBlipFillHandler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.parent.mTypedBlipFillHandler.borrow_mut();
        if slot.is_none() {
            *slot = Some(BlipFillHandler::new(
                self.parent.parent.mTypedDmlImporter.as_ref()?.clone(),
                self.parent.parent.mTypedPart.clone(), self.parent.parent.mSubDocType,
                94623710,
            ).as_handler_ref());
        }
        slot.clone()
    }
}

impl ElementHandlerSimple for ChartHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, _: i32, _: &str, _: &str, _: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        // Java: if (this.mBelonger != 1727895397) this.onStart(..., false);
        if self.parent.parent.mBelonger != 1727895397 { self.parent.onStart_typed(false); }
        // Java: importer.startChart(attributes, this.mPart, this.mBelonger);
        let part = self.parent.parent.mTypedPart.as_ref().expect("mPart");
        self.parent.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut()
            .startChart(super::DmlHandlerBase::sax_attributes(attrs),
                super::DmlHandlerBase::document_part(part.as_ref()), self.parent.parent.mBelonger);
        Ok(())
    }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Java: if (this.mBelonger != 1727895397) this.onEnd(..., false);
        if self.parent.parent.mBelonger != 1727895397 { self.parent.onEnd_typed(false); }
        Ok(())
    }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        match id {
            -825444332 => self.getBlipFillHandler(),
            3536191 => self.parent.getSPPRHandler_typed(),
            _ => None,
        }
    }
}
