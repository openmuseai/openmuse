#![allow(non_snake_case)]

use std::any::Any;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;

// Java: class SettingsChildHandler extends XmlSimpleNodeElementHandler
pub struct SettingsChildHandler {
    // Java: inherited mDocImporter from XmlSimpleNodeElementHandler.
    mDocImporter: Rc<dyn IDocumentImporter>,
    selfRef: RefCell<Option<Weak<RefCell<dyn ElementHandlerSimple>>>>,
}

impl SettingsChildHandler {
    // Java: SettingsChildHandler(IDocumentImporter importer)
    pub fn new(importer: Rc<dyn IDocumentImporter>) -> Self {
        // Java: super(importer);
        Self { mDocImporter: importer, selfRef: RefCell::new(None) }
    }

    pub fn as_handler_ref(importer: Rc<dyn IDocumentImporter>) -> ElementHandlerRef {
        let handler: ElementHandlerRef = Rc::new(RefCell::new(Self::new(importer)));
        handler.borrow_mut().as_any_mut().downcast_mut::<Self>().unwrap().selfRef.replace(Some(Rc::downgrade(&handler)));
        handler
    }

    // Java: public void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, elementId: i32, _namespaceURI: &str, _localName: &str, _qualifiedName: &str, attributes: &dyn Any) {
        // Java: this.mDocImporter.onImportSettingsChild(elementId, attributes);
        self.mDocImporter.onImportSettingsChild(elementId, attributes);
    }

    // Java: public ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, _elementId: i32, _namespaceURI: &str) -> Option<ElementHandlerRef> {
        // Java: return this;
        self.selfRef.borrow().as_ref().and_then(Weak::upgrade)
    }
}

impl ElementHandlerSimple for SettingsChildHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, ns: &str, local: &str, qualified: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        SettingsChildHandler::onStart(self, id, ns, local, qualified, attrs);
        Ok(())
    }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> { SettingsChildHandler::getElementHandler(self, id, ns) }
}
