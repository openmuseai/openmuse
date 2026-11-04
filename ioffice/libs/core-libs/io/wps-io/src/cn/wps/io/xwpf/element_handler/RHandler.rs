#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.simple.ElementHandlerSimple
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.moffice.property.MutablePropertySet
use core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet;
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use super::prop::Prop::Prop;
use super::RprBaseHandler::RprBaseHandler;
use super::THandler::THandler;

// Java: public class RHandler extends RunInnerContentHandler implements IPropHandler
pub struct RHandler {
    // Java field: RprHandler mRprHandler
    pub(crate) mRprHandler: Option<Box<dyn Any>>,
    // Java field: RprHandler mMathRprHandler
    pub(crate) mMathRprHandler: Option<Box<dyn Any>>,
    // Java field: MutablePropertySet mRprProp
    pub(crate) mRprProp: Option<MutablePropertySet>,
    pub(crate) mRunProp: Rc<RefCell<Prop>>,
    pub(crate) mRprBase: RefCell<Option<ElementHandlerRef>>,
    // Java field: RHandler mRHandler
    pub(crate) mInnerRHandler: RefCell<Option<ElementHandlerRef>>,
    pub(crate) mTHandler: RefCell<Option<ElementHandlerRef>>,
    pub(crate) mPart: Rc<dyn Any>,
    pub(crate) mDocImporter: Option<Rc<dyn IDocumentImporter>>,
    pub(crate) mSubDocType: i32,
    pub(crate) mTableLayer: i32,
    pub(crate) mIsRuby: bool,
    // rust: ★P0/L0-2 —— Java `super.getElementHandler(subElement, this)` 需要把
    //   **本对象的引用**交给子 handler（`BrHandler.mRHandler` 用它取 run 属性）。
    //   用 `Weak` 保存自引用，避免 `Rc` 强引用环（`RHandler → BrHandler → RHandler`）
    //   造成永久泄漏。`upgrade()` 失败即对象已析构。
    pub(crate) mSelf: std::rc::Weak<RefCell<RHandler>>,
    // Java field: RunInnerContentHandler（父类实例，承载 getElementHandler 分发表）
    pub(crate) mInnerContent: RefCell<Option<Rc<super::RunInnerContentHandler::RunInnerContentHandler>>>,
}

// Java: public class RHandler extends RunInnerContentHandler implements IPropHandler
impl RHandler {
    // Java: RHandler(POIXMLDocumentPart part, IDocumentImporter importer, SubDocType subDocType, int tableLayer)
    pub fn new(
        part: Box<dyn Any>,
        importer: Rc<dyn IDocumentImporter>,
        subDocType: i32,
        tableLayer: i32,
    ) -> ElementHandlerRef {
        Self::new_rc(Rc::from(part), importer, subDocType, tableLayer)
    }

    // Java: RHandler(POIXMLDocumentPart part, IDocumentImporter importer, SubDocType subDocType, int tableLayer)
    pub fn new_rc(
        part: Rc<dyn Any>,
        importer: Rc<dyn IDocumentImporter>,
        subDocType: i32,
        tableLayer: i32,
    ) -> ElementHandlerRef {
        // Java: super(part, importer, subDocType, tableLayer);
        let rc = Rc::new(RefCell::new(RHandler {
            mRprHandler: None,
            mMathRprHandler: None,
            mRprProp: None,
            mRunProp: Rc::new(RefCell::new(Prop::new())),
            mRprBase: RefCell::new(None),
            mInnerRHandler: RefCell::new(None),
            mTHandler: RefCell::new(None),
            // Java: the same POIXMLDocumentPart reference is held by RHandler and its parent.
            mPart: part,
            mDocImporter: Some(importer),
            mSubDocType: subDocType,
            mTableLayer: tableLayer,
            mIsRuby: false,
            mSelf: std::rc::Weak::new(),
            mInnerContent: RefCell::new(None),
        }));
        // Java: `this` —— 子 handler 通过它回调本对象取属性。
        {
            let weak = Rc::downgrade(&rc);
            rc.borrow_mut().mSelf = weak;
        }
        rc.borrow().bind_inner_content();
        rc
    }

    /// Build the `RunInnerContentHandler` "super" instance and give it this
    /// `RHandler` so `super.getElementHandler(sub, this)` works as in Java.
    fn bind_inner_content(&self) {
        let importer = self.mDocImporter.clone().expect("mDocImporter");
        let h = Rc::new(super::RunInnerContentHandler::RunInnerContentHandler::new(
            Some(self.mPart.clone()),
            importer,
            self.mSubDocType,
            self.mTableLayer,
        ));
        // Java: super.getElementHandler(subElement, this)
        if let Some(me) = self.mSelf.upgrade() {
            h.bind_r_handler(me);
        }
        *self.mInnerContent.borrow_mut() = Some(h);
    }

    fn inner_content_handler(&self) -> Rc<super::RunInnerContentHandler::RunInnerContentHandler> {
        let mut slot = self.mInnerContent.borrow_mut();
        if slot.is_none() {
            drop(slot);
            self.bind_inner_content();
            slot = self.mInnerContent.borrow_mut();
        }
        slot.as_ref().expect("inner content handler").clone()
    }

    // Java: MutablePropertySet getProp()
    pub fn getProp(&self) -> MutablePropertySet {
        // Java: MutablePropertySet mutablePropertySet = null == this.mRprHandler ? this.mRprProp : this.mRprHandler.getProp();
        // Rust's RprBaseHandler owns the same Prop object as mRunProp. Read it
        // when w:rPr was visited, as Java reads mRprHandler.getProp().
        if self.mRprBase.borrow().is_some() {
            let mut prop = self.mRunProp.borrow_mut();
            // Java RprHandler.getProp() constructs a property set even when
            // this particular run has no rPr children.
            return prop.getProp().clone_mps();
        }
        if let Some(ref prop) = self.mRprProp {
            // Java: if (null == mutablePropertySet) mutablePropertySet = new MutablePropertySet();
            MutablePropertySet::new_from_mutable(prop)
        } else {
            // Java: return mutablePropertySet;
            MutablePropertySet::new()
        }
    }

    // Java: void setRprHandlerProp(MutablePropertySet rprProp)
    pub fn setRprHandlerProp(&mut self, rprProp: MutablePropertySet) {
        // Java: this.mRprProp = rprProp;
        self.mRprProp = Some(rprProp);
    }

    // Java: void clearProp()
    pub fn clearProp(&mut self) {
        // Java: this.mRprProp = null;
        self.mRprProp = None;
        self.mRunProp.borrow_mut().clearProp();
    }

    fn getRprBaseHandler(&self) -> ElementHandlerRef {
        let mut slot = self.mRprBase.borrow_mut();
        if slot.is_none() {
            let importer = self.mDocImporter.clone().expect("importer");
            *slot = Some(Rc::new(RefCell::new(RprBaseHandler::new(
                importer,
                self.mRunProp.clone(),
            ))));
        }
        slot.clone().expect("rpr base handler")
    }

    fn getTHandler(&self) -> ElementHandlerRef {
        // Java: if (null == this.mTHandler)
        // Java: this.mTHandler = new THandler(this.mDocImporter, this.mSubDocType, rHandler, this.mIsRuby);
        let mut slot = self.mTHandler.borrow_mut();
        if slot.is_none() {
            let importer = self.mDocImporter.clone().expect("importer");
            *slot = Some(
                THandler::new_with_ruby(importer, self.mSubDocType, None, self.mIsRuby)
                    .with_run_prop(self.mRunProp.clone())
                    .as_handler_ref(),
            );
        }
        slot.clone().expect("t handler")
    }

    // Java: ElementHandlerSimple getElementHandler(int subElement, String namespaceURI)
    pub fn getElementHandler_impl(&self, subElement: i32, namespaceURI: &str) -> Option<ElementHandlerRef> {
        // Java: switch (subElement)
        match subElement {
            // Java: case 44: return this.getMathRprHandler();
            44 => {
                // DEFERRED_NON_FUNCTION: math rPr 不在本里程碑（STAGE7）
                None
            }
            // Java: case 114: return this.getRHandler();
            114 => {
                // DEFERRED_NON_FUNCTION: 嵌套 `w:r`（Java 的 getRHandler）本批未接线
                None
            }
            // Java: case 112148: return this.getRprHandler();
            112148 => {
                // Java: return this.getRprHandler();
                Some(self.getRprBaseHandler())
            }
            // Java: case 103772132: return new MediaHandler(this.mDocImporter, this.mSubDocType);
            103772132 => {
                // DEFERRED_NON_FUNCTION: MediaHandler 未移植
                None
            }
            // Java: case 116: return this.getTHandler();
            116 => Some(self.getTHandler()),
            // Java: default: return super.getElementHandler(subElement, this);
            //
            // rust: ★P0/L0-2 —— Java 的 `default` 落到**父类**
            //   `RunInnerContentHandler.getElementHandler(subElement, this)`。
            //   Rust 侧原先此分支硬编码「非 116 即 None」，使父类整张分发表
            //   （含 `case 3152: getBrHandler`）失效 ⇒ `w:br` 丢失、分页符进不了 rope。
            //   现按 Java 逐字转调真实父类实例。
            _ => {
                let parent = self.inner_content_handler();
                match self.mSelf.upgrade() {
                    Some(me) => parent.getElementHandler(subElement, &me),
                    // rust: 对象已析构（无强引用）⇒ 与 Java 中 `this` 不可用等价。
                    None => None,
                }
            }
        }
    }

    // Java: void onStart(...)
    pub fn onStart(
        &self,
        elementId: i32,
        namespaceURI: &str,
        localName: &str,
        qualifiedName: &str,
        attributes: &dyn Any,
    ) {
        let _ = (elementId, namespaceURI, localName, qualifiedName, attributes);
        self.mRunProp.borrow_mut().clearProp();
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd(&mut self, elementId: i32, namespaceURI: &str) {
        let _ = namespaceURI;
        // Java: if (!this.mIsRuby) this.clearProp(); — only when `w:r` ends (114).
        if !self.mIsRuby && (elementId == 114 || elementId == 0) {
            self.clearProp();
        }
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 0;
        0
    }
}

impl ElementHandlerSimple for RHandler {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn filterIndex(&self) -> i32 {
        RHandler::filterIndex(self)
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
        RHandler::onStart(self, elementId, namespaceURI, localName, qualifiedName, attributes);
        Ok(())
    }
    fn onEnd_with_id(
        &mut self,
        elementId: i32,
        namespaceURI: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        RHandler::onEnd(self, elementId, namespaceURI);
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
        self.getElementHandler_impl(id, ns)
    }
}
