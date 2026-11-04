#![allow(dead_code, non_snake_case)]

use util::moffice::util::UnitUtil::UnitUtil;
use crate::cn::wps::moffice::writer::cache::TypoLayoutPage::TypoLayoutPage;
use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
use crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj;
use crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage;
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
use crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;
use crate::cn::wps::moffice::writer::cache::style::TypoBRCs::TypoBRCs;
use crate::cn::wps::moffice::writer::data::value::BRC::BRC;
use crate::cn::wps::moffice::writer::data::value::PageBorderProp::PageBorderProp;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::PageEnv;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::page::MainPageLayouter::MainPageLayouter;
use crate::cn::wps::moffice::writer::layout::base::tools::BRCBorderTool::BRCBorderTool;

// java public class PageViewPageLayouter extends MainPageLayouter
pub struct PageViewPageLayouter {
    // java MainPageLayouter super_
    super_: MainPageLayouter,
}

impl Default for PageViewPageLayouter {
    fn default() -> Self {
        Self { super_: MainPageLayouter::default() }
    }
}

impl PageViewPageLayouter {
    // java public PageViewPageLayouter(MainState mainState, DocumentEnv documentEnv, ViewEnv viewEnv)
    pub fn PageViewPageLayouter(main_state: *mut MainState, doc_env: *mut DocumentEnv, view_env: *mut ViewEnv) -> Self {
        Self { super_: MainPageLayouter::MainPageLayouter(main_state, doc_env, view_env) }
    }

    fn main(&mut self) -> &mut MainState {
        unsafe { &mut *self.super_.mMainState }
    }

    // Java constructor stores the caller's MainState/DocumentEnv/ViewEnv.
    pub fn rebind(&mut self, main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) {
        self.super_.mMainState = main;
        self.super_.mDocEnv = doc;
        self.super_.mViewEnv = view;
    }

    // java protected void onStart(PageLayouter.IStateEnvoy stateEnvoy)
    pub fn onStart(&mut self, layout_state: &mut LayoutState) {
        // java super.onStart(stateEnvoy);
        let _ = &self.super_.super_;
        // java TypoLayoutPage layoutPage = this.createNewPage(pageViewLayoutState);
        self.createNewPage(layout_state);
        // java if (pageEnv.isFirstPageOfSection) layoutPage.setFirstPageOfSection(true);
        if layout_state.pageEnv.isFirstPageOfSection {
            let main = self.super_.mMainState;
            unsafe { (*main).layoutPage.setFirstPageOfSection(true); }
        }
        let main = self.super_.mMainState;
        unsafe {
            let snap = &mut (*main).snapshot as *mut TypoSnapshot;
            // java this.mMainState.drawingLayers.alloc(snapshot);
            (*main).drawingLayers.alloc(&mut *snap);
            // java resetQuick()
            (*main).layoutPageRelParaQuasiFloatFCs.clear();
            (*main).layoutPageRelParaQuasiFloats.clear();
        }
    }

    // java protected void onEnd(PageLayouter.IStateEnvoy stateEnvoy)
    pub fn onEnd(&mut self, state: &mut LayoutState) {
        // java super.onEnd(stateEnvoy);
        let _ = &self.super_.super_;
        // java this.flowPageBorders((PageViewLayoutState) ...);
        self.flowPageBorders(state);
    }

    // java private TypoLayoutPage createNewPage(LayoutState layoutState)
    pub fn createNewPage(&mut self, layout_state: &mut LayoutState) {
        let main = self.super_.mMainState;
        let page_env = &mut layout_state.pageEnv;
        unsafe {
            let snap = &mut (*main).snapshot as *mut TypoSnapshot;
            let page = &mut (*main).layoutPage;
            // java preFlow calls layoutPage.alloc before onStart. createNewPage writes that page.
            if page.getPointer() == 0 {
                page.alloc(&mut *snap);
            }
            // java newPage.setWidth / setHeight / setMargin / setTextFlow / setRotationAsia
            page.setWidth(page_env.width);
            page.setHeight(page_env.height);
            match page_env.textFlow {
                // java case 1: setRotationAsia(true); fall through case 5
                1 => {
                    page.setRotationAsia(true);
                    page.setMargin(page_env.marginRight, page_env.marginTop, page_env.marginLeft, page_env.marginBottom);
                    page.setTextFlow(1);
                }
                5 => {
                    page.setMargin(page_env.marginRight, page_env.marginTop, page_env.marginLeft, page_env.marginBottom);
                    page.setTextFlow(1);
                }
                3 => {
                    page.setMargin(page_env.marginLeft, page_env.marginBottom, page_env.marginRight, page_env.marginTop);
                    page.setTextFlow(3);
                }
                // java case 4: setRotationAsia(true); fall through default
                4 => {
                    page.setRotationAsia(true);
                    page.setMargin(page_env.marginTop, page_env.marginLeft, page_env.marginBottom, page_env.marginRight);
                }
                _ => {
                    page.setMargin(page_env.marginTop, page_env.marginLeft, page_env.marginBottom, page_env.marginRight);
                }
            }
            // java newPage.setDocumentType(0);
            page.setDocumentType(0);
            // java newPage.offsetTo((snapshot.getDocumentWidth() - pageEnv.width) / 2, this.mMainState.curPageTop);
            let doc_width = (*snap).getDocumentWidth();
            let top = (*main).curPageTop;
            page.offsetTo((doc_width - page_env.width) / 2, top);
        }
        // java pageEnv.orgPageMarginLeft = pageEnv.marginLeft;
        page_env.orgPageMarginLeft = page_env.marginLeft;
        // java pageEnv.sectionMarginOffset = 0;
        page_env.sectionMarginOffset = 0;
        self.sync_cur_page(layout_state);
    }

    fn sync_cur_page(&mut self, layout_state: &mut LayoutState) {
        let main = self.super_.mMainState;
        unsafe {
            if let Some(page) = layout_state.curPage.as_mut() {
                (*main).layoutPage.fill_page(page);
            } else {
                let mut page = crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::default();
                (*main).layoutPage.fill_page(&mut page);
                layout_state.curPage = Some(page);
            }
        }
    }

    // java private void flowPageBorders(PageViewLayoutState pageViewLayoutState)
    pub fn flowPageBorders(&mut self, state: &mut LayoutState) {
        if !self.isValidedPageBRC(state.brcBottom.as_ref())
            && !self.isValidedPageBRC(state.brcTop.as_ref())
            && !self.isValidedPageBRC(state.brcLeft.as_ref())
            && !self.isValidedPageBRC(state.brcRight.as_ref())
        {
            return;
        }
        let mut brc_prop = state.pageBorderProp.clone().unwrap_or_else(PageBorderProp::default);
        let page_env = &state.pageEnv;
        let first = page_env.isFirstPageOfSection;
        // java (applyto != 2 || !first) && (applyto != 1 || first)
        if (brc_prop.getApplyto() == 2 && first) || (brc_prop.getApplyto() == 1 && !first) {
            return;
        }
        let main = self.super_.mMainState;
        let doc = self.super_.mDocEnv;
        unsafe {
            let snap = &mut (*main).snapshot as *mut TypoSnapshot;
            let mut typo_brcs = TypoBRCs::default();
            typo_brcs.alloc(&mut *snap);
            if brc_prop.getPageDepth() == 0 {
                (*main).layoutPage.setShowPageBRCsOnTop(true);
            }
            let prop = (*doc).textDocument.as_ref().expect("textDocument").getProperty();
            let align_borders = prop.getBoolean__int_boolean(409, false);
            let page_pointer = (*main).layoutPage.getPointer();
            let p_header = TypoLayoutPage::getHeaderPage(page_pointer, &*snap);
            let p_footer = TypoLayoutPage::getFooterPage(page_pointer, &*snap);
            if brc_prop.getPgbOffsetFrom() == 0 {
                let surround_header = !prop.getBoolean__int_boolean(410, true);
                let surround_footer = !prop.getBoolean__int_boolean(411, true);
                let mut left = page_env.marginLeft;
                let mut right = page_env.width - page_env.marginRight;
                let mut top = page_env.marginTop;
                let mut bottom = page_env.height - page_env.marginBottom;
                if p_header != 0 {
                    let rect = header_footer_rect(&*snap, p_header);
                    top = if surround_header { rect.1 } else { rect.3.max(top) };
                }
                if p_footer != 0 {
                    let rect = header_footer_rect(&*snap, p_footer);
                    bottom = if surround_footer { rect.3 } else { rect.1.min(bottom) };
                }
                if let Some(brc) = state.brcLeft.clone() {
                    left -= UnitUtil::PT2TWIPS(brc.getSpace() + self.getLineWidth(&brc) / 2.0);
                    typo_brcs.setBrcLeft(brc);
                }
                if let Some(brc) = state.brcRight.clone() {
                    right += UnitUtil::PT2TWIPS(brc.getSpace() + self.getLineWidth(&brc) / 2.0);
                    typo_brcs.setBrcRight(brc);
                }
                if let Some(brc) = state.brcTop.clone() {
                    top -= UnitUtil::PT2TWIPS(brc.getSpace() + self.getLineWidth(&brc) / 2.0);
                    typo_brcs.setBrcTop(brc);
                }
                if let Some(brc) = state.brcBottom.clone() {
                    bottom += UnitUtil::PT2TWIPS(brc.getSpace() + self.getLineWidth(&brc) / 2.0);
                    typo_brcs.setBrcBottom(brc);
                }
                typo_brcs.set(left, top, right, bottom);
            } else {
                let mut left = 0;
                let mut top = 0;
                let mut right = page_env.width;
                let mut bottom = page_env.height;
                if let Some(brc) = state.brcLeft.clone() {
                    left += UnitUtil::PT2TWIPS(brc.getSpace() + self.getLineWidth(&brc) / 2.0);
                    typo_brcs.setBrcLeft(brc);
                }
                if let Some(brc) = state.brcRight.clone() {
                    right -= UnitUtil::PT2TWIPS(brc.getSpace() + self.getLineWidth(&brc) / 2.0);
                    typo_brcs.setBrcRight(brc);
                }
                if let Some(brc) = state.brcTop.clone() {
                    top += UnitUtil::PT2TWIPS(brc.getSpace() + self.getLineWidth(&brc) / 2.0);
                    typo_brcs.setBrcTop(brc);
                }
                if let Some(brc) = state.brcBottom.clone() {
                    bottom -= UnitUtil::PT2TWIPS(brc.getSpace() + self.getLineWidth(&brc) / 2.0);
                    typo_brcs.setBrcBottom(brc);
                }
                typo_brcs.set(left, top, right, bottom);
            }
            if align_borders {
                self.alignBorders(state, &typo_brcs, &mut *snap, p_header, p_footer);
            }
            let pointer = typo_brcs.getPointer();
            (*main).layoutPage.setPageBRCs(pointer);
        }
    }

    // java the alignBorders block inside flowPageBorders
    fn alignBorders(&mut self, state: &mut LayoutState, typo_brcs: &TypoBRCs, snapshot: &mut TypoSnapshot, p_header: i32, p_footer: i32) {
        let page_env = &state.pageEnv;
        if state.curPage.as_ref().map(|p| TypoPage::hasParaShdAndBrcs__int_typosnapshot(p.getPointer(), snapshot)).unwrap_or(false) {
            let (left, top, right, bottom, brc_left, brc_top, brc_right, brc_bottom) = self.rotated_border(page_env, typo_brcs, false);
            let lines = state.curPage.as_ref().map(|p| p.getLines()).unwrap_or(0);
            self.dealLeftAndRightBRCs(lines, left, brc_left.as_ref(), right, brc_right.as_ref(), snapshot);
            self.combineVertical(lines, top, brc_top.as_ref(), bottom, brc_bottom.as_ref(), snapshot);
        }
        let need_header = p_header != 0 && TypoPage::hasParaShdAndBrcs__int_typosnapshot(p_header, snapshot);
        let need_footer = p_footer != 0 && TypoPage::hasParaShdAndBrcs__int_typosnapshot(p_footer, snapshot);
        if need_header || need_footer {
            let (left, top, right, bottom, brc_left, brc_top, brc_right, brc_bottom) = self.rotated_border(page_env, typo_brcs, true);
            if need_header {
                self.dealWithHeaderFooter(p_header, left, brc_left.as_ref(), right, brc_right.as_ref(), top, brc_top.as_ref(), snapshot);
            }
            if need_footer {
                self.dealWithHeaderFooter(p_footer, left, brc_left.as_ref(), right, brc_right.as_ref(), bottom, brc_bottom.as_ref(), snapshot);
            }
        }
    }

    fn rotated_border(&self, page_env: &PageEnv::PageEnv, typo_brcs: &TypoBRCs, header: bool) -> (i32, i32, i32, i32, Option<BRC>, Option<BRC>, Option<BRC>, Option<BRC>) {
        match page_env.textFlow {
            1 | 5 => {
                let left = if header { typo_brcs.getTop() } else { typo_brcs.getTop() - page_env.marginTop };
                let top = page_env.width - typo_brcs.getRight() - page_env.marginRight;
                let right = left + typo_brcs.height();
                let bottom = top + typo_brcs.width();
                (left, top, right, bottom, typo_brcs.getBrcTop(), typo_brcs.getBrcRight(), typo_brcs.getBrcBottom(), typo_brcs.getBrcLeft())
            }
            _ => {
                let left = typo_brcs.getLeft() - page_env.marginLeft;
                let top = if header { typo_brcs.getTop() } else { typo_brcs.getTop() - page_env.marginTop };
                let right = left + typo_brcs.width();
                let bottom = top + typo_brcs.height();
                (left, top, right, bottom, typo_brcs.getBrcLeft(), typo_brcs.getBrcTop(), typo_brcs.getBrcRight(), typo_brcs.getBrcBottom())
            }
        }
    }

    // java private void dealLeftAndRightBRCs(...)
    fn dealLeftAndRightBRCs(&mut self, lines: i32, left: i32, brc_left: Option<&BRC>, right: i32, brc_right: Option<&BRC>, snapshot: &mut TypoSnapshot) {
        if brc_left.is_none() && brc_right.is_none() {
            return;
        }
        let page_left = match brc_left {
            Some(brc) => left + UnitUtil::PT2TWIPS(self.getLineWidth(brc) / 2.0),
            None => 0,
        };
        let page_right = match brc_right {
            Some(brc) => right - UnitUtil::PT2TWIPS(self.getLineWidth(brc) / 2.0),
            None => 0,
        };
        let size = TypoList::size__int_typosnapshot(lines, snapshot);
        let mut i = 0;
        while i < size {
            let line = TypoList::quickGet__int_int_typosnapshot(i, lines, snapshot);
            if TypoMemoryObj::getType__int_typosnapshot(line, snapshot) == 3 {
                let shd = TypoTextLine::getSHDAndBRCs__int_typosnapshot(line, snapshot);
                if shd != 0 {
                    let count = snapshot.get__int(shd + 2);
                    let mut j = 0;
                    while j < count {
                        if brc_left.is_some() {
                            let cur = shd_item(snapshot, shd, j, 5);
                            if cur != 0 && self.canCombine(page_left, shd_item(snapshot, shd, j, 0)) {
                                set_shd_item(snapshot, shd, j, 5, 0);
                                set_shd_item(snapshot, shd, j, 0, page_left);
                            }
                        }
                        if brc_right.is_some() {
                            let cur = shd_item(snapshot, shd, j, 7);
                            if cur != 0 && self.canCombine(page_right, shd_item(snapshot, shd, j, 2)) {
                                set_shd_item(snapshot, shd, j, 7, 0);
                                set_shd_item(snapshot, shd, j, 2, page_right);
                            }
                        }
                        j += 1;
                    }
                }
            }
            i += 1;
        }
    }

    // java flowPageBorders alignBorders: first line top and last line bottom
    fn combineVertical(&mut self, lines: i32, top: i32, brc_top: Option<&BRC>, bottom: i32, brc_bottom: Option<&BRC>, snapshot: &mut TypoSnapshot) {
        let first = TypoList::first__int_typosnapshot(lines, snapshot);
        if brc_top.is_some() && TypoMemoryObj::getType__int_typosnapshot(first, snapshot) == 3 {
            let shd = TypoTextLine::getSHDAndBRCs__int_typosnapshot(first, snapshot);
            if shd != 0 && shd_item(snapshot, shd, 0, 6) != 0 {
                let text_top = snapshot.get__int(first + 4);
                let page_top = top + UnitUtil::PT2TWIPS(self.getLineWidth(brc_top.unwrap()) / 2.0);
                let para_top = shd_item(snapshot, shd, 0, 1) + text_top;
                if self.canCombine(page_top, para_top) {
                    let cur_top = top - text_top;
                    let count = snapshot.get__int(shd + 2);
                    let mut i = 0;
                    while i < count {
                        set_shd_item(snapshot, shd, i, 6, 0);
                        set_shd_item(snapshot, shd, i, 1, cur_top);
                        i += 1;
                    }
                }
            }
        }
        let last = TypoList::last__int_typosnapshot(lines, snapshot);
        if brc_bottom.is_some() && TypoMemoryObj::getType__int_typosnapshot(last, snapshot) == 3 {
            let shd = TypoTextLine::getSHDAndBRCs__int_typosnapshot(last, snapshot);
            if shd != 0 && shd_item(snapshot, shd, 0, 8) != 0 {
                let page_bottom = bottom - UnitUtil::PT2TWIPS(self.getLineWidth(brc_bottom.unwrap()) / 2.0);
                let text_top = snapshot.get__int(last + 4);
                let para_bottom = text_top + shd_item(snapshot, shd, 0, 3);
                if self.canCombine(page_bottom, para_bottom) {
                    let cur_bottom = bottom - text_top;
                    let count = snapshot.get__int(shd + 2);
                    let mut i = 0;
                    while i < count {
                        set_shd_item(snapshot, shd, i, 8, 0);
                        set_shd_item(snapshot, shd, i, 3, cur_bottom);
                        i += 1;
                    }
                }
            }
        }
    }

    // java private void dealWithHeaderFooter(...)
    fn dealWithHeaderFooter(&mut self, p_header_footer: i32, left: i32, brc_left: Option<&BRC>, right: i32, brc_right: Option<&BRC>, brc_pos: i32, brc: Option<&BRC>, snapshot: &mut TypoSnapshot) {
        let lines = TypoPage::getLines__int_typosnapshot(p_header_footer, snapshot);
        self.dealLeftAndRightBRCs(lines, left, brc_left, right, brc_right, snapshot);
        let Some(brc) = brc else { return; };
        let first = TypoList::first__int_typosnapshot(lines, snapshot);
        if TypoMemoryObj::getType__int_typosnapshot(first, snapshot) == 3 {
            let shd = TypoTextLine::getSHDAndBRCs__int_typosnapshot(first, snapshot);
            if shd != 0 && shd_item(snapshot, shd, 0, 6) != 0 {
                let page_bottom = brc_pos + UnitUtil::PT2TWIPS(self.getLineWidth(brc) / 2.0);
                let text_top = snapshot.get__int(first + 4);
                let page_top = snapshot.get__int(p_header_footer + 4);
                let para_top = shd_item(snapshot, shd, 0, 1) + text_top + page_top;
                if self.canCombine(page_bottom, para_top) {
                    let cur_top = brc_pos - page_top - text_top;
                    let count = snapshot.get__int(shd + 2);
                    let mut i = 0;
                    while i < count {
                        set_shd_item(snapshot, shd, i, 6, 0);
                        set_shd_item(snapshot, shd, i, 1, cur_top);
                        i += 1;
                    }
                }
            }
        }
        let last = TypoList::last__int_typosnapshot(lines, snapshot);
        if TypoMemoryObj::getType__int_typosnapshot(last, snapshot) == 3 {
            let shd = TypoTextLine::getSHDAndBRCs__int_typosnapshot(last, snapshot);
            if shd != 0 && shd_item(snapshot, shd, 0, 8) != 0 {
                let page_top = brc_pos - UnitUtil::PT2TWIPS(self.getLineWidth(brc) / 2.0);
                let text_top = snapshot.get__int(last + 4);
                let host_top = snapshot.get__int(p_header_footer + 4);
                let para_bottom = shd_item(snapshot, shd, 0, 3) + text_top + host_top;
                if self.canCombine(page_top, para_bottom) {
                    let cur_bottom = brc_pos - host_top - text_top;
                    let count = snapshot.get__int(shd + 2);
                    let mut i = 0;
                    while i < count {
                        set_shd_item(snapshot, shd, i, 8, 0);
                        set_shd_item(snapshot, shd, i, 3, cur_bottom);
                        i += 1;
                    }
                }
            }
        }
    }

    // java boolean isValidedPageBRC(BRC brc)
    pub fn isValidedPageBRC(&self, brc: Option<&BRC>) -> bool {
        let Some(brc) = brc else { return false; };
        let type_id = brc.getBorderType();
        type_id != 0 && (type_id <= 27 || (type_id >= 64 && type_id <= 227))
    }

    // java protected final boolean canCombine(int pageBrcSpace, int paraBrcSpace)
    pub fn canCombine(&self, page_brc_space: i32, para_brc_space: i32) -> bool {
        if page_brc_space >= para_brc_space {
            page_brc_space - para_brc_space < UnitUtil::PT2TWIPS(11.0)
        } else {
            para_brc_space - page_brc_space < UnitUtil::PT2TWIPS(9.0)
        }
    }

    // java protected final float getLineWidth(BRC brc)
    pub fn getLineWidth(&self, brc: &BRC) -> f32 {
        BRCBorderTool::getActualStrokeWidth(brc.getBorderType(), brc.getLineWidth())
    }
}

// java TypoHeaderFooterPage.rectForPage: copy the page rect, then mirror-margin of its layout page.
fn header_footer_rect(snapshot: &TypoSnapshot, pointer: i32) -> (i32, i32, i32, i32) {
    let mut left = snapshot.get__int(pointer + 3);
    let top = snapshot.get__int(pointer + 4);
    let mut right = snapshot.get__int(pointer + 5);
    let bottom = snapshot.get__int(pointer + 6);
    let layout_page = layout_page_of(snapshot, pointer);
    if layout_page != 0 {
        let mirror = snapshot.get__int(layout_page + 28);
        if mirror != 0 {
            left = left.wrapping_add(mirror);
            right = right.wrapping_add(mirror);
        }
    }
    (left, top, right, bottom)
}

fn layout_page_of(snapshot: &TypoSnapshot, pointer: i32) -> i32 {
    let mut root = pointer;
    let mut parent = snapshot.get__int(pointer + 2);
    while parent != 0 {
        root = parent;
        parent = snapshot.get__int(parent + 2);
    }
    match snapshot.getTypoType(root) {
        2 => root,
        _ => 0,
    }
}

// java TypoSHDAndBRCs item: data at pointer + 3, item size 9.
fn shd_item(snapshot: &TypoSnapshot, shd: i32, index: i32, field: i32) -> i32 {
    snapshot.get__int(shd + 3 + index * 9 + field)
}

fn set_shd_item(snapshot: &mut TypoSnapshot, shd: i32, index: i32, field: i32, value: i32) {
    snapshot.set__int_int(shd + 3 + index * 9 + field, value);
}
