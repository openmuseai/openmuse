#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.assertion.IOAssert
use core_libs_util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.io.simple.ElementHandlerSimple
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
use super::super::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;
// Java: import cn.wps.moffice.property.MutablePropertySet
// Java: import org.xml.sax.Attributes
use super::IPropHandlerHelper::IPropHandlerHelper;
use super::Prop::Prop;
use super::PropBaseHandler::PropBaseHandler;

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

// Java: public class PropChangeHandler extends XmlSimpleNodeElementHandler
pub struct PropChangeHandler {
    base: XmlSimpleNodeElementHandler,
    // Java field: IProp mIProp
    mIProp: Rc<RefCell<Prop>>,
    // Java field: IPropHandlerHelper mHandlerHelper
    mHandlerHelper: Rc<dyn IPropHandlerHelper>,
    // Java field: PropBaseHandler mPropBaseHandler
    mPropBaseHandler: RefCell<Option<ElementHandlerRef>>,
}

impl PropChangeHandler {
    // Java: PropChangeHandler(IDocumentImporter importer, IPropHandlerHelper handlerHelper, IProp iProp)
    pub fn new_rc(
        importer: Rc<dyn IDocumentImporter>,
        handler_helper: Rc<dyn IPropHandlerHelper>,
        i_prop: Rc<RefCell<Prop>>,
    ) -> Self {
        // Java: super(importer);
        // Java: this.mIProp = iProp;
        // Java: this.mHandlerHelper = handlerHelper;
        PropChangeHandler {
            base: XmlSimpleNodeElementHandler::new_from_doc_rc(importer),
            mIProp: i_prop,
            mHandlerHelper: handler_helper,
            mPropBaseHandler: RefCell::new(None),
        }
    }

    pub fn as_handler_ref(self) -> ElementHandlerRef {
        Rc::new(RefCell::new(self))
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, elementId: i32, namespaceURI: &str, _localName: &str, _qualifiedName: &str, attributes: &dyn Any) {
        // Java: MutablePropertySet prop = this.mIProp.getProp();
        let mut i_prop = self.mIProp.borrow_mut();
        let prop = i_prop.getChangeHandler().getProp();
        // Java: PropHandlerHelper.PropType propType = this.mHandlerHelper.getPropType();
        let propType = self.mHandlerHelper.getPropType();
        // Java: this.mDocImporter.convertProp(propType, prop, namespaceURI, elementId, attributes);
        if let Some(importer) = self.base.mDocImporter.as_ref() {
            importer.convertProp(propType as i32, prop, namespaceURI, elementId, attributes);
        }
    }

    // Java: ElementHandlerSimple getElementHandler(int subElement, String namespaceURI)
    pub fn getElementHandler(&self, subElement: i32, _namespaceURI: &str) -> Option<ElementHandlerRef> {
        // Java: if (this.mHandlerHelper.isBaseElement(subElement))
        if self.mHandlerHelper.isBaseElement(subElement) {
            // Java: return this.getBaseHandler();
            self.getBaseHandler()
        } else {
            // Java: IOAssert.fail("It should only contain base tag!");
            IOAssert::fail_msg("It should only contain base tag!");
            // Java: return null;
            None
        }
    }

    // Java: ElementHandlerSimple getBaseHandler()
    fn getBaseHandler(&self) -> Option<ElementHandlerRef> {
        // Java: if (null == this.mPropBaseHandler)
        let mut slot = self.mPropBaseHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mPropBaseHandler = new PropBaseHandler(this.mDocImporter, this.mHandlerHelper, this.mIProp);
            let importer = self.base.mDocImporter.clone()?;
            *slot = Some(
                PropBaseHandler::new(importer, self.mHandlerHelper.clone(), self.mIProp.clone())
                    .bind_change()
                    .as_handler_ref(),
            );
        }
        // Java: return this.mPropBaseHandler;
        slot.clone()
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: int elementId = this.mHandlerHelper.filterChangeIndex();
        let elementId = self.mHandlerHelper.filterChangeIndex();
        // Java: return elementId != 0 ? elementId : super.filterIndex();
        if elementId != 0 {
            elementId
        } else {
            self.base.filterIndex()
        }
    }
}

impl ElementHandlerSimple for PropChangeHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { PropChangeHandler::filterIndex(self) }
    fn onStart(&mut self, _a: &str, _b: &str, _c: &str, _d: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _a: &str, _b: &str, _c: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, element_id: i32, namespace_uri: &str, local_name: &str, qualified_name: &str, attributes: &dyn Any) -> Result<(), Box<dyn std::error::Error>> {
        PropChangeHandler::onStart(self, element_id, namespace_uri, local_name, qualified_name, attributes);
        Ok(())
    }
    fn onEnd_with_id(&mut self, _id: i32, _ns: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, b: &[char], s: i32, l: i32) -> Result<(), Box<dyn std::error::Error>> { let _ = (b, s, l); Ok(()) }
    fn onCharacters_str(&mut self, s: &str) -> Result<(), Box<dyn std::error::Error>> { let _ = s; Ok(()) }
    fn getElementHandler(&self, id: i32, ns: &str) -> Option<ElementHandlerRef> {
        PropChangeHandler::getElementHandler(self, id, ns)
    }
}
