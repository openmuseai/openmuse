#![allow(non_snake_case, dead_code)]
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::ElementHandlerRef;
use crate::io::xwpf::IDmlImporter::DmlImporterRef;
use super::{DmlHandlerBase::DmlHandlerBase, GraphicHandler::GraphicHandler,
    SizeRelHBranchHandler::SizeRelHBranchHandler, SizeRelVBranchHandler::SizeRelVBranchHandler};

// Java: class AnchorInlineHandler extends DmlHandlerBase
pub struct AnchorInlineHandler {
    pub(crate) parent: DmlHandlerBase,
    // Java field: GraphicHandler mGraphicHandler
    mGraphicHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: SizeRelHBranchHandler mSizeRelHHandler
    mSizeRelHHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: SizeRelVBranchHandler mSizeRelVHandler
    mSizeRelVHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: int mTableLayer
    pub(crate) mTableLayer: i32,
}
impl AnchorInlineHandler {
    // Java: AnchorInlineHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType, int tableLayer)
    pub fn new(dmlImporter: DmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32, tableLayer: i32) -> Self {
        Self { parent: DmlHandlerBase::new_typed(dmlImporter, part, subDocType),
            mGraphicHandler: RefCell::new(None), mSizeRelHHandler: RefCell::new(None),
            mSizeRelVHandler: RefCell::new(None), mTableLayer: tableLayer }
    }
    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, _namespaceURI: &str) -> Option<ElementHandlerRef> {
        match elementId {
            // Java: case 280343272: return this.getGraphicHandler();
            280343272 => {
                let mut slot = self.mGraphicHandler.borrow_mut();
                if slot.is_none() { *slot = Some(GraphicHandler::new(
                    self.parent.mTypedDmlImporter.as_ref()?.clone(), self.parent.mTypedPart.clone(),
                    self.parent.mSubDocType, 0, self.mTableLayer).as_handler_ref()); }
                slot.clone()
            }
            // Java: case 846906992: return this.getSizeRelHHandler();
            846906992 => {
                let mut slot = self.mSizeRelHHandler.borrow_mut();
                if slot.is_none() { *slot = Some(SizeRelHBranchHandler::new(
                    self.parent.mTypedDmlImporter.as_ref()?.clone(), self.parent.mTypedPart.clone(),
                    self.parent.mSubDocType).as_handler_ref()); }
                slot.clone()
            }
            // Java: case 846907006: return this.getSizeRelVHandler();
            846907006 => {
                let mut slot = self.mSizeRelVHandler.borrow_mut();
                if slot.is_none() { *slot = Some(SizeRelVBranchHandler::new(
                    self.parent.mTypedDmlImporter.as_ref()?.clone(), self.parent.mTypedPart.clone(),
                    self.parent.mSubDocType).as_handler_ref()); }
                slot.clone()
            }
            _ => None,
        }
    }
}
