#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::ops::{Deref, DerefMut};

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IDocumentImporter
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
use crate::io::xwpf::element_handler::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException

use crate::io::xwpf::IDocumentImporter::IDocumentImporter;

// Java: public class SdtBlock4TxbxHandler extends XmlSimpleNodeElementHandler
pub struct SdtBlock4TxbxHandler {
    parent: XmlSimpleNodeElementHandler,
    // Java field: POIXMLDocumentPart mPart
    mPart: Option<Box<dyn std::any::Any>>,
    // Java field: SubDocType mSubDocType
    mSubDocType: i32,
    // Java field: int mTableLayer
    mTableLayer: i32,
    // Java field: SdtContentBlock4TxbxHandler mSdtContentBlockHandler
    mSdtContentBlockHandler: Option<Box<dyn std::any::Any>>,
}

impl Deref for SdtBlock4TxbxHandler {
    type Target = XmlSimpleNodeElementHandler;
    fn deref(&self) -> &Self::Target { &self.parent }
}

impl DerefMut for SdtBlock4TxbxHandler {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.parent }
}

// Java: public class SdtBlock4TxbxHandler extends XmlSimpleNodeElementHandler
impl SdtBlock4TxbxHandler {
    // Java: SdtBlock4TxbxHandler(POIXMLDocumentPart part, SubDocType subDocType, IDocumentImporter importer, int tableLayer)
    pub fn new(part: Box<dyn std::any::Any>, subDocType: i32, importer: Box<dyn IDocumentImporter>, tableLayer: i32) -> Self {
        // Java: super(importer);
        let parent = XmlSimpleNodeElementHandler::new_from_doc(importer);
        let mut s = SdtBlock4TxbxHandler {
            parent,
            mPart: None,
            mSubDocType: 0,
            mTableLayer: 0,
            mSdtContentBlockHandler: None,
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

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart_with_id(&self, elementId: i32, namespaceURI: &str, localName: &str, qualifiedName: &str, attributes: &dyn std::any::Any) -> Result<(), Box<dyn std::error::Error>> {
      Ok(())
    }

    // Java: ElementHandlerSimple getElementHandler(int subElement, String namespaceURI)
    pub fn getElementHandler(&self, subElement: i32, namespaceURI: &str) -> Option<Box<dyn std::any::Any + Send + Sync>> {
      // Java: switch (subElement)
      match subElement {
         // Java: case 1437683382:
         1437683382 => { None }
         // Java: default:
         _ => { None }
      }
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
      // Java: return 0;
      0
    }
}
