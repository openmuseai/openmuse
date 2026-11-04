#![allow(non_snake_case)]

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IVmlImporter::VmlImporterRef;

// Java: class EGShapeElementsHandler extends XmlSimpleNodeElementHandler
pub struct EGShapeElementsHandler {
    // Java: inherited IVmlImporter mVmlImporter
    mVmlImporter: VmlImporterRef,
    mSelf: RefCell<Option<std::rc::Weak<RefCell<dyn ElementHandlerSimple>>>>,
}

impl EGShapeElementsHandler {
    // Java: EGShapeElementsHandler(IVmlImporter importer)
    pub fn new(importer: VmlImporterRef) -> Self { Self { mVmlImporter: importer, mSelf: RefCell::new(None) } }
    pub fn as_handler_ref(self) -> ElementHandlerRef {
        let concrete = Rc::new(RefCell::new(self));
        let handler: ElementHandlerRef = concrete.clone();
        *concrete.borrow().mSelf.borrow_mut() = Some(Rc::downgrade(&handler));
        handler
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart_with_id(&self, elementId: i32, namespaceURI: &str, _localName: &str, _qualifiedName: &str, attributes: &dyn Any) {
        // Java: this.mVmlImporter.onImportEGShapeElementsStart(elementId, namespaceURI, attributes);
        self.mVmlImporter.borrow_mut().onImportEGShapeElementsStart(elementId, namespaceURI, attributes);
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, namespaceURI: &str) -> Option<ElementHandlerRef> {
        // Java: case -1383228885: case 115029: case 3317767: case 108511772: return this;
        let self_branch = match elementId {
            -1383228885 | 115029 | 3317767 | 108511772 => true,
            // Java: case 102: VML namespace only
            102 => namespaceURI == "urn:schemas-microsoft-com:vml",
            // Java: case 3143043: Office namespace only
            3143043 => namespaceURI == "urn:schemas-microsoft-com:office:office",
            // Java: default: IOAssert.fail(...); return null;
            _ => false,
        };
        if self_branch { self.mSelf.borrow().as_ref().and_then(|weak| weak.upgrade()) } else { None }
    }
}

impl ElementHandlerSimple for EGShapeElementsHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, uri: &str, local: &str, qualified: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { EGShapeElementsHandler::onStart_with_id(self, id, uri, local, qualified, attrs); Ok(()) }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, uri: &str) -> Option<ElementHandlerRef> { EGShapeElementsHandler::getElementHandler(self, id, uri) }
}
