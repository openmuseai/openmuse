#![allow(non_snake_case, dead_code)]
use std::{any::Any, cell::RefCell, rc::Rc};
use core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet;
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::{DmlImporterRef, DrawingInfoRef};
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
use super::{AnchorHandler::AnchorHandler, DmlHandlerBase::DmlHandlerBase, InlineHandler::InlineHandler};

// Java: public class DrawingHandler extends DmlHandlerBase
pub struct DrawingHandler {
    parent: DmlHandlerBase,
    // Java field: MutablePropertySet mRPrSet
    mRPrSet: RefCell<Option<DrawingInfoRef>>,
    // Java field: AnchorHandler mAnchorHandler
    mAnchorHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: InlineHandler mInlineHandler
    mInlineHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: int mTableLayer
    mTableLayer: i32,
}
impl DrawingHandler {
    // Java: DrawingHandler(IDocumentImporter importer, IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType, int tableLayer)
    pub fn new(importer: Option<Rc<dyn IDocumentImporter>>, dmlImporter: Option<DmlImporterRef>,
               part: Option<Rc<dyn Any>>, subDocType: i32, tableLayer: i32) -> Option<Self> {
        // Java: super(null == importer ? dmlImporter : importer.getDmlImporter(subDocType), part, subDocType);
        let selected = match importer { Some(i) => i.getDmlImporter(subDocType), None => dmlImporter }?;
        Some(Self { parent: DmlHandlerBase::new_typed(selected, part, subDocType),
               mRPrSet: RefCell::new(None), mAnchorHandler: RefCell::new(None),
               mInlineHandler: RefCell::new(None), mTableLayer: tableLayer })
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
    // Java: void setRunProperty(MutablePropertySet rPrSet)
    pub fn setRunProperty(&self, rPrSet: MutablePropertySet) {
        let importer = self.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter should not be null!");
        // Java: this.mRPrSet = rPrSet; importer.setDrawingInfo(this.mRPrSet);
        let shared = Rc::new(RefCell::new(rPrSet));
        *self.mRPrSet.borrow_mut() = Some(shared.clone());
        importer.borrow_mut().setDrawingInfo(shared);
    }
    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, _namespaceURI: &str) -> Option<ElementHandlerRef> {
        let importer = self.parent.mTypedDmlImporter.as_ref()?.clone();
        match elementId {
            // Java: case -1413299531: return this.getAnchorHandler();
            -1413299531 => {
                let mut slot = self.mAnchorHandler.borrow_mut();
                if slot.is_none() { *slot = Some(AnchorHandler::new(importer,
                    self.parent.mTypedPart.clone(), self.parent.mSubDocType, self.mTableLayer).as_handler_ref()); }
                slot.clone()
            }
            // Java: case -1183997287: return this.getInlineHandler();
            -1183997287 => {
                let mut slot = self.mInlineHandler.borrow_mut();
                if slot.is_none() { *slot = Some(InlineHandler::new(importer,
                    self.parent.mTypedPart.clone(), self.parent.mSubDocType, self.mTableLayer).as_handler_ref()); }
                slot.clone()
            }
            _ => None,
        }
    }
}
impl ElementHandlerSimple for DrawingHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { 1913009182 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, _: i32, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Java: importer.finish();
        self.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut().finish(); Ok(())
    }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> { DrawingHandler::getElementHandler(self, id, ns) }
}
