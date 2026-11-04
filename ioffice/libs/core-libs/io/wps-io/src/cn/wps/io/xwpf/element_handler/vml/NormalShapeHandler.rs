#![allow(non_snake_case)]

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IVmlImporter::VmlImporterRef;
use super::EGShapeElementsHandler::EGShapeElementsHandler;
use super::InkHandler::InkHandler;

// Java: public class NormalShapeHandler extends XmlSimpleNodeElementHandler
pub struct NormalShapeHandler {
    // Java: protected SubDocType mSubDocType;
    pub(crate) mSubDocType: i32,
    // Java: protected POIXMLDocumentPart mPart;
    pub(crate) mPart: Option<Rc<dyn Any>>,
    // Java: private EGShapeElementsHandler mEGShapeElementsHandler;
    mEGShapeElementsHandler: RefCell<Option<ElementHandlerRef>>,
    // Java: private InkHandler mInkHandler;
    mInkHandler: RefCell<Option<ElementHandlerRef>>,
    // Java: inherited IVmlImporter mVmlImporter
    mVmlImporter: VmlImporterRef,
}

impl NormalShapeHandler {
    // Java: NormalShapeHandler(POIXMLDocumentPart part, IVmlImporter importer, SubDocType subDocType)
    pub fn new(part: Option<Rc<dyn Any>>, importer: VmlImporterRef, subDocType: i32) -> Self {
        // Java: super(importer); this.mPart = part; this.mSubDocType = subDocType;
        Self { mPart: part, mVmlImporter: importer, mSubDocType: subDocType,
            mEGShapeElementsHandler: RefCell::new(None), mInkHandler: RefCell::new(None) }
    }

    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart_with_id(&self, elementId: i32, _namespaceURI: &str, _localName: &str, _qualifiedName: &str, attributes: &dyn Any) {
        // Java: this.mVmlImporter.onImportPictChildStart(elementId, attributes);
        self.mVmlImporter.borrow_mut().onImportPictChildStart(elementId, attributes);
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, namespaceURI: &str) -> Option<ElementHandlerRef> {
        // Java: switch (elementId)
        match elementId {
            // Java: EGShapeElementsHandler branches
            -1296358048 | -1002812718 | -891980232 | -877694075 | -233914151 |
            3327275 | 3433509 | 3657802 | 474667981 | 548646960 => self.getEGShapeElementsHandler(),
            // Java: case -903579360: case 3143043: only VML namespace
            -903579360 | 3143043 if namespaceURI == "urn:schemas-microsoft-com:vml" => self.getEGShapeElementsHandler(),
            // Java: case 104422: return this.getInkHandler();
            104422 => self.getInkHandler(),
            // Java: other branches (textbox, ink, xml03 binData) retain their own handlers.
            _ => None,
        }
    }

    // Java: private ElementHandlerSimple getEGShapeElementsHandler()
    fn getEGShapeElementsHandler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mEGShapeElementsHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mEGShapeElementsHandler = new EGShapeElementsHandler(this.mVmlImporter);
            *slot = Some(EGShapeElementsHandler::new(self.mVmlImporter.clone()).as_handler_ref());
        }
        slot.clone()
    }

    // Java: private ElementHandlerSimple getInkHandler()
    fn getInkHandler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mInkHandler.borrow_mut();
        if slot.is_none() { *slot = Some(InkHandler::new(self.mVmlImporter.clone()).as_handler_ref()); }
        slot.clone()
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd_with_id(&self, elementId: i32, _namespaceURI: &str) {
        // Java: this.mVmlImporter.onImportPictChildEnd(elementId);
        self.mVmlImporter.borrow_mut().onImportPictChildEnd(elementId);
    }
}

impl ElementHandlerSimple for NormalShapeHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, uri: &str, local: &str, qualified: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { NormalShapeHandler::onStart_with_id(self, id, uri, local, qualified, attrs); Ok(()) }
    fn onEnd_with_id(&mut self, id: i32, uri: &str) -> Result<(), Box<dyn std::error::Error>> { NormalShapeHandler::onEnd_with_id(self, id, uri); Ok(()) }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, uri: &str) -> Option<ElementHandlerRef> { NormalShapeHandler::getElementHandler(self, id, uri) }
}
