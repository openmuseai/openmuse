#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::ops::{Deref, DerefMut};

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IDocumentImporter
// Java: import cn.wps.io.xwpf.IVmlImporter
// Java: import cn.wps.io.xwpf.element_handler.TxbxContentHandler
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
use crate::io::xwpf::element_handler::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException

use crate::io::xwpf::IImporterBase::IImporterBase;

// Java: class TextboxHandler extends XmlSimpleNodeElementHandler
pub struct TextboxHandler {
    parent: XmlSimpleNodeElementHandler,
    // Java field: POIXMLDocumentPart mPart
    mPart: Option<Box<dyn std::any::Any>>,
    // Java field: SubDocType mSubDocType
    mSubDocType: i32,
    // Java field: boolean mHasTxbxContent
    mHasTxbxContent: bool,
    // Java field: TxbxContentHandler mTxbxContentHandler
    mTxbxContentHandler: Option<Box<dyn std::any::Any>>,
}

impl Deref for TextboxHandler {
    type Target = XmlSimpleNodeElementHandler;
    fn deref(&self) -> &Self::Target { &self.parent }
}

impl DerefMut for TextboxHandler {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.parent }
}

// Java: class TextboxHandler extends XmlSimpleNodeElementHandler
impl TextboxHandler {
    // Java: TextboxHandler(POIXMLDocumentPart part, IVmlImporter importer, SubDocType subDocType)
    pub fn new(part: Box<dyn std::any::Any>, importer: Box<dyn IImporterBase>, subDocType: i32) -> Self {
        // Java: super(importer);
        let parent = XmlSimpleNodeElementHandler::new_from_vml(importer);
        let mut s = TextboxHandler {
            parent,
            mPart: None,
            mSubDocType: 0,
            mHasTxbxContent: false,
            mTxbxContentHandler: None,
        };
        s._init(part, subDocType);
        s
    }

    fn _init(&mut self, part: Box<dyn std::any::Any>, subDocType: i32) {
        // Java: this.mPart = part;
        self.mPart = Some(part);
        // Java: this.mSubDocType = subDocType;
        self.mSubDocType = subDocType;
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart_with_id(&self, elementId: i32, namespaceURI: &str, localName: &str, qualifiedName: &str, attributes: &dyn std::any::Any) {
      // Java: this.mVmlImporter.onImportEGShapeElementsStart(elementId, namespaceURI, attributes);
      // LLM Rust type-safety: IImporterBase does not expose this method
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, namespaceURI: &str) -> Option<Box<dyn std::any::Any + Send + Sync>> {
      // Java: switch (elementId)
      match elementId {
         // Java: case 163404095:
         163404095 => { None }
         // Java: default:
         _ => {
            // Java: IOAssert.fail("it should not reach here");
            // Java: return null;
            None
         }
      }
    }

    // Java: SubDocType getCurrentSubDocType(SubDocType subDocType)
    fn getCurrentSubDocType(&self, subDocType: i32) -> i32 {
      // Java: switch (subDocType)
      // LLM Rust type-safety: SubDocType enum values not available, returning default
      subDocType
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd_with_id(&mut self, elementId: i32, namespaceURI: &str) -> Result<(), Box<dyn std::error::Error>> {
      // Java: this.mVmlImporter.onImportTextboxEnd(this.mHasTxbxContent);
      // LLM Rust type-safety: IImporterBase does not expose this method
      // Java: this.mHasTxbxContent = false;
      self.mHasTxbxContent = false;
      Ok(())
    }
}
