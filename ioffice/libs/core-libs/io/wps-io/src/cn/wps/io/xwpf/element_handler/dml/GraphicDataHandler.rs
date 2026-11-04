#![allow(non_snake_case, dead_code)]
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;
use super::{ChartHandler::ChartHandler, ContentPartHandler::ContentPartHandler, DmlHandlerBase::DmlHandlerBase, PicHandler::PicHandler, RelIdsHandler::RelIdsHandler, WgpHandler::WgpHandler, WspHandler::WspHandler, WPCHandler::WPCHandler};

// Java: public class GraphicDataHandler extends DmlHandlerBase
pub struct GraphicDataHandler {
    parent: DmlHandlerBase,
    // Java field: PicHandler mPicHandler
    mPicHandler: RefCell<Option<ElementHandlerRef>>,
    mContentPartHandler: RefCell<Option<ElementHandlerRef>>,
    mRelIdsHandler: RefCell<Option<ElementHandlerRef>>,
    mChartHandler: RefCell<Option<ElementHandlerRef>>,
    mWspHandler: RefCell<Option<ElementHandlerRef>>,
    mWgpHandler: RefCell<Option<ElementHandlerRef>>,
    mWPCHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: int mTableLayer
    mTableLayer: i32,
}
impl GraphicDataHandler {
    // Java: GraphicDataHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType, int belonger, int tableLayer)
    pub fn new(dmlImporter: DmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32, belonger: i32, tableLayer: i32) -> Self {
        Self { parent: DmlHandlerBase::new_typed_with_belonger(dmlImporter, part, subDocType, belonger),
            mPicHandler: RefCell::new(None), mContentPartHandler: RefCell::new(None),
            mRelIdsHandler: RefCell::new(None), mChartHandler: RefCell::new(None),
            mWspHandler: RefCell::new(None), mWgpHandler: RefCell::new(None),
            mWPCHandler: RefCell::new(None), mTableLayer: tableLayer }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, _: &str) -> Option<ElementHandlerRef> {
        match elementId {
            // Java: case 3677 / 118036: return this.getWspHandler();
            3677 | 118036 => {
                let mut slot = self.mWspHandler.borrow_mut();
                if slot.is_none() { *slot = Some(WspHandler::new(
                    self.parent.mTypedDmlImporter.as_ref()?.clone(), self.parent.mTypedPart.clone(),
                    self.parent.mSubDocType, self.filterIndex()).as_handler_ref()); }
                slot.clone()
            }
            // Java: case 117664: return this.getWgpHandler();
            117664 => {
                let mut slot = self.mWgpHandler.borrow_mut();
                if slot.is_none() { *slot = Some(WgpHandler::new(
                    self.parent.mTypedDmlImporter.as_ref()?.clone(), self.parent.mTypedPart.clone(),
                    self.parent.mSubDocType, self.filterIndex(), self.mTableLayer).as_handler_ref()); }
                slot.clone()
            }
            // Java: case 117930: return this.getWPCHandler();
            117930 => {
                let mut slot = self.mWPCHandler.borrow_mut();
                if slot.is_none() { *slot = Some(WPCHandler::new(
                    self.parent.mTypedPart.clone(), self.parent.mTypedDmlImporter.as_ref()?.clone(),
                    self.parent.mSubDocType, self.mTableLayer).as_handler_ref()); }
                slot.clone()
            }
            // Java: case -934677665: return this.getRelIdsHandler();
            -934677665 => {
                let mut slot = self.mRelIdsHandler.borrow_mut();
                if slot.is_none() { *slot = Some(RelIdsHandler::new(
                    self.parent.mTypedPart.clone(), self.parent.mTypedDmlImporter.as_ref()?.clone(),
                    self.parent.mSubDocType, self.parent.mBelonger).as_handler_ref()); }
                slot.clone()
            }
            // Java: case -389273588: return this.getContentPartHandler();
            -389273588 => {
                let mut slot = self.mContentPartHandler.borrow_mut();
                if slot.is_none() { *slot = Some(ContentPartHandler::new_typed(
                    self.parent.mTypedPart.clone(), self.parent.mTypedDmlImporter.as_ref()?.clone(),
                    self.parent.mSubDocType, self.filterIndex()).as_handler_ref()); }
                slot.clone()
            }
            // Java: case 110986: return this.getPicHandler();
            110986 => {
                let mut slot = self.mPicHandler.borrow_mut();
                if slot.is_none() { *slot = Some(PicHandler::new(
                    self.parent.mTypedDmlImporter.as_ref()?.clone(), self.parent.mTypedPart.clone(),
                    self.parent.mSubDocType, self.filterIndex()).as_handler_ref()); }
                slot.clone()
            }
            // Java: case 94623710: return this.getChartHandler();
            94623710 => {
                let mut slot = self.mChartHandler.borrow_mut();
                if slot.is_none() { *slot = Some(ChartHandler::new(
                    self.parent.mTypedPart.clone(), self.parent.mTypedDmlImporter.as_ref()?.clone(),
                    self.parent.mSubDocType, self.parent.mBelonger).as_handler_ref()); }
                slot.clone()
            }
            // Other Java graphicData branches have not yet been brought onto the typed SAX path.
            _ => None,
        }
    }
}
impl ElementHandlerSimple for GraphicDataHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -2022546766 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, _: i32, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> { GraphicDataHandler::getElementHandler(self, id, ns) }
}
