#![allow(non_snake_case)]
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;
use super::{CxnHandler::CxnHandler, DmlHandlerBase::DmlHandlerBase};

// Java: public class CxnlstHandler extends DmlHandlerBase
pub struct CxnlstHandler {
    parent: DmlHandlerBase,
    mCxnHandler: RefCell<Option<ElementHandlerRef>>,
}
impl CxnlstHandler {
    // Java: CxnlstHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType)
    pub fn new(dmlImporter: DmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32) -> Self {
        Self { parent: DmlHandlerBase::new_typed(dmlImporter, part, subDocType), mCxnHandler: RefCell::new(None) }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
}
impl ElementHandlerSimple for CxnlstHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: importer.startCxnLst(elementId, attributes)
    fn onStart_with_id(&mut self, id: i32, _: &str, _: &str, _: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        self.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut()
            .startCxnLst(id, super::DmlHandlerBase::sax_attributes(attrs)); Ok(())
    }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        if id != 98969 { return None; }
        // Java: getCxnHandler() caches the child.
        let mut slot = self.mCxnHandler.borrow_mut();
        if slot.is_none() { *slot = Some(CxnHandler::new(self.parent.mTypedDmlImporter.as_ref()?.clone(),
            self.parent.mTypedPart.clone(), self.parent.mSubDocType).as_handler_ref()); }
        slot.clone()
    }
}
