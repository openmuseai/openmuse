#![allow(non_snake_case)]

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
use crate::io::xwpf::usermodel::XWPFSettings::XWPFSettings;
use super::SettingsChildHandler::SettingsChildHandler;

// Java: public class SettingsHandler extends XmlSimpleNodeElementHandler
pub struct SettingsHandler {
    // Java: private XWPFSettings mXWPFSettings;
    mXWPFSettings: *const XWPFSettings,
    // Java: private SettingsChildHandler mSettingsSimpleChildHandler;
    mSettingsSimpleChildHandler: RefCell<Option<ElementHandlerRef>>,
    // Java: inherited mDocImporter from XmlSimpleNodeElementHandler.
    mDocImporter: Rc<dyn IDocumentImporter>,
}

impl SettingsHandler {
    // Java: SettingsHandler(IDocumentImporter importer, XWPFSettings xwpfSettings)
    pub fn new(importer: Rc<dyn IDocumentImporter>, xwpfSettings: *const XWPFSettings) -> Self {
        // Java: super(importer);
        // Java: this.mXWPFSettings = xwpfSettings;
        Self { mDocImporter: importer, mXWPFSettings: xwpfSettings, mSettingsSimpleChildHandler: RefCell::new(None) }
    }

    pub fn as_handler_ref(importer: Rc<dyn IDocumentImporter>, xwpfSettings: *const XWPFSettings) -> ElementHandlerRef {
        Rc::new(RefCell::new(Self::new(importer, xwpfSettings)))
    }

    // Java: public void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, _elementId: i32, _namespaceURI: &str, _localName: &str, _qualifiedName: &str, _attributes: &dyn Any) {
        // Java: this.mDocImporter.onImportSettingsStart(this.mXWPFSettings);
        if let Some(settings) = unsafe { self.mXWPFSettings.as_ref() } {
            self.mDocImporter.onImportSettingsStart(settings);
        }
    }

    // Java: public ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, _namespaceURI: &str) -> Option<ElementHandlerRef> {
        // Java: return 108811479 == elementId ? null : this.getSettingsChildHandler();
        if elementId == 108811479 { None } else { Some(self.getSettingsChildHandler()) }
    }

    // Java: private ElementHandlerSimple getSettingsChildHandler()
    fn getSettingsChildHandler(&self) -> ElementHandlerRef {
        // Java: if (null == this.mSettingsSimpleChildHandler)
        if self.mSettingsSimpleChildHandler.borrow().is_none() {
            // Java: this.mSettingsSimpleChildHandler = new SettingsChildHandler(this.mDocImporter);
            *self.mSettingsSimpleChildHandler.borrow_mut() = Some(SettingsChildHandler::as_handler_ref(self.mDocImporter.clone()));
        }
        // Java: return this.mSettingsSimpleChildHandler;
        self.mSettingsSimpleChildHandler.borrow().as_ref().unwrap().clone()
    }

    // Java: public int filterIndex()
    pub fn filterIndex(&self) -> i32 { 1434631203 }

    // Java: public void onEnd(int elementId, String namespaceURI)
    pub fn onEnd(&self, _elementId: i32, _namespaceURI: &str) {
        // Java: this.mDocImporter.onImportSettingsEnd();
        self.mDocImporter.onImportSettingsEnd();
    }
}

impl ElementHandlerSimple for SettingsHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { SettingsHandler::filterIndex(self) }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, ns: &str, local: &str, qualified: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        SettingsHandler::onStart(self, id, ns, local, qualified, attrs);
        Ok(())
    }
    fn onEnd_with_id(&mut self, id: i32, ns: &str) -> Result<(), Box<dyn std::error::Error>> {
        SettingsHandler::onEnd(self, id, ns);
        Ok(())
    }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> { SettingsHandler::getElementHandler(self, id, ns) }
}
