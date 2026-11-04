#![allow(non_snake_case)]

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IVmlImporter::VmlImporterRef;
use core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet;
use super::vml::NormalShapeHandler::NormalShapeHandler;
use super::vml::GroupHandler::GroupHandler;
use super::vml::InkHandler::InkHandler;
use super::vml::ControlHandler::ControlHandler;
use super::vml::OLEObjectHandler::OLEObjectHandler;
use super::vml::ShapeHandler::ShapeHandler;

// Java: public class PictHandler extends XmlSimpleNodeElementHandler
pub struct PictHandler {
    // Java: private POIXMLDocumentPart mPart;
    mPart: Option<Rc<dyn Any>>,
    // Java: private SubDocType mSubDocType;
    mSubDocType: i32,
    // Java: private MutablePropertySet mRPrSet;
    mRPrSet: RefCell<Option<MutablePropertySet>>,
    // Java: private ShapeHandler mShapeHandler;
    mShapeHandler: RefCell<Option<ElementHandlerRef>>,
    // Java: private NormalShapeHandler mNormalShapeHandler;
    mNormalShapeHandler: RefCell<Option<ElementHandlerRef>>,
    // Java: private GroupHandler mGroupHandler;
    mGroupHandler: RefCell<Option<ElementHandlerRef>>,
    // Java: private InkHandler mInkHandler;
    mInkHandler: RefCell<Option<ElementHandlerRef>>,
    // Java: private ControlHandler mControlHandler;
    mControlHandler: RefCell<Option<ElementHandlerRef>>,
    // Java: private OLEObjectHandler mOLEObjectHandler;
    mOLEObjectHandler: RefCell<Option<ElementHandlerRef>>,
    // Java: inherited IVmlImporter mVmlImporter
    mVmlImporter: VmlImporterRef,
}

impl PictHandler {
    // Java: PictHandler(IVmlImporter vmlImporter, POIXMLDocumentPart part, SubDocType subDocType)
    pub fn new(vmlImporter: VmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32) -> Self {
        // Java: super(vmlImporter); this.mPart = part; this.mSubDocType = subDocType;
        Self { mVmlImporter: vmlImporter, mPart: part, mSubDocType: subDocType,
            mRPrSet: RefCell::new(None), mShapeHandler: RefCell::new(None),
            mNormalShapeHandler: RefCell::new(None), mGroupHandler: RefCell::new(None),
            mOLEObjectHandler: RefCell::new(None), mInkHandler: RefCell::new(None),
            mControlHandler: RefCell::new(None) }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }

    // Java: void setRunProperty(MutablePropertySet rPrSet)
    pub fn setRunProperty(&self, rPrSet: MutablePropertySet) {
        // Java: this.mRPrSet = rPrSet;
        *self.mRPrSet.borrow_mut() = Some(rPrSet);
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart_with_id(&self, _id: i32, _uri: &str, _local: &str, _qualified: &str, _attrs: &dyn Any) {
        // Java: this.mVmlImporter.onImportVmlStart(this.mPart, this.mRPrSet);
        let part = self.mPart.as_ref().map(|p| p.as_ref() as &dyn Any).unwrap_or(&());
        let prop = self.mRPrSet.borrow();
        let prop_any = prop.as_ref().map(|p| p as &dyn Any).unwrap_or(&());
        self.mVmlImporter.borrow_mut().onImportVmlStart(part, prop_any);
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, _namespaceURI: &str) -> Option<ElementHandlerRef> {
        // Java: switch (elementId)
        match elementId {
            // Java: six normal-shape child IDs → getPictChildHandler()
            -1638324645 | -4156302 | 3321844 | 3423314 | 3496420 | 100313435 => self.getPictChildHandler(),
            // Java: case 109399969: return this.getShapeHandler();
            109399969 => self.getShapeHandler(),
            // Java: case 98629247: return this.getGroupHandler();
            98629247 => self.getGroupHandler(),
            // Java: case 1718295399: return this.getOLEObjectHandler();
            1718295399 => self.getOLEObjectHandler(),
            // Java: case 104422: return this.getInkHandler();
            104422 => self.getInkHandler(),
            // Java: case 951543133: return this.getControlHandler();
            951543133 => self.getControlHandler(),
            // Java: case 96850: case 95027439: return null;
            // Remaining Java branches have separate group/control/ink/xml03 handlers.
            _ => None,
        }
    }

    // Java: private ElementHandlerSimple getPictChildHandler()
    fn getPictChildHandler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mNormalShapeHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mNormalShapeHandler = new NormalShapeHandler(this.mPart, this.mVmlImporter, this.mSubDocType);
            *slot = Some(NormalShapeHandler::new(self.mPart.clone(), self.mVmlImporter.clone(), self.mSubDocType).as_handler_ref());
        }
        slot.clone()
    }

    // Java: private ElementHandlerSimple getShapeHandler()
    fn getShapeHandler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mShapeHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mShapeHandler = new ShapeHandler(this.mPart, this.mVmlImporter, this.mSubDocType);
            *slot = Some(ShapeHandler::new(self.mPart.clone(), self.mVmlImporter.clone(), self.mSubDocType).as_handler_ref());
        }
        slot.clone()
    }

    // Java: private ElementHandlerSimple getGroupHandler()
    fn getGroupHandler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mGroupHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mGroupHandler = new GroupHandler(this.mPart, this.mVmlImporter, this.mSubDocType);
            *slot = Some(GroupHandler::new(self.mPart.clone(), self.mVmlImporter.clone(), self.mSubDocType).as_handler_ref());
        }
        slot.clone()
    }

    // Java: private ElementHandlerSimple getOLEObjectHandler()
    fn getOLEObjectHandler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mOLEObjectHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mOLEObjectHandler = new OLEObjectHandler(this.mPart, this.mVmlImporter, this.mSubDocType);
            *slot = Some(OLEObjectHandler::new(self.mPart.clone(), self.mVmlImporter.clone(), self.mSubDocType).as_handler_ref());
        }
        slot.clone()
    }

    // Java: private ElementHandlerSimple getInkHandler()
    fn getInkHandler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mInkHandler.borrow_mut();
        if slot.is_none() { *slot = Some(InkHandler::new(self.mVmlImporter.clone()).as_handler_ref()); }
        slot.clone()
    }

    // Java: private ElementHandlerSimple getControlHandler()
    fn getControlHandler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mControlHandler.borrow_mut();
        if slot.is_none() {
            *slot = Some(ControlHandler::new(self.mPart.clone(), self.mVmlImporter.clone(), self.mSubDocType).as_handler_ref());
        }
        slot.clone()
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd_with_id(&self, _id: i32, _uri: &str) {
        // Java: this.mVmlImporter.onImportVmlEnd();
        self.mVmlImporter.borrow_mut().onImportVmlEnd();
    }
    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 { 3440682 }
}

impl ElementHandlerSimple for PictHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { PictHandler::filterIndex(self) }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, uri: &str, local: &str, qualified: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { PictHandler::onStart_with_id(self, id, uri, local, qualified, attrs); Ok(()) }
    fn onEnd_with_id(&mut self, id: i32, uri: &str) -> Result<(), Box<dyn std::error::Error>> { PictHandler::onEnd_with_id(self, id, uri); Ok(()) }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, uri: &str) -> Option<ElementHandlerRef> { PictHandler::getElementHandler(self, id, uri) }
}
