#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::ops::{Deref, DerefMut};

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IDocumentImporter
// Java: import cn.wps.io.xwpf.element_handler.PHandler
// Java: import cn.wps.io.xwpf.element_handler.TblHandler
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
use crate::io::xwpf::element_handler::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;

use crate::io::xwpf::IDocumentImporter::IDocumentImporter;

// Java: public class SdtContentBlock4TxbxHandler extends XmlSimpleNodeElementHandler
pub struct SdtContentBlock4TxbxHandler {
    parent: XmlSimpleNodeElementHandler,
    // Java field: POIXMLDocumentPart mPart
    mPart: Option<Box<dyn std::any::Any>>,
    // Java field: SubDocType mSubDocType
    mSubDocType: i32,
    // Java field: int mTableLayer
    mTableLayer: i32,
    // Java field: PHandler mPHandler
    mPHandler: Option<Box<dyn std::any::Any>>,
    // Java field: TblHandler mTblHandler
    mTblHandler: Option<Box<dyn std::any::Any>>,
    // Java field: SdtBlock4TxbxHandler mSdt4TxbxHandler
    mSdt4TxbxHandler: Option<Box<dyn std::any::Any>>,
}

impl Deref for SdtContentBlock4TxbxHandler {
    type Target = XmlSimpleNodeElementHandler;
    fn deref(&self) -> &Self::Target { &self.parent }
}

impl DerefMut for SdtContentBlock4TxbxHandler {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.parent }
}

// Java: public class SdtContentBlock4TxbxHandler extends XmlSimpleNodeElementHandler
impl SdtContentBlock4TxbxHandler {
    // Java: SdtContentBlock4TxbxHandler(POIXMLDocumentPart part, SubDocType subDocType, IDocumentImporter importer, int tableLayer)
    pub fn new(part: Box<dyn std::any::Any>, subDocType: i32, importer: Box<dyn IDocumentImporter>, tableLayer: i32) -> Self {
        // Java: super(importer);
        let parent = XmlSimpleNodeElementHandler::new_from_doc(importer);
        let mut s = SdtContentBlock4TxbxHandler {
            parent,
            mPart: None,
            mSubDocType: 0,
            mTableLayer: 0,
            mPHandler: None,
            mTblHandler: None,
            mSdt4TxbxHandler: None,
        };
        s._init(part, subDocType, tableLayer);
        s
    }

    fn _init(&mut self, part: Box<dyn std::any::Any>, subDocType: i32, tableLayer: i32) {
        // Java: this.mPart = part;
        self.mPart = Some(part);
        // Java: this.mSubDocType = subDocType;
        self.mSubDocType = subDocType;
        // Java: this.mTableLayer = tableLayer;
        self.mTableLayer = tableLayer;
    }

    // Java: ElementHandlerSimple getElementHandler(int subElement, String namespaceURI)
    pub fn getElementHandler(&self, subElement: i32, namespaceURI: &str) -> Option<Box<dyn std::any::Any + Send + Sync>> {
      // Java: switch (subElement)
      match subElement {
         // Java: case 112:
         112 => { None }
         // Java: case 113731:
         113731 => { None }
         // Java: case 114622:
         114622 => { None }
         // Java: default:
         _ => { None }
      }
    }
}
