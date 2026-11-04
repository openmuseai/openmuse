#![allow(non_snake_case, dead_code)]
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;
use super::{AnchorInlineHandler::AnchorInlineHandler, InlineChildHandler::InlineChildHandler};

// Java: public class InlineHandler extends AnchorInlineHandler
pub struct InlineHandler {
    parent: AnchorInlineHandler,
    // Java field: String mTmpContent
    mTmpContent: Option<String>,
    // Java field: InlineChildHandler mInlineChildHandler
    mInlineChildHandler: RefCell<Option<ElementHandlerRef>>,
}
impl InlineHandler {
    // Java: InlineHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType, int tableLayer)
    pub fn new(dmlImporter: DmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32, tableLayer: i32) -> Self {
        Self { parent: AnchorInlineHandler::new(dmlImporter, part, subDocType, tableLayer),
            mTmpContent: None, mInlineChildHandler: RefCell::new(None) }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, namespaceURI: &str) -> Option<ElementHandlerRef> {
        match elementId {
            // Java: case -1289044182: case -781241317: case 95756634:
            -1289044182 | -781241317 | 95756634 => {
                let mut slot = self.mInlineChildHandler.borrow_mut();
                if slot.is_none() { *slot = Some(InlineChildHandler::new(
                    self.parent.parent.mTypedDmlImporter.as_ref()?.clone(),
                    self.parent.parent.mTypedPart.clone(), self.parent.parent.mSubDocType).as_handler_ref()); }
                slot.clone()
            }
            _ => self.parent.getElementHandler(elementId, namespaceURI),
        }
    }
}
impl ElementHandlerSimple for InlineHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, _: &str, _: &str, _: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        // Java: importer.startInline(elementId, attributes);
        self.parent.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut().startInline(id, super::DmlHandlerBase::sax_attributes(attrs)); Ok(())
    }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Java: importer.endInline(); this.mTmpContent = null;
        self.parent.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut().endInline();
        self.mTmpContent = None; Ok(())
    }
    fn onCharacters(&mut self, chars: &[char], start: i32, len: i32) -> Result<(), Box<dyn std::error::Error>> {
        // Java: this.mTmpContent = new String(buffer, start, len);
        if start >= 0 && len >= 0 { self.mTmpContent = Some(chars.iter().skip(start as usize).take(len as usize).collect()); }
        Ok(())
    }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> { InlineHandler::getElementHandler(self, id, ns) }
}
