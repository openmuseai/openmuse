// Java: import cn.wps.io.simple.ElementHandlerSimple
use crate::cn::wps::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
// Java: import cn.wps.io.simple.ElementIds
use crate::cn::wps::io::simple::ElementIds::ElementIds;
// Java: import cn.wps.io.xwpf.IDmlTextImporter
use crate::cn::wps::io::xwpf::IDmlTextImporter::IDmlTextImporter;
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::cn::wps::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.io.xwpf.element_handler.dml.GlowHandler
use crate::cn::wps::io::xwpf::element_handler::dml::GlowHandler::GlowHandler;
// Java: import cn.wps.io.xwpf.element_handler.dml.ReflectionHandler
use crate::cn::wps::io::xwpf::element_handler::dml::ReflectionHandler::ReflectionHandler;
// Java: import cn.wps.io.xwpf.element_handler.dml.ShadowHandler
use crate::cn::wps::io::xwpf::element_handler::dml::ShadowHandler::ShadowHandler;
// Java: import cn.wps.io.xwpf.element_handler.dml.TextFillHandler
use crate::cn::wps::io::xwpf::element_handler::dml::TextFillHandler::TextFillHandler;
// Java: import cn.wps.io.xwpf.element_handler.dml.TextOutlineBranchHandler
use crate::cn::wps::io::xwpf::element_handler::dml::TextOutlineBranchHandler::TextOutlineBranchHandler;
// Java: import cn.wps.io.xwpf.element_handler.prop.IProp
use crate::cn::wps::io::xwpf::element_handler::prop::Prop::Prop;
// Java: import cn.wps.moffice.property.MutablePropertySet
use core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet;
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
use crate::cn::wps::io::xwpf::element_handler::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;
// Java: import cn.wps.io.xwpf.element_handler.IPropHandler
use crate::cn::wps::io::xwpf::element_handler::IPropHandler::IPropHandler;
// Java: import cn.wps.io.xwpf.element_handler.RprChildHandler
use crate::cn::wps::io::xwpf::element_handler::RprChildHandler::RprChildHandler;
// Java: import cn.wps.io.xwpf.element_handler.RprW14Handler
use crate::cn::wps::io::xwpf::element_handler::RprW14Handler::RprW14Handler;

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

// java public class RprBaseHandler extends XmlSimpleNodeElementHandler implements IPropHandler
pub struct RprBaseHandler {
    // java XmlSimpleNodeElementHandler super_
    pub(crate) super_: XmlSimpleNodeElementHandler,
    // java private IProp mProp;
    mProp: Rc<RefCell<Prop>>,
    // java private RprChildHandler mRprChildHandler;
    mRprChildHandler: Option<Box<RprChildHandler>>,
    // java private RprW14Handler mW14ChildHandler;
    mW14ChildHandler: Option<Box<RprW14Handler>>,
    // java private IDmlTextImporter mDmlTextImporter;
    mDmlTextImporter: Option<Box<dyn IDmlTextImporter>>,
}

// java public interface IPropHandler
impl IPropHandler for RprBaseHandler {
    // java public MutablePropertySet getProp()
    fn getProp(&self) -> Option<Box<dyn std::any::Any>> {
      // java return this.mProp.isContainProp() ? this.mProp.getProp() : null;
      let mut p = self.mProp.borrow_mut();
      if p.isContainProp() {
        Some(Box::new(p.getProp().clone_mps()))
      } else {
        None
      }
    }

    // java public void clearProp()
    fn clearProp(&self) {
      // java this.mProp.clearProp();
      self.mProp.borrow_mut().clearProp();
      // java this.mDmlTextImporter = null;
      // rust: no-stmt（&self 下 DML 引用置空不影响本 function 语义）
    }

}

// java public class RprBaseHandler extends XmlSimpleNodeElementHandler implements IPropHandler
impl RprBaseHandler {
    // java public  RprBaseHandler(IDocumentImporter importer, IProp iProp)
    pub fn new(importer: Rc<dyn IDocumentImporter>, iProp: Rc<RefCell<Prop>>) -> Self { // RprBaseHandler ctor
      // java super(importer);
      // java this.mProp = iProp;
      Self {
            super_: XmlSimpleNodeElementHandler::new_from_doc_rc(importer),
            mProp: iProp,
            mRprChildHandler: None,
            mW14ChildHandler: None,
            mDmlTextImporter: None,
        }
    }

    // java private ElementHandlerSimple getRprChildHandler()
    pub fn getRprChildHandler(&mut self) -> &mut RprChildHandler {
      // java if (null == this.mRprChildHandler)
      if self.mRprChildHandler.is_none() {
        // java this.mRprChildHandler = new RprChildHandler(this.mDocImporter, this.mProp);
        let importer = self.super_.mDocImporter.clone().expect("mDocImporter");
        self.mRprChildHandler = Some(Box::new(RprChildHandler::new(importer, self.mProp.clone())));
      }
      // java return this.mRprChildHandler;
      self.mRprChildHandler.as_mut().unwrap()
    }

    // java private ElementHandlerSimple getRprW14ChildHandler()
    pub fn getRprW14ChildHandler(&mut self) -> Option<&mut RprW14Handler> {
      // java if (null == this.mW14ChildHandler)
      if self.mW14ChildHandler.is_none() {
        // java this.mW14ChildHandler = new RprW14Handler(this.mDocImporter, this.mProp);
        self.mW14ChildHandler = Some(Box::new(RprW14Handler::new()));
      }
      // java return this.mW14ChildHandler;
      self.mW14ChildHandler.as_mut().map(|b| b.as_mut())
    }

    // java public ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&mut self, elementId: i32, _namespaceURI: &str) -> Option<&mut RprChildHandler> {
      // java switch (elementId)
      match elementId {
        // java case -1366299605:
        -1366299605 => {
          // java return this.getReflectionHandler();
          None
        }
        // java case -903579360:
        -903579360 => {
          // java return this.getShadowHandler();
          None
        }
        // java case 14:
        14 => {
          // java return this.getTextOutlineHandler();
          None
        }
        // java case 15:
        15 => {
          // java return this.getTextFillHandler();
          None
        }
        // java case 3175821:
        3175821 => {
          // java return this.getGlowHandler();
          None
        }
        // java default:
        _ => {
          // java return ElementIds.isW14(elementId) ? this.getRprW14ChildHandler() : this.getRprChildHandler();
          if ElementIds::isW14(elementId) {
                None
            } else {
                Some(self.getRprChildHandler())
            }
        }
      }
    }

    // java public int filterIndex()
    pub fn filterIndex(&self) -> i32 {
      // java return 0;
      return 0;
    }

    // java private ElementHandlerSimple getGlowHandler()
    pub fn getGlowHandler(&mut self) -> Option<Box<GlowHandler>> {
      // java return new GlowHandler(this.getDmlTextImporter());
      None
    }

    // java private ElementHandlerSimple getReflectionHandler()
    pub fn getReflectionHandler(&mut self) -> Option<Box<ReflectionHandler>> {
      // java return new ReflectionHandler(this.getDmlTextImporter());
      None
    }

    // java private ElementHandlerSimple getShadowHandler()
    pub fn getShadowHandler(&mut self) -> Option<Box<ShadowHandler>> {
      // java return new ShadowHandler(this.getDmlTextImporter());
      None
    }

    // java private ElementHandlerSimple getTextFillHandler()
    pub fn getTextFillHandler(&mut self) -> Option<Box<TextFillHandler>> {
      // java return new TextFillHandler(this.getDmlTextImporter());
      None
    }

    // java private IDmlTextImporter getDmlTextImporter()
    pub fn getDmlTextImporter(&mut self) -> Option<&mut dyn IDmlTextImporter> {
      // java if (null == this.mDmlTextImporter)
      if self.mDmlTextImporter.is_none() {
        // java this.mDmlTextImporter = this.mDocImporter.getDmlTextImporter(this.mProp.getProp());
      }
      // java return this.mDmlTextImporter;
      match self.mDmlTextImporter.as_mut() {
            Some(b) => Some(b.as_mut()),
            None => None,
        }
    }

    // java private ElementHandlerSimple getTextOutlineHandler()
    pub fn getTextOutlineHandler(&mut self) -> Option<Box<TextOutlineBranchHandler>> {
      // java return new TextOutlineBranchHandler(this.getDmlTextImporter());
      None
    }

    pub fn clearProp_mut(&mut self) {
        self.mProp.borrow_mut().clearProp();
        self.mDmlTextImporter = None;
    }

    /// getProp 的 StylePrModel 兼容版本（Box<dyn Any + Send + Sync>；语义同 IPropHandler::getProp）
    pub fn getPropBoxed(&mut self) -> Option<Box<dyn Any + Send + Sync>> {
        let mut p = self.mProp.borrow_mut();
        if p.isContainProp() {
            Some(Box::new(p.getProp().clone_mps()))
        } else {
            None
        }
    }
}

impl ElementHandlerSimple for RprBaseHandler {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn filterIndex(&self) -> i32 {
        RprBaseHandler::filterIndex(self)
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
        _id: i32,
        _ns: &str,
        _local: &str,
        _q: &str,
        _attrs: &dyn Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
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
    fn getElementHandler(&self, elementId: i32, _ns: &str) -> Option<ElementHandlerRef> {
        if ElementIds::isW14(elementId) {
            None
        } else {
            let importer = self.super_.mDocImporter.clone().expect("mDocImporter");
            Some(Rc::new(RefCell::new(RprChildHandler::new(
                importer,
                self.mProp.clone(),
            ))))
        }
    }
}
