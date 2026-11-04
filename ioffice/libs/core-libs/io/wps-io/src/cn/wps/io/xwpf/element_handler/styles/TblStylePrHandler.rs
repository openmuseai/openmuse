#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.io.simple.ElementHandlerSimple
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.io.xwpf.element_handler.RprBaseHandler
use crate::io::xwpf::element_handler::RprBaseHandler::RprBaseHandler;
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
use crate::io::xwpf::element_handler::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;
// Java: import cn.wps.io.xwpf.element_handler.prop.NormalPropBaseHandler
use crate::io::xwpf::element_handler::prop::NormalPropBaseHandler::NormalPropBaseHandler;
// Java: import cn.wps.io.xwpf.element_handler.prop.PropBase
use crate::io::xwpf::element_handler::prop::Prop::Prop;
// Java: import cn.wps.io.xwpf.element_handler.prop.PropHandlerHelper
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use super::StylePrModel::StylePrModel;

// Java: class TblStylePrHandler extends XmlSimpleNodeElementHandler
pub struct TblStylePrHandler {
    base: XmlSimpleNodeElementHandler,
    // Java field: RprBaseHandler mRprHandler
    mRprHandler: RefCell<Option<Rc<RefCell<RprBaseHandler>>>>,
    // Java field: NormalPropBaseHandler mPprGeneralHandler
    mPprGeneralHandler: RefCell<Option<Rc<RefCell<NormalPropBaseHandler>>>>,
    // Java field: NormalPropBaseHandler mTcStylePrHandler
    mTcStylePrHandler: RefCell<Option<Rc<RefCell<NormalPropBaseHandler>>>>,
    // Java field: NormalPropBaseHandler mTblPrBaseHandler
    mTblPrBaseHandler: RefCell<Option<Rc<RefCell<NormalPropBaseHandler>>>>,
    // Java field: StylePrModel mStylePrModel
    mStylePrModel: RefCell<StylePrModel>,
}

// Java: class TblStylePrHandler extends XmlSimpleNodeElementHandler
impl TblStylePrHandler {
    pub fn new_rc(importer: Rc<dyn IDocumentImporter>) -> Self {
        // Java: super(importer);
        TblStylePrHandler {
            base: XmlSimpleNodeElementHandler::new_from_doc_rc(importer),
            mRprHandler: RefCell::new(None),
            mPprGeneralHandler: RefCell::new(None),
            mTcStylePrHandler: RefCell::new(None),
            mTblPrBaseHandler: RefCell::new(None),
            // Java: this.mStylePrModel = new StylePrModel();
            mStylePrModel: RefCell::new(StylePrModel::new()),
        }
    }

    pub fn as_handler_ref(importer: Rc<dyn IDocumentImporter>) -> ElementHandlerRef {
        Rc::new(RefCell::new(Self::new_rc(importer)))
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, elementId: i32, namespaceURI: &str, _localName: &str, _qualifiedName: &str, attributes: &dyn Any) {
        // Java: this.mDocImporter.onImportStyleChild(elementId, attributes);
        if let Some(ref importer) = self.base.mDocImporter {
            importer.onImportStyleChild(elementId, attributes);
        }
        let _ = namespaceURI;
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, namespaceURI: &str) -> Option<ElementHandlerRef> {
        // Java: switch (elementId)
        match elementId {
            // Java: case 110226:
            110226 => {
                // Java: return this.getPprHandler();
                Some(self.getPprHandler() as ElementHandlerRef)
            }
            // Java: case 112148:
            112148 => {
                // Java: return this.getRprHandler();
                Some(self.getRprHandler() as ElementHandlerRef)
            }
            // Java: case 3553489:
            3553489 => {
                // Java: return this.getTcStylePrHandler();
                Some(self.getTcStylePrHandler() as ElementHandlerRef)
            }
            // Java: case 3567904:
            3567904 => {
                // Java: return this.getTrPrHandler();
                let _ = namespaceURI;
                None
            }
            // Java: case 110154336:
            110154336 => {
                // Java: return this.getTblPrBaseHandler();
                Some(self.getTblPrBaseHandler() as ElementHandlerRef)
            }
            // Java: default:
            _ => {
                // Java: IOAssert.fail("it should not reach here");
                // rust: no-stmt（IOAssert.fail 语法无对应 Rust 语句，未知子元素不炸）
                let _ = namespaceURI;
                None
            }
        }
    }

    // Java: private ElementHandlerSimple getTcStylePrHandler()
    fn getTcStylePrHandler(&self) -> Rc<RefCell<NormalPropBaseHandler>> {
        // Java: if (null == this.mTcStylePrHandler)
        if self.mTcStylePrHandler.borrow().is_none() {
            // Java: this.mTcStylePrHandler = new NormalPropBaseHandler(this.mDocImporter, PropHandlerHelper.TcPrStyleBaseHandlerHelper.geHandlerHelper());
            let importer = self.base.mDocImporter.clone().expect("mDocImporter");
            *self.mTcStylePrHandler.borrow_mut() = Some(Rc::new(RefCell::new(NormalPropBaseHandler::new_rc(importer))));
        }
        // Java: return this.mTcStylePrHandler;
        self.mTcStylePrHandler.borrow().clone().expect("tc style pr handler")
    }

    // Java: private ElementHandlerSimple getTrPrHandler()
    fn getTrPrHandler(&self) -> Option<ElementHandlerRef> {
        // Java: return null;
        None
    }

    // Java: private ElementHandlerSimple getRprHandler()
    fn getRprHandler(&self) -> Rc<RefCell<RprBaseHandler>> {
        // Java: if (null == this.mRprHandler)
        if self.mRprHandler.borrow().is_none() {
            // Java: this.mRprHandler = new RprBaseHandler(this.mDocImporter, new PropBase());
            let importer = self.base.mDocImporter.clone().expect("mDocImporter");
            // rust: 构造函数取 PropBase（Java PropBase → Rust Prop，与 RprHandler 同一 ProP 用绳）
            *self.mRprHandler.borrow_mut() = Some(Rc::new(RefCell::new(RprBaseHandler::new(
                importer,
                Rc::new(RefCell::new(Prop::new())),
            ))));
        }
        // Java: return this.mRprHandler;
        self.mRprHandler.borrow().clone().expect("rpr handler")
    }

    // Java: private ElementHandlerSimple getPprHandler()
    fn getPprHandler(&self) -> Rc<RefCell<NormalPropBaseHandler>> {
        // Java: if (null == this.mPprGeneralHandler)
        if self.mPprGeneralHandler.borrow().is_none() {
            // Java: this.mPprGeneralHandler = new NormalPropBaseHandler(this.mDocImporter, PropHandlerHelper.PprGeneralBaseHandlerHelper.geHandlerHelper());
            let importer = self.base.mDocImporter.clone().expect("mDocImporter");
            *self.mPprGeneralHandler.borrow_mut() = Some(Rc::new(RefCell::new(NormalPropBaseHandler::new_ppr_general_rc(importer))));
        }
        // Java: return this.mPprGeneralHandler;
        self.mPprGeneralHandler.borrow().clone().expect("ppr handler")
    }

    // Java: private ElementHandlerSimple getTblPrBaseHandler()
    fn getTblPrBaseHandler(&self) -> Rc<RefCell<NormalPropBaseHandler>> {
        // Java: if (null == this.mTblPrBaseHandler)
        if self.mTblPrBaseHandler.borrow().is_none() {
            // Java: this.mTblPrBaseHandler = new NormalPropBaseHandler(this.mDocImporter, PropHandlerHelper.TblPrBaseHandlerHelper.getPropHandlerHelper());
            let importer = self.base.mDocImporter.clone().expect("mDocImporter");
            *self.mTblPrBaseHandler.borrow_mut() = Some(Rc::new(RefCell::new(NormalPropBaseHandler::new_rc(importer))));
        }
        // Java: return this.mTblPrBaseHandler;
        self.mTblPrBaseHandler.borrow().clone().expect("tbl pr base handler")
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 971752949;
        971752949
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd(&self, elementId: i32, namespaceURI: &str) {
        // Java: StylePrModel stylePrModel = this.getStylePrModel();
        let stylePrModel = self.getStylePrModel();
        // Java: this.mDocImporter.onImportStyleTblStylePrEnd(stylePrModel);
        if let Some(ref importer) = self.base.mDocImporter {
            importer.onImportStyleTblStylePrEnd(&stylePrModel);
        }
        // Java: this.clearProp();
        self.clearProp();
        let _ = (elementId, namespaceURI);
    }

    // Java: private StylePrModel getStylePrModel()
    fn getStylePrModel(&self) -> StylePrModel {
        // Java: IOAssert.assertNotNull(this.mStylePrModel);
        // Java: StylePrModel stylePrModel = this.mStylePrModel;
        let mut stylePrModel = StylePrModel::new();
        // Java: stylePrModel.mTblPrBaseSet = null == this.mTblPrBaseHandler ? null : this.mTblPrBaseHandler.getProp();
        stylePrModel.mTblPrBaseSet = match self.mTblPrBaseHandler.borrow().as_ref() {
            Some(h) => h.borrow_mut().getProp(),
            None => None,
        };
        // Java: stylePrModel.mPPrGeneralSet = null == this.mPprGeneralHandler ? null : this.mPprGeneralHandler.getProp();
        stylePrModel.mPPrGeneralSet = match self.mPprGeneralHandler.borrow().as_ref() {
            Some(h) => h.borrow_mut().getProp(),
            None => None,
        };
        // Java: stylePrModel.mTcPrSet = null == this.mTcStylePrHandler ? null : this.mTcStylePrHandler.getProp();
        stylePrModel.mTcPrSet = match self.mTcStylePrHandler.borrow().as_ref() {
            Some(h) => h.borrow_mut().getProp(),
            None => None,
        };
        // Java: stylePrModel.mRprBaseSet = null == this.mRprHandler ? null : this.mRprHandler.getProp();
        stylePrModel.mRprBaseSet = match self.mRprHandler.borrow().as_ref() {
            Some(h) => h.borrow_mut().getPropBoxed(),
            None => None,
        };
        // Java: return stylePrModel;
        stylePrModel
    }

    // Java: private void clearProp()
    fn clearProp(&self) {
        // Java: this.mStylePrModel.clear();
        self.mStylePrModel.borrow_mut().clear();
        // Java: if (null != this.mTblPrBaseHandler)
        if let Some(ref h) = *self.mTblPrBaseHandler.borrow() {
            // Java: this.mTblPrBaseHandler.clearProp();
            h.borrow_mut().clearProp();
        }
        // Java: if (null != this.mPprGeneralHandler)
        if let Some(ref h) = *self.mPprGeneralHandler.borrow() {
            // Java: this.mPprGeneralHandler.clearProp();
            h.borrow_mut().clearProp();
        }
        // Java: if (null != this.mTcStylePrHandler)
        if let Some(ref h) = *self.mTcStylePrHandler.borrow() {
            // Java: this.mTcStylePrHandler.clearProp();
            h.borrow_mut().clearProp();
        }
        // Java: if (null != this.mRprHandler)
        if let Some(ref h) = *self.mRprHandler.borrow() {
            // Java: this.mRprHandler.clearProp();
            h.borrow_mut().clearProp_mut();
        }
    }
}

impl ElementHandlerSimple for TblStylePrHandler {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn filterIndex(&self) -> i32 {
        TblStylePrHandler::filterIndex(self)
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
        TblStylePrHandler::onStart(self, elementId, namespaceURI, localName, qualifiedName, attributes);
        Ok(())
    }
    fn onEnd_with_id(
        &mut self,
        elementId: i32,
        namespaceURI: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        TblStylePrHandler::onEnd(self, elementId, namespaceURI);
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
        TblStylePrHandler::getElementHandler(self, id, ns)
    }
}
