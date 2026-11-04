#![allow(non_snake_case)]

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
use super::RHandler::RHandler;

// Java: public class DelTextHandler extends XmlSimpleNodeElementHandler
pub struct DelTextHandler {
    // Java: private RHandler mRprHandler;
    mRprHandler: Option<Rc<RefCell<RHandler>>>,
    // Java: private SubDocType mSubDocType;
    mSubDocType: i32,
    // Java: inherited IDocumentImporter mDocImporter
    mDocImporter: Rc<dyn IDocumentImporter>,
}

impl DelTextHandler {
    // Java: DelTextHandler(IDocumentImporter importer, SubDocType subDocType, RHandler rprHandler)
    pub fn new(importer: Rc<dyn IDocumentImporter>, subDocType: i32, rprHandler: Option<Rc<RefCell<RHandler>>>) -> Self {
        // Java: super(importer); this.mRprHandler = rprHandler; this.mSubDocType = subDocType;
        Self { mDocImporter: importer, mSubDocType: subDocType, mRprHandler: rprHandler }
    }

    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }

    // Java: void onCharacters(char[] buffer, int start, int end)
    pub fn onCharacters_impl(&self, buffer: &[char], start: i32, end: i32) {
        // Java: String str = new String(buffer, start, end);
        let str_value: String = buffer.iter().skip(start as usize).take(end as usize).collect();
        // Java: onImportRun(mSubDocType, null == mRprHandler ? null : mRprHandler.getProp(), str);
        let prop = self.mRprHandler.as_ref().map(|r| r.borrow().getProp());
        self.mDocImporter.onImportRun_str(self.mSubDocType, prop.as_ref().map(|p| p as &dyn Any).unwrap_or(&()), &str_value);
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart_with_id(&self, _id: i32, _uri: &str, _local: &str, _qualified: &str, _attributes: &dyn Any) {}

    // Java: int filterIndex() { return 0; }
    pub fn filterIndex(&self) -> i32 { 0 }
}

impl ElementHandlerSimple for DelTextHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { DelTextHandler::filterIndex(self) }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, uri: &str, local: &str, qualified: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { DelTextHandler::onStart_with_id(self, id, uri, local, qualified, attrs); Ok(()) }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, buffer: &[char], start: i32, len: i32) -> Result<(), Box<dyn std::error::Error>> { self.onCharacters_impl(buffer, start, len); Ok(()) }
    fn onCharacters_str(&mut self, s: &str) -> Result<(), Box<dyn std::error::Error>> { let chars: Vec<char> = s.chars().collect(); self.onCharacters_impl(&chars, 0, chars.len() as i32); Ok(()) }
    fn getElementHandler(&self, _: i32, _: &str) -> Option<ElementHandlerRef> { None }
}
