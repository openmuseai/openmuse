#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.io.gvml.handler.HighlightHandler
// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IDmlTextImporter
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.io.xwpf.IGVmlDocImporter
// Java: import cn.wps.io.xwpf.element_handler.dml.GlowHandler
// Java: import cn.wps.io.xwpf.element_handler.dml.GradFillHandler
// Java: import cn.wps.io.xwpf.element_handler.dml.NoFillHandler
// Java: import cn.wps.io.xwpf.element_handler.dml.ReflectionHandler
// Java: import cn.wps.io.xwpf.element_handler.dml.ShadowHandler
// Java: import cn.wps.io.xwpf.element_handler.dml.SolidFillBranchHandler
// Java: import cn.wps.io.xwpf.element_handler.dml.TextFillHandler
// Java: import cn.wps.io.xwpf.element_handler.dml.TextOutlineBranchHandler
// Java: import cn.wps.io.xwpf.element_handler.prop.Prop
use super::prop::Prop::Prop;
// Java: import cn.wps.io.xwpf.element_handler.xml03.AnnotationHandler
// Java: import cn.wps.moffice.property.MutablePropertySet
use core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet;
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException

use super::RprBaseHandler::RprBaseHandler;
use super::RprChildHandler::RprChildHandler;
use super::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use std::any::Any;

// Java: public class RprHandler extends XmlSimpleNodeElementHandler
pub struct RprHandler {
    base: XmlSimpleNodeElementHandler,
    // Java field: RprBaseHandler mRprBaseHandler
    pub(crate) mRprBaseHandler: Option<Box<RprBaseHandler>>,
    // Java field: RprChangeHandler mRprChangeHandler
    pub(crate) mRprChangeHandler: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: AnnotationHandler mAnnotationHandler
    pub(crate) mAnnotationHandler: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: Prop mProp
    pub(crate) mProp: Rc<RefCell<Prop>>,
    // Java field: MutablePropertySet mCorrectProp
    pub(crate) mCorrectProp: Option<MutablePropertySet>,
    // Java field: IDmlTextImporter mDmlTextImporter
    pub(crate) mDmlTextImporter: Option<Box<dyn std::any::Any + Send + Sync>>,
}

// Rust SAX dispatch for Java RprHandler; child properties share the same mProp.
impl ElementHandlerSimple for RprHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { RprHandler::filterIndex(self) }
    fn onStart(&mut self, _a: &str, _b: &str, _c: &str, _d: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _a: &str, _b: &str, _c: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, id: i32, ns: &str, local: &str, name: &str, attrs: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        RprHandler::onStart(self, id, ns, local, name, attrs);
        Ok(())
    }
    fn onEnd_with_id(&mut self, id: i32, ns: &str) -> Result<(), Box<dyn std::error::Error>> {
        RprHandler::onEnd(self, id, ns);
        Ok(())
    }
    fn onCharacters(&mut self, _b: &[char], _s: i32, _l: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _s: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, elementId: i32, _ns: &str) -> Option<ElementHandlerRef> {
        // Java: return this.getRprBaseHandler().getElementHandler(subElement, namespaceURI);
        let importer = self.base.mDocImporter.clone()?;
        let base = RprBaseHandler::new(importer, self.mProp.clone());
        ElementHandlerSimple::getElementHandler(&base, elementId, _ns)
    }
}

// Java: public class RprHandler extends XmlSimpleNodeElementHandler
impl RprHandler {
    // Java: RprHandler(IDocumentImporter importer)
    pub fn new(importer: Rc<dyn IDocumentImporter>) -> Self {
        // Java: super(importer);
        RprHandler {
            base: XmlSimpleNodeElementHandler::new_from_doc_rc(importer),
            mRprBaseHandler: None,
            mRprChangeHandler: None,
            mAnnotationHandler: None,
            mProp: Rc::new(RefCell::new(Prop::new())),
            mCorrectProp: None,
            mDmlTextImporter: None,
        }
    }

    // Java: ElementHandlerSimple getElementHandler(int subElement, String namespaceURI)
    pub fn getElementHandler(&mut self, subElement: i32, namespaceURI: &str) -> Option<&mut RprChildHandler> {
        // Java: if (this.isGVml())
        if self.base.isGVml() {
            // Java: return this.getElementHandler4GVml(subElement);
            return None;
        }
        // Java: switch (subElement)
        match subElement {
            // Java: case -2087069980:
            -2087069980 => {
                // Java: return this.getRprChangeHandler();
                None
            }
            // Java: case -1555043537:
            -1555043537 => {
                // Java: return this.getAnnotationHandler();
                None
            }
            // Java: case -1366299605:
            -1366299605 => {
                // Java: return this.getReflectionHandler();
                None
            }
            // Java: case -903579360:
            -903579360 => {
                // Java: return this.getShadowHandler();
                None
            }
            // Java: case 14:
            14 => {
                // Java: return this.getTextOutlineHandler();
                None
            }
            // Java: case 15:
            15 => {
                // Java: return this.getTextFillHandler();
                None
            }
            // Java: case 3175821:
            3175821 => {
                // Java: return this.getGlowHandler();
                None
            }
            // Java: default:
            _ => {
                // Java: return this.getRprBaseHandler().getElementHandler(subElement, namespaceURI);
                self.getRprBaseHandler().getElementHandler(subElement, namespaceURI)
            }
        }
    }

    // Java: ElementHandlerSimple getRprBaseHandler()
    fn getRprBaseHandler(&mut self) -> &mut RprBaseHandler {
        // Java: if (null == this.mRprBaseHandler)
        if self.mRprBaseHandler.is_none() {
            // Java: this.mRprBaseHandler = new RprBaseHandler(this.mDocImporter, this.mProp);
            let importer = self.base.mDocImporter.clone().expect("mDocImporter");
            self.mRprBaseHandler = Some(Box::new(RprBaseHandler::new(importer, self.mProp.clone())));
        }
        // Java: return this.mRprBaseHandler;
        self.mRprBaseHandler.as_mut().unwrap()
    }

    // Java: void clearProp()
    pub fn clearProp(&mut self) {
        // Java: this.mCorrectProp = null;
        self.mCorrectProp = None;
        // Java: this.mProp.clearProp();
        self.mProp.borrow_mut().clearProp();
        // Java: this.mDmlTextImporter = null;
        self.mDmlTextImporter = None;
        // Java: if (null != this.mRprChangeHandler)
        // Java: this.mRprChangeHandler.clearProp();
    }

    // Java: MutablePropertySet getProp()
    pub fn getProp(&self) -> MutablePropertySet {
        // Java: return new MutablePropertySet(this.mCorrectProp);
        match &self.mCorrectProp {
            Some(p) => MutablePropertySet::new_from_mutable(p),
            None => MutablePropertySet::new(),
        }
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&mut self, elementId: i32, namespaceURI: &str, _localName: &str, _qualifiedName: &str, attributes: &dyn std::any::Any) {
        // Java: if (this.isGVml())
        if self.base.isGVml() {
            // Java: this.mDocImporter.convertRunProp(this.mProp.getProp(), namespaceURI, elementId, attributes);
            if let Some(importer) = self.base.mDocImporter.as_ref() {
                let mut g = self.mProp.borrow_mut();
                importer.convertRunProp(g.getProp(), namespaceURI, elementId, attributes);
            }
        }
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd(&mut self, elementId: i32, _namespaceURI: &str) {
        // Java: if (this.isGVml())
        if self.base.isGVml() {
            // Java: this.mCorrectProp = ((IGVmlDocImporter)this.mDocImporter).adjustRunProps(this.mProp.getProp());
            return;
        }
        // Java: MutablePropertySet prop = this.mProp.getProp();
        // Java: MutablePropertySet originalProp = this.mProp.getChangeProp();
        let has_original = {
            let mut g = self.mProp.borrow_mut();
            g.getChangeProp().is_some()
        };
        // Java: if (null != originalProp)
        if has_original {
            // Java: this.mCorrectProp = this.mDocImporter.correctProps(elementId, prop, originalProp);
            if let Some(importer) = self.base.mDocImporter.clone() {
                let mut g = self.mProp.borrow_mut();
                let boxed = importer.correctProps(elementId, g.getProp(), None);
                self.mCorrectProp = boxed.downcast::<MutablePropertySet>().ok().map(|b| *b);
            }
        } else {
            // Java: this.mCorrectProp = prop;
            let mut g = self.mProp.borrow_mut();
            self.mCorrectProp = Some(MutablePropertySet::new_from_mutable(g.getProp()));
        }
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 0;
        0
    }
}
