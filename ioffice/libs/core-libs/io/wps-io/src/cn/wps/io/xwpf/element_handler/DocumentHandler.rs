#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
use crate::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
// Java: import cn.wps.io.simple.ElementHandlerSimple
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException
use super::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use super::DocumentBodyHandler::DocumentBodyHandler;

// Java: public class DocumentHandler extends XmlSimpleNodeElementHandler
pub struct DocumentHandler {
    base: XmlSimpleNodeElementHandler,
    // Java field: POIXMLDocumentPart mPart
    pub(crate) mPart: Option<Rc<dyn Any>>,
    mDocImporter: Option<Rc<dyn IDocumentImporter>>,
    mBodyHandler: RefCell<Option<ElementHandlerRef>>,
}

// Java: public class DocumentHandler extends XmlSimpleNodeElementHandler
impl DocumentHandler {
    // Java: DocumentHandler(IDocumentImporter importer, POIXMLDocumentPart part)
    pub fn new(importer: Box<dyn Any + Send + Sync>, part: Option<Box<dyn Any + Send + Sync>>) -> Self {
        let _ = importer;
        // Java: super(importer);
        // Java: this.mPart = part;
        DocumentHandler {
            base: XmlSimpleNodeElementHandler::new(),
            mPart: part.map(|part| { let shared: Rc<dyn Any + Send + Sync> = Rc::from(part); let plain: Rc<dyn Any> = shared; plain }),
            mDocImporter: None,
            mBodyHandler: RefCell::new(None),
        }
    }

    pub fn new_rc(importer: Rc<dyn IDocumentImporter>, part: Option<Box<dyn Any + Send + Sync>>) -> Self {
        let shared = part.map(|part| { let shared: Rc<dyn Any + Send + Sync> = Rc::from(part); let plain: Rc<dyn Any> = shared; plain });
        Self::new_shared(importer, shared)
    }

    pub fn new_shared(importer: Rc<dyn IDocumentImporter>, part: Option<Rc<dyn Any>>) -> Self {
        // Java: super(importer);
        // Java: this.mPart = part;
        DocumentHandler {
            base: XmlSimpleNodeElementHandler::new_from_doc_rc(importer.clone()),
            mPart: part,
            mDocImporter: Some(importer),
            mBodyHandler: RefCell::new(None),
        }
    }

    pub fn as_handler_ref(importer: Rc<dyn IDocumentImporter>) -> ElementHandlerRef {
        Rc::new(RefCell::new(Self::new_rc(importer, None)))
    }

    /// Java: `new DocumentHandler(importer, this)` during synchronous document parse.
    pub(crate) fn as_handler_ref_with_part(importer: Rc<dyn IDocumentImporter>, part: &POIXMLDocumentPart) -> ElementHandlerRef {
        let borrowed = super::XmlConvertHelper::BorrowedDocumentPart::new(part);
        Rc::new(RefCell::new(Self::new_shared(importer, Some(Rc::new(borrowed)))))
    }

    fn getBodyHandler(&self) -> ElementHandlerRef {
        let mut slot = self.mBodyHandler.borrow_mut();
        if slot.is_none() {
            let importer = self.mDocImporter.clone().expect("importer");
            *slot = Some(DocumentBodyHandler::new_rc(self.mPart.clone(), importer).as_handler_ref());
        }
        slot.clone().expect("body handler")
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, namespaceURI: &str) -> Option<ElementHandlerRef> {
        // Java: switch (elementId)
        match elementId {
            // Java: case -1332194002:
            -1332194002_i32 => {
                // Java: return new BackgroundHandler(this.mDocImporter.getVmlImporter(SubDocType.MAIN_DOCUMENT), this.mPart);
                None
            }
            // Java: case 3029410:
            3029410 => {
                // Java: return new DocumentBodyHandler(this.mPart, this.mDocImporter);
                Some(self.getBodyHandler())
            }
            // Java: default:
            _ => {
                // Java: return null;
                None
            }
        }
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, elementId: i32, namespaceURI: &str, localName: &str, qualifiedName: &str, attributes: &dyn Any) {
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 861720859;
        861720859
    }
}

impl ElementHandlerSimple for DocumentHandler {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn filterIndex(&self) -> i32 {
        DocumentHandler::filterIndex(self)
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
        DocumentHandler::onStart(self, elementId, namespaceURI, localName, qualifiedName, attributes);
        Ok(())
    }
    fn onEnd_with_id(
        &mut self,
        _elementId: i32,
        _namespaceURI: &str,
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
    fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> {
        DocumentHandler::getElementHandler(self, id, ns)
    }
}
