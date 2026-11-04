#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::ops::{Deref, DerefMut};
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::ElementHandlerRef;
use crate::io::xwpf::IDmlImporter::DmlImporterRef;

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.gvml.IGVmlImporter
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IDmlImporter
// Java: import org.xml.sax.Attributes

use super::DmlHandlerBase::DmlHandlerBase;
use super::SpprHandler::SpprHandler;

// Java: public abstract class GraphicDataChildHandler extends DmlHandlerBase
pub struct GraphicDataChildHandler {
    pub(crate) parent: DmlHandlerBase,
    // Java field: SpprHandler mSpprHandler
    mSpprHandler: Option<SpprHandler>,
    // Java field: StyleHandler mStyleHandler
    mStyleHandler: Option<Box<dyn std::any::Any>>,
    // Java field: TxbxHandler mTxbxHandler
    mTxbxHandler: Option<Box<dyn std::any::Any>>,
    // Java field: BlipFillHandler mBlipFillHandler
    pub(crate) mBlipFillHandler: Option<Box<dyn std::any::Any>>,
    pub(crate) mTypedBlipFillHandler: RefCell<Option<ElementHandlerRef>>,
    pub(crate) mTypedSpprHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: LinkedTxbxHandler mLinkedTxbxHandler
    mLinkedTxbxHandler: Option<Box<dyn std::any::Any>>,
    // Java field: BodyPrHandler mBodyprHandler
    mBodyprHandler: Option<Box<dyn std::any::Any>>,
}

impl Deref for GraphicDataChildHandler {
    type Target = DmlHandlerBase;
    fn deref(&self) -> &Self::Target { &self.parent }
}

impl DerefMut for GraphicDataChildHandler {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.parent }
}

// Java: public abstract class GraphicDataChildHandler extends DmlHandlerBase
impl GraphicDataChildHandler {
    // Java: GraphicDataChildHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType)
    pub fn new(dmlImporter: Box<dyn std::any::Any + Send + Sync>, part: Option<Box<dyn std::any::Any + Send + Sync>>, subDocType: i32) -> Self {
        // Java: super(dmlImporter, part, subDocType);
        GraphicDataChildHandler {
            parent: DmlHandlerBase::new(dmlImporter, part, subDocType),
            mSpprHandler: None,
            mStyleHandler: None,
            mTxbxHandler: None,
            mBlipFillHandler: None,
            mTypedBlipFillHandler: RefCell::new(None),
            mTypedSpprHandler: RefCell::new(None),
            mLinkedTxbxHandler: None,
            mBodyprHandler: None,
        }
    }

    // Java: GraphicDataChildHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType, int belonger)
    pub fn new_with_belonger(dmlImporter: Box<dyn std::any::Any + Send + Sync>, part: Option<Box<dyn std::any::Any + Send + Sync>>, subDocType: i32, belonger: i32) -> Self {
        // Java: super(dmlImporter, part, subDocType, belonger);
        GraphicDataChildHandler {
            parent: DmlHandlerBase::new_with_belonger(dmlImporter, part, subDocType, belonger),
            mSpprHandler: None,
            mStyleHandler: None,
            mTxbxHandler: None,
            mBlipFillHandler: None,
            mTypedBlipFillHandler: RefCell::new(None),
            mTypedSpprHandler: RefCell::new(None),
            mLinkedTxbxHandler: None,
            mBodyprHandler: None,
        }
    }

    // Java: GraphicDataChildHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType, int belonger)
    pub fn new_typed_with_belonger(dmlImporter: DmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32, belonger: i32) -> Self {
        Self {
            parent: DmlHandlerBase::new_typed_with_belonger(dmlImporter, part, subDocType, belonger),
            mSpprHandler: None, mStyleHandler: None, mTxbxHandler: None,
            mBlipFillHandler: None, mTypedBlipFillHandler: RefCell::new(None),
            mTypedSpprHandler: RefCell::new(None),
            mLinkedTxbxHandler: None, mBodyprHandler: None,
        }
    }

    // Java: protected void onStart(..., boolean isGroup)
    pub fn onStart_typed(&self, isGroup: bool) {
        self.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut().startSingleShape(isGroup);
    }

    // Java: protected void onEnd(..., boolean isGroup)
    pub fn onEnd_typed(&self, isGroup: bool) {
        self.parent.mTypedDmlImporter.as_ref().expect("mDmlImporter").borrow_mut().endSingleShape(isGroup);
    }

    // Java: private ElementHandlerSimple getSPPRHandler()
    pub fn getSPPRHandler_typed(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mTypedSpprHandler.borrow_mut();
        if slot.is_none() {
            *slot = Some(SpprHandler::new_typed(
                self.parent.mTypedDmlImporter.as_ref()?.clone(),
                self.parent.mTypedPart.clone(), self.parent.mSubDocType,
            ).as_handler_ref());
        }
        slot.clone()
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes, boolean isGroup)
    pub fn onStart_with_group(&self, elementId: i32, namespaceURI: &str, localName: &str, qualifiedName: &str, attributes: &dyn std::any::Any, isGroup: bool) {
      // Java: IDmlImporter importer = (IDmlImporter)this.mDmlImporter;
      // LLM Rust type-safety: deferred
      // Java: importer.startSingleShape(isGroup);
      // LLM Rust type-safety: deferred
    }

    // Java: void onEnd(int elementId, String namespaceURI, boolean isGroup)
    pub fn onEnd_with_group(&self, elementId: i32, namespaceURI: &str, isGroup: bool) {
      // Java: IDmlImporter importer = (IDmlImporter)this.mDmlImporter;
      // LLM Rust type-safety: deferred
      // Java: importer.endSingleShape(isGroup);
      // LLM Rust type-safety: deferred
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, namespaceURI: &str) -> Option<Box<dyn std::any::Any + Send + Sync>> {
      // Java: switch (elementId)
      match elementId {
         // Java: case -1383701692:
         -1383701692_i32 => { None }
         // Java: case -825444332:
         -825444332_i32 => { None }
         // Java: case 3536191:
         3536191 => { None }
         // Java: case 3574234:
         3574234 => { None }
         // Java: case 109780401:
         109780401 => { None }
         // Java: case 1342959507:
         1342959507 => { None }
         // Java: default:
         _ => { None }
      }
    }
}
