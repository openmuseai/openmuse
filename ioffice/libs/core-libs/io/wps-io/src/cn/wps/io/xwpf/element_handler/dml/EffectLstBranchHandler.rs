#![allow(non_snake_case)]
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;
use super::{
    DmlHandlerBase::DmlHandlerBase, GlowHandler::GlowHandler,
    InnerShdwHandler::InnerShdwHandler, OuterShdwHandler::OuterShdwHandler,
    PrstShdwHandler::PrstShdwHandler, ReflectionHandler::ReflectionHandler,
    SoftEdgeHandler::SoftEdgeHandler,
};

// Java: public class EffectLstBranchHandler extends DmlHandlerBase
pub struct EffectLstBranchHandler {
    parent: DmlHandlerBase,
    mInnerShdwHandler: RefCell<Option<ElementHandlerRef>>,
    mOuterShdwHandler: RefCell<Option<ElementHandlerRef>>,
    mGlowHandler: RefCell<Option<ElementHandlerRef>>,
    mReflectionHandler: RefCell<Option<ElementHandlerRef>>,
    mSoftEdgeHandler: RefCell<Option<ElementHandlerRef>>,
    mPrstShdwHandler: RefCell<Option<ElementHandlerRef>>,
}
impl EffectLstBranchHandler {
    // Java: EffectLstBranchHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType, int belonger)
    pub fn new(dmlImporter: DmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32, belonger: i32) -> Self {
        Self {
            parent: DmlHandlerBase::new_typed_with_belonger(dmlImporter, part, subDocType, belonger),
            mInnerShdwHandler: RefCell::new(None), mOuterShdwHandler: RefCell::new(None),
            mGlowHandler: RefCell::new(None), mReflectionHandler: RefCell::new(None),
            mSoftEdgeHandler: RefCell::new(None), mPrstShdwHandler: RefCell::new(None),
        }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
}
impl ElementHandlerSimple for EffectLstBranchHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, _: i32, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        let importer = self.parent.mTypedDmlImporter.as_ref()?.clone();
        let part = self.parent.mTypedPart.clone();
        let sub_doc_type = self.parent.mSubDocType;
        match id {
            -1366299605 => { // Java: getReflectionHandler()
                let mut slot = self.mReflectionHandler.borrow_mut();
                if slot.is_none() { *slot = Some(ReflectionHandler::new(importer).as_handler_ref()); }
                slot.clone()
            }
            -875476949 => { // Java: getPrstShdwHandler()
                let mut slot = self.mPrstShdwHandler.borrow_mut();
                if slot.is_none() { *slot = Some(PrstShdwHandler::new(importer, part, sub_doc_type).as_handler_ref()); }
                slot.clone()
            }
            -527990498 => { // Java: getInnerShdwHandler()
                let mut slot = self.mInnerShdwHandler.borrow_mut();
                if slot.is_none() { *slot = Some(InnerShdwHandler::new(importer, part, sub_doc_type).as_handler_ref()); }
                slot.clone()
            }
            3027047 | 122240333 => None, // Java: return null
            3175821 => { // Java: getGlowHandler()
                let mut slot = self.mGlowHandler.borrow_mut();
                if slot.is_none() { *slot = Some(GlowHandler::new(importer).as_handler_ref()); }
                slot.clone()
            }
            1317843207 => { // Java: getSoftEdgeHandler()
                let mut slot = self.mSoftEdgeHandler.borrow_mut();
                if slot.is_none() { *slot = Some(SoftEdgeHandler::new(importer, part, sub_doc_type).as_handler_ref()); }
                slot.clone()
            }
            1857009859 => { // Java: getOuterShdwHandler()
                let mut slot = self.mOuterShdwHandler.borrow_mut();
                if slot.is_none() { *slot = Some(OuterShdwHandler::new(importer, part, sub_doc_type).as_handler_ref()); }
                slot.clone()
            }
            _ => None,
        }
    }
}
