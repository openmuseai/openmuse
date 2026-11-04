#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::ops::{Deref, DerefMut};
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IDmlImporter
// Java: import org.xml.sax.Attributes

use super::GraphicDataChildHandler::GraphicDataChildHandler;
use super::XfrmBranchHandler::XfrmBranchHandler;

// Java: public class ContentPartHandler extends GraphicDataChildHandler
pub struct ContentPartHandler {
    parent: GraphicDataChildHandler,
    // Java field: XfrmBranchHandler mXfrmBranchHandler
    mXfrmBranchHandler: Option<XfrmBranchHandler>,
    mTypedXfrmBranchHandler: RefCell<Option<ElementHandlerRef>>,
}

impl Deref for ContentPartHandler {
    type Target = GraphicDataChildHandler;
    fn deref(&self) -> &Self::Target { &self.parent }
}

impl DerefMut for ContentPartHandler {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.parent }
}

// Java: public class ContentPartHandler extends GraphicDataChildHandler
impl ContentPartHandler {
    // Java: ContentPartHandler(POIXMLDocumentPart part, IDmlImporter dmlImporter, SubDocType subDocType, int belonger)
    pub fn new(part: Option<Box<dyn std::any::Any + Send + Sync>>, dmlImporter: Box<dyn std::any::Any + Send + Sync>, subDocType: i32, belonger: i32) -> Self {
        // Java: super(dmlImporter, part, subDocType, belonger);
        ContentPartHandler {
            parent: GraphicDataChildHandler::new_with_belonger(dmlImporter, part, subDocType, belonger),
            mXfrmBranchHandler: None,
            mTypedXfrmBranchHandler: RefCell::new(None),
        }
    }

    // Java: ContentPartHandler(POIXMLDocumentPart part, IDmlImporter dmlImporter, SubDocType subDocType, int belonger)
    pub fn new_typed(part: Option<Rc<dyn Any>>, dmlImporter: DmlImporterRef, subDocType: i32, belonger: i32) -> Self {
        Self { parent: GraphicDataChildHandler::new_typed_with_belonger(dmlImporter, part, subDocType, belonger),
            mXfrmBranchHandler: None, mTypedXfrmBranchHandler: RefCell::new(None) }
    }

    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }

    // Java: private ElementHandlerSimple getXfrmBranchHandler()
    fn get_typed_xfrm_handler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mTypedXfrmBranchHandler.borrow_mut();
        if slot.is_none() {
            *slot = Some(XfrmBranchHandler::new_typed(
                self.parent.parent.mTypedDmlImporter.as_ref()?.clone(),
                self.parent.parent.mTypedPart.clone(), self.parent.parent.mSubDocType, false,
            ).as_handler_ref());
        }
        slot.clone()
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart_with_id(&self, elementId: i32, namespaceURI: &str, localName: &str, qualifiedName: &str, attributes: &dyn std::any::Any) {
      // Java: this.onStart(elementId, namespaceURI, localName, qualifiedName, attributes, false);
      self.onStart_with_group(elementId, namespaceURI, localName, qualifiedName, attributes, false);
      // Java: IDmlImporter importer = (IDmlImporter)this.mDmlImporter;
      // LLM Rust type-safety: deferred
      // Java: importer.startContentPart(attributes, this.mPart);
      // LLM Rust type-safety: deferred
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd_with_id(&self, elementId: i32, namespaceURI: &str) {
      // Java: this.onEnd(elementId, namespaceURI, false);
      self.onEnd_with_group(elementId, namespaceURI, false);
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler_override(&self, elementId: i32, namespaceURI: &str) -> Option<Box<dyn std::any::Any + Send + Sync>> {
      // Java: switch (elementId)
      match elementId {
         // Java: case -830200250:
         // Java: default:
         3676585 => {
            // Java: return this.getXfrmBranchHandler();
            // LLM Rust type-safety: deferred
            None
         }
         _ => {
            // Java: return null;
            None
         }
      }
    }

    // Java: ElementHandlerSimple getBlipFillHandler()
    pub fn getBlipFillHandler(&self) -> Option<&Box<dyn std::any::Any>> {
      // Java: if (null == this.mBlipFillHandler)
      // LLM Rust type-safety: deferred, lazy init
      // Java: return this.mBlipFillHandler;
      self.mBlipFillHandler.as_ref()
    }

    // Java: ElementHandlerSimple getXfrmBranchHandler()
    fn getXfrmBranchHandler(&self) -> Option<&XfrmBranchHandler> {
      // Java: if (null == this.mXfrmBranchHandler)
      // LLM Rust type-safety: deferred, lazy init
      // Java: return this.mXfrmBranchHandler;
      self.mXfrmBranchHandler.as_ref()
    }
}

impl ElementHandlerSimple for ContentPartHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, _: i32, _: &str, _: &str, _: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        // Java: this.onStart(..., false); importer.startContentPart(attributes, this.mPart);
        self.parent.onStart_typed(false);
        let part = self.parent.parent.mTypedPart.as_ref().expect("mPart");
        self.parent.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut()
            .startContentPart(super::DmlHandlerBase::sax_attributes(attrs),
                super::DmlHandlerBase::document_part(part.as_ref()));
        Ok(())
    }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Java: this.onEnd(elementId, namespaceURI, false);
        self.parent.onEnd_typed(false);
        Ok(())
    }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        // Java: case 3676585: return this.getXfrmBranchHandler();
        if id == 3676585 { self.get_typed_xfrm_handler() } else { None }
    }
}
