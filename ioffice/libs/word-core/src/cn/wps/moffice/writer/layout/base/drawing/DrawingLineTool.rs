#![allow(dead_code, non_snake_case)]

use crate::cn::wps::moffice::util::BoolBitSet::BoolBitSet;
use crate::cn::wps::moffice::writer::cache::TypoColumns::TypoColumns;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage;
use crate::cn::wps::moffice::writer::cache::TypoPara::TypoPara;
use crate::cn::wps::moffice::writer::cache::TypoRangeBase::TypoRangeBase;
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
use crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;
use crate::cn::wps::moffice::writer::data::RANGE::RANGE;
use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingCoordTool;
use crate::cn::wps::moffice::writer::layout::base::env::DrawingCacheItem::DrawingCacheItem;

pub struct DrawingLineTool;

struct LineCursor {
    page: i32,
    index: i32,
}

impl LineCursor {
    fn at_end(page: i32, snapshot: &TypoSnapshot) -> Self {
        let lines = TypoPage::getLines__int_typosnapshot(page, snapshot);
        let size = if lines == 0 { 0 } else { snapshot.get__int(lines + 2) };
        Self { page, index: size }
    }

    fn prev(&mut self, snapshot: &TypoSnapshot) -> i32 {
        if self.index <= 0 {
            return 0;
        }
        self.index -= 1;
        let lines = TypoPage::getLines__int_typosnapshot(self.page, snapshot);
        if lines == 0 { 0 } else { snapshot.get__int(lines + 3 + self.index) }
    }
}

impl DrawingLineTool {
    pub fn shouldFindAnchorLineBeforeAdjust(drawing: i32, rel_x: i32, rel_y: i32, snapshot: &TypoSnapshot) -> bool {
        if crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::isLayoutNotInCell__int_typosnapshot(drawing, snapshot) {
            return true;
        }
        if rel_x != 3 && rel_x != 2 {
            rel_y == 3 || rel_y == 2
        } else {
            true
        }
    }

    pub fn shouldFindParaLineBeforeAdjust(drawing: i32, textflow: i32, rel_x: i32, rel_y: i32, snapshot: &TypoSnapshot) -> bool {
        if crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::isLayoutNotInCell__int_typosnapshot(drawing, snapshot) {
            return true;
        }
        if rel_y == 2 {
            return true;
        }
        rel_x == 2 && textflow != 0
    }

    pub fn shouldFlowDrawingAfterFlowParaLine(row: i32, max_anchor_cp: i32, cp: i32, snapshot: &TypoSnapshot) -> bool {
        if max_anchor_cp >= 0 {
            return cp < max_anchor_cp;
        }
        if row != 0 {
            is_in_table_row(row, cp, None, true, snapshot)
        } else {
            false
        }
    }

    pub fn getFloatContainerPage__int(anchor_line_page: i32, snapshot: &TypoSnapshot) -> i32 {
        if mem_type(anchor_line_page, snapshot) == 14 {
            TypoObjBase::getLayoutPage__int_typosnapshot(anchor_line_page, snapshot)
        } else {
            anchor_line_page
        }
    }

    pub fn findAnchorLines(floats_page: i32, cur_line_page: i32, snapshot: &TypoSnapshot, floats_cache: &mut [DrawingCacheItem], row: i32, max_anchor_cp: i32) -> i32 {
        if floats_cache.is_empty() {
            return 0;
        }
        let textflow = DrawingCoordTool::visualTextflow(floats_page, snapshot);
        let is_in_column = mem_type(cur_line_page, snapshot) == 14;
        let mut item_index = floats_cache.len() as i32;
        let mut col_page = if is_in_column { cur_line_page } else { 0 };
        let mut line_it = LineCursor::at_end(cur_line_page, snapshot);
        let mut line = line_it.prev(snapshot);
        let mut last_it_line = line;
        let mut need_find_para_line = false;
        loop {
            let mut found = false;
            let mut found_index = -1;
            while item_index > 0 {
                item_index -= 1;
                let item = &floats_cache[item_index as usize];
                if Self::shouldFindAnchorLineBeforeAdjust(item.drawing, item.relX, item.relY, snapshot) {
                    if !need_find_para_line {
                        need_find_para_line = Self::shouldFindParaLineBeforeAdjust(item.drawing, textflow, item.relX, item.relY, snapshot);
                    }
                    found = true;
                    found_index = item_index;
                    break;
                }
            }
            if !found {
                break;
            }
            let anchor_cp = floats_cache[found_index as usize].cp;
            if Self::shouldFlowDrawingAfterFlowParaLine(row, max_anchor_cp, anchor_cp, snapshot) {
                let mut next_range = RANGE::default();
                let mut cur_range = RANGE::default();
                let mut next_line = 0;
                let mut jumped = false;
                while !is_anchor_in_range(line, anchor_cp, &mut cur_range, snapshot) {
                    if anchor_cp >= cur_range.end && !next_range.isEmpty() && anchor_cp < next_range.start {
                        line = next_line;
                        last_it_line = next_line;
                        jumped = true;
                        break;
                    }
                    next_line = line;
                    line = line_it.prev(snapshot);
                    if line == 0 {
                        while is_in_column && line == 0 {
                            let prev = prev_column_page(col_page, snapshot);
                            if prev == 0 && !cur_range.isEmpty() && anchor_cp < cur_range.start {
                                line = next_line;
                                last_it_line = next_line;
                                jumped = true;
                                break;
                            }
                            col_page = prev;
                            line_it = LineCursor::at_end(col_page, snapshot);
                            line = line_it.prev(snapshot);
                        }
                        if jumped {
                            break;
                        }
                        if line == 0 {
                            let p_para = TypoTextLine::getPara__int_typosnapshot(next_line, snapshot);
                            let graph_index = TypoPara::getGraphIndexFromCP__int_int_typosnapshot(anchor_cp, p_para, snapshot);
                            if graph_index >= 0
                                && TypoPara::getChar__int_int_typosnapshot(graph_index, p_para, snapshot) == '\u{0008}'
                                && graph_index < TypoTextLine::getGraphIndexStart__int_typosnapshot(next_line, snapshot)
                            {
                                line = next_line;
                                last_it_line = next_line;
                                jumped = true;
                            }
                        }
                        if !jumped && line == 0 {
                            break;
                        }
                    }
                    if jumped {
                        break;
                    }
                    last_it_line = line;
                    next_range.set__range(&cur_range);
                }
                if line == 0 {
                    break;
                }
                floats_cache[found_index as usize].anchor = line;
            }
        }
        if last_it_line == 0 {
            let lines = TypoPage::getLines__int_typosnapshot(cur_line_page, snapshot);
            let size = if lines == 0 { 0 } else { snapshot.get__int(lines + 2) };
            last_it_line = if size == 0 { 0 } else { snapshot.get__int(lines + 3 + size - 1) };
        }
        if need_find_para_line { get_para_start_line(last_it_line, snapshot) } else { 0 }
    }

    pub fn getParaStartLine(line: i32, snapshot: &TypoSnapshot) -> i32 {
        get_para_start_line(line, snapshot)
    }
}

fn mem_type(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
    if pointer == 0 { -1 } else { snapshot.get__int(pointer) }
}

fn prev_column_page(column_page: i32, snapshot: &TypoSnapshot) -> i32 {
    let columns = TypoObjBase::getParent__int_typosnapshot(column_page, snapshot);
    let count = TypoColumns::getColumnsCount__int_typosnapshot(columns, snapshot);
    let mut i = 0;
    while i < count {
        if TypoColumns::getColumnPage__int_int_typosnapshot(i, columns, snapshot) == column_page {
            if i > 0 {
                return TypoColumns::getColumnPage__int_int_typosnapshot(i - 1, columns, snapshot);
            }
            break;
        }
        i += 1;
    }
    0
}

fn is_anchor_in_range(line: i32, anchor_cp: i32, range: &mut RANGE, snapshot: &TypoSnapshot) -> bool {
    let kind = snapshot.getTypoType(line);
    if kind == 5 {
        is_in_table_row(line, anchor_cp, Some(range), false, snapshot)
    } else if kind == 10 || kind == 13 {
        TypoRangeBase::contains__int_int_range_typosnapshot(anchor_cp, line, Some(range), snapshot)
    } else if kind == 3 {
        let start_cp = TypoTextLine::getStartCP__int_typosnapshot(line, snapshot);
        let end_cp = TypoTextLine::getEndCP__int_typosnapshot(line, snapshot);
        range.start = start_cp;
        range.end = end_cp;
        anchor_cp >= start_cp && anchor_cp < end_cp
    } else {
        false
    }
}

fn is_in_table_row(row: i32, cp: i32, range: Option<&mut RANGE>, find_inside: bool, snapshot: &TypoSnapshot) -> bool {
    let start = TypoRangeBase::getStartCP__int_typosnapshot(row, snapshot);
    let end = TypoRangeBase::getEndCP__int_typosnapshot(row, snapshot);
    if let Some(range) = range {
        range.start = start;
        range.end = end;
    }
    let is_in = cp >= start && cp < end;
    let wrap = BoolBitSet::new(0).get(snapshot.get__int(row + 10));
    if wrap {
        return is_in;
    }
    if !is_in && !find_inside {
        return false;
    }
    let size = snapshot.get__int(row + 12);
    let mut i = 0;
    while i < size {
        let mut p_cell = snapshot.get__int(row + 16 + i);
        i += 1;
        if p_cell == 0 {
            continue;
        }
        let flag = snapshot.get__int(p_cell + 11);
        let vert = BoolBitSet::new(6).get(flag);
        let is_first = BoolBitSet::new(7).get(flag);
        if vert {
            if !find_inside && !is_first {
                continue;
            }
            p_cell = first_vert_merge_cell(p_cell, snapshot);
        }
        if TypoRangeBase::contains__int_int_typosnapshot(cp, p_cell, snapshot) {
            return true;
        }
    }
    false
}

fn first_vert_merge_cell(cell: i32, snapshot: &TypoSnapshot) -> i32 {
    let mut first = cell;
    let flag = snapshot.get__int(cell + 11);
    if BoolBitSet::new(6).get(flag) && !BoolBitSet::new(7).get(flag) {
        loop {
            let pre = pre_vert_merge_cell(first, snapshot);
            if pre == 0 {
                break;
            }
            first = pre;
        }
    }
    first
}

fn pre_vert_merge_cell(cell: i32, snapshot: &TypoSnapshot) -> i32 {
    let flag = snapshot.get__int(cell + 11);
    if !(BoolBitSet::new(6).get(flag) && !BoolBitSet::new(7).get(flag)) {
        return 0;
    }
    let p_page = TypoObjBase::getPage__int_typosnapshot(cell, snapshot);
    if p_page == 0 {
        return 0;
    }
    let p_lines = snapshot.get__int(p_page + 17);
    let parent = snapshot.get__int(cell + 2);
    let mut row_index = list_index_of(parent, p_lines, snapshot);
    if row_index == -1 {
        return 0;
    }
    let rgdxa_left = snapshot.get__int(cell + 3);
    let rgdxa_right = snapshot.get__int(cell + 5);
    let mut ret = 0;
    let mut last_left = 0;
    let mut last_right = 0;
    while row_index > 0 {
        row_index -= 1;
        let p_row = list_get(row_index, p_lines, snapshot);
        if mem_type(p_row, snapshot) != 5 {
            break;
        }
        let size = snapshot.get__int(p_row + 12);
        let mut cell_index = 0;
        while cell_index < size {
            let temp_cell = snapshot.get__int(p_row + 16 + cell_index);
            cell_index += 1;
            let temp_flag = if temp_cell == 0 { 0 } else { snapshot.get__int(temp_cell + 11) };
            let temp_vert = BoolBitSet::new(6).get(temp_flag);
            let temp_first = BoolBitSet::new(7).get(temp_flag);
            if !temp_vert && !temp_first {
                if ret != 0 {
                    break;
                }
            } else if temp_cell != 0 {
                let left = snapshot.get__int(temp_cell + 3);
                let right = snapshot.get__int(temp_cell + 5);
                let left_diff = (left - rgdxa_left).abs();
                let right_diff = (right - rgdxa_right).abs();
                if left_diff < 6 && right_diff < 6 {
                    if ret == 0 || (left_diff < (last_left - rgdxa_left).abs() && right_diff <= (last_right - rgdxa_right).abs()) {
                        ret = temp_cell;
                        last_left = left;
                        last_right = right;
                    }
                } else if ret != 0 {
                    break;
                }
            }
        }
        if ret != 0 {
            break;
        }
    }
    ret
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

fn get_para_start_line(line: i32, snapshot: &TypoSnapshot) -> i32 {
    let line_type = mem_type(line, snapshot);
    if line_type == 5 || line_type == 10 {
        return line;
    }
    let p_lines_page = TypoObjBase::getPage__int_typosnapshot(line, snapshot);
    if mem_type(p_lines_page, snapshot) != 14 {
        let mut result = para_start_line_in_page(line, snapshot);
        if result == 0 {
            result = first_text_line_in_page(TypoObjBase::getPage__int_typosnapshot(line, snapshot), snapshot);
        }
        return result;
    }
    let mut result = 0;
    let mut start_line = 0;
    let columns = TypoObjBase::getParent__int_typosnapshot(p_lines_page, snapshot);
    let mut p_column_page = p_lines_page;
    while p_column_page != 0 {
        start_line = if start_line == 0 {
            line
        } else {
            let lines = TypoPage::getLines__int_typosnapshot(p_column_page, snapshot);
            let size = if lines == 0 { 0 } else { snapshot.get__int(lines + 2) };
            if size == 0 { 0 } else { snapshot.get__int(lines + 3 + size - 1) }
        };
        result = para_start_line_in_page(start_line, snapshot);
        if result != 0 {
            break;
        }
        p_column_page = prev_column_page(p_column_page, snapshot);
    }
    if result == 0 {
        let count = TypoColumns::getColumnsCount__int_typosnapshot(columns, snapshot);
        if count > 0 {
            let first_col = TypoColumns::getColumnPage__int_int_typosnapshot(0, columns, snapshot);
            result = first_text_line_in_page(first_col, snapshot);
        }
    }
    result
}

fn para_start_line_in_page(start_line: i32, snapshot: &TypoSnapshot) -> i32 {
    let p_lines_page = TypoObjBase::getPage__int_typosnapshot(start_line, snapshot);
    if p_lines_page == 0 {
        return 0;
    }
    let lines = TypoPage::getLines__int_typosnapshot(p_lines_page, snapshot);
    let mut index = list_index_of(start_line, lines, snapshot);
    while index > 0 {
        index -= 1;
        let p_line = list_get(index, lines, snapshot);
        if p_line != 0 && mem_type(p_line, snapshot) == 3 && TypoTextLine::isFirstLineOfPara__int_typosnapshot(p_line, snapshot) {
            return p_line;
        }
    }
    0
}

fn first_text_line_in_page(page: i32, snapshot: &TypoSnapshot) -> i32 {
    let lines = TypoPage::getLines__int_typosnapshot(page, snapshot);
    let size = if lines == 0 { 0 } else { snapshot.get__int(lines + 2) };
    let mut i = 0;
    while i < size {
        let p_line = snapshot.get__int(lines + 3 + i);
        if mem_type(p_line, snapshot) == 3 {
            return p_line;
        }
        i += 1;
    }
    0
}
