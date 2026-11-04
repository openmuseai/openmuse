#![allow(non_snake_case)]

// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.data.table.TableManagerUtil
use crate::cn::wps::moffice::writer::data::table::TableManagerUtil::KTableRow;
// Java: import cn.wps.moffice.writer.data.table.TableTool
use crate::cn::wps::moffice::writer::data::table::TableTool::RowCalc;
// Java: import cn.wps.moffice.writer.data.table.tree.CellInfoNode
use crate::cn::wps::moffice::writer::data::table::tree::CellInfoNode::CellInfoNode;
// Java: import cn.wps.moffice.writer.data.table.Data
use crate::cn::wps::moffice::writer::data::table::Data::RebuildCache;

// Java: public class RowInfoNode
#[derive(Clone)]
pub struct RowInfoNode {
    // Java field: int mLength
    mLength: i32,
    // Java field: int mGeneration
    mGeneration: i32,
    start_cp: i32,
    // Java field: int[] mRgdxaCenter
    mRgdxaCenter: Option<Vec<i32>>,
    // Java field: int[] mCellWidths
    mCellWidths: Option<Vec<i32>>,
    // Java field: int mWidthBefore
    mWidthBefore: i32,
    // Java field: int mRowHeight
    mRowHeight: i32,
    // Java field: boolean mCantSplit
    mCantSplit: bool,
    // Java: rowFinalProperty.getBoolean(320, false)
    mTableHeader: bool,
    // Java field: boolean mIsFormatChange
    mIsFormatChange: bool,
    // Java field: boolean mIsRevisionDelete
    mIsRevisionDelete: bool,
    // Java field: boolean mIsRevisionInsert
    mIsRevisionInsert: bool,
    cells: Vec<CellInfoNode>,
}

impl RowInfoNode {
    // Java: void init(ITableManager.IRowEntry rowEntry)
    pub fn init(
        row: &KTableRow,
        document: &dyn Document,
        table_level: i32,
        rebuild: *mut RebuildCache,
    ) -> Self {
        // Java: this.mLength = rowEntry.getEnd() - rowEntry.getStart();
        let mut this = RowInfoNode {
            mLength: row.getEnd() - row.getStart(),
            mGeneration: 0,
            start_cp: row.getStart(),
            mRgdxaCenter: None,
            mCellWidths: None,
            mWidthBefore: 0,
            mRowHeight: 0,
            mCantSplit: false,
            mTableHeader: false,
            mIsFormatChange: false,
            mIsRevisionDelete: false,
            mIsRevisionInsert: false,
            cells: Vec::new(),
        };
        let _ = this.start_cp;
        // Java: for (cellEntry = rowEntry.getFirstChildCell(); cellEntry.isValid(); cellEntry = cellEntry.getNext())
        for cell in row.cells() {
            // Java: CellInfoNode newCell = (CellInfoNode)this.createSubNode(); newCell.init(cellEntry);
            this.cells.push(CellInfoNode::init(cell, document, table_level, rebuild));
        }
        let _ = this.mLength;
        this
    }

    pub fn cells(&self) -> &[CellInfoNode] {
        &self.cells
    }

    pub fn cells_mut(&mut self) -> &mut [CellInfoNode] {
        &mut self.cells
    }

    pub fn getStartCP(&self) -> i32 {
        self.start_cp
    }

    pub fn getEndCP(&self) -> i32 {
        // Java: return this.getStartCP() + this.mLength;
        self.start_cp + self.mLength
    }

    pub fn contains_cp(&self, cp: i32) -> bool {
        self.getStartCP() <= cp && cp < self.getEndCP()
    }

    pub fn generation(&self) -> i32 {
        self.mGeneration
    }

    pub fn set_generation(&mut self, generation: i32) {
        self.mGeneration = generation;
    }

    pub fn has_widths(&self) -> bool {
        self.mCellWidths.is_some()
    }

    // Java: int getRgdxaCount()
    pub fn rgdxa_count(&self) -> i32 {
        self.mRgdxaCenter.as_ref().map(|v| v.len() as i32).unwrap_or(0)
    }

    // Java: int[] mRgdxaCenter
    pub fn rgdxa_center_mut(&mut self) -> Option<&mut [i32]> {
        self.mRgdxaCenter.as_deref_mut()
    }

    // Java: int getRgdxa(int index)
    pub fn getRgdxa(&self, index: i32) -> i32 {
        self.mRgdxaCenter
            .as_ref()
            .and_then(|v| v.get(index as usize).copied())
            .unwrap_or(0)
    }

    // Java: CellInfo getCellByCP — index is the rgdxa slot of that cell
    pub fn getCellIndexByCP(&self, cp: i32) -> Option<i32> {
        self.cells.iter().position(|cell| cell.contains_cp(cp)).map(|i| i as i32)
    }

    // Java: Data.RowData.getRowHeight
    pub fn getRowHeight(&self) -> i32 {
        self.mRowHeight
    }

    // Java: RowInfo.cantSplit()
    pub fn cantSplit(&self) -> bool { self.mCantSplit }

    // Java: RowInfo.isRevisionDelete()
    pub fn isRevisionDelete(&self) -> bool { self.mIsRevisionDelete }

    // Java: RowInfo.isRevisionInsert()
    pub fn isRevisionInsert(&self) -> bool { self.mIsRevisionInsert }

    // Java: RowInfo.isFormatChange()
    pub fn isFormatChange(&self) -> bool { self.mIsFormatChange }

    // Java: rowFinalProperty.getBoolean(320, false)
    pub fn isTableHeader(&self) -> bool { self.mTableHeader }

    // Java: RowInfo.size()
    pub fn size(&self) -> i32 { self.cells.len() as i32 }

    pub fn rgdxa(&self, index: i32) -> i32 {
        if index < 0 {
            return 0;
        }
        self.mRgdxaCenter
            .as_ref()
            .and_then(|v| v.get(index as usize).copied())
            .unwrap_or(0)
    }

    pub fn width_before(&self) -> i32 {
        self.mWidthBefore
    }

    pub fn cell_width(&self, index: usize) -> i32 {
        self.mCellWidths
            .as_ref()
            .and_then(|v| v.get(index).copied())
            .unwrap_or(0)
    }

    // Java: data.mCellWidths = null
    pub fn clear_cell_widths(&mut self) {
        self.mCellWidths = None;
        self.mGeneration = 0;
    }

    // Java: data.mCellWidths = TableTool.calcCellWidths(...)
    pub fn set_cell_widths(&mut self, widths: Option<Vec<i32>>, generation: i32) {
        self.mCellWidths = widths.or_else(|| Some(Vec::new()));
        self.mGeneration = generation;
    }

    // Java: calcRowStyle writes RowData then each CellData
    pub fn apply_row_calc(&mut self, calc: RowCalc, generation: i32) {
        // Java: rgdxaCount > 0 assigns mRgdxaCenter; otherwise it stays null
        self.mRgdxaCenter = if calc.rgdxa.is_empty() { None } else { Some(calc.rgdxa) };
        self.mWidthBefore = calc.width_before;
        self.mRowHeight = calc.row_height;
        self.mCantSplit = calc.cant_split;
        self.mTableHeader = calc.table_header;
        self.mIsFormatChange = calc.format_change;
        self.mIsRevisionDelete = calc.revision_delete;
        self.mIsRevisionInsert = calc.revision_insert;
        let _ = generation;
        for (cell, paint) in self.cells.iter_mut().zip(calc.cells) {
            cell.apply_cell_calc(paint);
        }
    }

    // Java: CellInfo getCellByCP(int cp)
    pub fn getCellByCP(&self, cp: i32) -> Option<&CellInfoNode> {
        // Java: return (CellInfo)this.getNodeByCP(cp);
        self.cells.iter().find(|cell| cell.contains_cp(cp))
    }

    // Java: CellInfo getCellByIndex(int index)
    pub fn getCellByIndex(&self, index: i32) -> Option<&CellInfoNode> {
        if index < 0 { None } else { self.cells.get(index as usize) }
    }

    pub fn indexOfCP(&self, cp: i32) -> Option<i32> {
        self.cells.iter().position(|cell| cell.contains_cp(cp)).map(|i| i as i32)
    }

    pub fn shift_cp(&mut self, delta: i32) {
        if delta == 0 {
            return;
        }
        self.start_cp += delta;
        for cell in &mut self.cells {
            cell.shift_cp(delta);
        }
    }

    // Java: void positionFix(int offsetFromThis, int steps, int[] offsetBuf)
    pub fn positionFix(&mut self, offset_from_this: i32, steps: i32, marks: &mut Vec<i32>) {
        // Java: this.mLength += steps;
        self.mLength += steps;
        position_fix_cells(&mut self.cells, self.start_cp, offset_from_this, steps, marks);
    }

    // Java: boolean removeNodeInRange(int start, int end, int[] offsetBuf)
    pub fn removeNodeInRange(&mut self, start: i32, end: i32, rebuild: *mut RebuildCache) -> bool {
        remove_cells(&mut self.cells, self.start_cp, start, end, rebuild)
    }

    // Java: protected void close()
    pub fn close(&mut self, cache: &mut RebuildCache) {
        // Java: for (subNode = leftmost; !subNode.isHead(); subNode = subNode.getNext()) subNode.close();
        for cell in &mut self.cells {
            cell.close(cache);
        }
    }
}

fn position_fix_cells(
    cells: &mut [CellInfoNode],
    parent_start: i32,
    offset_from_this: i32,
    steps: i32,
    marks: &mut Vec<i32>,
) {
    if cells.is_empty() {
        return;
    }
    let edit = parent_start + offset_from_this;
    let prev_i = cells.iter().rposition(|cell| cell.getStartCP() <= edit);
    if steps > 0 {
        if let Some(i) = prev_i {
            let within = edit - cells[i].getStartCP();
            let len = cells[i].getEndCP() - cells[i].getStartCP();
            if within < len {
                cells[i].positionFix(within, steps, marks);
            }
        }
    } else {
        let deleted = -steps;
        if let Some(i) = prev_i {
            let within = edit - cells[i].getStartCP();
            let len = cells[i].getEndCP() - cells[i].getStartCP();
            if within < len {
                let sub_deleted = deleted.min(len - within);
                cells[i].positionFix(within, -sub_deleted, marks);
            }
        }
        if let Some(upper_i) = cells.iter().position(|cell| cell.getStartCP() > edit) {
            let upper_off = cells[upper_i].getStartCP() - parent_start;
            let upper_deleted = offset_from_this + deleted - upper_off;
            if upper_deleted > 0 {
                cells[upper_i].shift_cp(upper_deleted);
                cells[upper_i].positionFix(0, -upper_deleted, marks);
            }
        }
    }
    for cell in cells.iter_mut() {
        if cell.getStartCP() > edit {
            cell.shift_cp(steps);
        }
    }
}

fn remove_cells(
    cells: &mut Vec<CellInfoNode>,
    parent_start: i32,
    start: i32,
    end: i32,
    rebuild: *mut RebuildCache,
) -> bool {
    if cells.is_empty() || end <= start {
        return false;
    }
    let range_lo = parent_start + start;
    let range_hi = parent_start + end;
    let mut changed = false;
    let mut i = 0;
    while i < cells.len() {
        let lo = cells[i].getStartCP();
        let hi = cells[i].getEndCP();
        if hi <= range_lo || lo >= range_hi {
            i += 1;
            continue;
        }
        if lo >= range_lo && hi <= range_hi {
            // Java: eraseSubNode → close() then erase
            let mut cell = cells.remove(i);
            if !rebuild.is_null() {
                cell.close(unsafe { &mut *rebuild });
            }
            changed = true;
            continue;
        }
        let sub_start = (range_lo - lo).max(0);
        let sub_end = (range_hi - lo).min(hi - lo);
        changed |= cells[i].removeNodeInRange(sub_start, sub_end, rebuild);
        i += 1;
    }
    changed
}
