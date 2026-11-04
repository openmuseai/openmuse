#![allow(non_snake_case, dead_code)]
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;
use super::super::TxbxContentHandler::TxbxContentHandler;
use super::DmlHandlerBase::{DmlHandlerBase, sax_attributes};

// Java: public class TxbxHandler extends DmlHandlerBase
pub struct TxbxHandler {
    parent: DmlHandlerBase,
    // Java field: TxbxContentHandler mTxbxContentHandler
    mTxbxContentHandler: RefCell<Option<ElementHandlerRef>>,
}
impl TxbxHandler {
    // Java: TxbxHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType)
    pub fn new(dml: DmlImporterRef, part: Option<Rc<dyn Any>>, sub_doc_type: i32) -> Self {
        Self { parent: DmlHandlerBase::new_typed(dml, part, sub_doc_type), mTxbxContentHandler: RefCell::new(None) }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
    // Java: MAIN_DOCUMENT → TEXTBOX_DOCUMENT, HEADER_DOCUMENT → HEADERTEXTBOX_DOCUMENT.
    fn getCurrentSubDocType(&self) -> i32 {
        match self.parent.mSubDocType { 0 => 5, 2 => 6,
            other => panic!("unexpected textbox parent SubDocType {other}") }
    }
    // Java: private TxbxContentHandler getTxbxContentHandler()
    fn getTxbxContentHandler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mTxbxContentHandler.borrow_mut();
        if slot.is_none() {
            let dml = self.parent.mTypedDmlImporter.as_ref()?;
            let importer = dml.borrow().getDocumentImporterRef()?;
            *slot = Some(TxbxContentHandler::new_shared(self.parent.mTypedPart.clone()?, importer,
                self.getCurrentSubDocType()).as_handler_ref());
        }
        slot.clone()
    }
}
impl ElementHandlerSimple for TxbxHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: importer.startTxbx(elementId, attributes).
    fn onStart_with_id(&mut self, id: i32, _: &str, _: &str, _: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        self.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut().startTxbx(id, sax_attributes(attrs)); Ok(())
    }
    // Java: importer.endTxbx(elementId).
    fn onEnd_with_id(&mut self, id: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut().endTxbx(id); Ok(())
    }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: case 163404095 (w:txbxContent).
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        if id == 163404095 { self.getTxbxContentHandler() } else { None }
    }
}
