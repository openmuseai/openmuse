#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use super::super::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;
use super::IPropHandlerHelper::IPropHandlerHelper;
use super::IPropBaseHandlerHelper::IPropBaseHandlerHelper;
use super::Prop::Prop;
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
use core_libs_util::base::assertion::IOAssert::IOAssert;

// Java: class PropChildHandler extends XmlSimpleNodeElementHandler
pub struct PropChildHandler {
    pub(crate) base: XmlSimpleNodeElementHandler,
    // Java field: IProp mProp
    pub(crate) mProp: Rc<RefCell<Prop>>,
    // Java field: IPropBaseHandlerHelper mHandlerHelper
    pub(crate) mHandlerHelper: Rc<dyn IPropHandlerHelper>,
    // Java field: int mElementId
    pub(crate) mElementId: i32,
    // Java field: int mParentId
    pub mParentId: i32,
    // Java field: PropChildHandler mChildHandler
    pub(crate) mChildHandler: Option<Box<PropChildHandler>>,
    // rust: Java 的 IProp 若是 Prop.getChangeHandler()，getProp() 取修订集合
    pub(crate) target_change: bool,
}

impl PropChildHandler {
    // Java: PropChildHandler(IDocumentImporter importer, IPropBaseHandlerHelper handlerHelper, IProp iProp)
    pub fn new(
        importer: Rc<dyn IDocumentImporter>,
        handlerHelper: Rc<dyn IPropHandlerHelper>,
        iProp: Rc<RefCell<Prop>>,
    ) -> Self {
        // Java: super(importer);
        // Java: this.mProp = iProp;
        // Java: this.mHandlerHelper = handlerHelper;
        PropChildHandler {
            base: XmlSimpleNodeElementHandler::new_from_doc_rc(importer),
            mProp: iProp,
            mHandlerHelper: handlerHelper,
            mElementId: -1,
            mParentId: -1,
            mChildHandler: None,
            target_change: false,
        }
    }

    pub(crate) fn bind_change_base(mut self) -> Self {
        self.target_change = true;
        self
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&mut self, elementId: i32, namespaceURI: &str, _localName: &str, _qualifiedName: &str, attributes: &dyn std::any::Any) {
        // Java: this.mElementId = elementId;
        self.mElementId = elementId;
        // Java: IOAssert.assertNotNull("mHandlerHelper should not be null", this.mHandlerHelper);
        IOAssert::assertNotNull_msg("mHandlerHelper should not be null", &Some(()));
        // Java: IOAssert.assertNotNull("mProp should not be null", this.mProp);
        IOAssert::assertNotNull_msg("mProp should not be null", &Some(()));
        // Java: int propId = this.mHandlerHelper.convertProp(this.mParentId, this.mElementId);
        let propId = self.mHandlerHelper.convertProp(self.mParentId, self.mElementId);
        // Java: if (-1 != propId)
        if -1 != propId {
            // Java: PropHandlerHelper.PropType propType = this.mHandlerHelper.getPropType();
            let propType = self.mHandlerHelper.getPropType();
            // Java: MutablePropertySet prop = this.mProp.getProp();
            let mut prop_ref = self.mProp.borrow_mut();
            let prop = if self.target_change {
                prop_ref.getChangeHandler().getProp()
            } else {
                prop_ref.getProp()
            };
            // Java: this.mDocImporter.convertProp(propType, prop, namespaceURI, propId, attributes);
            if let Some(importer) = self.base.mDocImporter.as_ref() {
                importer.convertProp(propType as i32, prop, namespaceURI, propId, attributes);
            }
        }
    }

    // Java: ElementHandlerSimple getElementHandler(int subElement, String namespaceURI)
    pub fn getElementHandler(&mut self, subElement: i32, _namespaceURI: &str) -> Option<&mut PropChildHandler> {
        // Java: IOAssert.assertNotNull("mHandlerHelper should not be null", this.mHandlerHelper);
        IOAssert::assertNotNull_msg("mHandlerHelper should not be null", &Some(()));
        // Java: if (!this.mHandlerHelper.isGetThirdSubElementHandler(subElement))
        if !self.mHandlerHelper.isGetThirdSubElementHandler(subElement) {
            // Java: return null;
            return None;
        }
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
        // Java: this.mChildHandler.mParentId = this.filterIndex();
        let parent_id = self.filterIndex();
        if let Some(ref mut child) = self.mChildHandler {
            child.mParentId = parent_id;
            // Java: return this.mChildHandler;
            return Some(child.as_mut());
        }
        None
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return this.mElementId;
        self.mElementId
    }
}

impl ElementHandlerSimple for PropChildHandler {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn filterIndex(&self) -> i32 {
        PropChildHandler::filterIndex(self)
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
        PropChildHandler::onStart(self, elementId, namespaceURI, localName, qualifiedName, attributes);
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
    fn getElementHandler(&self, subElement: i32, _ns: &str) -> Option<ElementHandlerRef> {
        if !self.mHandlerHelper.isGetThirdSubElementHandler(subElement) {
            return None;
        }
        let importer = self.base.mDocImporter.clone()?;
        let mut child = PropChildHandler::new(importer, self.mHandlerHelper.clone(), self.mProp.clone());
        child.mParentId = self.mElementId;
        child.target_change = self.target_change;
        Some(Rc::new(RefCell::new(child)))
    }
}
