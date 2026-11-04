#![allow(non_snake_case, dead_code)]
use std::{any::Any, cell::RefCell, rc::{Rc, Weak}};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;
use super::DmlHandlerBase::DmlHandlerBase;

// Java: public class PositionVBranchHandler extends DmlHandlerBase
pub struct PositionVBranchHandler {
    parent: DmlHandlerBase,
    // Java field: String mTmpContent
    mTmpContent: Option<String>,
    mSelf: Weak<RefCell<PositionVBranchHandler>>,
}
impl PositionVBranchHandler {
    // Java: PositionVBranchHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType)
    pub fn new(dmlImporter: DmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32) -> Self {
        Self { parent: DmlHandlerBase::new_typed(dmlImporter, part, subDocType),
            mTmpContent: None, mSelf: Weak::new() }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef {
        let rc = Rc::new(RefCell::new(self));
        rc.borrow_mut().mSelf = Rc::downgrade(&rc);
        rc
    }
}
impl ElementHandlerSimple for PositionVBranchHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, _: &str, _: &str, _: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        // Java: importer.startPositionV(elementId, attributes);
        self.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut().startPositionV(id, super::DmlHandlerBase::sax_attributes(attrs)); Ok(())
    }
    fn onEnd_with_id(&mut self, id: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Java: importer.endPositionV(elementId, this.mTmpContent);
        self.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut().endPositionV(id, self.mTmpContent.as_deref()); Ok(())
    }
    fn onCharacters(&mut self, chars: &[char], start: i32, len: i32) -> Result<(), Box<dyn std::error::Error>> {
        // Java: this.mTmpContent = new String(buffer, start, len);
        if start >= 0 && len >= 0 { self.mTmpContent = Some(chars.iter().skip(start as usize).take(len as usize).collect()); }
        Ok(())
    }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        // Java: matching child ids return this.
        match id { 92903173 | 949744647 | 1734453110 => self.mSelf.upgrade().map(|h| h as ElementHandlerRef), _ => None }
    }
}
