#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use super::prop::PropChildHandler::PropChildHandler;
use super::prop::PropHandler::PropHandler;
use super::SectPrHandlerHelper::SectPrHandlerHelper;
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::element_handler::prop::IPropHandlerHelper::IPropHandlerHelper;
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
use core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet;

// Java: public class SectPrHandler extends PropHandler implements IPropHandler
pub struct SectPrHandler {
    pub(crate) base: PropHandler,
    // Java field: POIXMLDocumentPart mPart
    mPart: Option<Box<dyn Any + Send + Sync>>,
    // Java field: MutablePropertySet mPropertySet
    mPropertySet: Option<MutablePropertySet>,
}

impl SectPrHandler {
    // Java: public SectPrHandler(POIXMLDocumentPart part, IDocumentImporter importer)
    pub fn new(importer: Rc<dyn IDocumentImporter>) -> Self {
        // Java: super(importer, SectPrHandlerHelper.getHandlerHelper());
        let helper: Rc<dyn IPropHandlerHelper> = SectPrHandlerHelper::getHandlerHelper();
        // Java: this.mPart = part;
        SectPrHandler {
            base: PropHandler::new(importer, helper),
            mPart: None,
            mPropertySet: None,
        }
    }

    pub fn as_handler_ref(self) -> ElementHandlerRef {
        Rc::new(RefCell::new(self))
    }

    // Java: public MutablePropertySet getProp()
    pub fn getProp(&self) -> Option<&MutablePropertySet> {
        // Java: return this.mPropertySet;
        self.mPropertySet.as_ref()
    }

    // Java: public void clearProp()
    pub fn clearProp(&mut self) {
        // Java: this.mPropertySet = null;
        self.mPropertySet = None;
        // Java: this.mProp.clearProp();
        self.base.mProp.borrow_mut().clearProp();
    }

    // Java: public void onEnd(int elementId, String namespaceURI)
    pub fn onEnd(&mut self, elementId: i32, _namespaceURI: &str) {
        // Java: MutablePropertySet prop = this.getProp(elementId);
        let importer = self.base.doc_importer();
        {
            let mut g = self.base.mProp.borrow_mut();
            let prop = g.getProp();
            // Java: this.mPropertySet = this.mDocImporter.correctProps(elementId, prop, changeProp);
            if let Some(ref imp) = importer {
                let boxed = imp.correctProps(elementId, prop, None);
                if let Ok(mps) = boxed.downcast::<MutablePropertySet>() {
                    self.mPropertySet = Some(*mps);
                }
            }
            if self.mPropertySet.is_none() {
                self.mPropertySet = Some(prop.clone_mps());
            }
        }
        // Java: if (this.isDocx()) { this.mDocImporter.onImportSectProp(prop, headFooterProp); }
        let snapshot = self.mPropertySet.as_ref().map(|p| p.clone_mps());
        if let (Some(imp), Some(mps)) = (importer, snapshot) {
            imp.onImportSectProp(&mps, &());
        }
        let _ = elementId;
    }
}

impl ElementHandlerSimple for SectPrHandler {
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
        // Java: public void onStart(...) { }
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
        _elementId: i32,
        _namespaceURI: &str,
        _localName: &str,
        _qualifiedName: &str,
        _attributes: &dyn Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onEnd_with_id(
        &mut self,
        elementId: i32,
        namespaceURI: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        SectPrHandler::onEnd(self, elementId, namespaceURI);
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
    fn getElementHandler(&self, _subElement: i32, _ns: &str) -> Option<ElementHandlerRef> {
        // Java: default: return super.getElementHandler(elementId, namespaceURI);
        // java PropHandler → PropChildHandler（docGrid / pgSz / pgMar）
        let importer = self.base.doc_importer()?;
        Some(Rc::new(RefCell::new(PropChildHandler::new(
            importer,
            self.base.mHandlerHelper.clone(),
            self.base.mProp.clone(),
        ))))
    }
}
