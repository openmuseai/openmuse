#![allow(non_snake_case)]

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IVmlImporter::VmlImporterRef;
use super::NormalShapeHandler::NormalShapeHandler;
use super::ShapeHandler::ShapeHandler;

// Java: public class GroupHandler extends NormalShapeHandler
pub struct GroupHandler {
    parent: NormalShapeHandler,
    // Java: private ShapeHandler mShapeHandler;
    mShapeHandler: RefCell<Option<ElementHandlerRef>>,
    // Java: private GroupHandler mGroupHandler;
    mGroupHandler: RefCell<Option<ElementHandlerRef>>,
    // Java: private NormalShapeHandler mNormalShapeHandler;
    mNormalShapeHandler: RefCell<Option<ElementHandlerRef>>,
    mPart: Option<Rc<dyn Any>>,
    mVmlImporter: VmlImporterRef,
    mSubDocType: i32,
}

impl GroupHandler {
    // Java: GroupHandler(POIXMLDocumentPart part, IVmlImporter importer, SubDocType subDocType)
    pub fn new(part: Option<Rc<dyn Any>>, importer: VmlImporterRef, subDocType: i32) -> Self {
        // Java: super(part, importer, subDocType);
        Self { parent: NormalShapeHandler::new(part.clone(), importer.clone(), subDocType),
            mPart: part, mVmlImporter: importer, mSubDocType: subDocType,
            mShapeHandler: RefCell::new(None), mGroupHandler: RefCell::new(None),
            mNormalShapeHandler: RefCell::new(None) }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, id: i32, uri: &str) -> Option<ElementHandlerRef> {
        match id {
            // Java: six normal-shape child IDs
            -1638324645 | -4156302 | 3321844 | 3423314 | 3496420 | 100313435 => self.getNormalShapeHandler(),
            // Java: case 98629247: return this.getGroupHandler();
            98629247 => self.getGroupHandler(),
            // Java: case 109399969: return this.getShapeImporter();
            109399969 => self.getShapeImporter(),
            // Java: case 96850: case 95027439: case 1654878419: return null;
            96850 | 95027439 | 1654878419 => None,
            // Java: default: return super.getElementHandler(elementId, namespaceURI);
            _ => self.parent.getElementHandler(id, uri),
        }
    }
    fn getNormalShapeHandler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mNormalShapeHandler.borrow_mut();
        if slot.is_none() { *slot = Some(NormalShapeHandler::new(self.mPart.clone(), self.mVmlImporter.clone(), self.mSubDocType).as_handler_ref()); }
        slot.clone()
    }
    fn getShapeImporter(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mShapeHandler.borrow_mut();
        if slot.is_none() { *slot = Some(ShapeHandler::new(self.mPart.clone(), self.mVmlImporter.clone(), self.mSubDocType).as_handler_ref()); }
        slot.clone()
    }
    fn getGroupHandler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mGroupHandler.borrow_mut();
        if slot.is_none() { *slot = Some(GroupHandler::new(self.mPart.clone(), self.mVmlImporter.clone(), self.mSubDocType).as_handler_ref()); }
        slot.clone()
    }
}

impl ElementHandlerSimple for GroupHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, uri: &str, local: &str, qualified: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { self.parent.onStart_with_id(id, uri, local, qualified, attrs); Ok(()) }
    fn onEnd_with_id(&mut self, id: i32, uri: &str) -> Result<(), Box<dyn std::error::Error>> { self.parent.onEnd_with_id(id, uri); Ok(()) }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, uri: &str) -> Option<ElementHandlerRef> { GroupHandler::getElementHandler(self, id, uri) }
}
