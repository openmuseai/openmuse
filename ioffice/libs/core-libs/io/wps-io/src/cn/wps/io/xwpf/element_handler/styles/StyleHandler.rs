#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

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
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use super::StyleChildHandler::StyleChildHandler;
use super::StylePrModel::StylePrModel;
use super::TblStylePrHandler::TblStylePrHandler;

// Java: class StyleHandler extends XmlSimpleNodeElementHandler
pub struct StyleHandler {
    base: XmlSimpleNodeElementHandler,
    // Java field: RprBaseHandler mRprBaseHandler
    mRprBaseHandler: RefCell<Option<Rc<RefCell<RprBaseHandler>>>>,
    // Java field: NormalPropBaseHandler mPprGeneralHandler
    mPprGeneralHandler: RefCell<Option<Rc<RefCell<NormalPropBaseHandler>>>>,
    // Java field: NormalPropBaseHandler mTblPrBaseHandler
    mTblPrBaseHandler: RefCell<Option<Rc<RefCell<NormalPropBaseHandler>>>>,
    // Java field: NormalPropBaseHandler mTcStylePrHandler
    mTcStylePrHandler: RefCell<Option<Rc<RefCell<NormalPropBaseHandler>>>>,
    // Java field: TblStylePrHandler mTblStylePrHandler
    mTblStylePrHandler: RefCell<Option<Rc<RefCell<TblStylePrHandler>>>>,
    // Java field: StyleChildHandler mStyleSimpleChildHandler
    mStyleSimpleChildHandler: RefCell<Option<Rc<RefCell<StyleChildHandler>>>>,
    // Java field: StylePrModel mStylePrModel
    mStylePrModel: RefCell<StylePrModel>,
}

// Java: class StyleHandler extends XmlSimpleNodeElementHandler
impl StyleHandler {
    pub fn new_rc(importer: Rc<dyn IDocumentImporter>) -> Self {
        // Java: super(importer);
        StyleHandler {
            base: XmlSimpleNodeElementHandler::new_from_doc_rc(importer),
            mRprBaseHandler: RefCell::new(None),
            mPprGeneralHandler: RefCell::new(None),
            mTblPrBaseHandler: RefCell::new(None),
            mTcStylePrHandler: RefCell::new(None),
            mTblStylePrHandler: RefCell::new(None),
            mStyleSimpleChildHandler: RefCell::new(None),
            // Java: this.mStylePrModel = new StylePrModel();
            mStylePrModel: RefCell::new(StylePrModel::new()),
        }
    }

    pub fn as_handler_ref(importer: Rc<dyn IDocumentImporter>) -> ElementHandlerRef {
        Rc::new(RefCell::new(Self::new_rc(importer)))
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, elementId: i32, namespaceURI: &str, localName: &str, qualifiedName: &str, attributes: &dyn Any) {
        // Java: this.mDocImporter.onImportStyleStart(attributes);
        if let Some(ref importer) = self.base.mDocImporter {
            importer.onImportStyleStart(attributes);
        }
        let _ = (elementId, namespaceURI, localName, qualifiedName);
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, namespaceURI: &str) -> Option<ElementHandlerRef> {
        // Java: switch (elementId)
        match elementId {
            // Java: case 110226:
            110226 => {
                // Java: return this.getPprGeneralHandler();
                Some(self.getPprGeneralHandler() as ElementHandlerRef)
            }
            // Java: case 112148:
            112148 => {
                // Java: return this.getRprBaseHandler();
                Some(self.getRprBaseHandler() as ElementHandlerRef)
            }
            // Java: case 3553489:
            3553489 => {
                // Java: return this.getTcPrHandler();
                Some(self.getTcPrHandler() as ElementHandlerRef)
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
            // Java: case 971752949:
            971752949 => {
                // Java: return this.getTblStylePrHandler();
                Some(self.getTblStylePrHandler() as ElementHandlerRef)
            }
            // Java: default:
            _ => {
                // Java: return this.getStyleSimpleChildHandler();
                Some(self.getStyleSimpleChildHandler() as ElementHandlerRef)
            }
        }
    }

    // Java: private ElementHandlerSimple getTblStylePrHandler()
    fn getTblStylePrHandler(&self) -> Rc<RefCell<TblStylePrHandler>> {
        // Java: if (null == this.mTblStylePrHandler)
        if self.mTblStylePrHandler.borrow().is_none() {
            // Java: this.mTblStylePrHandler = new TblStylePrHandler(this.mDocImporter);
            let importer = self.base.mDocImporter.clone().expect("mDocImporter");
            *self.mTblStylePrHandler.borrow_mut() = Some(Rc::new(RefCell::new(TblStylePrHandler::new_rc(importer))));
        }
        // Java: return this.mTblStylePrHandler;
        self.mTblStylePrHandler.borrow().clone().expect("tbl style pr handler")
    }

    // Java: private ElementHandlerSimple getTcPrHandler()
    fn getTcPrHandler(&self) -> Rc<RefCell<NormalPropBaseHandler>> {
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

    // Java: private ElementHandlerSimple getRprBaseHandler()
    fn getRprBaseHandler(&self) -> Rc<RefCell<RprBaseHandler>> {
        // Java: if (null == this.mRprBaseHandler)
        if self.mRprBaseHandler.borrow().is_none() {
            // Java: this.mRprBaseHandler = new RprBaseHandler(this.mDocImporter, new PropBase());
            let importer = self.base.mDocImporter.clone().expect("mDocImporter");
            // rust: 构造函数取 PropBase（Java PropBase → Rust Prop，与 RprHandler 同一 ProP 用绳）
            *self.mRprBaseHandler.borrow_mut() = Some(Rc::new(RefCell::new(RprBaseHandler::new(
                importer,
                Rc::new(RefCell::new(crate::io::xwpf::element_handler::prop::Prop::Prop::new())),
            ))));
        }
        // Java: return this.mRprBaseHandler;
        self.mRprBaseHandler.borrow().clone().expect("rpr base handler")
    }

    // Java: private ElementHandlerSimple getPprGeneralHandler()
    fn getPprGeneralHandler(&self) -> Rc<RefCell<NormalPropBaseHandler>> {
        // Java: if (null == this.mPprGeneralHandler)
        if self.mPprGeneralHandler.borrow().is_none() {
            // Java: this.mPprGeneralHandler = new NormalPropBaseHandler(this.mDocImporter, PropHandlerHelper.PprGeneralBaseHandlerHelper.geHandlerHelper());
            let importer = self.base.mDocImporter.clone().expect("mDocImporter");
            *self.mPprGeneralHandler.borrow_mut() = Some(Rc::new(RefCell::new(NormalPropBaseHandler::new_ppr_general_rc(importer))));
        }
        // Java: return this.mPprGeneralHandler;
        self.mPprGeneralHandler.borrow().clone().expect("ppr general handler")
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

    // Java: private ElementHandlerSimple getStyleSimpleChildHandler()
    fn getStyleSimpleChildHandler(&self) -> Rc<RefCell<StyleChildHandler>> {
        // Java: if (null == this.mStyleSimpleChildHandler)
        if self.mStyleSimpleChildHandler.borrow().is_none() {
            // Java: this.mStyleSimpleChildHandler = new StyleChildHandler(this.mDocImporter);
            let importer = self.base.mDocImporter.clone().expect("mDocImporter");
            *self.mStyleSimpleChildHandler.borrow_mut() = Some(Rc::new(RefCell::new(StyleChildHandler::new_rc(importer))));
        }
        // Java: return this.mStyleSimpleChildHandler;
        self.mStyleSimpleChildHandler.borrow().clone().expect("style child handler")
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd(&self, elementId: i32, namespaceURI: &str) {
        // Java: StylePrModel stylePrModel = this.getStylePrModel();
        let stylePrModel = self.getStylePrModel();
        // Java: this.mDocImporter.onImportStyleEnd(stylePrModel);
        if let Some(ref importer) = self.base.mDocImporter {
            importer.onImportStyleEnd(&stylePrModel);
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
        // Java: stylePrModel.mRprBaseSet = null == this.mRprBaseHandler ? null : this.mRprBaseHandler.getProp();
        stylePrModel.mRprBaseSet = match self.mRprBaseHandler.borrow().as_ref() {
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
        // Java: if (null != this.mRprBaseHandler)
        if let Some(ref h) = *self.mRprBaseHandler.borrow() {
            // Java: this.mRprBaseHandler.clearProp();
            h.borrow_mut().clearProp_mut();
        }
    }
}

impl ElementHandlerSimple for StyleHandler {
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
        StyleHandler::onStart(self, elementId, namespaceURI, localName, qualifiedName, attributes);
        Ok(())
    }
    fn onEnd_with_id(
        &mut self,
        elementId: i32,
        namespaceURI: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        StyleHandler::onEnd(self, elementId, namespaceURI);
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
        StyleHandler::getElementHandler(self, id, ns)
    }
}
