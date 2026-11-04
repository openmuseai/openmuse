#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.io.xwpf.element_handler.prop.IProp
use super::prop::Prop::Prop;
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException
use super::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};

// Java: public class RprChildHandler extends XmlSimpleNodeElementHandler
pub struct RprChildHandler {
    pub(crate) base: XmlSimpleNodeElementHandler,
    // Java field: IProp mProp
    pub(crate) mProp: Rc<RefCell<Prop>>,
}

// Java: public class RprChildHandler extends XmlSimpleNodeElementHandler
impl RprChildHandler {
    // Java: RprChildHandler(IDocumentImporter importer, IProp iProp)
    pub fn new(importer: Rc<dyn IDocumentImporter>, iProp: Rc<RefCell<Prop>>) -> Self {
        // Java: super(importer);
        // Java: this.mProp = iProp;
        RprChildHandler {
            base: XmlSimpleNodeElementHandler::new_from_doc_rc(importer),
            mProp: iProp,
        }
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&mut self, elementId: i32, namespaceURI: &str, _localName: &str, _qualifiedName: &str, attributes: &dyn std::any::Any) {
        // Java: this.mDocImporter.convertRunProp(this.mProp.getProp(), namespaceURI, elementId, attributes);
        if let Some(importer) = self.base.mDocImporter.as_ref() {
            let mut g = self.mProp.borrow_mut();
            importer.convertRunProp(g.getProp(), namespaceURI, elementId, attributes);
        }
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 0;
        0
    }
}

impl ElementHandlerSimple for RprChildHandler {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn filterIndex(&self) -> i32 {
        RprChildHandler::filterIndex(self)
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
        RprChildHandler::onStart(self, elementId, namespaceURI, localName, qualifiedName, attributes);
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
