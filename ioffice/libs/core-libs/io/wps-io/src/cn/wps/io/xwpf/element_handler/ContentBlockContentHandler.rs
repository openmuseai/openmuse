#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::ops::{Deref, DerefMut};

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IDmlImporter
// Java: import cn.wps.io.xwpf.IDocumentImporter
// Java: import cn.wps.io.xwpf.element_handler.prop.Prop
// Java: import cn.wps.moffice.property.MutablePropertySet

use crate::io::xwpf::element_handler::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;

// Java: public abstract class ContentBlockContentHandler extends EGRunLevelEltsHandler
pub struct ContentBlockContentHandler {
    parent: XmlSimpleNodeElementHandler,
    // Java field: PHandler mPHandler
    pub(crate) mPHandler: Option<Box<dyn std::any::Any>>,
    // Java field: TblHandler mTblHandler
    pub(crate) mTblHandler: Option<Box<dyn std::any::Any>>,
    // Java field: SdtBlockHandler mSdtHandler
    pub(crate) mSdtHandler: Option<Box<dyn std::any::Any>>,
    // Java field: AlternateContentHandler mAlternateContentHandler
    pub(crate) mAlternateContentHandler: Option<Box<dyn std::any::Any>>,
    // Java field: POIXMLDocumentPart mPart
    pub(crate) mPart: Option<Box<dyn std::any::Any>>,
    // Java field: SubDocType mSubDocType
    pub(crate) mSubDocType: i32,
    // Java field: int mTableLayer
    pub(crate) mTableLayer: i32,
}

impl Deref for ContentBlockContentHandler {
    type Target = XmlSimpleNodeElementHandler;
    fn deref(&self) -> &Self::Target { &self.parent }
}

impl DerefMut for ContentBlockContentHandler {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.parent }
}

// Java: public abstract class ContentBlockContentHandler extends EGRunLevelEltsHandler
impl ContentBlockContentHandler {
    // Java: ContentBlockContentHandler(POIXMLDocumentPart part, IDocumentImporter importer, SubDocType subDocType, int tableLayer)
    pub fn new(part: Box<dyn std::any::Any>, importer: Box<dyn IDocumentImporter>, subDocType: i32, tableLayer: i32) -> Self {
        // Java: super(part, importer, subDocType, tableLayer);
        let parent = XmlSimpleNodeElementHandler::new_from_doc(importer);
        ContentBlockContentHandler {
            parent,
            mPHandler: None,
            mTblHandler: None,
            mSdtHandler: None,
            mAlternateContentHandler: None,
            mPart: Some(part),
            mSubDocType: subDocType,
            mTableLayer: tableLayer,
        }
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
         // Java: case 1590178495:
         1590178495 => { None }
         // Java: default:
         _ => { None }
      }
    }
}
