#![allow(dead_code, non_snake_case, unused_variables)]
// Java: import cn.wps.moffice.writer.cache.TypoDrawing
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
// Java: import cn.wps.moffice.writer.cache.TypoRectCoreImpl
use crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::table::TablePos::TablePos;
use drawing::cn::wps::moffice::drawing::IPos::IPos;
use crate::cn::wps::moffice::writer::layout::base::drawing::ShapeCollision::{ClipRect, RectFWrapImp};
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.MainState
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
// Java: import cn.wps.moffice.writer.layout.base.env.SuperEnvoy
use crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewEnv
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;

// java WrapObj: drawing pointer, doc type, and the page-core layout rect.
#[derive(Clone)]
pub struct WrapObj {
    pub pDrawing: i32,
    pub docType: i32,
    // Java WrapObj.pos is the IPos attached to TypoDrawing. The active
    // table branch carries its concrete TablePos from snapshot slot +8.
    pub pos: Option<TablePos>,
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub subWrapObjs: Option<Vec<WrapObj>>,
}

impl WrapObj {
    pub fn getTop(&self) -> i32 { self.top }
    pub fn getBottom(&self) -> i32 { self.bottom }
    pub fn getLeft(&self) -> i32 { self.left }
    pub fn getRight(&self) -> i32 { self.right }

    // java WrapObj.set(WrapObj)
    fn set_from(&mut self, other: &WrapObj) {
        self.pDrawing = other.pDrawing;
        self.docType = other.docType;
        self.pos = other.pos.clone();
        self.left = other.left;
        self.top = other.top;
        self.right = other.right;
        self.bottom = other.bottom;
    }

    // java TypoRect.union
    fn union(&mut self, other: &WrapObj) {
        self.left = self.left.min(other.left);
        self.top = self.top.min(other.top);
        self.right = self.right.max(other.right);
        self.bottom = self.bottom.max(other.bottom);
    }
}

// java public class TextWrapLayouter
pub struct TextWrapLayouter {
    // java protected MainState mMainState;
    mMainState: *mut MainState,
    // java protected DocumentEnv mDocEnv;
    mDocEnv: *mut DocumentEnv,
    // java protected ViewEnv mViewEnv;
    mViewEnv: *mut ViewEnv,
    // java protected ArrayList<WrapObj> mWrapObjs
    mWrapObjs: Vec<WrapObj>,
    // java private TypoSnapshot.IntArrayList mWrapDrawingCache
    mWrapDrawingCache: Vec<i32>,
}

impl TextWrapLayouter {
    // Java: TextWrapLayouter.clean() recycles wrap objects and resets the
    // drawing cache while retaining the layouter for the next update.
    pub fn clean(&mut self) {
        self.mWrapObjs.clear();
        self.mWrapDrawingCache.clear();
    }

    // java public TextWrapLayouter(MainState mainState, DocumentEnv documentEnv, ViewEnv viewEnv)
    pub fn new(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
        Self {
            mMainState: main,
            mDocEnv: doc,
            mViewEnv: view,
            mWrapObjs: Vec::new(),
            mWrapDrawingCache: Vec::new(),
        }
    }

    pub fn rebind(&mut self, main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) {
        self.mMainState = main;
        self.mDocEnv = doc;
        self.mViewEnv = view;
    }

    // java protected void getWrapDrawingPageList(IIntList outList, IStateEnvoy stateEnvoy)
    fn getWrapDrawingPageList(&self, out: &mut Vec<i32>, envoy: &SuperEnvoy) {
        // java outList.add(stateEnvoy.getCurPage().getPointer());
        out.push(envoy.getCurPage().getPointer());
    }

    // java public void update(IStateEnvoy stateEnvoy)
    pub fn update(&mut self, envoy: &mut SuperEnvoy) {
        if self.mMainState.is_null() {
            return;
        }
        let mut page_list = Vec::new();
        // java pageList.resetQuick(); this.getWrapDrawingPageList(pageList, stateEnvoy);
        self.getWrapDrawingPageList(&mut page_list, envoy);
        let lines_page = envoy.getCurPage().getPointer();
        let core_width = envoy.getPageEnv().getCoreWidth();
        let core_height = envoy.getPageEnv().getCoreHeight();
        // java int docType = stateEnvoy.getDocument().getType();
        let doc_type = envoy.getDocument().getType__on_IDocument();
        let mut wrap_drawings = Vec::new();
        for p_page in page_list {
            wrap_drawings.clear();
            let found = {
                let main = unsafe { &mut *self.mMainState };
                let doc = envoy.getDocument();
                // java TypoDrawingLayers drawingLayers = this.mMainState.getDrawingLayers(page.getDocument());
                if let Some(layers) = main.getDrawingLayers(doc) {
                    // java drawingLayers.getWraps(wrapDrawings, pPage) > 0
                    layers.getWraps__list_int(&mut wrap_drawings, p_page)
                } else {
                    0
                }
            };
            if std::env::var_os("WORD_PARITY_WRAP_TRACE").is_some() {
                eprintln!("wrap_update page={} found={} drawings={:?}", p_page, found, wrap_drawings);
            }
            if found <= 0 {
                continue;
            }
            let drawings = std::mem::take(&mut wrap_drawings);
            for p_drawing in drawings {
                // java int index = wrapDrawingCache.binarySearch(pDrawing); if (index < 0) insert
                if self.mWrapDrawingCache.binary_search(&p_drawing).is_ok() {
                    continue;
                }
                let index = self.mWrapDrawingCache.binary_search(&p_drawing).unwrap_err();
                self.mWrapDrawingCache.insert(index, p_drawing);
                let mut rect = TypoRectCoreImpl::default();
                let pos;
                {
                    let main = unsafe { &mut *self.mMainState };
                    let mut drawing = TypoDrawing::new();
                    drawing.assignment(p_drawing, &mut main.snapshot);
                    // java drawing.layoutRectForPageCore(wrapObj);
                    drawing.layoutRectForPageCore(&mut rect);
                    pos = drawing.getTablePos();
                }
                // java doOffsetLayoutRect: no-op when linesPage and floatsPage are the same pointer.
                let _ = (lines_page, p_page);
                let wrap = WrapObj {
                    pDrawing: p_drawing,
                    docType: doc_type,
                    pos,
                    left: rect.getLeft(),
                    top: rect.getTop(),
                    right: rect.getRight(),
                    bottom: rect.getBottom(),
                    subWrapObjs: None,
                };
                if std::env::var_os("WORD_PARITY_WRAP_TRACE").is_some() {
                    eprintln!("wrap_object drawing={} pos={} box={},{},{},{} core={},{}", p_drawing, wrap.pos.is_some(), wrap.left, wrap.top, wrap.right, wrap.bottom, core_width, core_height);
                }
                // java if (right > 0 && bottom > 0 && left < coreWidth && top < coreHeight) add else recycle
                if wrap.getRight() > 0 && wrap.getBottom() > 0 && wrap.getLeft() < core_width && wrap.getTop() < core_height {
                    self.addWrapObj(wrap);
                }
            }
        }
        let _ = self.mDocEnv;
        let _ = self.mViewEnv;
    }

    // java private void addWrapObj(WrapObj wrapObj)
    fn addWrapObj(&mut self, wrap_obj: WrapObj) {
        let mut index = self.findWrapObj(wrap_obj.getTop());
        if index < 0 {
            index = -index - 1;
            // java if (index >= size || wrapObj.getBottom() <= mWrapObjs.get(index).getTop()) add(index); return;
            if index >= self.mWrapObjs.len() as i32 || wrap_obj.getBottom() <= self.mWrapObjs[index as usize].getTop() {
                self.mWrapObjs.insert(index as usize, wrap_obj);
                return;
            }
        }
        // java parent has no sub list: move the parent drawing into a new sub object, then union
        let parent = &mut self.mWrapObjs[index as usize];
        if parent.subWrapObjs.is_none() {
            let mut sub = WrapObj {
                pDrawing: parent.pDrawing,
                docType: parent.docType,
                pos: parent.pos.clone(),
                left: parent.left,
                top: parent.top,
                right: parent.right,
                bottom: parent.bottom,
                subWrapObjs: None,
            };
            sub.set_from(parent);
            parent.subWrapObjs = Some(vec![sub]);
            parent.pDrawing = 0;
            parent.docType = 0;
            parent.pos = None;
        }
        parent.union(&wrap_obj);
        parent.subWrapObjs.as_mut().unwrap().push(wrap_obj);
    }

    // java private int findWrapObj(int top)
    fn findWrapObj(&self, top: i32) -> i32 {
        let mut low: i32 = 0;
        let mut high: i32 = self.mWrapObjs.len() as i32 - 1;
        while low <= high {
            let mid = (low + high) >> 1;
            let wrap = &self.mWrapObjs[mid as usize];
            if top < wrap.getTop() {
                high = mid - 1;
            } else if top < wrap.getBottom() {
                return mid;
            } else {
                low = mid + 1;
            }
        }
        -(low + 1)
    }

    // Java TextWrapLayouter.getSubLineRects: table-position branch. The
    // snapshot's committed TablePos decides how its drawing cuts the line.
    pub fn getSubLineRects(&self, out: &mut Vec<RectFWrapImp>, top: i32, bottom: i32,
        exclude_start_cp: i32, exclude_end_cp: i32, envoy: &mut SuperEnvoy) -> bool {
        if std::env::var_os("WORD_PARITY_WRAP_TRACE").is_some() {
            eprintln!("wrap_clip top={} bottom={} count={}", top, bottom, self.mWrapObjs.len());
        }
        if self.mWrapObjs.is_empty() ||
            (!self.mViewEnv.is_null() && unsafe { (*self.mViewEnv).isPageMode() }
                && top >= envoy.getPageEnv().maxLayoutHeight) { return false; }
        let mut index = self.findWrapObj(top);
        if index < 0 { index = -index - 1; }
        let Some(first) = self.mWrapObjs.get(index as usize) else { return false; };
        if bottom <= first.top { return false; }
        let page = envoy.getCurPage();
        let text_flow = page.getTextFlow();
        // Java TextFlowTool.getPageEnvMarginByLogicDir(pageEnv, textflow, LEFT)
        let env_left = match text_flow {
            1 => envoy.getPageEnv().marginTop,
            3 => envoy.getPageEnv().marginBottom,
            _ => envoy.getPageEnv().marginLeft,
        };
        let layout_left = env_left - page.getMarginLeft();
        let core_width = envoy.getPageEnv().getCoreWidth();
        let mut clip = ClipRect::default();
        clip.init(layout_left, top, layout_left + core_width, bottom);
        let need_exclude = exclude_start_cp >= 0 && exclude_start_cp < exclude_end_cp;
        let cur_doc_type = envoy.getDocument().getType__on_IDocument();
        let mut processed = false;
        for wrap in self.mWrapObjs[index as usize..].iter() {
            if bottom <= wrap.top { break; }
            let items: Vec<&WrapObj> = match wrap.subWrapObjs.as_ref() {
                Some(subs) => subs.iter().collect(),
                None => vec![wrap],
            };
            for item in items {
                if item.top >= bottom || item.bottom <= top { continue; }
                let Some(pos) = item.pos.as_ref() else { continue; };
                if need_exclude && item.docType == cur_doc_type && !self.mMainState.is_null() {
                    let snapshot = unsafe { &(*self.mMainState).snapshot };
                    let anchor = TypoDrawing::getAnchorCP__int_typosnapshot(item.pDrawing, snapshot);
                    if anchor >= exclude_start_cp && anchor < exclude_end_cp { continue; }
                }
                processed = true;
                // Java calcDrawingWrapRect: TablePos.getTextWrapType() is 0;
                // retain the type branches for the IPos table interface.
                let (mut left, mut right) = (item.left, item.right);
                match pos.getTextWrapType() {
                    1 => if left < 360 { left = 0 } else { right = core_width },
                    2 => left = 0,
                    3 => if (item.left + item.right) / 2 > core_width / 2 {
                        right = core_width;
                    } else { left = 0; },
                    _ => {},
                }
                let mut rect = TypoRectCoreImpl::default();
                rect.set__int_int_int_int(left, item.top, right, item.bottom);
                // Java subtractDrawingWrapRect, including TablePos's
                // TABLESQUARE width threshold in ShapeCollision.ClipRect.
                match pos.getTextWrap() {
                    0 | 2 => clip.subtract(&rect, ClipRect::TABLESQUARE),
                    1 => if item.left < core_width + 180 && item.right > -180 { clip.clear(); },
                    4 | 5 => clip.subtract(&rect, ClipRect::TIGHT),
                    _ => {},
                }
            }
        }
        if processed { clip.getRects(out); }
        processed
    }

    // Java: TextWrapLayouter.getNearestDrawingBottom(IStateEnvoy stateEnvoy)
    pub fn getNearestDrawingBottom(&self, envoy: &mut SuperEnvoy) -> i32 {
        // java int curLayoutHeight = stateEnvoy.getCurLayoutHeight();
        let cur_layout_height = envoy.getCurLayoutHeight();
        // java int index = this.findWrapObj(curLayoutHeight);
        let mut index = self.findWrapObj(cur_layout_height);
        if index < 0 { index = -index - 1; }
        let mut min_wrap_bottom = i32::MAX;
        let mut valid = false;
        if let Some(wrap_obj) = self.mWrapObjs.get(index as usize) {
            if let Some(sub_wrap_objs) = wrap_obj.subWrapObjs.as_ref() {
                for sub_wrap in sub_wrap_objs {
                    if sub_wrap.getBottom() > cur_layout_height {
                        min_wrap_bottom = min_wrap_bottom.min(sub_wrap.getBottom());
                        valid = true;
                    }
                }
            } else if wrap_obj.getBottom() > cur_layout_height {
                min_wrap_bottom = wrap_obj.getBottom();
                valid = true;
            }
        }
        if !valid { return cur_layout_height; }
        // java !mViewEnv.isPageMode() || pageType != 2 && pageType != 14
        let page_type = envoy.getCurPage().getType();
        let page_mode = !self.mViewEnv.is_null() && unsafe { (*self.mViewEnv).isPageMode() };
        if !page_mode || (page_type != 2 && page_type != 14) {
            min_wrap_bottom
        } else {
            min_wrap_bottom.min(envoy.getPageEnv().maxLayoutHeight)
        }
    }
}
