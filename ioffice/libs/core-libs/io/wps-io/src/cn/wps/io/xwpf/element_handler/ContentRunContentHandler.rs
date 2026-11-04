#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::any::Any;

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IDmlImporter
// Java: import cn.wps.io.xwpf.IDocumentImporter
// Java: import cn.wps.io.xwpf.IGVmlDocImporter
// Java: import cn.wps.io.xwpf.element_handler.dml.DrawingHandler
// Java: import cn.wps.moffice.property.MutablePropertySet

use crate::io::xwpf::element_handler::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;

// Java: abstract class ContentRunContentHandler extends EGRunLevelEltsHandler
pub struct ContentRunContentHandler {
    parent: XmlSimpleNodeElementHandler,
    // Java field: POIXMLDocumentPart mPart
    pub(crate) mPart: Option<Rc<dyn Any>>,
    // Java field: SubDocType mSubDocType
    pub(crate) mSubDocType: i32,
    // Java field: int mTableLayer
    pub(crate) mTableLayer: i32,
}

impl Deref for ContentRunContentHandler {
    type Target = XmlSimpleNodeElementHandler;
    fn deref(&self) -> &Self::Target { &self.parent }
}

impl DerefMut for ContentRunContentHandler {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.parent }
}

// Java: abstract class ContentRunContentHandler extends EGRunLevelEltsHandler
impl ContentRunContentHandler {
    // Java: ContentRunContentHandler(POIXMLDocumentPart part, IDocumentImporter importer, SubDocType subDocType, int tableLayer)
    pub fn new(part: Box<dyn std::any::Any>, importer: Box<dyn IDocumentImporter>, subDocType: i32, tableLayer: i32) -> Self {
        Self::new_shared(Rc::from(part), Rc::from(importer), subDocType, tableLayer)
    }

    pub fn new_shared(part: Rc<dyn Any>, importer: Rc<dyn IDocumentImporter>, subDocType: i32, tableLayer: i32) -> Self {
        // Java: super(part, importer, subDocType, tableLayer);
        let parent = XmlSimpleNodeElementHandler::new_from_doc_rc(importer);
        ContentRunContentHandler {
            parent,
            mPart: Some(part),
            mSubDocType: subDocType,
            mTableLayer: tableLayer,
        }
    }
}
