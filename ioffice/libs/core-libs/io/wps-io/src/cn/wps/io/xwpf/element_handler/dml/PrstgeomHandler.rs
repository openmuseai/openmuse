#![allow(non_snake_case)]
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;
use super::{AvlstBranchHandler::AvlstBranchHandler, DmlHandlerBase::DmlHandlerBase};

// Java: public class PrstgeomHandler extends DmlHandlerBase
pub struct PrstgeomHandler {
    parent: DmlHandlerBase,
    // Java field: AvlstBranchHandler mAvlstBranchHandler
    mAvlstBranchHandler: RefCell<Option<ElementHandlerRef>>,
}

impl PrstgeomHandler {
    // Java: PrstgeomHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType)
    pub fn new(dmlImporter: DmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32) -> Self {
        Self { parent: DmlHandlerBase::new_typed(dmlImporter, part, subDocType),
            mAvlstBranchHandler: RefCell::new(None) }
    }
    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }

    // Java: private ElementHandlerSimple getAvlstBranchHandler()
    fn getAvlstBranchHandler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mAvlstBranchHandler.borrow_mut();
        if slot.is_none() {
            *slot = Some(AvlstBranchHandler::new(
                self.parent.mTypedDmlImporter.as_ref()?.clone(),
                self.parent.mTypedPart.clone(), self.parent.mSubDocType, -875836993,
            ).as_handler_ref());
        }
        slot.clone()
    }
}

impl ElementHandlerSimple for PrstgeomHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, _: i32, _: &str, _: &str, _: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        // Java: importer.startPrstgeom(attributes);
        self.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut()
            .startPrstgeom(super::DmlHandlerBase::sax_attributes(attrs));
        Ok(())
    }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Java: importer.endPrstgeom();
        self.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut().endPrstgeom();
        Ok(())
    }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        // Java: case 93173592: return this.getAvlstBranchHandler();
        if id == 93173592 { self.getAvlstBranchHandler() } else { None }
    }
}
