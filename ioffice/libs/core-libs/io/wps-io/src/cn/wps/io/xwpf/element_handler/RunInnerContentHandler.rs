#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.simple.ElementHandlerSimple
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
// Java: import cn.wps.io.xwpf.IDmlImporter
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.io.xwpf.element_handler.dml.DrawingHandler
// Java: import cn.wps.io.xwpf.element_handler.xml03.AnnotationHandler

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use super::BrHandler::BrHandler;
use super::AlternateContentHandler::AlternateContentHandler;
use super::ContentHandler::ContentHandler;
use super::DelTextHandler::DelTextHandler;
use super::dml::DrawingHandler::DrawingHandler;
use super::EmptyHandler::EmptyHandler;
use super::PictHandler::PictHandler;
use super::RHandler::RHandler;
use super::THandler::THandler;

// Java: abstract class RunInnerContentHandler extends RunTrackChangeChildHandler
//
// rust: ★P0/L0-2 —— 本类在 Java 中是 `RHandler` 的**父类**，承载 `w:r` 内部子元素的
//   分发表（`getElementHandler`）。Rust 侧原先整类为空壳（`struct { _private }`），
//   导致 `RHandler` 的 `super.getElementHandler(subElement, this)` 无处可去，
//   `w:br`（elementId 3152）因此丢失 ⇒ 分页符进不了 rope。
//
//   此处分发 ID 按 `RunInnerContentHandler.java` 的 switch 登记，但：
//     · `w:t`(116)、`w:br`(3152)、EmptyHandler 与 ContentHandler
//       已接真实 handler 和 importer 回调；
//     · `w:object`(-1023368385)、`w:pict`(3440682) 和
//       `w:drawing`(1913009182) 在 Java 均构造真实 handler；Rust 现按
//       typed importer 构造相应 handler，但产品 importer 尚须提供实例。
//     · 其余 `None` 分支须逐个与 Java 的具体条件核对，不能当成等价实现。
pub struct RunInnerContentHandler {
    // Java field: SubDocType mSubDocType
    pub(crate) mSubDocType: i32,
    // Java field: POIXMLDocumentPart mPart
    pub(crate) mPart: Option<Rc<dyn Any>>,
    // Java field: THandler mTHandler
    pub(crate) mTHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: BrHandler mBrHandler
    pub(crate) mBrHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: EmptyHandler mEmptyHandler
    pub(crate) mEmptyHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: ContentHandler mContentHandler
    pub(crate) mContentHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: DelTextHandler mDelTextHandler
    pub(crate) mDelTextHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: PictHandler mPictHandler
    pub(crate) mPictHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: DrawingHandler mDrawingHandler
    pub(crate) mDrawingHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: AlternateContentHandler mAlternateContentHandler
    pub(crate) mAlternateContentHandler: RefCell<Option<ElementHandlerRef>>,
    // Java field: boolean mIsRuby
    pub(crate) mIsRuby: bool,
    // Java field: int mTableLayer
    pub(crate) mTableLayer: i32,
    pub(crate) mDocImporter: Option<Rc<dyn IDocumentImporter>>,
    // Java: `super.getElementHandler(subElement, this)` passes the enclosing RHandler.
    pub(crate) mSelfRHandler: RefCell<Option<Rc<RefCell<RHandler>>>>,
}

impl RunInnerContentHandler {
    // Java: RunInnerContentHandler(POIXMLDocumentPart part, IDocumentImporter importer, SubDocType subDocType, int tableLayer)
    pub fn new(
        part: Option<Rc<dyn Any>>,
        importer: Rc<dyn IDocumentImporter>,
        subDocType: i32,
        tableLayer: i32,
    ) -> Self {
        // Java: super(importer);
        // Java: this.mPart = part; this.mSubDocType = subDocType;
        // Java: this.mIsRuby = false; this.mTableLayer = tableLayer;
        RunInnerContentHandler {
            mSubDocType: subDocType,
            mPart: part,
            mTHandler: RefCell::new(None),
            mBrHandler: RefCell::new(None),
            mEmptyHandler: RefCell::new(None),
            mContentHandler: RefCell::new(None),
            mDelTextHandler: RefCell::new(None),
            mPictHandler: RefCell::new(None),
            mDrawingHandler: RefCell::new(None),
            mAlternateContentHandler: RefCell::new(None),
            mIsRuby: false,
            mTableLayer: tableLayer,
            mDocImporter: Some(importer),
            mSelfRHandler: RefCell::new(None),
        }
    }

    // Java: RunInnerContentHandler(POIXMLDocumentPart part, IDocumentImporter importer, SubDocType subDocType, boolean isRuby, int tableLayer)
    pub fn new_with_ruby(
        part: Option<Rc<dyn Any>>,
        importer: Rc<dyn IDocumentImporter>,
        subDocType: i32,
        isRuby: bool,
        tableLayer: i32,
    ) -> Self {
        // Java: this(part, importer, subDocType, tableLayer);
        let mut h = Self::new(part, importer, subDocType, tableLayer);
        // Java: this.mIsRuby = isRuby;
        h.mIsRuby = isRuby;
        h
    }

    /// Back-reference so `super.getElementHandler(sub, this)` can hand the
    /// enclosing `RHandler` to the child handler (Java passes `this`).
    pub fn bind_r_handler(&self, r: Rc<RefCell<RHandler>>) {
        *self.mSelfRHandler.borrow_mut() = Some(r);
    }

    // Java: private ElementHandlerSimple getBrHandler(RHandler rHandler)
    fn getBrHandler(&self) -> Option<ElementHandlerRef> {
        // Java: if (null == this.mBrHandler)
        let mut slot = self.mBrHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mBrHandler = new BrHandler(this.mDocImporter, this.mSubDocType, rHandler);
            let importer = self.mDocImporter.clone().expect("mDocImporter");
            let r = self.mSelfRHandler.borrow().clone();
            *slot = Some(BrHandler::new(importer, self.mSubDocType, r).as_handler_ref());
        }
        // Java: return this.mBrHandler;
        slot.clone()
    }

    // Java: private ElementHandlerSimple getTHandler(RHandler rHandler)
    fn getTHandler(&self) -> Option<ElementHandlerRef> {
        // Java: if (null == this.mTHandler)
        let mut slot = self.mTHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mTHandler = new THandler(this.mDocImporter, this.mSubDocType, rHandler, this.mIsRuby);
            let importer = self.mDocImporter.clone().expect("mDocImporter");
            let r = self.mSelfRHandler.borrow().clone();
            *slot = Some(
                THandler::new_with_ruby(importer, self.mSubDocType, r, self.mIsRuby).as_handler_ref(),
            );
        }
        // Java: return this.mTHandler;
        slot.clone()
    }

    // Java: private ElementHandlerSimple getEmptyHandler(RHandler rHandler)
    fn getEmptyHandler(&self) -> Option<ElementHandlerRef> {
        // Java: if (null == this.mEmptyHandler)
        let mut slot = self.mEmptyHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mEmptyHandler = new EmptyHandler(this.mDocImporter, this.mSubDocType, rHandler);
            let importer = self.mDocImporter.clone().expect("mDocImporter");
            let r = self.mSelfRHandler.borrow().clone();
            *slot = Some(EmptyHandler::new(importer, self.mSubDocType, r).as_handler_ref());
        }
        // Java: return this.mEmptyHandler;
        slot.clone()
    }

    // Java: private ElementHandlerSimple getContentHandler(RHandler rHandler)
    fn getContentHandler(&self) -> Option<ElementHandlerRef> {
        // Java: if (null == this.mContentHandler)
        let mut slot = self.mContentHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mContentHandler = new ContentHandler(this.mDocImporter, this.mSubDocType, rHandler);
            let importer = self.mDocImporter.clone().expect("mDocImporter");
            let r = self.mSelfRHandler.borrow().clone();
            *slot = Some(ContentHandler::new(importer, self.mSubDocType, r).as_handler_ref());
        }
        // Java: return this.mContentHandler;
        slot.clone()
    }

    // Java: private ElementHandlerSimple getDelTextHandler(RHandler rHandler)
    fn getDelTextHandler(&self) -> Option<ElementHandlerRef> {
        // Java: if (null == this.mDelTextHandler)
        let mut slot = self.mDelTextHandler.borrow_mut();
        if slot.is_none() {
            // Java: this.mDelTextHandler = new DelTextHandler(this.mDocImporter, this.mSubDocType, rHandler);
            let importer = self.mDocImporter.clone().expect("mDocImporter");
            let r = self.mSelfRHandler.borrow().clone();
            *slot = Some(DelTextHandler::new(importer, self.mSubDocType, r).as_handler_ref());
        }
        // Java: return this.mDelTextHandler;
        slot.clone()
    }

    // Java: private ElementHandlerSimple getPictHandler(RHandler rHandler)
    fn getPictHandler(&self, rHandler: &Rc<RefCell<RHandler>>) -> Option<ElementHandlerRef> {
        let mut slot = self.mPictHandler.borrow_mut();
        // Java: if (null == this.mPictHandler)
        if slot.is_none() {
            let importer = self.mDocImporter.as_ref().expect("mDocImporter");
            // Java: new PictHandler(this.mDocImporter.getVmlImporter(this.mSubDocType), this.mPart, this.mSubDocType)
            let vml = importer.getVmlImporter(self.mSubDocType)?;
            *slot = Some(PictHandler::new(vml, self.mPart.clone(), self.mSubDocType).as_handler_ref());
        }
        // Java: this.mPictHandler.setRunProperty(rHandler.getProp());
        let handler = slot.as_ref().expect("mPictHandler").clone();
        handler.borrow().as_any().downcast_ref::<PictHandler>().expect("PictHandler")
            .setRunProperty(rHandler.borrow().getProp());
        // Java: return this.mPictHandler;
        Some(handler)
    }

    // Java: private ElementHandlerSimple getDrawingHandler(RHandler rHandler)
    fn getDrawingHandler(&self, rHandler: &Rc<RefCell<RHandler>>) -> Option<ElementHandlerRef> {
        let mut slot = self.mDrawingHandler.borrow_mut();
        // Java: if (null == this.mDrawingHandler) new DrawingHandler(this.mDocImporter, null, this.mPart, ...)
        if slot.is_none() {
            let importer = self.mDocImporter.as_ref()?.clone();
            let drawing = DrawingHandler::new(Some(importer), None, self.mPart.clone(),
                self.mSubDocType, self.mTableLayer)?;
            *slot = Some(drawing.as_handler_ref());
        }
        // Java: this.mDrawingHandler.setRunProperty(rHandler.getProp());
        let handler = slot.as_ref()?.clone();
        handler.borrow().as_any().downcast_ref::<DrawingHandler>().expect("DrawingHandler")
            .setRunProperty(rHandler.borrow().getProp());
        Some(handler)
    }

    // Java: private ElementHandlerSimple getAlternateContentHandler(RHandler rHandler)
    fn getAlternateContentHandler(&self, rHandler: &Rc<RefCell<RHandler>>) -> Option<ElementHandlerRef> {
        let mut slot = self.mAlternateContentHandler.borrow_mut();
        if slot.is_none() {
            // Java: new AlternateContentHandler(mDocImporter, null, mPart, mSubDocType, filterIndex(), mTableLayer)
            *slot = Some(AlternateContentHandler::new(self.mDocImporter.as_ref()?.clone(), None,
                self.mPart.clone(), self.mSubDocType, self.filterIndex(), self.mTableLayer).as_handler_ref());
        }
        let handler = slot.as_ref()?.clone();
        // Java: setRHandlerProp(null == rHandler ? null : rHandler.getProp())
        handler.borrow().as_any().downcast_ref::<AlternateContentHandler>().expect("AlternateContentHandler")
            .setRHandlerProp(Some(rHandler.borrow().getProp()));
        Some(handler)
    }

    // Java: public ElementHandlerSimple getElementHandler(int subElement, RHandler rHandler)
    pub fn getElementHandler(
        &self,
        subElement: i32,
        rHandler: &Rc<RefCell<RHandler>>,
    ) -> Option<ElementHandlerRef> {
        // Java: switch (subElement)
        match subElement {
            // Java: case -1606462643: return this.getEndnoteHandler(rHandler);
            -1606462643 => {
                // DEFERRED_NON_FUNCTION: xml03 EndnoteHandler 未移植；Java 在 !isXml03 时同样返回 null
                None
            }
            // Java: case -1555043537: return this.getAnnotationHandler(rHandler);
            -1555043537 => {
                // DEFERRED_NON_FUNCTION: xml03 AnnotationHandler 未移植
                None
            }
            // Java: case -1068263892: case -104827109: case 99339: case 104430:
            // Java:    return this.getInsDelHandler(rHandler);
            -1068263892 | -104827109 | 99339 | 104430 => {
                // DEFERRED_NON_FUNCTION: InnerInsDelHandler 仍为空壳（修订 run，非本里程碑）
                None
            }
            // Java: case -1062047106: case 734762443: return this.getFootnoteEndnoteHandler(rHandler);
            -1062047106 | 734762443 => {
                // DEFERRED_NON_FUNCTION: FootnoteEndnoteReferenceHandler 仍为空壳（STAGE6）
                None
            }
            // Java: case -1023368385: case 3440682: return this.getPictHandler(rHandler);
            -1023368385 | 3440682 => self.getPictHandler(rHandler),
            // Java: case -784674076: return this.getContentHandler(rHandler);
            -784674076 => self.getContentHandler(),
            // Java: case -777505580: return this.getContentHandler(rHandler);
            -777505580 => self.getContentHandler(),
            // Java: case -541840468: return this.getCommentHandler(rHandler);
            -541840468 => {
                // DEFERRED_NON_FUNCTION: CommentReferenceHandler 仍为空壳（STAGE7）
                None
            }
            // Java: case -462474756: case 3183: case 114581: case 2001013392:
            // Java:    return this.getEmptyHandler(rHandler);
            -462474756 | 3183 | 114581 | 2001013392 => {
                self.getEmptyHandler()
            }
            // Java: case 116: return this.getTHandler(rHandler);
            116 => self.getTHandler(),
            // Java: case 3152: return this.getBrHandler(rHandler);
            // rust: ★P0 —— `w:br`。`WMap` 中 `"br" -> 3152`（`WMap.rs:997`，与 Java 同表）。
            //   `BrHandler.convertBreakText` 把 `type="page"` 转 `'\f'` 并
            //   `onImportRun(subDocType, dst, text)` 写入 rope ⇒ 分页符的唯一真生产者。
            3152 => self.getBrHandler(),
            // Java: case 114375: return this.getContentHandler(rHandler);
            114375 => self.getContentHandler(),
            // Java: case 3511770: return this.getRubyHandler(rHandler);
            3511770 => {
                // DEFERRED_NON_FUNCTION: RubyHandler 仍为空壳
                None
            }
            // Java: case 44025401: case 1219280494: return this.getTHandler(rHandler);
            44025401 | 1219280494 => self.getTHandler(),
            // Java: case 395040096: return this.getFootnoteHandler(rHandler);
            395040096 => {
                // DEFERRED_NON_FUNCTION: xml03 FootnoteHandler 未移植
                None
            }
            // Java: case 429190931: return this.getContentHandler(rHandler);
            429190931 => self.getContentHandler(),
            // Java: case 692063750: return this.getContentHandler(rHandler);
            692063750 => self.getContentHandler(),
            // Java: case 1549942744: return this.getDelTextHandler(rHandler);
            1549942744 => self.getDelTextHandler(),
            // Java: case 1590178495: return this.getAlternateContentHandler(rHandler);
            1590178495 => self.getAlternateContentHandler(rHandler),
            // Java: case 1913009182: return this.getDrawingHandler(rHandler);
            1913009182 => self.getDrawingHandler(rHandler),
            // Java: default: return null;
            _ => None,
        }
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 0;
        0
    }
}
