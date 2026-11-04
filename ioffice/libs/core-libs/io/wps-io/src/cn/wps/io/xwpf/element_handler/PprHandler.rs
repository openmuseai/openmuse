#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use super::prop::PropChildHandler::PropChildHandler;
use super::prop::PropHandler::PropHandler;
use super::PprHandlerHelper::PprHandlerHelper;
use super::RprHandler::RprHandler;
use super::SectPrHandler::SectPrHandler;
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
use crate::io::xwpf::element_handler::prop::IPropHandlerHelper::IPropHandlerHelper;
use core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet;

pub enum PprSubHandler<'a> {
    PropChild(&'a mut PropChildHandler),
    Rpr(&'a mut RprHandler),
    SectPr(&'a mut SectPrHandler),
}

// Java: public class PprHandler extends PropHandler implements IPropHandler
pub struct PprHandler {
    pub(crate) base: PropHandler,
    // Java field: POIXMLDocumentPart mPart
    pub(crate) mPart: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: MutablePropertySet mPropertySet
    pub(crate) mPropertySet: Option<MutablePropertySet>,
    // Java field: RprHandler mRprHandler
    pub(crate) mRprHandler: Option<Box<RprHandler>>,
    // Rust SAX trait adapter for Java's mRprHandler object identity.
    mRprHandlerProduct: RefCell<Option<ElementHandlerRef>>,
    // Java field: SectPrHandler mSectPrHandler
    pub(crate) mSectPrHandler: Option<Box<SectPrHandler>>,
    // Rust SAX trait adapter retaining Java's mSectPrHandler object identity.
    mSectPrHandlerProduct: RefCell<Option<ElementHandlerRef>>,
    // Java field: int mShapeBelonger
    pub(crate) mShapeBelonger: i32,
}

impl PprHandler {
    // Java: PprHandler(POIXMLDocumentPart part, IDocumentImporter importer)
    pub fn new(part: Box<dyn std::any::Any + Send + Sync>, importer: Rc<dyn IDocumentImporter>) -> Self {
        // Java: super(importer, PprHandlerHelper.getPprHandlerHelper());
        let helper: Rc<dyn IPropHandlerHelper> = PprHandlerHelper::getPprHandlerHelper();
        PprHandler {
            base: PropHandler::new(importer, helper),
            // Java: this.mPart = part;
            mPart: Some(part),
            mPropertySet: None,
            mRprHandler: None,
            mRprHandlerProduct: RefCell::new(None),
            mSectPrHandler: None,
            mSectPrHandlerProduct: RefCell::new(None),
            mShapeBelonger: 0,
        }
    }

    // Java: ElementHandlerSimple getElementHandler(int subElement, String namespaceURI)
    pub fn getElementHandler(&mut self, subElement: i32, namespaceURI: &str) -> Option<PprSubHandler<'_>> {
        // Java: if (this.isGVml())
        if self.base.isGVml() {
            // Java: return this.getElementHandler4GVml(subElement);
            return None;
        }
        // Java: switch (subElement)
        match subElement {
            // Java: case -906275931:
            -906275931 => {
                // Java: return this.mSectPrHandler = new SectPrHandler(this.mPart, this.mDocImporter);
                if let Some(importer) = self.base.doc_importer() {
                    self.mSectPrHandler = Some(Box::new(SectPrHandler::new(importer)));
                }
                self.mSectPrHandler.as_deref_mut().map(PprSubHandler::SectPr)
            }
            // Java: case 112148:
            112148 => {
                // Java: return this.getRprHandler();
                Some(PprSubHandler::Rpr(self.getRprHandler()))
            }
            // Java: default:
            _ => {
                // Java: return super.getElementHandler(subElement, namespaceURI);
                PropHandler::getElementHandler(&mut self.base, subElement, namespaceURI)
                    .map(PprSubHandler::PropChild)
            }
        }
    }

    // Java: RprHandler getRprHandler()
    pub fn getRprHandler(&mut self) -> &mut RprHandler {
        // Java: if (null == this.mRprHandler)
        if self.mRprHandler.is_none() {
            // Java: this.mRprHandler = new RprHandler(this.mDocImporter);
            let importer = self.base.doc_importer().expect("mDocImporter");
            self.mRprHandler = Some(Box::new(RprHandler::new(importer)));
        }
        // Java: return this.mRprHandler;
        self.mRprHandler.as_mut().unwrap()
    }

    // Java: void clearProp()
    pub fn clearProp(&mut self) {
        // Java: this.mProp.clearProp();
        self.base.mProp.borrow_mut().clearProp();
        // Java: this.mPropertySet = null;
        self.mPropertySet = None;
        // Java: if (null != this.mSectPrHandler)
        // Java: this.mSectPrHandler.clearProp();
        if let Some(ref mut sect) = self.mSectPrHandler {
            sect.clearProp();
        }
        if let Some(ref handler) = *self.mSectPrHandlerProduct.borrow() {
            if let Some(sect) = handler.borrow_mut().as_any_mut().downcast_mut::<SectPrHandler>() {
                sect.clearProp();
            }
        }
        // Java: if (null != this.mRprHandler)
        if let Some(ref mut rpr) = self.mRprHandler {
            // Java: this.mRprHandler.clearProp();
            rpr.clearProp();
        }
        if let Some(ref handler) = *self.mRprHandlerProduct.borrow() {
            if let Some(rpr) = handler.borrow_mut().as_any_mut().downcast_mut::<RprHandler>() {
                rpr.clearProp();
            }
        }
    }

    // Java: MutablePropertySet getProp()
    pub fn getProp(&self) -> Option<&MutablePropertySet> {
        // Java: return this.mPropertySet;
        self.mPropertySet.as_ref()
    }

    // Java: MutablePropertySet getRunProp()
    pub fn getRunProp(&self) -> Option<MutablePropertySet> {
        // Java: return null == this.mRprHandler ? null : this.mRprHandler.getProp();
        if let Some(ref handler) = *self.mRprHandlerProduct.borrow() {
            return handler.borrow().as_any().downcast_ref::<RprHandler>().map(|h| h.getProp());
        }
        self.mRprHandler.as_ref().map(|h| h.getProp())
    }

    // Java: MutablePropertySet getSectProp()
    pub fn getSectProp(&self) -> Option<MutablePropertySet> {
        // Java: return null == this.mSectPrHandler ? null : this.mSectPrHandler.getProp();
        if let Some(ref handler) = *self.mSectPrHandlerProduct.borrow() {
            if let Some(sect) = handler.borrow().as_any().downcast_ref::<SectPrHandler>() {
                if let Some(prop) = sect.getProp() {
                    return Some(prop.clone_mps());
                }
            }
        }
        self.mSectPrHandler.as_ref()
            .and_then(|handler| handler.getProp())
            .map(|prop| prop.clone_mps())
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&mut self, elementId: i32, namespaceURI: &str, _localName: &str, _qualifiedName: &str, attributes: &dyn std::any::Any) {
        // Java: if (this.isGVml() || 1556194562 == this.mShapeBelonger)
        if self.base.isGVml() || 1556194562 == self.mShapeBelonger {
            // Java: this.mDocImporter.convertProp(PropHandlerHelper.PropType.PPR, this.mProp.getProp(), namespaceURI, elementId, attributes);
            if let Some(importer) = self.base.doc_importer() {
                let mut g = self.base.mProp.borrow_mut();
                importer.convertProp(1, g.getProp(), namespaceURI, elementId, attributes);
            }
        }
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd(&mut self, elementId: i32, _namespaceURI: &str) {
        // Java: if (this.isGVml())
        if self.base.isGVml() {
            // Java: this.mPropertySet = ((IGVmlDocImporter)this.mDocImporter).adjustParaProps(this.mProp.getProp());
            return;
        }
        // Java: MutablePropertySet prop = this.mProp.getProp();
        // Java: MutablePropertySet originalProp = this.mProp.getChangeProp();
        // Java: this.mPropertySet = this.mDocImporter.correctProps(elementId, prop, originalProp);
        if let Some(importer) = self.base.doc_importer() {
            let mut g = self.base.mProp.borrow_mut();
            let original: Option<&dyn std::any::Any> = None;
            let _ = g.getChangeProp();
            let boxed = importer.correctProps(elementId, g.getProp(), original);
            self.mPropertySet = boxed.downcast::<MutablePropertySet>().ok().map(|b| *b);
        }
        if self.mPropertySet.is_none() {
            self.mPropertySet = Some(self.base.mProp.borrow_mut().getProp().clone_mps());
        }
    }
}

impl ElementHandlerSimple for PprHandler {
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
        PprHandler::onStart(self, elementId, namespaceURI, localName, qualifiedName, attributes);
        Ok(())
    }
    fn onEnd_with_id(
        &mut self,
        elementId: i32,
        namespaceURI: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        PprHandler::onEnd(self, elementId, namespaceURI);
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
    fn getElementHandler(&self, subElement: i32, _ns: &str) -> Option<ElementHandlerRef> {
        // Java: case -906275931: return this.mSectPrHandler = new SectPrHandler(this.mPart, this.mDocImporter);
        if subElement == -906275931 {
            let handler = SectPrHandler::new(self.base.doc_importer()?).as_handler_ref();
            *self.mSectPrHandlerProduct.borrow_mut() = Some(handler.clone());
            return Some(handler);
        }
        // Java: case 112148: return this.getRprHandler();
        if subElement == 112148 {
            let mut slot = self.mRprHandlerProduct.borrow_mut();
            if slot.is_none() {
                // Java: this.mRprHandler = new RprHandler(this.mDocImporter);
                *slot = Some(Rc::new(RefCell::new(RprHandler::new(self.base.doc_importer()?))));
            }
            return slot.clone();
        }
        // Java: default: return super.getElementHandler(subElement, namespaceURI);
        let importer = self.base.doc_importer()?;
        Some(Rc::new(RefCell::new(PropChildHandler::new(
            importer,
            self.base.mHandlerHelper.clone(),
            self.base.mProp.clone(),
        ))))
    }
}
