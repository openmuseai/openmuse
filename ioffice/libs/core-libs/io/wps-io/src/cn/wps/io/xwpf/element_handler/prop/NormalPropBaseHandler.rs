#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use super::IPropHandlerHelper::IPropHandlerHelper;
use super::Prop::Prop;
use super::PropBaseHandler::PropBaseHandler;
use super::PropChildHandler::PropChildHandler;
use super::PropHandlerHelper::PprGeneralBaseHandlerHelper;
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
use crate::io::xwpf::element_handler::PprHandlerHelper::PprHandlerHelper;
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};

// Java: public class NormalPropBaseHandler extends PropBaseHandler implements IPropHandler
pub struct NormalPropBaseHandler {
    pub(crate) base: PropBaseHandler,
}

impl NormalPropBaseHandler {
    // Java: NormalPropBaseHandler(IDocumentImporter importer, IPropBaseHandlerHelper handlerHelper)
    // rust: helper 在 Rust 侧统一取 PprHandlerHelper（PropBaseHandler::new 要求 Rc<dyn IPropHandlerHelper>；
    //       PprGeneral/TcPrStyle/TblPrBase 三个专用单例未实现该 trait，沿用本文件既有 new_with 约定）
    pub fn new_rc(importer: Rc<dyn IDocumentImporter>) -> Self {
        let helper: Rc<dyn IPropHandlerHelper> = PprHandlerHelper::getPprHandlerHelper();
        NormalPropBaseHandler {
            base: PropBaseHandler::new(
                importer,
                helper,
                Rc::new(RefCell::new(Prop::new())),
            ),
        }
    }

    // Java: new NormalPropBaseHandler(importer,
    // PropHandlerHelper.PprGeneralBaseHandlerHelper.geHandlerHelper())
    pub fn new_ppr_general_rc(importer: Rc<dyn IDocumentImporter>) -> Self {
        NormalPropBaseHandler {
            base: PropBaseHandler::new(
                importer,
                Rc::new(PprGeneralBaseHandlerHelper::new()),
                Rc::new(RefCell::new(Prop::new())),
            ),
        }
    }

    // Java: public void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, _element_id: i32, _namespace_uri: &str, _local_name: &str, _qualified_name: &str, _attributes: &dyn Any) {
        // Java: this.mProp.getProp();
        let _ = self.base.mProp.borrow_mut().getProp();
    }

    // Java: public void onEnd(int elementId, String namespaceURI)
    pub fn onEnd(&self, element_id: i32, _namespace_uri: &str) {
        // Java: if (this.mProp.isContainProp())
        let mut prop = self.base.mProp.borrow_mut();
        if prop.isContainProp() {
            // Java: this.mDocImporter.correctProps(elementId, this.mProp.getProp(), (MutablePropertySet)null);
            if let Some(ref importer) = self.base.base.mDocImporter {
                let _ = importer.correctProps(element_id, &*prop.getProp(), None);
            }
        }
    }

    // Java: public MutablePropertySet getProp()
    pub fn getProp(&mut self) -> Option<Box<dyn Any + Send + Sync>> {
        // Java: return this.mProp.isContainProp() ? this.mProp.getProp() : null;
        let mut prop = self.base.mProp.borrow_mut();
        if prop.isContainProp() {
            Some(Box::new(prop.getProp().clone_mps()))
        } else {
            None
        }
    }

    // Java: public void clearProp()
    pub fn clearProp(&self) {
        // Java: this.mProp.clearProp();
        self.base.mProp.borrow_mut().clearProp();
    }
}

impl ElementHandlerSimple for NormalPropBaseHandler {
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
        element_id: i32,
        namespace_uri: &str,
        local_name: &str,
        qualified_name: &str,
        attributes: &dyn Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        NormalPropBaseHandler::onStart(self, element_id, namespace_uri, local_name, qualified_name, attributes);
        Ok(())
    }
    fn onEnd_with_id(
        &mut self,
        element_id: i32,
        namespace_uri: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        NormalPropBaseHandler::onEnd(self, element_id, namespace_uri);
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
    fn getElementHandler(&self, _id: i32, _ns: &str) -> Option<ElementHandlerRef> {
        // Java: PropBaseHandler.getElementHandler -> mChildHandler;
        // the child shares mProp, so parsed w:jc reaches StylePrModel.
        let importer = self.base.base.mDocImporter.clone()?;
        Some(Rc::new(RefCell::new(PropChildHandler::new(
            importer, self.base.mHandlerHelper.clone(), self.base.mProp.clone(),
        ))))
    }
}
