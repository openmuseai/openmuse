#![allow(non_snake_case, dead_code)]
use std::{any::Any, cell::RefCell, rc::Rc};
use core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet;
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::{IDmlImporter::DmlImporterRef, IDocumentImporter::IDocumentImporter};
use super::ChoiceHandler::ChoiceHandler;

// Java: public class AlternateContentHandler extends XmlSimpleNodeElementHandler
pub struct AlternateContentHandler {
    // Java: inherited mDocImporter and mDmlImporter
    mDocImporter: Rc<dyn IDocumentImporter>,
    mDmlImporter: Option<DmlImporterRef>,
    // Java fields: mPart, mSubDocType, mRHandlerProp, mChoiceHandler, mBelonger, mTableLayer.
    mPart: Option<Rc<dyn Any>>,
    mSubDocType: i32,
    mRHandlerProp: RefCell<Option<MutablePropertySet>>,
    mChoiceHandler: RefCell<Option<ElementHandlerRef>>,
    mBelonger: i32,
    mTableLayer: i32,
    // Java mFallbackHandler is never constructed: getElementHandler(Fallback) returns null.
}
impl AlternateContentHandler {
    // Java: AlternateContentHandler(IDocumentImporter importer, IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType, int belonger, int tableLayer)
    pub fn new(importer: Rc<dyn IDocumentImporter>, dml: Option<DmlImporterRef>, part: Option<Rc<dyn Any>>,
        sub_doc_type: i32, belonger: i32, table_layer: i32) -> Self {
        Self { mDocImporter: importer, mDmlImporter: dml, mPart: part, mSubDocType: sub_doc_type,
            mRHandlerProp: RefCell::new(None), mChoiceHandler: RefCell::new(None),
            mBelonger: belonger, mTableLayer: table_layer }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
    // Java: void setRHandlerProp(MutablePropertySet rHandlerProp)
    pub fn setRHandlerProp(&self, prop: Option<MutablePropertySet>) { *self.mRHandlerProp.borrow_mut() = prop; }
    // Java: private ChoiceHandler getChoiceHandler()
    fn getChoiceHandler(&self) -> ElementHandlerRef {
        let mut slot = self.mChoiceHandler.borrow_mut();
        if slot.is_none() { *slot = Some(ChoiceHandler::new(self.mDocImporter.clone(), self.mDmlImporter.clone(),
            self.mPart.clone(), self.mSubDocType, self.mBelonger, self.mTableLayer).as_handler_ref()); }
        let choice = slot.as_ref().expect("mChoiceHandler").clone();
        if let Some(prop) = self.mRHandlerProp.borrow().as_ref() {
            choice.borrow().as_any().downcast_ref::<ChoiceHandler>().expect("ChoiceHandler").setRHandlerProp(prop.clone_mps());
        }
        choice
    }
}
impl ElementHandlerSimple for AlternateContentHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    // Java: filterIndex() returns 1590178495.
    fn filterIndex(&self) -> i32 { 1590178495 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, _: i32, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: onEnd clears ChoiceHandler and mRHandlerProp.
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(choice) = self.mChoiceHandler.borrow().as_ref() {
            choice.borrow().as_any().downcast_ref::<ChoiceHandler>().expect("ChoiceHandler").clear();
        }
        *self.mRHandlerProp.borrow_mut() = None;
        Ok(())
    }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: getElementHandler returns null for Fallback and ChoiceHandler for Choice.
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        match id { 825887490 => None, 2017610177 => Some(self.getChoiceHandler()), _ => None }
    }
}
