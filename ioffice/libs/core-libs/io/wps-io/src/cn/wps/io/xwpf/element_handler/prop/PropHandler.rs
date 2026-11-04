#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use super::super::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;
use super::IPropHandlerHelper::IPropHandlerHelper;
use super::Prop::Prop;
use super::PropBaseHandler::PropBaseHandler;
use super::PropChangeHandler::PropChangeHandler;
use super::PropChildHandler::PropChildHandler;
use crate::io::simple::ElementHandlerSimple::ElementHandlerRef;
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;

// Java: public class PropHandler extends XmlSimpleNodeElementHandler
pub struct PropHandler {
    pub(crate) base: XmlSimpleNodeElementHandler,
    // Java field: Prop mProp
    pub(crate) mProp: Rc<RefCell<Prop>>,
    // Java field: IPropHandlerHelper mHandlerHelper
    pub(crate) mHandlerHelper: Rc<dyn IPropHandlerHelper>,
    // Java field: AnnotationHandler mAnnotationHandler
    pub(crate) mAnnotationHandler: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: PropBaseHandler mBaseHandler
    pub(crate) mBaseHandler: Option<PropBaseHandler>,
    // Java field: PropChangeHandler mChangeHandler
    pub(crate) mChangeHandler: Option<PropChangeHandler>,
    // rust: SAX 门面缓存 PropBaseHandler 的同一个 PropChildHandler
    mSaxChild: RefCell<Option<ElementHandlerRef>>,
    // rust: SAX 门面缓存 PropChangeHandler
    mSaxChange: RefCell<Option<ElementHandlerRef>>,
}

impl PropHandler {
    // Java: PropHandler(IDocumentImporter importer, IPropHandlerHelper handlerHelper)
    pub fn new(importer: Rc<dyn IDocumentImporter>, handlerHelper: Rc<dyn IPropHandlerHelper>) -> Self {
        // Java: this(importer, handlerHelper, new Prop());
        Self::new_with_prop(importer, handlerHelper, Prop::new())
    }

    // Java: PropHandler(IDocumentImporter importer, IPropHandlerHelper handlerHelper, Prop prop)
    pub fn new_with_prop(importer: Rc<dyn IDocumentImporter>, handlerHelper: Rc<dyn IPropHandlerHelper>, prop: Prop) -> Self {
        // Java: super(importer);
        // Java: this.mProp = prop;
        // Java: this.mHandlerHelper = handlerHelper;
        Self::new_shared(importer, handlerHelper, Rc::new(RefCell::new(prop)))
    }

    // rust: 与三参数构造器相同，但 Prop 由调用方共享
    pub fn new_shared(
        importer: Rc<dyn IDocumentImporter>,
        handlerHelper: Rc<dyn IPropHandlerHelper>,
        prop: Rc<RefCell<Prop>>,
    ) -> Self {
        // Java: super(importer);
        // Java: this.mProp = prop;
        PropHandler {
            base: XmlSimpleNodeElementHandler::new_from_doc_rc(importer),
            mProp: prop,
            mHandlerHelper: handlerHelper,
            mAnnotationHandler: None,
            mBaseHandler: None,
            mChangeHandler: None,
            mSaxChild: RefCell::new(None),
            mSaxChange: RefCell::new(None),
        }
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, _elementId: i32, _namespaceURI: &str, _localName: &str, _qualifiedName: &str, _attributes: &dyn std::any::Any) {
    }

    // Java: ElementHandlerSimple getElementHandler(int subElement, String namespaceURI)
    pub fn getElementHandler(&mut self, subElement: i32, namespaceURI: &str) -> Option<&mut PropChildHandler> {
        // Java: if (subElement == -1555043537)
        if subElement == -1555043537 {
            // Java: return this.getAnnotationHandler();
            return None;
        }
        // Java: return (ElementHandlerSimple)(this.mHandlerHelper.isChangeElement(subElement) ? this.getChangeHandler() : this.getBaseHandler().getElementHandler(subElement, namespaceURI));
        if self.mHandlerHelper.isChangeElement(subElement) {
            let _ = self.getChangeHandler();
            return None;
        }
        PropBaseHandler::getElementHandler(self.getBaseHandler(), subElement, namespaceURI)
    }

    // Java: ElementHandlerSimple getAnnotationHandler()
    fn getAnnotationHandler(&mut self) -> Option<ElementHandlerRef> {
        // Java: if (!this.isXml03())
        if !self.isXml03() {
            // Java: return null;
            return None;
        }
        // Java: if (this.mAnnotationHandler == null)
        // Java: this.mAnnotationHandler = new AnnotationHandler((IXml03Importer)this.mDocImporter, this.getChangeHandler(), (PropBaseHandler)this.getBaseHandler());
        // Java: return this.mAnnotationHandler;
        // rust: xml03 AnnotationHandler 未接（登记 §1.7.1）
        None
    }

    // Java: PropBaseHandler getBaseHandler()
    fn getBaseHandler(&mut self) -> &mut PropBaseHandler {
        // Java: if (null == this.mBaseHandler)
        if self.mBaseHandler.is_none() {
            // Java: this.mBaseHandler = new PropBaseHandler(this.mDocImporter, this.mHandlerHelper, this.mProp);
            let importer = self.base.mDocImporter.clone().expect("mDocImporter");
            self.mBaseHandler = Some(PropBaseHandler::new(
                importer,
                self.mHandlerHelper.clone(),
                self.mProp.clone(),
            ));
        }
        // Java: return this.mBaseHandler;
        self.mBaseHandler.as_mut().unwrap()
    }

    // Java: PropChangeHandler getChangeHandler()
    pub fn getChangeHandler(&mut self) -> &mut PropChangeHandler {
        // Java: if (null == this.mChangeHandler)
        if self.mChangeHandler.is_none() {
            // Java: this.mChangeHandler = new PropChangeHandler(this.mDocImporter, this.mHandlerHelper, this.mProp.getChangeHandler());
            let importer = self.base.mDocImporter.clone().expect("mDocImporter");
            self.mChangeHandler = Some(PropChangeHandler::new_rc(
                importer,
                self.mHandlerHelper.clone(),
                self.mProp.clone(),
            ));
        }
        // Java: return this.mChangeHandler;
        self.mChangeHandler.as_mut().unwrap()
    }

    pub fn isGVml(&self) -> bool {
        self.base.isGVml()
    }

    pub fn isXml03(&self) -> bool {
        self.base.isXml03()
    }

    pub fn doc_importer(&self) -> Option<Rc<dyn IDocumentImporter>> {
        self.base.mDocImporter.clone()
    }

    pub fn as_handler_ref(self) -> crate::io::simple::ElementHandlerSimple::ElementHandlerRef {
        std::rc::Rc::new(std::cell::RefCell::new(self))
    }
}

impl crate::io::simple::ElementHandlerSimple::ElementHandlerSimple for PropHandler {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
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
        _d: &dyn std::any::Any,
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
        attributes: &dyn std::any::Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        PropHandler::onStart(self, element_id, namespace_uri, local_name, qualified_name, attributes);
        Ok(())
    }
    fn onEnd_with_id(
        &mut self,
        _element_id: i32,
        _namespace_uri: &str,
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
    fn getElementHandler(
        &self,
        id: i32,
        _ns: &str,
    ) -> Option<ElementHandlerRef> {
        // rust: 与 inherent getElementHandler 同一分派。docx 的 annotation 为 null。
        if id == -1555043537 {
            return None;
        }
        if self.mHandlerHelper.isChangeElement(id) {
            let mut slot = self.mSaxChange.borrow_mut();
            if slot.is_none() {
                let importer = self.base.mDocImporter.clone()?;
                *slot = Some(
                    PropChangeHandler::new_rc(importer, self.mHandlerHelper.clone(), self.mProp.clone())
                        .as_handler_ref(),
                );
            }
            return slot.clone();
        }
        let mut slot = self.mSaxChild.borrow_mut();
        if slot.is_none() {
            let importer = self.base.mDocImporter.clone()?;
            *slot = Some(std::rc::Rc::new(std::cell::RefCell::new(PropChildHandler::new(
                importer,
                self.mHandlerHelper.clone(),
                self.mProp.clone(),
            ))));
        }
        slot.clone()
    }
}
