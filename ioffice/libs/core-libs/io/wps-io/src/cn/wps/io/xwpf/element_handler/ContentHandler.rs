#![allow(non_snake_case)]

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.io.simple.ElementHandlerSimple
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use super::RHandler::RHandler;

// Java: public class ContentHandler extends XmlSimpleNodeElementHandler
pub struct ContentHandler {
    // Java: private RHandler mRprHandler;
    mRprHandler: Option<Rc<RefCell<RHandler>>>,
    // Java: private SubDocType mSubDocType;
    mSubDocType: i32,
    // Java: inherited mDocImporter
    mDocImporter: Rc<dyn IDocumentImporter>,
}

impl ContentHandler {
    // Java: ContentHandler(IDocumentImporter importer, SubDocType subDocType, RHandler rprHandler)
    pub fn new(importer: Rc<dyn IDocumentImporter>, subDocType: i32, rprHandler: Option<Rc<RefCell<RHandler>>>) -> Self {
        // Java: super(importer); this.mRprHandler = rprHandler; this.mSubDocType = subDocType;
        Self { mDocImporter: importer, mSubDocType: subDocType, mRprHandler: rprHandler }
    }

    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart_with_id(&self, elementId: i32, _namespaceURI: &str, _localName: &str, _qualifiedName: &str, attributes: &dyn Any) {
        // Java: this.mDocImporter.onImportRunContent(this.mSubDocType, elementId,
        // Java:     null == this.mRprHandler ? null : this.mRprHandler.getProp(), attributes);
        let prop = self.mRprHandler.as_ref().map(|handler| handler.borrow().getProp());
        match prop.as_ref() {
            Some(prop) => self.mDocImporter.onImportRunContent(self.mSubDocType, elementId, prop as &dyn Any, attributes),
            None => self.mDocImporter.onImportRunContent(self.mSubDocType, elementId, &() as &dyn Any, attributes),
        }
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 { 0 }
}

impl ElementHandlerSimple for ContentHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { ContentHandler::filterIndex(self) }
    fn onStart(&mut self, _uri: &str, _local: &str, _qualified: &str, _attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _uri: &str, _local: &str, _qualified: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, uri: &str, local: &str, qualified: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        ContentHandler::onStart_with_id(self, id, uri, local, qualified, attrs);
        Ok(())
    }
    fn onEnd_with_id(&mut self, _id: i32, _uri: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, _buffer: &[char], _start: i32, _len: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _text: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, _id: i32, _uri: &str) -> Option<ElementHandlerRef> { None }
}
