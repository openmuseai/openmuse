#![allow(non_snake_case, dead_code)]
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
use super::vml::TxbxPHandler::TxbxPHandler;

// Java: public class TxbxContentHandler extends ContentBlockContentHandler
pub struct TxbxContentHandler {
    // Java: inherited mPart, mDocImporter, mSubDocType, mTableLayer=0
    mPart: Rc<dyn Any>,
    mDocImporter: Rc<dyn IDocumentImporter>,
    mSubDocType: i32,
    mTableLayer: i32,
    // Java field: TxbxPHandler mTxbxPHandler
    mTxbxPHandler: RefCell<Option<ElementHandlerRef>>,
}
impl TxbxContentHandler {
    // Java: TxbxContentHandler(POIXMLDocumentPart part, IDocumentImporter importer, SubDocType subDocType)
    pub fn new_shared(part: Rc<dyn Any>, importer: Rc<dyn IDocumentImporter>, sub_doc_type: i32) -> Self {
        Self { mPart: part, mDocImporter: importer, mSubDocType: sub_doc_type,
            mTableLayer: 0, mTxbxPHandler: RefCell::new(None) }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
    // Java: private TxbxPHandler getTxbxPHandler()
    fn getTxbxPHandler(&self) -> ElementHandlerRef {
        let mut slot = self.mTxbxPHandler.borrow_mut();
        if slot.is_none() { *slot = Some(TxbxPHandler::new(self.mPart.clone(), self.mDocImporter.clone(),
            self.mSubDocType, self.mTableLayer).as_handler_ref()); }
        slot.as_ref().expect("mTxbxPHandler").clone()
    }
}
impl ElementHandlerSimple for TxbxContentHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    // Java: inherited XmlSimpleNodeElementHandler.filterIndex() returns -1.
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, _: i32, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: TxbxContentHandler.getElementHandler; w:p is TxbxPHandler.
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        match id { 112 => Some(self.getTxbxPHandler()), _ => None }
    }
}
