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

// Java: public class EmptyHandler extends XmlSimpleNodeElementHandler
pub struct EmptyHandler {
    // Java: private SubDocType mSubDocType;
    mSubDocType: i32,
    // Java: private RHandler mRHandler;
    mRHandler: Option<Rc<RefCell<RHandler>>>,
    // Java: inherited mDocImporter
    mDocImporter: Rc<dyn IDocumentImporter>,
}

impl EmptyHandler {
    // Java: EmptyHandler(IDocumentImporter importer, SubDocType subDocType, RHandler rHandler)
    pub fn new(importer: Rc<dyn IDocumentImporter>, subDocType: i32, rHandler: Option<Rc<RefCell<RHandler>>>) -> Self {
        // Java: super(importer);
        // Java: this.mSubDocType = subDocType;
        // Java: this.mRHandler = rHandler;
        Self { mDocImporter: importer, mSubDocType: subDocType, mRHandler: rHandler }
    }

    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart_with_id(&self, _elementId: i32, _namespaceURI: &str, _localName: &str, _qualifiedName: &str, _attributes: &dyn Any) {
        // Java method body is empty.
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd_with_id(&self, elementId: i32, _namespaceURI: &str) {
        // Java: String str = null; switch (elementId) ...
        let text = match elementId {
            // Java: case -462474756: str = "\u001f";
            -462474756 => "\u{001f}",
            // Java: case 3183: str = "\r";
            3183 => "\r",
            // Java: case 114581: str = "\t";
            114581 => "\t",
            // Java: case 2001013392: str = "\u001e";
            2001013392 => "\u{001e}",
            // Java: default: IOAssert.fail(); return;
            _ => return,
        };
        // Java: this.mDocImporter.onImportRun(this.mSubDocType,
        // Java:     null == this.mRHandler ? null : this.mRHandler.getProp(), str);
        let prop = self.mRHandler.as_ref().map(|handler| handler.borrow().getProp());
        match prop.as_ref() {
            Some(prop) => self.mDocImporter.onImportRun_str(self.mSubDocType, prop as &dyn Any, text),
            None => self.mDocImporter.onImportRun_str(self.mSubDocType, &() as &dyn Any, text),
        }
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 0;
        0
    }
}

impl ElementHandlerSimple for EmptyHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { EmptyHandler::filterIndex(self) }
    fn onStart(&mut self, _uri: &str, _local: &str, _qualified: &str, _attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _uri: &str, _local: &str, _qualified: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, uri: &str, local: &str, qualified: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        EmptyHandler::onStart_with_id(self, id, uri, local, qualified, attrs);
        Ok(())
    }
    fn onEnd_with_id(&mut self, id: i32, uri: &str) -> Result<(), Box<dyn std::error::Error>> {
        EmptyHandler::onEnd_with_id(self, id, uri);
        Ok(())
    }
    fn onCharacters(&mut self, _buffer: &[char], _start: i32, _len: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _text: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, _id: i32, _uri: &str) -> Option<ElementHandlerRef> { None }
}
