#![allow(non_snake_case, dead_code)]
use std::{any::Any, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
use super::super::PHandler::PHandler;

// Java: public class TxbxPHandler extends PHandler
pub struct TxbxPHandler { parent: PHandler }
impl TxbxPHandler {
    // Java: TxbxPHandler(POIXMLDocumentPart part, IDocumentImporter importer, SubDocType subDocType, int tableLevel)
    pub fn new(part: Rc<dyn Any>, importer: Rc<dyn IDocumentImporter>, sub_doc_type: i32, table_level: i32) -> Self {
        Self { parent: PHandler::new_shared(part, importer, sub_doc_type, table_level) }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(std::cell::RefCell::new(self)) }
}
impl ElementHandlerSimple for TxbxPHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { ElementHandlerSimple::filterIndex(&self.parent) }
    fn onStart(&mut self, a: &str, b: &str, c: &str, d: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { ElementHandlerSimple::onStart(&mut self.parent, a,b,c,d) }
    fn onEnd(&mut self, a: &str, b: &str, c: &str) -> Result<(), Box<dyn std::error::Error>> { ElementHandlerSimple::onEnd(&mut self.parent,a,b,c) }
    fn onStart_with_id(&mut self, id: i32, a: &str, b: &str, c: &str, d: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { ElementHandlerSimple::onStart_with_id(&mut self.parent,id,a,b,c,d) }
    fn onEnd_with_id(&mut self, id: i32, ns: &str) -> Result<(), Box<dyn std::error::Error>> { ElementHandlerSimple::onEnd_with_id(&mut self.parent,id,ns) }
    fn onCharacters(&mut self, a: &[char], b: i32, c: i32) -> Result<(), Box<dyn std::error::Error>> { ElementHandlerSimple::onCharacters(&mut self.parent,a,b,c) }
    fn onCharacters_str(&mut self, a: &str) -> Result<(), Box<dyn std::error::Error>> { ElementHandlerSimple::onCharacters_str(&mut self.parent,a) }
    // Java: only these run/paragraph children are delegated to PHandler.
    fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> {
        match id {
            -1082293607 | -1068263892 | -768354363 | -688961888 | -600772335 | -104827109 |
            114 | 99339 | 104430 | 110226 | 113731 | 99368034 | 104901655 | 358940876 |
            600796080 | 751294566 | 1452770417 | 1466661092 | 1590178495 | 1601478807 | 1695350954 =>
                ElementHandlerSimple::getElementHandler(&self.parent, id, ns),
            _ => None,
        }
    }
}
