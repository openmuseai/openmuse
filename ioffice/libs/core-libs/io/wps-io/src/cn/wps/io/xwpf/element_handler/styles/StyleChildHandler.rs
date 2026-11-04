#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
use crate::io::xwpf::element_handler::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;
// Java: import org.xml.sax.Attributes

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};

// Java: public class StyleChildHandler extends XmlSimpleNodeElementHandler
pub struct StyleChildHandler {
    base: XmlSimpleNodeElementHandler,
}

// Java: public class StyleChildHandler extends XmlSimpleNodeElementHandler
impl StyleChildHandler {
    pub fn new_rc(importer: Rc<dyn IDocumentImporter>) -> Self {
        // Java: super(importer);
        StyleChildHandler {
            base: XmlSimpleNodeElementHandler::new_from_doc_rc(importer),
        }
    }

    pub fn as_handler_ref(importer: Rc<dyn IDocumentImporter>) -> ElementHandlerRef {
        Rc::new(RefCell::new(Self::new_rc(importer)))
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, elementId: i32, namespaceURI: &str, _localName: &str, _qualifiedName: &str, attributes: &dyn Any) {
        // Java: this.mDocImporter.onImportStyleChild(elementId, attributes);
        if let Some(ref importer) = self.base.mDocImporter {
            importer.onImportStyleChild(elementId, attributes);
        }
        let _ = namespaceURI;
    }
}

impl ElementHandlerSimple for StyleChildHandler {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn filterIndex(&self) -> i32 {
        0
    }
    fn onStart(
        &mut self,
        _a: &str,
        _b: &str,
        _c: &str,
        _d: &dyn Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onEnd(
        &mut self,
        _a: &str,
        _b: &str,
        _c: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onStart_with_id(
        &mut self,
        elementId: i32,
        namespaceURI: &str,
        localName: &str,
        qualifiedName: &str,
        attributes: &dyn Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        StyleChildHandler::onStart(self, elementId, namespaceURI, localName, qualifiedName, attributes);
        Ok(())
    }
    fn onEnd_with_id(
        &mut self,
        _id: i32,
        _ns: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onCharacters(
        &mut self,
        _b: &[char],
        _s: i32,
        _l: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onCharacters_str(&mut self, _s: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn getElementHandler(&self, _id: i32, _ns: &str) -> Option<ElementHandlerRef> {
        None
    }
}