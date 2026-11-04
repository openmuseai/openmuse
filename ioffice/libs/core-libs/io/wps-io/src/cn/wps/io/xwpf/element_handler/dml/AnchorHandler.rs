#![allow(non_snake_case, dead_code)]
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;
use super::{AnchorChildHandler::AnchorChildHandler, AnchorInlineHandler::AnchorInlineHandler,
    PositionHBranchHandler::PositionHBranchHandler, PositionVBranchHandler::PositionVBranchHandler};

// Java: public class AnchorHandler extends AnchorInlineHandler
pub struct AnchorHandler {
    parent: AnchorInlineHandler,
    // Java field: PositionHBranchHandler mPositionHBranchHandler
    mPositionHBranchHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: PositionVBranchHandler mPositionVBranchHandler
    mPositionVBranchHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: AnchorChildHandler mAnchorChildHandler
    mAnchorChildHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: AlternateContentHandler mAlternateContentHandler
    mAlternateContentHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: String mTmpContent
    mTmpContent: Option<String>,
}
impl AnchorHandler {
    // Java: AnchorHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType, int tableLayer)
    pub fn new(dmlImporter: DmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32, tableLayer: i32) -> Self {
        Self { parent: AnchorInlineHandler::new(dmlImporter, part, subDocType, tableLayer),
            mPositionHBranchHandler: RefCell::new(None), mPositionVBranchHandler: RefCell::new(None),
            mAnchorChildHandler: RefCell::new(None), mAlternateContentHandler: RefCell::new(None), mTmpContent: None }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, namespaceURI: &str) -> Option<ElementHandlerRef> {
        match elementId {
            // Java: cases routed to AnchorChildHandler
            -2124393662 | -2079867230 | -1447414265 | -1292678373 | -1289044182 |
            -781241317 | -46018828 | 95756634 | 729290359 => {
                let mut slot = self.mAnchorChildHandler.borrow_mut();
                if slot.is_none() { *slot = Some(AnchorChildHandler::new(
                    self.parent.parent.mTypedDmlImporter.as_ref()?.clone(),
                    self.parent.parent.mTypedPart.clone(), self.parent.parent.mSubDocType).as_handler_ref()); }
                slot.clone()
            }
            // Java: case 1590178495: return this.getAlternateContentHandler();
            1590178495 => self.mAlternateContentHandler.borrow().clone(),
            // Java: case 1707117631: return this.getPositionHBranchHandler();
            1707117631 => {
                let mut slot = self.mPositionHBranchHandler.borrow_mut();
                if slot.is_none() { *slot = Some(PositionHBranchHandler::new(
                    self.parent.parent.mTypedDmlImporter.as_ref()?.clone(),
                    self.parent.parent.mTypedPart.clone(), self.parent.parent.mSubDocType).as_handler_ref()); }
                slot.clone()
            }
            // Java: case 1707117645: return this.getPositionVBranchHandler();
            1707117645 => {
                let mut slot = self.mPositionVBranchHandler.borrow_mut();
                if slot.is_none() { *slot = Some(PositionVBranchHandler::new(
                    self.parent.parent.mTypedDmlImporter.as_ref()?.clone(),
                    self.parent.parent.mTypedPart.clone(), self.parent.parent.mSubDocType).as_handler_ref()); }
                slot.clone()
            }
            _ => self.parent.getElementHandler(elementId, namespaceURI),
        }
    }
}
impl ElementHandlerSimple for AnchorHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, _: &str, _: &str, _: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        // Java: importer.startAnchor(elementId, attributes);
        self.parent.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut().startAnchor(id, super::DmlHandlerBase::sax_attributes(attrs)); Ok(())
    }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Java: importer.endAnchor(); this.mTmpContent = null;
        self.parent.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut().endAnchor();
        self.mTmpContent = None; Ok(())
    }
    fn onCharacters(&mut self, chars: &[char], start: i32, len: i32) -> Result<(), Box<dyn std::error::Error>> {
        // Java: this.mTmpContent = new String(buffer, start, len);
        if start >= 0 && len >= 0 { self.mTmpContent = Some(chars.iter().skip(start as usize).take(len as usize).collect()); }
        Ok(())
    }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> { AnchorHandler::getElementHandler(self, id, ns) }
}
