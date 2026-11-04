#![allow(dead_code, non_snake_case)]

use crate::cn::wps::moffice::util::BoolBitSet::BoolBitSet;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage;
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;

// java ITypoRect carrier used by the static coordinate methods.
#[derive(Clone, Copy, Default)]
pub struct CoordRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl CoordRect {
    pub fn width(&self) -> i32 { self.right - self.left }
    pub fn height(&self) -> i32 { self.bottom - self.top }
    pub fn set(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        self.left = left;
        self.top = top;
        self.right = right;
        self.bottom = bottom;
    }
    pub fn set_empty(&mut self) { self.set(0, 0, 0, 0); }
    pub fn offset(&mut self, dx: i32, dy: i32) {
        self.left += dx;
        self.right += dx;
        self.top += dy;
        self.bottom += dy;
    }
    pub fn offset_to(&mut self, x: i32, y: i32) {
        let w = self.width();
        let h = self.height();
        self.left = x;
        self.top = y;
        self.right = x + w;
        self.bottom = y + h;
    }
}

pub struct DrawingCoordTool;

fn mem_type(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
    if pointer != 0 { snapshot.get__int(pointer) } else { -1 }
}

fn own_rect(pointer: i32, snapshot: &TypoSnapshot) -> CoordRect {
    CoordRect {
        left: TypoObjBase::getLeft__int_typosnapshot(pointer, snapshot),
        top: TypoObjBase::getTop__int_typosnapshot(pointer, snapshot),
        right: TypoObjBase::getRight__int_typosnapshot(pointer, snapshot),
        bottom: TypoObjBase::getBottom__int_typosnapshot(pointer, snapshot),
    }
}

fn page_span_width(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
    crate::cn::wps::moffice::writer::cache::TypoRectMemoryImpl::TypoRectMemoryImpl::width__int_typosnapshot(pointer + 3, snapshot)
}

fn page_span_height(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
    crate::cn::wps::moffice::writer::cache::TypoRectMemoryImpl::TypoRectMemoryImpl::height__int_typosnapshot(pointer + 3, snapshot)
}

// java TextFlowTool.convertVisualDirToLogicDir
pub fn convertVisualDirToLogicDir(textflow: i32, visual_dir: i32) -> i32 {
    const NORMAL: [i32; 4] = [0, 1, 2, 3];
    const BT2LR: [i32; 4] = [1, 2, 3, 0];
    const TB2RL: [i32; 4] = [3, 0, 1, 2];
    let map = match textflow {
        1 => &TB2RL,
        3 => &BT2LR,
        _ => &NORMAL,
    };
    if visual_dir >= 0 && (visual_dir as usize) < map.len() { map[visual_dir as usize] } else { 0 }
}

// java TextFlowTool.doGetPageMarginByLogicDir
pub fn marginByLogicDir(pointer: i32, logic_dir: i32, snapshot: &TypoSnapshot) -> i32 {
    match logic_dir {
        0 => TypoPage::getMarginLeft__int_typosnapshot(pointer, snapshot),
        1 => TypoPage::getMarginTop__int_typosnapshot(pointer, snapshot),
        2 => TypoPage::getMarginRight__int_typosnapshot(pointer, snapshot),
        3 => TypoPage::getMarginBottom__int_typosnapshot(pointer, snapshot),
        _ => 0,
    }
}

// java TypoDrawing.isRotatedWithPage / TypoTextBox.isRotatedWithParentPage / TypoPage.isRotatedWithParentPage
pub fn isRotatedWithParentPage(pointer: i32, snapshot: &TypoSnapshot) -> bool {
    match mem_type(pointer, snapshot) {
        14 | 11 => true,
        8 => {
            let parent = TypoObjBase::getParent__int_typosnapshot(pointer, snapshot);
            mem_type(parent, snapshot) == 7 && BoolBitSet::new(18).get(snapshot.get__int(parent + 7))
        }
        _ => false,
    }
}

// java TypoPage.getVisualTextflow, including column / footnote / textbox overrides
pub fn visualTextflow(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
    match mem_type(pointer, snapshot) {
        14 | 11 => {
            let page = TypoObjBase::getPage__int_typosnapshot(pointer, snapshot);
            if page == 0 { 0 } else { visualTextflow(page, snapshot) }
        }
        8 => {
            if isRotatedWithParentPage(pointer, snapshot) {
                let page = TypoObjBase::getPage__int_typosnapshot(pointer, snapshot);
                if page == 0 { 0 } else { visualTextflow(page, snapshot) }
            } else {
                TypoPage::getTextFlow__int_typosnapshot(pointer, snapshot)
            }
        }
        _ => TypoPage::getTextFlow__int_typosnapshot(pointer, snapshot),
    }
}

pub fn isVertTextflow__int(textflow: i32) -> bool { textflow != 0 }

pub fn getMarginLeft(pointer: i32, textflow: i32, snapshot: &TypoSnapshot) -> i32 {
    marginByLogicDir(pointer, convertVisualDirToLogicDir(textflow, 0), snapshot)
}
pub fn getMarginTop(pointer: i32, textflow: i32, snapshot: &TypoSnapshot) -> i32 {
    marginByLogicDir(pointer, convertVisualDirToLogicDir(textflow, 1), snapshot)
}
pub fn getMarginRight(pointer: i32, textflow: i32, snapshot: &TypoSnapshot) -> i32 {
    marginByLogicDir(pointer, convertVisualDirToLogicDir(textflow, 2), snapshot)
}
pub fn getMarginBottom(pointer: i32, textflow: i32, snapshot: &TypoSnapshot) -> i32 {
    marginByLogicDir(pointer, convertVisualDirToLogicDir(textflow, 3), snapshot)
}

pub fn getPageWidth(pointer: i32, visual_textflow: i32, snapshot: &TypoSnapshot) -> i32 {
    if isRotatedWithParentPage(pointer, snapshot) {
        if isVertTextflow__int(visual_textflow) { page_span_height(pointer, snapshot) } else { page_span_width(pointer, snapshot) }
    } else {
        page_span_width(pointer, snapshot)
    }
}

pub fn getPageHeight(pointer: i32, visual_textflow: i32, snapshot: &TypoSnapshot) -> i32 {
    if isRotatedWithParentPage(pointer, snapshot) {
        if isVertTextflow__int(visual_textflow) { page_span_width(pointer, snapshot) } else { page_span_height(pointer, snapshot) }
    } else {
        page_span_height(pointer, snapshot)
    }
}

pub fn getPageWidthOf(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
    getPageWidth(pointer, visualTextflow(pointer, snapshot), snapshot)
}

pub fn getPageHeightOf(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
    getPageHeight(pointer, visualTextflow(pointer, snapshot), snapshot)
}

fn rect_page_core_to_page(page: i32, rect: &mut CoordRect, snapshot: &TypoSnapshot) {
    let flow = visualTextflow(page, snapshot);
    match flow {
        0 => {
            rect.offset(getMarginLeft(page, flow, snapshot), getMarginTop(page, flow, snapshot));
        }
        1 => {
            let org_w = rect.width();
            let org_h = rect.height();
            let visual_w = getPageWidth(page, flow, snapshot);
            let x = visual_w - getMarginTop(page, flow, snapshot) - rect.bottom;
            let y = getMarginLeft(page, flow, snapshot) + rect.left;
            rect.offset_to(x, y);
            rect.right = rect.left + org_h;
            rect.bottom = rect.top + org_w;
        }
        3 => {
            let org_w = rect.width();
            let org_h = rect.height();
            let visual_h = getPageHeight(page, flow, snapshot);
            let x = getMarginTop(page, flow, snapshot) + rect.top;
            let y = visual_h - getMarginLeft(page, flow, snapshot) - rect.right;
            rect.offset_to(x, y);
            rect.right = rect.left + org_h;
            rect.bottom = rect.top + org_w;
        }
        _ => {}
    }
}

fn rect_page_to_page_core(page: i32, rect: &mut CoordRect, snapshot: &TypoSnapshot) {
    let flow = visualTextflow(page, snapshot);
    match flow {
        0 => {
            rect.offset(-getMarginLeft(page, flow, snapshot), -getMarginTop(page, flow, snapshot));
        }
        1 => {
            let org_w = rect.width();
            let org_h = rect.height();
            let visual_w = getPageWidth(page, flow, snapshot);
            let x = rect.top - getMarginLeft(page, flow, snapshot);
            let y = visual_w - getMarginTop(page, flow, snapshot) - rect.right;
            rect.offset_to(x, y);
            rect.right = rect.left + org_h;
            rect.bottom = rect.top + org_w;
        }
        3 => {
            let org_w = rect.width();
            let org_h = rect.height();
            let visual_h = getPageHeight(page, flow, snapshot);
            let x = visual_h - getMarginLeft(page, flow, snapshot) - rect.bottom;
            let y = rect.left - getMarginTop(page, flow, snapshot);
            rect.offset_to(x, y);
            rect.right = rect.left + org_h;
            rect.bottom = rect.top + org_w;
        }
        _ => {}
    }
}

// java TypoObjBase.rectForPageCore for the page and line types used by drawing alignment
pub fn rectForPageCore(pointer: i32, rect: &mut CoordRect, snapshot: &TypoSnapshot) {
    match mem_type(pointer, snapshot) {
        3 => {
            *rect = own_rect(pointer, snapshot);
            let flags = snapshot.get__int(pointer + 12);
            if BoolBitSet::new(5).get(flags) {
                let width = rect.width();
                let page = TypoObjBase::getPage__int_typosnapshot(pointer, snapshot);
                rect.right = TypoPage::getCoreWidth__int_typosnapshot(page, snapshot) - TypoObjBase::getLeft__int_typosnapshot(pointer, snapshot);
                rect.left = rect.right - width;
            }
        }
        14 => {
            *rect = own_rect(pointer, snapshot);
            let parent = TypoObjBase::getParent__int_typosnapshot(pointer, snapshot);
            if parent != 0 {
                rect.offset(0, TypoObjBase::getTop__int_typosnapshot(parent, snapshot));
            }
        }
        11 => {
            *rect = own_rect(pointer, snapshot);
            let parent = TypoObjBase::getParent__int_typosnapshot(pointer, snapshot);
            if mem_type(parent, snapshot) == 12 {
                rect.offset(0, TypoObjBase::getTop__int_typosnapshot(parent, snapshot));
            }
        }
        9 => {
            let layout = TypoObjBase::getLayoutPage__int_typosnapshot(pointer, snapshot);
            if layout != 0 {
                *rect = own_rect(pointer, snapshot);
                let mirror = snapshot.get__int(layout + 28);
                if mirror != 0 {
                    rect.offset(mirror, 0);
                }
                rect_page_to_page_core(layout, rect, snapshot);
            }
        }
        _ => {
            *rect = own_rect(pointer, snapshot);
        }
    }
}

fn rect_for_layout_page(pointer: i32, rect: &mut CoordRect, snapshot: &TypoSnapshot) {
    rect.set_empty();
    let page = TypoObjBase::getPage__int_typosnapshot(pointer, snapshot);
    if page != 0 {
        rectForPageCore(pointer, rect, snapshot);
        rect_page_core_to_layout(page, rect, snapshot);
    }
}

fn rect_page_core_to_layout(page: i32, rect: &mut CoordRect, snapshot: &TypoSnapshot) {
    rect_page_core_to_page(page, rect, snapshot);
    let left = rect.left;
    let top = rect.top;
    let right = rect.right;
    let bottom = rect.bottom;
    rect_for_layout_page(page, rect, snapshot);
    let page_left = rect.left;
    let page_top = rect.top;
    rect.set(left + page_left, top + page_top, right + page_left, bottom + page_top);
}

fn obj_rect_for_page(pointer: i32, rect: &mut CoordRect, snapshot: &TypoSnapshot) {
    rectForPageCore(pointer, rect, snapshot);
    let page = TypoObjBase::getPage__int_typosnapshot(pointer, snapshot);
    if page != 0 {
        rect_page_core_to_page(page, rect, snapshot);
    }
}

fn calc_rect_for_layout_page(pointer: i32, rect: &mut CoordRect, snapshot: &TypoSnapshot) {
    if mem_type(pointer, snapshot) == 2 {
        rect.set(0, 0, getPageWidthOf(pointer, snapshot), getPageHeightOf(pointer, snapshot));
    } else {
        rect_for_layout_page(pointer, rect, snapshot);
    }
}

fn relative_left(textflow: i32, page: i32, rect: &CoordRect, snapshot: &TypoSnapshot) -> i32 {
    match textflow {
        0 => rect.left,
        1 => getPageWidth(page, textflow, snapshot) - rect.bottom,
        3 => rect.top,
        _ => 0,
    }
}

fn relative_top(textflow: i32, page: i32, rect: &CoordRect, snapshot: &TypoSnapshot) -> i32 {
    match textflow {
        0 => rect.top,
        1 => rect.left,
        3 => getPageHeight(page, textflow, snapshot) - rect.width() - rect.left,
        _ => 0,
    }
}

// java DrawingCoordTool.calcRectForPage
pub fn calcRectForPage(rect_page: i32, relative_page: i32, rect: &mut CoordRect, snapshot: &TypoSnapshot) {
    let flow = visualTextflow(rect_page, snapshot);
    rect.offset(
        TypoPage::getMarginLeft__int_typosnapshot(rect_page, snapshot),
        TypoPage::getMarginTop__int_typosnapshot(rect_page, snapshot),
    );
    let swap = isVertTextflow__int(flow);
    let logic_w = rect.width().abs();
    let logic_h = rect.height().abs();
    let visual_w = if swap { logic_h } else { logic_w };
    let visual_h = if swap { logic_w } else { logic_h };
    let rel_left = relative_left(flow, rect_page, rect, snapshot);
    let rel_top = relative_top(flow, rect_page, rect, snapshot);
    if rect_page == relative_page {
        *rect = own_rect(rect_page, snapshot);
        rect.offset_to(0, 0);
    } else if mem_type(rect_page, snapshot) == 9 {
        if mem_type(relative_page, snapshot) == 2 {
            let layout = TypoObjBase::getLayoutPage__int_typosnapshot(relative_page, snapshot);
            *rect = own_rect(rect_page, snapshot);
            if layout != 0 {
                let mirror = snapshot.get__int(layout + 28);
                if mirror != 0 {
                    rect.offset(mirror, 0);
                }
            }
            rect_page_core_to_page(relative_page, rect, snapshot);
        } else {
            rect_for_layout_page(relative_page, rect, snapshot);
            let rel_page_left = rect.left;
            let rel_page_top = rect.top;
            rect_for_layout_page(rect_page, rect, snapshot);
            let dx = rect.left - rel_page_left;
            let dy = rect.top - rel_page_top;
            rect.set(dx, dy, dx, dy);
        }
    } else if TypoObjBase::getPage__int_typosnapshot(rect_page, snapshot) == relative_page {
        obj_rect_for_page(rect_page, rect, snapshot);
    } else {
        rect_for_layout_page(relative_page, rect, snapshot);
        let rel_page_left = rect.left;
        let rel_page_top = rect.top;
        rect_for_layout_page(rect_page, rect, snapshot);
        let dx = rect.left - rel_page_left;
        let dy = rect.top - rel_page_top;
        rect.set(dx, dy, dx, dy);
    }
    let dest_left = rect.left + rel_left;
    let dest_top = rect.top + rel_top;
    rect.set(dest_left, dest_top, dest_left + visual_w, dest_top + visual_h);
}

// java DrawingCoordTool.rectForPage
pub fn rectForPage(p_obj: i32, p_rel: i32, rect: &mut CoordRect, snapshot: &TypoSnapshot) {
    let obj_type = mem_type(p_obj, snapshot);
    if p_obj == p_rel {
        rect.set(0, 0, getPageWidthOf(p_obj, snapshot), getPageHeightOf(p_obj, snapshot));
    } else if obj_type != 9 && obj_type != 26 {
        if TypoObjBase::getPage__int_typosnapshot(p_obj, snapshot) == p_rel {
            obj_rect_for_page(p_obj, rect, snapshot);
        } else {
            rect_for_layout_page(p_rel, rect, snapshot);
            let page_left = rect.left;
            let page_top = rect.top;
            calc_rect_for_layout_page(p_obj, rect, snapshot);
            rect.offset(-page_left, -page_top);
        }
    } else if mem_type(p_rel, snapshot) == 2 {
        obj_rect_for_page(p_obj, rect, snapshot);
    } else {
        rect_for_layout_page(p_rel, rect, snapshot);
        let page_left = rect.left;
        let page_top = rect.top;
        calc_rect_for_layout_page(p_obj, rect, snapshot);
        rect.offset(-page_left, -page_top);
    }
}

pub fn reverseOffsetMarginLeftAndTop(page: i32, rect: &mut CoordRect, snapshot: &TypoSnapshot) {
    if page != 0 {
        let flow = visualTextflow(page, snapshot);
        rect.offset(-getMarginLeft(page, flow, snapshot), -getMarginTop(page, flow, snapshot));
    }
}

// java DrawingCoordTool.rectForPageCore(int, TypoPage, ITypoRect)
pub fn rectForPageCoreRel(p_obj: i32, p_rel: i32, rect: &mut CoordRect, snapshot: &TypoSnapshot) {
    rectForPage(p_obj, p_rel, rect, snapshot);
    reverseOffsetMarginLeftAndTop(p_rel, rect, snapshot);
}

pub fn getLineXForFloatsPageCore(lines_page: i32, floats_page: i32, line_y: i32, snapshot: &TypoSnapshot) -> i32 {
    let mut tmp = CoordRect { left: 0, top: line_y, right: 0, bottom: line_y };
    calcRectForPage(lines_page, floats_page, &mut tmp, snapshot);
    reverseOffsetMarginLeftAndTop(floats_page, &mut tmp, snapshot);
    tmp.left
}

fn list_index_of(item: i32, p_list: i32, snapshot: &TypoSnapshot) -> i32 {
    let size = if p_list == 0 { 0 } else { snapshot.get__int(p_list + 2) };
    let mut i = 0;
    while i < size {
        if snapshot.get__int(p_list + 3 + i) == item {
            return i;
        }
        i += 1;
    }
    -1
}

fn list_get(index: i32, p_list: i32, snapshot: &TypoSnapshot) -> i32 {
    let size = if p_list == 0 { 0 } else { snapshot.get__int(p_list + 2) };
    if index >= 0 && index < size { snapshot.get__int(p_list + 3 + index) } else { 0 }
}

fn header_excess(layout_page: i32, snapshot: &TypoSnapshot) -> i32 {
    let template_index = snapshot.get__int(layout_page + 20);
    let template = snapshot.getTemplete(template_index);
    let header = if template == 0 { 0 } else { snapshot.get__int(template + 12) };
    if header == 0 {
        return 0;
    }
    let mut core = CoordRect::default();
    rectForPageCore(header, &mut core, snapshot);
    core.bottom.max(0)
}

// java DrawingCoordTool.getPrevLineBottom
pub fn getPrevLineBottom(floats_page: i32, line: i32, snapshot: &TypoSnapshot) -> i32 {
    if line == 0 {
        return 0;
    }
    let lines_page = TypoObjBase::getPage__int_typosnapshot(line, snapshot);
    let lines = snapshot.get__int(lines_page + 17);
    let line_pos = list_index_of(line, lines, snapshot);
    let mut prev_bottom = 0;
    if line_pos != 0 {
        prev_bottom = TypoObjBase::getBottom__int_typosnapshot(list_get(line_pos - 1, lines, snapshot), snapshot);
    } else if !isVertTextflow__int(visualTextflow(lines_page, snapshot)) {
        match mem_type(lines_page, snapshot) {
            2 => prev_bottom = header_excess(lines_page, snapshot),
            6 => prev_bottom = TypoObjBase::getTop__int_typosnapshot(line, snapshot),
            _ => {}
        }
    }
    let mut line_rect = CoordRect { left: 0, top: prev_bottom, right: 0, bottom: prev_bottom };
    calcRectForPage(lines_page, floats_page, &mut line_rect, snapshot);
    reverseOffsetMarginLeftAndTop(floats_page, &mut line_rect, snapshot);
    if isVertTextflow__int(visualTextflow(lines_page, snapshot)) { line_rect.left } else { line_rect.top }
}

// java DrawingAlignRectTool.calcAlignRectLeftAndRightWithPage
pub fn calcAlignRectLeftAndRightWithPage(rel_x: i32, align_page: i32, floats_page: i32, rect: &mut CoordRect, snapshot: &TypoSnapshot) {
    let flow = visualTextflow(align_page, snapshot);
    let margin_left = getMarginLeft(align_page, flow, snapshot);
    let margin_right = getMarginRight(align_page, flow, snapshot);
    let top_bak = rect.top;
    let bottom_bak = rect.bottom;
    rectForPageCoreRel(align_page, floats_page, rect, snapshot);
    let mut left = rect.left;
    let mut right = rect.right;
    match rel_x {
        0 | 2 => {
            left += margin_left;
            right -= margin_right;
        }
        1 => {}
        4 => right = left + margin_left,
        5 => left = right - margin_right,
        _ => {
            left = 0;
            right = 0;
        }
    }
    rect.set(left, top_bak, right, bottom_bak);
}

// java TypoObjBase.rectForLayoutPageCore
pub fn rectForLayoutPageCore(pointer: i32, rect: &mut CoordRect, snapshot: &TypoSnapshot) {
    let page = TypoObjBase::getPage__int_typosnapshot(pointer, snapshot);
    if page == 0 {
        return;
    }
    if mem_type(page, snapshot) == 2 {
        rectForPageCore(pointer, rect, snapshot);
    } else {
        rect_for_layout_page(pointer, rect, snapshot);
        let layout = TypoObjBase::getLayoutPage__int_typosnapshot(pointer, snapshot);
        if layout != 0 {
            rect_page_to_page_core(layout, rect, snapshot);
        }
    }
}

fn normalized_page_core_offset(page: i32, snapshot: &TypoSnapshot) -> CoordRect {
    let mut rect = CoordRect::default();
    if mem_type(page, snapshot) == 2 {
        rect.set_empty();
    } else {
        rectForLayoutPageCore(page, &mut rect, snapshot);
        rect.offset(
            TypoPage::getMarginLeft__int_typosnapshot(page, snapshot),
            TypoPage::getMarginTop__int_typosnapshot(page, snapshot),
        );
    }
    rect
}

pub fn pack_int_int(a: i32, b: i32) -> i64 {
    ((a as i64) << 32) | (b as u32 as i64)
}

pub fn unpack_first_int(value: i64) -> i32 { (value >> 32) as i32 }
pub fn unpack_second_int(value: i64) -> i32 { value as i32 }

// java DrawingPositionTool.getColPageDrawingOffsetResult
pub fn colPageDrawingOffset(cur_page: i32, dest_page: i32, snapshot: &TypoSnapshot) -> i64 {
    if cur_page == dest_page {
        return 0;
    }
    let cur = normalized_page_core_offset(cur_page, snapshot);
    let dest = normalized_page_core_offset(dest_page, snapshot);
    pack_int_int(cur.left - dest.left, cur.top - dest.top)
}

fn footer_page(layout_page: i32, snapshot: &TypoSnapshot) -> i32 {
    let template = snapshot.getTemplete(snapshot.get__int(layout_page + 20));
    if template == 0 { 0 } else { snapshot.get__int(template + 13) }
}

fn footer_excess(layout_page: i32, snapshot: &TypoSnapshot) -> i32 {
    let footer = footer_page(layout_page, snapshot);
    if footer == 0 {
        return 0;
    }
    let flow = visualTextflow(layout_page, snapshot);
    let core = getPageHeight(layout_page, flow, snapshot)
        - getMarginTop(layout_page, flow, snapshot)
        - getMarginBottom(layout_page, flow, snapshot);
    (core - TypoObjBase::getTop__int_typosnapshot(footer, snapshot)).max(0)
}

fn should_adjust_excess(rel_y: i32, is_wrap: bool, align_page: i32, floats_page: i32, snapshot: &TypoSnapshot) -> bool {
    if rel_y == 2 || rel_y == 3 || is_wrap || mem_type(align_page, snapshot) != 2 {
        return false;
    }
    !isVertTextflow__int(visualTextflow(floats_page, snapshot))
}

// java DrawingAlignRectTool.calcAlignRectTopAndBottomWithPage
pub fn calcAlignRectTopAndBottomWithPage(rel_y: i32, drawing: i32, align_page: i32, floats_page: i32, rect: &mut CoordRect, snapshot: &TypoSnapshot) {
    let flow = visualTextflow(align_page, snapshot);
    let mut margin_top = getMarginTop(align_page, flow, snapshot);
    let mut margin_bottom = getMarginBottom(align_page, flow, snapshot);
    let left_bak = rect.left;
    let right_bak = rect.right;
    rectForPageCoreRel(align_page, floats_page, rect, snapshot);
    let mut top = rect.top;
    let mut bottom = rect.bottom;
    let is_wrap = if drawing == 0 {
        false
    } else {
        let kind = crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::getDrawingType__int_typosnapshot(drawing, snapshot);
        kind == 7 || kind == 8
    };
    if should_adjust_excess(rel_y, is_wrap, align_page, floats_page, snapshot) {
        let layout = TypoObjBase::getLayoutPage__int_typosnapshot(floats_page, snapshot);
        if layout != 0 {
            margin_top += header_excess(layout, snapshot);
            margin_bottom += footer_excess(layout, snapshot);
        }
    }
    match rel_y {
        0 => {
            top += margin_top;
            bottom -= margin_bottom;
        }
        1 => {}
        4 => bottom = top + margin_top,
        5 => top = bottom - margin_bottom,
        _ => {
            top = 0;
            bottom = 0;
        }
    }
    rect.set(left_bak, top, right_bak, bottom);
}
