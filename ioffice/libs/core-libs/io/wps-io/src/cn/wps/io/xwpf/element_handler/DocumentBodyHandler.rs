#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.opc.PackageRelationshipCollection
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.simple.ElementHandlerSimple
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use super::PHandler::PHandler;
use super::SectPrHandler::SectPrHandler;
use super::TblHandler::TblHandler;

// Java: public class DocumentBodyHandler extends SubDocumentHandler
pub struct DocumentBodyHandler {
    mPart: Option<Rc<dyn Any>>,
    mDocImporter: Option<Rc<dyn IDocumentImporter>>,
    mPHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: ContentBlockContentHandler.mTableLayer — body starts at 0
    mTableLayer: i32,
    mTblHandler: RefCell<Option<ElementHandlerRef>>,
}

// Java: public class DocumentBodyHandler extends SubDocumentHandler
impl DocumentBodyHandler {
    // Java: DocumentBodyHandler(POIXMLDocumentPart part, IDocumentImporter importer)
    pub fn new(part: Box<dyn Any + Send + Sync>, importer: Box<dyn Any + Send + Sync>) -> Self {
        let _ = importer;
        DocumentBodyHandler {
            mPart: Some({ let shared: Rc<dyn Any + Send + Sync> = Rc::from(part); shared }),
            mDocImporter: None,
            mPHandler: RefCell::new(None),
            mTableLayer: 0,
            mTblHandler: RefCell::new(None),
        }
    }

    pub fn new_rc(part: Option<Rc<dyn Any>>, importer: Rc<dyn IDocumentImporter>) -> Self {
        // Java: super(part, importer, SubDocType.MAIN_DOCUMENT);
        DocumentBodyHandler {
            mPart: part,
            mDocImporter: Some(importer),
            mPHandler: RefCell::new(None),
            mTableLayer: 0,
            mTblHandler: RefCell::new(None),
        }
    }

    pub fn as_handler_ref(self) -> ElementHandlerRef {
        Rc::new(RefCell::new(self))
    }

    fn getPHandler(&self) -> ElementHandlerRef {
        // Java: ContentBlockContentHandler.getPHandler → new PHandler(this.mPart, this.mDocImporter, this.mSubDocType, this.mTableLayer)
        let mut slot = self.mPHandler.borrow_mut();
        if slot.is_none() {
            let importer = self.mDocImporter.clone().expect("importer");
            let part = self.mPart.clone().unwrap_or_else(|| Rc::new(()));
            *slot = Some(PHandler::new_shared(part, importer, 0, 0).as_handler_ref());
        }
        slot.clone().expect("p handler")
    }

    fn getTblHandler(&self) -> ElementHandlerRef {
        // Java: ContentBlockContentHandler.getTblHandler
        // Java: if (null == this.mTblHandler)
        let mut slot = self.mTblHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mTblHandler = new TblHandler(this.mPart, this.mTableLayer + 1, this.mDocImporter, this.mSubDocType);
            let importer = self.mDocImporter.clone().expect("importer");
            *slot = Some(TblHandler::new_rc(self.mPart.clone(), importer, 0, self.mTableLayer + 1).as_handler_ref());
        }
        // Java: return this.mTblHandler;
        slot.clone().expect("tbl handler")
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(
        &self,
        element_id: i32,
        namespace_uri: &str,
        local_name: &str,
        qualified_name: &str,
        attributes: &dyn Any,
    ) {
        // Java: PackageRelationshipCollection relations = getRelationshipCollection(this.mPart);
        let relations = self.mPart.as_ref().and_then(|part| {
            super::SubDocumentHandler::SubDocumentHandler::getRelationshipCollection(part.as_ref())
        });
        // Java: IOAssert.assertNotNull("relations should not be null!", relations);
        // Java: if (null == relations)
        if let Some(ref importer) = self.mDocImporter {
            match relations.as_ref() {
                // Java: if (null == relations) this.mDocImporter.onMainDocumentStart(false);
                None => importer.onMainDocumentStart(false),
                // Java: else this.mDocImporter.onMainDocumentStart(hasHeaderFooter(relations));
                Some(relations) => importer.onMainDocumentStart(
                    super::SubDocumentHandler::SubDocumentHandler::hasHeaderFooter(relations),
                ),
            }
        }
        let _ = (element_id, namespace_uri, local_name, qualified_name, attributes);
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd(&self, element_id: i32, namespace_uri: &str) {
        // Java: this.mDocImporter.onMainDocumentEnd();
        if let Some(ref importer) = self.mDocImporter {
            importer.onMainDocumentEnd();
        }
        let _ = (element_id, namespace_uri);
    }

    // Java: ElementHandlerSimple getElementHandler(int subElement, String namespaceURI)
    pub fn getElementHandler(&self, sub_element: i32, namespace_uri: &str) -> Option<ElementHandlerRef> {
        // Java: switch (subElement)
        match sub_element {
            // Java: case -906275931:
            -906275931_i32 => {
                // Java: return new SectPrHandler(this.mPart, this.mDocImporter);
                let importer = self.mDocImporter.clone()?;
                Some(SectPrHandler::new(importer).as_handler_ref())
            }
            // Java: default: return super.getElementHandler(subElement, namespaceURI);
            // Java: default: return super.getElementHandler → ContentBlockContentHandler
            // Java: case 112:
            112 => Some(self.getPHandler()),
            // Java: case 114622:
            114622 => Some(self.getTblHandler()),
            _ => None,
        }
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 0;
        0
    }
}

impl ElementHandlerSimple for DocumentBodyHandler {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn filterIndex(&self) -> i32 {
        DocumentBodyHandler::filterIndex(self)
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
        element_id: i32,
        namespace_uri: &str,
        local_name: &str,
        qualified_name: &str,
        attributes: &dyn Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        DocumentBodyHandler::onStart(
            self,
            element_id,
            namespace_uri,
            local_name,
            qualified_name,
            attributes,
        );
        Ok(())
    }
    fn onEnd_with_id(
        &mut self,
        element_id: i32,
        namespace_uri: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        DocumentBodyHandler::onEnd(self, element_id, namespace_uri);
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
        DocumentBodyHandler::getElementHandler(self, id, ns)
    }
}
