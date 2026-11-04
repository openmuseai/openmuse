#![allow(non_snake_case, dead_code)]
use std::{any::Any, cell::RefCell, rc::Rc};
use core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet;
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::{IDmlImporter::DmlImporterRef, IDocumentImporter::IDocumentImporter};
use super::dml::{ContentPartHandler::ContentPartHandler, DrawingHandler::DrawingHandler,
    PositionHBranchHandler::PositionHBranchHandler, PositionVBranchHandler::PositionVBranchHandler};
use super::{PHandler::PHandler, PictHandler::PictHandler};

// Java: public class ChoiceHandler extends XmlSimpleNodeElementHandler
pub struct ChoiceHandler {
    // Java: inherited mDocImporter and mDmlImporter
    mDocImporter: Rc<dyn IDocumentImporter>,
    mDmlImporter: Option<DmlImporterRef>,
    // Java fields: mPart, mSubDocType, mBelonger, mTableLayer, mRHandlerProp.
    mPart: Option<Rc<dyn Any>>,
    mSubDocType: i32,
    mBelonger: i32,
    mTableLayer: i32,
    mRHandlerProp: RefCell<Option<MutablePropertySet>>,
    // Java child-handler identity is cached across getElementHandler calls.
    mDrawingHandler: RefCell<Option<ElementHandlerRef>>,
    mPictHandler: RefCell<Option<ElementHandlerRef>>,
    mContentPartHandler: RefCell<Option<ElementHandlerRef>>,
    mPHandler: RefCell<Option<ElementHandlerRef>>,
    mPositionHBranchHandler: RefCell<Option<ElementHandlerRef>>,
    mPositionVBranchHandler: RefCell<Option<ElementHandlerRef>>,
}
impl ChoiceHandler {
    // Java: ChoiceHandler(IDocumentImporter importer, IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType, int belonger, int tableLayer)
    pub fn new(importer: Rc<dyn IDocumentImporter>, dml: Option<DmlImporterRef>, part: Option<Rc<dyn Any>>,
        sub_doc_type: i32, belonger: i32, table_layer: i32) -> Self {
        Self { mDocImporter: importer, mDmlImporter: dml, mPart: part, mSubDocType: sub_doc_type,
            mBelonger: belonger, mTableLayer: table_layer, mRHandlerProp: RefCell::new(None),
            mDrawingHandler: RefCell::new(None), mPictHandler: RefCell::new(None),
            mContentPartHandler: RefCell::new(None), mPHandler: RefCell::new(None),
            mPositionHBranchHandler: RefCell::new(None), mPositionVBranchHandler: RefCell::new(None) }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }
    // Java: void setRHandlerProp(MutablePropertySet rHandlerProp)
    pub fn setRHandlerProp(&self, prop: MutablePropertySet) { *self.mRHandlerProp.borrow_mut() = Some(prop); }
    // Java: void clear() clears Choice_R_Handler then mRHandlerProp.
    pub fn clear(&self) { *self.mRHandlerProp.borrow_mut() = None; }
    // Java: private DrawingHandler getDrawingHandler()
    fn getDrawingHandler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mDrawingHandler.borrow_mut();
        if slot.is_none() { *slot = Some(DrawingHandler::new(Some(self.mDocImporter.clone()), self.mDmlImporter.clone(),
            self.mPart.clone(), self.mSubDocType, self.mTableLayer)?.as_handler_ref()); }
        let handler = slot.as_ref()?.clone();
        if let Some(prop) = self.mRHandlerProp.borrow().as_ref() {
            handler.borrow().as_any().downcast_ref::<DrawingHandler>().expect("DrawingHandler").setRunProperty(prop.clone_mps());
        }
        Some(handler)
    }
    // Java: private ElementHandlerSimple getPictHandler()
    fn getPictHandler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mPictHandler.borrow_mut();
        if slot.is_none() { *slot = Some(PictHandler::new(self.mDocImporter.getVmlImporter(self.mSubDocType)?,
            self.mPart.clone(), self.mSubDocType).as_handler_ref()); }
        let handler = slot.as_ref()?.clone();
        if let Some(prop) = self.mRHandlerProp.borrow().as_ref() {
            handler.borrow().as_any().downcast_ref::<PictHandler>().expect("PictHandler").setRunProperty(prop.clone_mps());
        }
        Some(handler)
    }
    // Java: private ElementHandlerSimple getPHandler()
    fn getPHandler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mPHandler.borrow_mut();
        if slot.is_none() { *slot = Some(PHandler::new_shared(self.mPart.clone()?, self.mDocImporter.clone(),
            self.mSubDocType, self.mTableLayer).as_handler_ref()); }
        slot.clone()
    }
    // Java: private ElementHandlerSimple getContentPartHandler()
    fn getContentPartHandler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mContentPartHandler.borrow_mut();
        if slot.is_none() { *slot = Some(ContentPartHandler::new_typed(self.mPart.clone(),
            self.mDmlImporter.clone().or_else(|| self.mDocImporter.getDmlImporter(self.mSubDocType))?,
            self.mSubDocType, self.mBelonger).as_handler_ref()); }
        slot.clone()
    }
    // Java: private ElementHandlerSimple getPositionHBranchHandler()
    fn getPositionHBranchHandler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mPositionHBranchHandler.borrow_mut();
        if slot.is_none() { *slot = Some(PositionHBranchHandler::new(self.mDmlImporter.clone()
            .or_else(|| self.mDocImporter.getDmlImporter(self.mSubDocType))?,
            self.mPart.clone(), self.mSubDocType).as_handler_ref()); }
        slot.clone()
    }
    // Java: private ElementHandlerSimple getPositionVBranchHandler()
    fn getPositionVBranchHandler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mPositionVBranchHandler.borrow_mut();
        if slot.is_none() { *slot = Some(PositionVBranchHandler::new(self.mDmlImporter.clone()
            .or_else(|| self.mDocImporter.getDmlImporter(self.mSubDocType))?,
            self.mPart.clone(), self.mSubDocType).as_handler_ref()); }
        slot.clone()
    }
}
impl ElementHandlerSimple for ChoiceHandler {
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
    // Java: ChoiceHandler.getElementHandler(int elementId, String namespaceURI)
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        match id {
            -1023368385 | 3440682 => self.getPictHandler(),
            -389273588 => self.getContentPartHandler(),
            112 => self.getPHandler(),
            // Java Choice_R_Handler and DocField handlers are separate outstanding parser dependencies.
            114 | 876929721 | 928980672 => None,
            1707117631 => self.getPositionHBranchHandler(),
            1707117645 => self.getPositionVBranchHandler(),
            1913009182 => self.getDrawingHandler(),
            _ => None,
        }
    }
}
