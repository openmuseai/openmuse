#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use super::super::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;
use super::IPropHandlerHelper::IPropHandlerHelper;
use super::Prop::Prop;
use super::PropChildHandler::PropChildHandler;
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;

// Java: public class PropBaseHandler extends XmlSimpleNodeElementHandler
pub struct PropBaseHandler {
    pub(crate) base: XmlSimpleNodeElementHandler,
    // Java field: IProp mProp
    pub(crate) mProp: Rc<RefCell<Prop>>,
    // Java field: IPropBaseHandlerHelper mHandlerHelper
    pub(crate) mHandlerHelper: Rc<dyn IPropHandlerHelper>,
    // Java field: PropChildHandler mChildHandler
    pub(crate) mChildHandler: Option<Box<PropChildHandler>>,
    // rust: Java 传入的 IProp 是修订 PropBase 时为 true
    target_change: bool,
    mSaxChild: RefCell<Option<ElementHandlerRef>>,
}

impl PropBaseHandler {
    // Java: PropBaseHandler(IDocumentImporter importer, IPropBaseHandlerHelper handlerHelper, IProp iProp)
    pub fn new(
        importer: Rc<dyn IDocumentImporter>,
        handlerHelper: Rc<dyn IPropHandlerHelper>,
        iProp: Rc<RefCell<Prop>>,
    ) -> Self {
        // Java: super(importer);
        // Java: this.mProp = iProp;
        // Java: this.mHandlerHelper = handlerHelper;
        PropBaseHandler {
            base: XmlSimpleNodeElementHandler::new_from_doc_rc(importer),
            mProp: iProp,
            mHandlerHelper: handlerHelper,
            mChildHandler: None,
            target_change: false,
            mSaxChild: RefCell::new(None),
        }
    }

    pub(crate) fn bind_change(mut self) -> Self {
        self.target_change = true;
        self
    }

    pub fn as_handler_ref(self) -> ElementHandlerRef {
        Rc::new(RefCell::new(self))
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, _elementId: i32, _namespaceURI: &str, _localName: &str, _qualifiedName: &str, _attributes: &dyn std::any::Any) {
    }

    // Java: ElementHandlerSimple getElementHandler(int subElement, String namespaceURI)
    pub fn getElementHandler(&mut self, _subElement: i32, _namespaceURI: &str) -> Option<&mut PropChildHandler> {
        // Java: if (null == this.mChildHandler)
        if self.mChildHandler.is_none() {
            // Java: this.mChildHandler = new PropChildHandler(this.mDocImporter, this.mHandlerHelper, this.mProp);
            let importer = self.base.mDocImporter.clone().expect("mDocImporter");
            let mut child = PropChildHandler::new(
                importer,
                self.mHandlerHelper.clone(),
                self.mProp.clone(),
            );
            child.target_change = self.target_change;
            self.mChildHandler = Some(Box::new(child));
        }
        // Java: return this.mChildHandler;
        self.mChildHandler.as_mut().map(|c| c.as_mut())
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 0;
        0
    }
}

impl ElementHandlerSimple for PropBaseHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { PropBaseHandler::filterIndex(self) }
    fn onStart(&mut self, _a: &str, _b: &str, _c: &str, _d: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _a: &str, _b: &str, _c: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, element_id: i32, namespace_uri: &str, local_name: &str, qualified_name: &str, attributes: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        PropBaseHandler::onStart(self, element_id, namespace_uri, local_name, qualified_name, attributes);
        Ok(())
    }
    fn onEnd_with_id(&mut self, _id: i32, _ns: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, b: &[char], s: i32, l: i32) -> Result<(), Box<dyn std::error::Error>> { let _ = (b, s, l); Ok(()) }
    fn onCharacters_str(&mut self, s: &str) -> Result<(), Box<dyn std::error::Error>> { let _ = s; Ok(()) }
    fn getElementHandler(&self, _sub_element: i32, _ns: &str) -> Option<ElementHandlerRef> {
        let mut slot = self.mSaxChild.borrow_mut();
        if slot.is_none() {
            let importer = self.base.mDocImporter.clone()?;
            let mut child = PropChildHandler::new(importer, self.mHandlerHelper.clone(), self.mProp.clone());
            child.target_change = self.target_change;
            *slot = Some(Rc::new(RefCell::new(child)));
        }
        slot.clone()
    }
}
