#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.simple.ElementHandlerSimple
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException

use super::PContentHandler::PContentHandler;
use super::ParagraphSimple::ParagraphSimple;
use super::PprHandler::PprHandler;
use super::RHandler::RHandler;

// Java: public class PHandler extends PContentHandler
pub struct PHandler {
    parent: PContentHandler,
    // Java field: PprHandler mPprHandler
    mPprHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: ParagraphSimple mParagraphSimple
    mParagraphSimple: Option<Box<dyn Any>>,
    // Java field: int mShapeBelonger
    mShapeBelonger: i32,
    mDocImporter: Option<Rc<dyn IDocumentImporter>>,
    mRHandler: RefCell<Option<ElementHandlerRef>>,
    mSubDocType: i32,
    mTableLayer: i32,
    // Java: inherited POIXMLDocumentPart mPart
    mPart: Rc<dyn Any>,
}

impl Deref for PHandler {
    type Target = PContentHandler;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for PHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

// Java: public class PHandler extends PContentHandler
impl PHandler {
    // Java: PHandler(POIXMLDocumentPart part, IDocumentImporter importer, SubDocType subDocType, int layerTable)
    pub fn new(
        part: Box<dyn Any>,
        importer: Box<dyn IDocumentImporter>,
        subDocType: i32,
        layerTable: i32,
    ) -> Self {
        Self::new_rc(part, Rc::from(importer), subDocType, layerTable)
    }

    pub fn new_rc(
        part: Box<dyn Any>,
        importer: Rc<dyn IDocumentImporter>,
        subDocType: i32,
        layerTable: i32,
    ) -> Self {
        Self::new_shared(Rc::from(part), importer, subDocType, layerTable)
    }

    pub fn new_shared(
        part: Rc<dyn Any>,
        importer: Rc<dyn IDocumentImporter>,
        subDocType: i32,
        layerTable: i32,
    ) -> Self {
        // Java: super(part, importer, subDocType, layerTable);
        PHandler {
            parent: PContentHandler::new_shared(part.clone(), importer.clone(), subDocType, layerTable),
            mPprHandler: RefCell::new(None),
            mParagraphSimple: None,
            mShapeBelonger: 0,
            mDocImporter: Some(importer),
            mRHandler: RefCell::new(None),
            mSubDocType: subDocType,
            mTableLayer: layerTable,
            mPart: part,
        }
    }

    pub fn as_handler_ref(self) -> ElementHandlerRef {
        Rc::new(RefCell::new(self))
    }

    fn getRHandler(&self) -> ElementHandlerRef {
        // Java: ContentRunContentHandler.getRHandler → new RHandler(this.mPart, this.mDocImporter, this.mSubDocType, this.mTableLayer)
        let mut slot = self.mRHandler.borrow_mut();
        if slot.is_none() {
            let importer = self.mDocImporter.clone().expect("importer");
            *slot = Some(RHandler::new_rc(
                self.mPart.clone(),
                importer,
                self.mSubDocType,
                self.mTableLayer,
            ));
        }
        slot.clone().expect("r handler")
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(
        &mut self,
        elementId: i32,
        namespaceURI: &str,
        localName: &str,
        qualifiedName: &str,
        attributes: &dyn Any,
    ) {
        // Java: this.mDocImporter.onImportParagraphStart(this.mSubDocType, attributes);
        if let Some(ref importer) = self.mDocImporter {
            importer.onImportParagraphStart(self.mSubDocType, attributes);
        }
        // Java: this.mPprHandler = null;
        *self.mPprHandler.borrow_mut() = None;
    }

    // Java: void onEnd(int element, String namespaceURI)
    pub fn onEnd(&mut self, element: i32, namespaceURI: &str) {
        // Java: ParagraphSimple paragraphSimple = this.getParagraphSimple();
        let paragraph = self.getParagraphSimple();
        // Java: this.mDocImporter.onImportParagraphEnd(paragraphSimple, this.mSubDocType, this.mTableLayer);
        if let Some(ref importer) = self.mDocImporter {
            importer.onImportParagraphEnd(&paragraph, self.mSubDocType, self.mTableLayer);
        }
        // Java: if (null != this.mPprHandler) this.mPprHandler.clearProp();
        if let Some(ref h) = *self.mPprHandler.borrow() {
            if let Some(ph) = h.borrow_mut().as_any_mut().downcast_mut::<PprHandler>() {
                ph.clearProp();
            }
        }
        let _ = (element, namespaceURI);
    }

    fn getPprHandler(&self) -> ElementHandlerRef {
        // Java: if (null == this.mPprHandler) this.mPprHandler = new PprHandler(this.mPart, this.mDocImporter);
        let mut slot = self.mPprHandler.borrow_mut();
        if slot.is_none() {
            let importer = self.mDocImporter.clone().expect("importer");
            *slot = Some(Rc::new(RefCell::new(PprHandler::new(Box::new(()), importer))));
        }
        slot.clone().expect("ppr handler")
    }

    // Java: private ParagraphSimple getParagraphSimple()
    fn getParagraphSimple(&mut self) -> ParagraphSimple {
        // Java: this.mParagraphSimple = new ParagraphSimple(this.mPart, this.mPprHandler);
        let importer = self.mDocImporter.clone().expect("importer");
        let mut ppr = PprHandler::new(Box::new(()), importer);
        if let Some(ref h) = *self.mPprHandler.borrow() {
            if let Some(ph) = h.borrow().as_any().downcast_ref::<PprHandler>() {
                ppr.mPropertySet = ph.getProp().map(|p| p.clone_mps()).or_else(|| {
                    Some(ph.base.mProp.borrow_mut().getProp().clone_mps())
                });
                // Java: this.mParagraphSimple.setPprHandler(this.mPprHandler);
                // The SAX trait stores mPprHandler behind Rc; carry its rPr bucket too.
                if let Some(run_prop) = ph.getRunProp() {
                    ppr.getRprHandler().mCorrectProp = Some(run_prop);
                }
            }
        }
        let mut paragraph = ParagraphSimple::new(Box::new(()), ppr);
        if let Some(ref handler) = *self.mPprHandler.borrow() {
            // Java: this.mParagraphSimple.setPprHandler(this.mPprHandler);
            paragraph.setSharedPprHandler(handler.clone());
        }
        paragraph
    }

    // Java: ElementHandlerSimple getElementHandler(int subElement, String namespaceURI)
    pub fn getElementHandler_impl(&self, subElement: i32, namespaceURI: &str) -> Option<ElementHandlerRef> {
        // Java: switch (subElement)
        match subElement {
            // Java: case 110226:
            110226 => {
                // Java: return this.getPprHandler();
                Some(self.getPprHandler())
            }
            // Java: default: return super.getElementHandler(subElement, namespaceURI);
            114 => Some(self.getRHandler()),
            _ => {
                if subElement == 114 {
                    Some(self.getRHandler())
                } else {
                    None
                }
            }
        }
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 0;
        0
    }
}

impl ElementHandlerSimple for PHandler {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn filterIndex(&self) -> i32 {
        PHandler::filterIndex(self)
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
        PHandler::onStart(self, elementId, namespaceURI, localName, qualifiedName, attributes);
        Ok(())
    }
    fn onEnd_with_id(
        &mut self,
        element: i32,
        namespaceURI: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        PHandler::onEnd(self, element, namespaceURI);
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
        self.getElementHandler_impl(id, ns)
    }
}
