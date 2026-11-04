#![allow(non_snake_case)]
use std::{any::Any, cell::RefCell, rc::{Rc, Weak}};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;
use super::DmlHandlerBase::DmlHandlerBase;

// Java: public class GdlstBranchHandler extends DmlHandlerBase
pub struct GdlstBranchHandler {
    parent: DmlHandlerBase,
    mSelf: Weak<RefCell<GdlstBranchHandler>>,
}
impl GdlstBranchHandler {
    // Java: GdlstBranchHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType, int belonger)
    pub fn new(dmlImporter: DmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32, belonger: i32) -> Self {
        Self { parent: DmlHandlerBase::new_typed_with_belonger(dmlImporter, part, subDocType, belonger), mSelf: Weak::new() }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef {
        let handler = Rc::new(RefCell::new(self));
        handler.borrow_mut().mSelf = Rc::downgrade(&handler);
        handler
    }
}
impl ElementHandlerSimple for GdlstBranchHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: importer.startGdlst(elementId, attributes)
    fn onStart_with_id(&mut self, id: i32, _: &str, _: &str, _: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        self.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut()
            .startGdlst(id, super::DmlHandlerBase::sax_attributes(attrs));
        Ok(())
    }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    // Java: case 3293: return this;
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        if id == 3293 { self.mSelf.upgrade().map(|h| h as ElementHandlerRef) } else { None }
    }
}
