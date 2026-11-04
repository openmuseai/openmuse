#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::any::Any;

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IDocumentImporter

use super::ContentRunContentHandler::ContentRunContentHandler;
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;

// Java: public abstract class PContentHandler extends ContentRunContentHandler
pub struct PContentHandler {
    parent: ContentRunContentHandler,
    // Java field: HyperlinkHandler mHyperlinkHandler
    mHyperlinkHandler: Option<Box<dyn std::any::Any>>,
    // Java field: FldSimpleHandler mFldSimpleHandler
    mFldSimpleHandler: Option<Box<dyn std::any::Any>>,
}

impl Deref for PContentHandler {
    type Target = ContentRunContentHandler;
    fn deref(&self) -> &Self::Target { &self.parent }
}

impl DerefMut for PContentHandler {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.parent }
}

// Java: public abstract class PContentHandler extends ContentRunContentHandler
impl PContentHandler {
    // Java: PContentHandler(POIXMLDocumentPart part, IDocumentImporter importer, SubDocType subDocType, int tableLayer)
    pub fn new(part: Box<dyn std::any::Any>, importer: Box<dyn IDocumentImporter>, subDocType: i32, tableLayer: i32) -> Self {
        Self::new_shared(Rc::from(part), Rc::from(importer), subDocType, tableLayer)
    }

    pub fn new_shared(part: Rc<dyn Any>, importer: Rc<dyn IDocumentImporter>, subDocType: i32, tableLayer: i32) -> Self {
        // Java: super(part, importer, subDocType, tableLayer);
        PContentHandler {
            parent: ContentRunContentHandler::new_shared(part, importer, subDocType, tableLayer),
            mHyperlinkHandler: None,
            mFldSimpleHandler: None,
        }
    }
}
