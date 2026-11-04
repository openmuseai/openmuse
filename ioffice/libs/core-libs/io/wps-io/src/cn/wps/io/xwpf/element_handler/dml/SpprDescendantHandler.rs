#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::ops::{Deref, DerefMut};
use std::{any::Any, cell::RefCell, rc::{Rc, Weak}};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IDmlImporter
// Java: import org.xml.sax.Attributes

use super::DmlHandlerBase::DmlHandlerBase;

// Java: public class SpprDescendantHandler extends DmlHandlerBase
pub struct SpprDescendantHandler {
    parent: DmlHandlerBase,
    mSelf: Weak<RefCell<SpprDescendantHandler>>,
}

impl Deref for SpprDescendantHandler {
    type Target = DmlHandlerBase;
    fn deref(&self) -> &Self::Target { &self.parent }
}

impl DerefMut for SpprDescendantHandler {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.parent }
}

// Java: public class SpprDescendantHandler extends DmlHandlerBase
impl SpprDescendantHandler {
    // Java: SpprDescendantHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType)
    pub fn new(dmlImporter: Box<dyn std::any::Any + Send + Sync>, part: Option<Box<dyn std::any::Any + Send + Sync>>, subDocType: i32) -> Self {
        // Java: super(dmlImporter, part, subDocType);
        SpprDescendantHandler {
            parent: DmlHandlerBase::new(dmlImporter, part, subDocType),
            mSelf: Weak::new(),
        }
    }

    // Java: SpprDescendantHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType)
    pub fn new_typed(dmlImporter: DmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32) -> Self {
        Self { parent: DmlHandlerBase::new_typed(dmlImporter, part, subDocType), mSelf: Weak::new() }
    }

    pub fn as_handler_ref(self) -> ElementHandlerRef {
        let rc = Rc::new(RefCell::new(self));
        rc.borrow_mut().mSelf = Rc::downgrade(&rc);
        rc
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart_with_id(&self, elementId: i32, namespaceURI: &str, localName: &str, qualifiedName: &str, attributes: &dyn std::any::Any) {
      // Java: IDmlImporter importer = (IDmlImporter)this.mDmlImporter;
      // LLM Rust type-safety: mDmlImporter is IImporterBase, cannot cast to IDmlImporter
      // Java: importer.startSppr(elementId, attributes);
      // LLM Rust type-safety: deferred, IDmlImporter not accessible
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd_with_id(&self, elementId: i32, namespaceURI: &str) {
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, namespaceURI: &str) -> Option<Box<dyn std::any::Any + Send + Sync>> {
      // Java: switch (elementId)
      match elementId {
         // Java: case 100897:
         100897 => {
            // Java: return this;
            // LLM Rust type-safety: cannot return self as Box, deferred
            None
         }
         // Java: default:
         _ => {
            // Java: return null;
            None
         }
      }
    }
}

impl ElementHandlerSimple for SpprDescendantHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, _: &str, _: &str, _: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        // Java: importer.startSppr(elementId, attributes);
        self.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut()
            .startSppr(id, super::DmlHandlerBase::sax_attributes(attrs));
        Ok(())
    }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        // Java: case 100897: return this;
        if id == 100897 { self.mSelf.upgrade().map(|h| h as ElementHandlerRef) } else { None }
    }
}
