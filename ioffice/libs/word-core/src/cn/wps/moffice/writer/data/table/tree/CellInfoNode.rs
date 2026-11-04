#![allow(non_snake_case)]

// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.data.table.KTableManager
use crate::cn::wps::moffice::writer::data::table::KTableManager::KTableManager;
// Java: import cn.wps.moffice.writer.data.table.TableManagerUtil
use crate::cn::wps::moffice::writer::data::table::TableManagerUtil::KTableCell;
// Java: import cn.wps.moffice.drawing.graphics.SHD
use drawing::cn::wps::moffice::drawing::graphics::SHD::SHD;
// Java: import cn.wps.moffice.writer.data.table.TableTool
use crate::cn::wps::moffice::writer::data::table::TableTool::CellCalc;
// Java: import cn.wps.moffice.writer.data.table.tree.TableInfoNode
use crate::cn::wps::moffice::writer::data::table::tree::TableInfoNode::TableInfoNode;
// Java: import cn.wps.moffice.writer.data.value.BRC
use crate::cn::wps::moffice::writer::data::value::BRC::BRC;
// Java: import cn.wps.moffice.writer.data.value.TableCell
use crate::cn::wps::moffice::writer::data::value::TableCell::TableCell;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory;
// Java: import cn.wps.moffice.writer.data.table.Data
use crate::cn::wps::moffice::writer::data::table::Data::{AutoFitInfo, RebuildCache};

// Java: public class CellInfoNode
#[derive(Clone)]
pub struct CellInfoNode {
    // Java field: int mLength
    mLength: i32,
    // Java field: int mGeneration
    mGeneration: i32,
    // Java field: TableCell width type == 1
    mAutoWidth: bool,
    // Java field: TableCell mTableCell
    mTableCell: Option<TableCell>,
    // Java field: SHD mShd
    mShd: Option<SHD>,
    // Java field: BRC mBrcTop
    mBrcTop: Option<BRC>,
    // Java field: BRC mBrcLeft
    mBrcLeft: Option<BRC>,
    // Java field: BRC mBrcBottom
    mBrcBottom: Option<BRC>,
    // Java field: BRC mBrcRight
    mBrcRight: Option<BRC>,
    // Java field: BRC mBrcFromTopLeftToBottomRight
    mBrcFromTopLeftToBottomRight: Option<BRC>,
    // Java field: BRC mBrcFromTopRightToBottomLeft
    mBrcFromTopRightToBottomLeft: Option<BRC>,
    // Java field: int mMarginLeft
    mMarginLeft: i32,
    // Java field: int mMarginTop
    mMarginTop: i32,
    // Java field: int mMarginRight
    mMarginRight: i32,
    // Java field: int mMarginBottom
    mMarginBottom: i32,
    // Java field: Data.CellData.mAutoFitInfo
    mAutoFitInfo: Option<AutoFitInfo>,
    // Java: getLevel() == parent table level
    mLevel: i32,
    start_cp: i32,
    tables: Vec<TableInfoNode>,
}

impl CellInfoNode {
    // Java: void init(ITableManager.ICellEntry cellEntry)
    pub fn init(
        cell: &KTableCell,
        document: &dyn Document,
        table_level: i32,
        rebuild: *mut RebuildCache,
    ) -> Self {
        // Java: this.mLength = cellEntry.getEnd() - cellEntry.getStart();
        let mut this = CellInfoNode {
            mLength: cell.getEnd() - cell.getStart(),
            mGeneration: 0,
            mAutoWidth: false,
            mTableCell: None,
            mShd: None,
            mBrcTop: None,
            mBrcLeft: None,
            mBrcBottom: None,
            mBrcRight: None,
            mBrcFromTopLeftToBottomRight: None,
            mBrcFromTopRightToBottomLeft: None,
            mMarginLeft: 0,
            mMarginTop: 0,
            mMarginRight: 0,
            mMarginBottom: 0,
            mAutoFitInfo: None,
            mLevel: table_level,
            start_cp: cell.getStart(),
            tables: Vec::new(),
        };
        // Java: this.tryRestoreData(cellEntry.getStart());
        if !rebuild.is_null() {
            this.tryRestoreData(unsafe { &mut *rebuild }, cell.getStart());
        }
        // Java: IRowEntry rowEntry = cellEntry.getFirstChildRow();
        let nested = cell.rows();
        // Java: if (rowEntry.isValid())
        if !nested.is_empty() {
            // Java: int subLevel = this.mParent.getParent().getLevel() + 1;
            let sub_level = table_level + 1;
            let mut index = 0usize;
            // Java: do { newTable.init(rowEntry, subLevel, false, 0); rowEntry = rowEntry.getNext(); } while (rowEntry.isValid());
            while index < nested.len() {
                let mut new_table = TableInfoNode::new();
                let used = new_table.init(&nested[index..], document, sub_level, false, 0, rebuild);
                if used == 0 {
                    break;
                }
                index += used;
                this.tables.push(new_table);
            }
        }
        let _ = this.mLength;
        this
    }

    // Java: public void tryRestoreData(int startCp)
    fn tryRestoreData(&mut self, cache: &mut RebuildCache, start_cp: i32) {
        // Java: if (this.mCache.getRebuildCache().isRebuilding())
        if !cache.isRebuilding() {
            return;
        }
        // Java: Data.AutoFitInfo info = this.mCache.getRebuildCache().get(this.getLevel(), startCp, this.mLength);
        let info = cache.get(self.mLevel, start_cp, self.mLength).cloned();
        if let Some(info) = info {
            // Java: this.getData().mAutoFitInfo = info;
            self.mAutoFitInfo = Some(info);
        } else {
            // Java: this.mCache.getRebuildCache().addInvalidRange(RANGEFactory.getRANGE(startCp, startCp + this.mLength));
            cache.addInvalidRange(RANGEFactory::getRANGE(start_cp, start_cp + self.mLength));
        }
    }

    // Java: Data.CellData.getAutoFitInfo()
    pub fn getAutoFitInfo(&mut self) -> &mut AutoFitInfo {
        // Java: if (null == this.mAutoFitInfo) this.mAutoFitInfo = new AutoFitInfo();
        if self.mAutoFitInfo.is_none() {
            self.mAutoFitInfo = Some(AutoFitInfo::default());
        }
        // Java: return this.mAutoFitInfo;
        self.mAutoFitInfo.as_mut().unwrap()
    }

    // Java: protected void close()
    pub fn close(&mut self, cache: &mut RebuildCache) {
        // Java: if (this.mData != null && this.mData.mAutoFitInfo != null)
        if let Some(info) = self.mAutoFitInfo.take() {
            // Java: this.mCache.getRebuildCache().backup(this.getLevel(), this.getStartCP(), this.mLength, this.mData.mAutoFitInfo);
            cache.backup(self.mLevel, self.getStartCP(), self.mLength, info);
        }
        // Java: for (subNode = leftmost; !subNode.isHead(); subNode = subNode.getNext()) subNode.close();
        for table in &mut self.tables {
            table.close(cache);
        }
    }

    pub fn tables(&self) -> &[TableInfoNode] {
        &self.tables
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

    // Java: TableInfo getTableByCP(int cp)
    pub fn getTableByCP(&self, cp: i32) -> Option<&TableInfoNode> {
        // Java: return (TableInfo)this.getNodeByCP(cp);
        self.tables.iter().find(|table| table.contains_cp(cp))
    }

    pub fn tables_mut(&mut self) -> &mut [TableInfoNode] {
        &mut self.tables
    }

    pub fn set_generation(&mut self, generation: i32) {
        self.mGeneration = generation;
    }

    // Java: boolean isAutoWidth() — TableCell.getWidthType() == 1
    pub fn set_auto(&mut self, auto: bool) {
        self.mAutoWidth = auto;
    }

    // Java: cell data written by calcRowStyle / calcRowInfo
    pub fn apply_cell_calc(&mut self, cell: CellCalc) {
        self.mAutoWidth = cell.auto;
        self.mTableCell = cell.table_cell;
        self.mShd = cell.shd;
        self.mBrcTop = cell.brc_top;
        self.mBrcLeft = cell.brc_left;
        self.mBrcBottom = cell.brc_bottom;
        self.mBrcRight = cell.brc_right;
        self.mBrcFromTopLeftToBottomRight = cell.brc_diag_down;
        self.mBrcFromTopRightToBottomLeft = cell.brc_diag_up;
        self.mMarginLeft = cell.margin_left;
        self.mMarginTop = cell.margin_top;
        self.mMarginRight = cell.margin_right;
        self.mMarginBottom = cell.margin_bottom;
    }

    pub fn is_auto(&self) -> bool {
        self.mAutoWidth
    }

    // Java: Data.CellData.isVertMergeCell — links come from TableCell.vertMerge 1 (continue) or 3 (restart).
    pub fn isVertMergeCell(&self) -> bool {
        matches!(self.vert_merge(), 1 | 3)
    }

    // Java: Data.CellData.isFirstVertMergeCell — restart cell, prev link is null.
    pub fn isFirstVertMergeCell(&self) -> bool {
        self.vert_merge() == 3
    }

    // Java: Data.CellData.getHoriMerge — 2 is a horizontal continuation cell
    pub fn getHoriMerge(&self) -> i32 {
        self.mTableCell.as_ref().map(|cell| cell.getHoriMerge()).unwrap_or(0)
    }

    // Java: Data.CellData.getVertAlign
    pub fn getVertAlign(&self) -> i32 {
        self.mTableCell.as_ref().map(|cell| cell.getVertAlign()).unwrap_or(0)
    }

    // Java: Data.CellData.getFitText — null table cell returns true
    pub fn getFitText(&self) -> bool {
        self.mTableCell.as_ref().map(|cell| cell.getFitText()).unwrap_or(true)
    }

    // Java: TableCell.getVertMerge — raw code, not the link-adjusted Data.getVertMerge
    pub fn getVertMerge(&self) -> i32 {
        self.vert_merge()
    }

    fn vert_merge(&self) -> i32 {
        self.mTableCell.as_ref().map(|cell| cell.getVertMerge()).unwrap_or(0)
    }

    // Java: Data.CellData.getTextFlow
    pub fn getTextFlow(&self) -> i32 {
        self.mTableCell.as_ref().map(|cell| cell.getTextFlow()).unwrap_or(0)
    }

    // Java: Data.CellData.isAutoWidth()
    pub fn isAutoWidth(&self) -> bool {
        // Java: return null != this.mTableCell && this.mTableCell.getWidthType() == 1;
        self.mTableCell
            .as_ref()
            .is_some_and(|cell| cell.getWidthType() == 1)
    }

    // Java: Data.CellData.getMinWidth()
    pub fn getMinWidth(&mut self) -> i32 {
        // Java: return this.mMarginLeft + this.mMarginRight + this.getAutoFitInfo().getMinWidth();
        let fit = self.getAutoFitInfo().getMinWidth();
        self.mMarginLeft + self.mMarginRight + fit
    }

    // Java: Data.CellData.getMaxWidth()
    pub fn getMaxWidth(&mut self) -> i32 {
        // Java: return this.mMarginLeft + this.mMarginRight + this.getAutoFitInfo().getMaxWidth();
        let fit = self.getAutoFitInfo().getMaxWidth();
        self.mMarginLeft + self.mMarginRight + fit
    }

    // Java: Data.CellData.getParaTotalMax()
    pub fn getParaTotalMax(&mut self) -> i32 {
        // Java: int paraTotal = this.getAutoFitInfo().getParaTotalMax();
        let para_total = self.getAutoFitInfo().getParaTotalMax();
        // Java: return paraTotal == 0 ? 0 : this.mMarginLeft + this.mMarginRight + paraTotal;
        if para_total == 0 {
            0
        } else {
            self.mMarginLeft + self.mMarginRight + para_total
        }
    }

    // Java: if (autoFitInfo.mChildBestFitInfo != null) autoFitInfo.mChildBestFitInfo.clear();
    pub fn clear_child_fit(&mut self) {
        let info = self.getAutoFitInfo();
        if let Some(child) = info.mChildBestFitInfo.as_mut() {
            child.clear();
        }
    }

    // Java: BRC getBrcTop()
    pub fn getBrcTop(&self) -> Option<&BRC> { self.mBrcTop.as_ref() }
    pub fn getBrcLeft(&self) -> Option<&BRC> { self.mBrcLeft.as_ref() }
    pub fn getBrcBottom(&self) -> Option<&BRC> { self.mBrcBottom.as_ref() }
    pub fn getBrcRight(&self) -> Option<&BRC> { self.mBrcRight.as_ref() }
    pub fn getBrcFromTopLeftToBottomRight(&self) -> Option<&BRC> { self.mBrcFromTopLeftToBottomRight.as_ref() }
    pub fn getBrcFromTopRightToBottomLeft(&self) -> Option<&BRC> { self.mBrcFromTopRightToBottomLeft.as_ref() }

    // Java: SHD getSHD()
    pub fn getSHD(&self) -> Option<&SHD> { self.mShd.as_ref() }

    // Java: getData().getMarginLeft()
    pub fn margin_left(&self) -> i32 {
        self.mMarginLeft
    }

    // Java: getData().getMarginRight()
    pub fn margin_right(&self) -> i32 {
        self.mMarginRight
    }

    // Java: getData().getMarginTop()
    pub fn margin_top(&self) -> i32 {
        self.mMarginTop
    }

    // Java: getData().getMarginBottom()
    pub fn margin_bottom(&self) -> i32 {
        self.mMarginBottom
    }

    pub fn shift_cp(&mut self, delta: i32) {
        if delta == 0 {
            return;
        }
        self.start_cp += delta;
        for table in &mut self.tables {
            table.shift_cp(delta);
        }
    }

    // Java: void positionFix(int offsetFromThis, int steps, int[] offsetBuf)
    pub fn positionFix(&mut self, offset_from_this: i32, steps: i32, marks: &mut Vec<i32>) {
        // Java: super.positionFix(offsetFromThis, steps, offsetBuf);
        self.mLength += steps;
        position_fix_tables(
            &mut self.tables,
            self.start_cp,
            offset_from_this,
            steps,
            marks,
        );
        // Java: if (steps < 0 && nodeCount >= 2 && next starts at the point and prev ends there)
        if steps < 0 && self.tables.len() >= 2 {
            let edit = self.start_cp + offset_from_this;
            if let Some(i) = self.tables.iter().position(|table| table.getStartCP() == edit) {
                if i > 0 && self.tables[i - 1].getEndCP() == edit {
                    // Java: this.mCache.markModify(this.getStartCP() + offsetFromThis - 1);
                    marks.push(edit - 1);
                }
            }
        }
    }

    // Java: boolean removeNodeInRange(int start, int end, int[] offsetBuf)
    pub fn removeNodeInRange(&mut self, start: i32, end: i32, rebuild: *mut RebuildCache) -> bool {
        remove_tables(&mut self.tables, self.start_cp, start, end, rebuild)
    }

    // Java: void valid(int start, int end)
    pub fn valid(
        &mut self,
        document: &dyn Document,
        start: i32,
        end: i32,
        level: i32,
        rebuild: *mut RebuildCache,
    ) {
        self.truncate_tables(start, end);
        let position = self.start_cp;
        let mut start_cp = position + start;
        let end_cp = position + end;
        let manager = KTableManager::create(document);
        while start_cp < end_cp {
            // Java: startCP = tableManager.findNextRowStart(startCP, endCP, level);
            start_cp = manager.findNextRowStart(start_cp, end_cp, level);
            if start_cp >= end_cp {
                break;
            }
            // Java: IRowEntry rowEntry = tableManager.createRow(startCP, level);
            let row = manager.createRow(start_cp, level, false);
            if !row.isComplete() {
                break;
            }
            let mut table = TableInfoNode::new();
            // Java: rowEntry = newTable.init(rowEntry, level, true, endCP);
            let used = table.init(std::slice::from_ref(&row), document, level, true, end_cp, rebuild);
            if used == 0 {
                break;
            }
            let next_cp = table.getEndCP();
            let at = table.getStartCP();
            let idx = self
                .tables
                .iter()
                .position(|nested| nested.getStartCP() >= at)
                .unwrap_or(self.tables.len());
            self.tables.insert(idx, table);
            if next_cp <= start_cp {
                break;
            }
            start_cp = next_cp;
        }
    }

    fn truncate_tables(&mut self, start: i32, end: i32) {
        let parent = self.start_cp;
        let start_abs = parent + start;
        let end_abs = parent + end;
        for table in &mut self.tables {
            if table.getStartCP() < start_abs && table.getEndCP() > start_abs {
                // Java: startTable.mLength = offsetBuf[0];
                table.set_length(start_abs - table.getStartCP());
            }
            if table.getStartCP() < end_abs && table.getEndCP() > end_abs {
                let offset = end_abs - table.getStartCP();
                let len = table.getEndCP() - table.getStartCP();
                // Java: endTable.mLength -= spaceBefore;
                if offset > 0 && offset < len {
                    table.set_length(len - offset);
                }
            }
        }
    }
}

fn position_fix_tables(
    tables: &mut [TableInfoNode],
    parent_start: i32,
    offset_from_this: i32,
    steps: i32,
    marks: &mut Vec<i32>,
) {
    if tables.is_empty() {
        return;
    }
    let edit = parent_start + offset_from_this;
    let prev_i = tables.iter().rposition(|table| table.getStartCP() <= edit);
    if steps > 0 {
        if let Some(i) = prev_i {
            let within = edit - tables[i].getStartCP();
            let len = tables[i].getEndCP() - tables[i].getStartCP();
            if within < len {
                tables[i].positionFix(within, steps, marks);
            }
        }
    } else {
        let deleted = -steps;
        if let Some(i) = prev_i {
            let within = edit - tables[i].getStartCP();
            let len = tables[i].getEndCP() - tables[i].getStartCP();
            if within < len {
                let sub_deleted = deleted.min(len - within);
                tables[i].positionFix(within, -sub_deleted, marks);
            }
        }
        if let Some(upper_i) = tables.iter().position(|table| table.getStartCP() > edit) {
            let upper_off = tables[upper_i].getStartCP() - parent_start;
            let upper_deleted = offset_from_this + deleted - upper_off;
            if upper_deleted > 0 {
                tables[upper_i].shift_cp(upper_deleted);
                tables[upper_i].positionFix(0, -upper_deleted, marks);
            }
        }
    }
    for table in tables.iter_mut() {
        if table.getStartCP() > edit {
            table.shift_cp(steps);
        }
    }
}

fn remove_tables(
    tables: &mut Vec<TableInfoNode>,
    parent_start: i32,
    start: i32,
    end: i32,
    rebuild: *mut RebuildCache,
) -> bool {
    if tables.is_empty() || end <= start {
        return false;
    }
    let range_lo = parent_start + start;
    let range_hi = parent_start + end;
    let mut changed = false;
    let mut i = 0;
    while i < tables.len() {
        let lo = tables[i].getStartCP();
        let hi = tables[i].getEndCP();
        if hi <= range_lo || lo >= range_hi {
            i += 1;
            continue;
        }
        if lo >= range_lo && hi <= range_hi {
            // Java: eraseSubNode → close() then erase
            let mut table = tables.remove(i);
            if !rebuild.is_null() {
                table.close(unsafe { &mut *rebuild });
            }
            changed = true;
            continue;
        }
        let sub_start = (range_lo - lo).max(0);
        let sub_end = (range_hi - lo).min(hi - lo);
        changed |= tables[i].removeNodeInRange(sub_start, sub_end, rebuild);
        i += 1;
    }
    changed
}
