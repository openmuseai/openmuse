#![allow(non_snake_case)]

// Java: import cn.wps.moffice.drawing.graphics.SHD
use drawing::cn::wps::moffice::drawing::graphics::SHD::SHD;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.data.table.TableManagerUtil
use crate::cn::wps::moffice::writer::data::table::TableManagerUtil::KTableRow;
// Java: import cn.wps.moffice.writer.data.table.KTableManager
use crate::cn::wps::moffice::writer::data::table::KTableManager::KTableManager;
// Java: import cn.wps.moffice.writer.data.table.TablePos
use crate::cn::wps::moffice::writer::data::table::TablePos::TablePos;
// Java: import cn.wps.moffice.writer.data.table.TableTool
use crate::cn::wps::moffice::writer::data::table::TableTool::{ParentCellFit, RowJcInput, TableTool};
// Java: import cn.wps.moffice.writer.data.table.TableJCData
use crate::cn::wps::moffice::writer::data::table::TableJCData::TableJCData;
// Java: import cn.wps.moffice.writer.data.table.grid.GridColsImpl
use crate::cn::wps::moffice::writer::data::table::grid::GridColsImpl::GridColsImpl;
// Java: import cn.wps.moffice.writer.data.value.TableBorders
use crate::cn::wps::moffice::writer::data::value::TableBorders::TableBorders;
// Java: import cn.wps.moffice.writer.data.value.TableLook
use crate::cn::wps::moffice::writer::data::value::TableLook::TableLook;
// Java: import cn.wps.moffice.writer.data.table.tree.RowInfoNode
use crate::cn::wps::moffice::writer::data::table::tree::RowInfoNode::RowInfoNode;
// Java: import cn.wps.moffice.writer.data.table.tree.CellInfoNode
use crate::cn::wps::moffice::writer::data::table::tree::CellInfoNode::CellInfoNode;
// Java: import cn.wps.moffice.writer.data.table.Data
use crate::cn::wps::moffice::writer::data::table::Data::RebuildCache;

// Java: public class TableInfoNode
#[derive(Clone)]
pub struct TableInfoNode {
    // Java field: int mLevel
    mLevel: i32,
    // Java field: int mLength
    mLength: i32,
    // Java field: boolean mIsComplete = true
    mIsComplete: bool,
    // Java field: boolean mValid
    mValid: bool,
    // Java field: Data.TableData.mPos
    mPos: Option<TablePos>,
    // Java field: Data.TableData.mShd
    mShd: Option<SHD>,
    // Java field: Data.TableData.mStyleId = 4095
    mStyleId: i32,
    // Java field: Data.TableData.mTableLook
    mTableLook: Option<TableLook>,
    // Java field: Data.TableData.mTableBorders
    mTableBorders: Option<TableBorders>,
    // Java field: Data.TableData.mIsRTL
    mIsRTL: bool,
    // Java field: Data.TableData.mWaitFitUpdate
    mWaitFitUpdate: bool,
    // Java field: Data.TableData.mTableJCData
    mTableJCData: Option<TableJCData>,
    // Java: getParentCell() rgdxa and margins, filled when the parent row geometry is known
    parent_fit: Option<ParentCellFit>,
    // Java field: int mGeneration = 1
    mGeneration: i32,
    // Java field: Document, used by getGrid / calcRowInfo
    mDocument: Option<*const (dyn Document + 'static)>,
    // Java field: Data.TableData.mGridCols
    mGridCols: Option<GridColsImpl>,
    start_cp: i32,
    row_count: i32,
    last_row_start: i32,
    rows: Vec<RowInfoNode>,
}

/// Java cell rectangle used by `TableRowLayouter.newCell` and border stroke.
#[derive(Clone, Copy)]
pub struct CellLayoutBox {
    pub text_dx: i32,
    pub border_left: i32,
    pub border_right: i32,
    pub content_width: i32,
    pub row_start: i32,
    pub row_height: i32,
    pub first_row: bool,
    pub margin_top: i32,
    pub margin_bottom: i32,
    pub top_w: i32,
    pub left_w: i32,
    pub bottom_w: i32,
    pub right_w: i32,
    pub color: u32,
}

fn brc_twip(brc: Option<&crate::cn::wps::moffice::writer::data::value::BRC::BRC>) -> i32 {
    // Java: UnitUtil.PT2TWIPS(BRCBorderTool.getActualStrokeWidth(type, width))
    let Some(brc) = brc else { return 0 };
    let pt = crate::cn::wps::moffice::writer::layout::base::tools::BRCBorderTool::BRCBorderTool::getActualStrokeWidth(
        brc.getBorderType(),
        brc.getLineWidth(),
    );
    if pt <= 0.0 { 0 } else { (pt * 20.0 + 0.5) as i32 }
}

impl TableInfoNode {
    pub fn new() -> Self {
        // Java: super(true); this.mGeneration = 1; this.mIsComplete = true;
        TableInfoNode {
            mLevel: 0,
            mLength: 0,
            mIsComplete: true,
            mValid: false,
            mPos: None,
            mShd: None,
            mStyleId: 4095,
            mTableLook: None,
            mTableBorders: None,
            mIsRTL: false,
            mWaitFitUpdate: false,
            mTableJCData: None,
            parent_fit: None,
            mGeneration: 1,
            mDocument: None,
            mGridCols: None,
            start_cp: 0,
            row_count: 0,
            last_row_start: 0,
            rows: Vec::new(),
        }
    }

    pub fn setComplete(&mut self, complete: bool) {
        // Java: setComplete
        self.mIsComplete = complete;
    }

    pub fn getStartCP(&self) -> i32 {
        // Java: return offset walked to this node;
        self.start_cp
    }

    pub fn getEndCP(&self) -> i32 {
        // Java: return this.getStartCP() + this.mLength;
        self.start_cp + self.mLength
    }

    pub fn isComplete(&self) -> bool {
        self.mIsComplete
    }

    // Java: TableRowLayouter places a cell at rgdxa and strokes BRC via BRCBorderTool.
    pub fn cell_layout_box(&mut self, cp: i32) -> Option<CellLayoutBox> {
        let row_index = self.rows.iter().position(|row| row.contains_cp(cp))?;
        self.ensure_row_metrics(row_index);
        let row = self.rows.get(row_index)?;
        let cell_index = row.cells().iter().position(|cell| cell.contains_cp(cp))?;
        let border_left = if row.rgdxa_count() > cell_index as i32 {
            row.rgdxa(cell_index as i32)
        } else {
            (0..cell_index).map(|index| row.cell_width(index)).sum()
        };
        let cell_width = row.cell_width(cell_index);
        let border_right = if row.rgdxa_count() > cell_index as i32 + 1 {
            row.rgdxa(cell_index as i32 + 1)
        } else {
            border_left + cell_width
        };
        let cell = row.cells().get(cell_index)?;
        let margin_left = cell.margin_left();
        let margin_right = cell.margin_right();
        let content_width = (border_right - border_left - margin_left - margin_right).max(0);
        Some(CellLayoutBox {
            text_dx: border_left + margin_left,
            border_left,
            border_right,
            content_width,
            row_start: row.getStartCP(),
            row_height: row.getRowHeight(),
            first_row: row_index == 0,
            margin_top: cell.margin_top(),
            margin_bottom: cell.margin_bottom(),
            top_w: brc_twip(cell.getBrcTop()),
            left_w: brc_twip(cell.getBrcLeft()),
            bottom_w: brc_twip(cell.getBrcBottom()),
            right_w: brc_twip(cell.getBrcRight()),
            color: cell.getBrcLeft().or(cell.getBrcTop()).map(|brc| brc.getColor() as u32).unwrap_or(0),
        })
    }

    // Java: RowInfo getRowByCP(int cp)
    pub fn getRowByCP(&self, cp: i32) -> Option<&RowInfoNode> {
        // Java: return (RowInfo)this.getNodeByCP(cp);
        self.rows.iter().find(|row| row.contains_cp(cp))
    }

    pub fn contains_cp(&self, cp: i32) -> bool {
        self.getStartCP() <= cp && cp < self.getEndCP()
    }

    pub fn row_count(&self) -> i32 {
        self.row_count
    }

    pub fn getLevel(&self) -> i32 {
        // Java: return this.mLevel;
        self.mLevel
    }

    // Java: public boolean isTableFit()
    // Java: TableInfo rootTable = this.mCache.getRootTable(this.getStartCP());
    // Java: return rootTable.getTableProperty().getBoolean(302, DefaultValue.table_fit);
    // rust: `mCache` 是 `TreeTableCache`。根表查找在 `TreeTableCache::isTableFit`，
    //   这里是根表 `getTableProperty().getBoolean(302, DefaultValue.table_fit)`。
    pub fn isTableFit(&self, document: &dyn Document) -> bool {
        let Some(row) = self.rows().first() else {
            return false;
        };
        let prop = TableTool::getFinalRowProperty(document, row.getEndCP() - 1);
        prop.getBoolean__int_boolean(
            302,
            crate::cn::wps::moffice::writer::data::DefaultValue::table_fit,
        )
    }

    pub fn rows(&self) -> &[RowInfoNode] {
        &self.rows
    }

    // Java: public TableBorders getTableBorders() { this.valid(); return this.getData().getTableBorders(); }
    pub fn getTableBorders(&mut self) -> Option<&TableBorders> {
        self.valid();
        self.mTableBorders.as_ref()
    }

    // Java: Tools.createCellEnvForCell vert/first/last flags.
    // Links are TableTool.getNextMergeCell / getPrevMergeCell: the neighbor row's cell
    // whose rgdxa edges are within 6 twips. Next must be vertMerge 1; prev may be 1 or 3.
    pub fn cell_env_flags(&self, row_index: usize, cell_index: usize) -> (bool, bool, bool) {
        let Some(row) = self.rows.get(row_index) else { return (false, false, false); };
        let Some(cell) = row.cells().get(cell_index) else { return (false, false, false); };
        if cell.getVertMerge() == 0 {
            return (false, false, false);
        }
        let (left, right) = Self::cell_edges(row, cell_index);
        let has_next = self.merge_cell_at(row_index.wrapping_add(1), left, right, |v| v == 1).is_some();
        // Java: TableTool.mergeCells links a previous row only when the *next*
        // raw cell is continuation (1). A restart (3) begins a new chain even
        // when the preceding row ends another chain at the same grid edges.
        let has_prev = cell.getVertMerge() != 3 && row_index > 0
            && self.merge_cell_at(row_index - 1, left, right, |v| v == 1 || v == 3).is_some();
        // Java: Data.isVertMergeCell = prev != null || next != null
        let is_vert = has_next || has_prev;
        // Java: Data.isFirstVertMergeCell = prev == null && isVertMergeCell
        let is_first = !has_prev && is_vert;
        let mut vert = false;
        let mut first = false;
        let mut last = false;
        if is_vert {
            if is_first {
                // Java: if (findValidNextVMergeCell) { vert; first; last = false }
                if self.find_valid_next(row_index, cell_index) {
                    vert = true;
                    first = true;
                }
            } else if self.find_valid_first(row_index, cell_index) {
                if !row.isRevisionDelete() {
                    vert = true;
                    // Java: lastMerged = !findValidNextVMergeCell
                    last = !self.find_valid_next(row_index, cell_index);
                } else if self.find_valid_next(row_index, cell_index) {
                    vert = true;
                }
            }
        }
        (vert, first, last)
    }

    fn cell_edges(row: &crate::cn::wps::moffice::writer::data::table::tree::RowInfoNode::RowInfoNode, cell_index: usize) -> (f32, f32) {
        (row.rgdxa(cell_index as i32) as f32, row.rgdxa(cell_index as i32 + 1) as f32)
    }

    // Java: TableTool.getNextMergeCell / getPrevMergeCell
    fn merge_cell_at(&self, row_index: usize, left: f32, right: f32, accept: impl Fn(i32) -> bool) -> Option<usize> {
        let row = self.rows.get(row_index)?;
        let mut best: Option<usize> = None;
        let mut best_left = 0f32;
        let mut best_right = 0f32;
        for (index, cell) in row.cells().iter().enumerate() {
            if !accept(cell.getVertMerge()) {
                continue;
            }
            let (cleft, cright) = Self::cell_edges(row, index);
            let left_diff = (cleft - left).abs();
            let right_diff = (cright - right).abs();
            if left_diff < 6.0 && right_diff < 6.0 {
                let closer = best.is_none() || (left_diff <= best_left && right_diff <= best_right);
                if closer {
                    best = Some(index);
                    best_left = left_diff;
                    best_right = right_diff;
                }
            }
        }
        best
    }

    // Java: Tools.findValidNextVMergeCell — walk next links, skip revision-delete rows
    fn find_valid_next(&self, mut row_index: usize, mut cell_index: usize) -> bool {
        loop {
            let Some(row) = self.rows.get(row_index) else { return false; };
            let (left, right) = Self::cell_edges(row, cell_index);
            let Some(next) = self.merge_cell_at(row_index.wrapping_add(1), left, right, |v| v == 1) else {
                return false;
            };
            row_index += 1;
            cell_index = next;
            if self.rows.get(row_index).is_some_and(|row| !row.isRevisionDelete()) {
                return true;
            }
        }
    }

    // Java: CellInfo.getPrevVertMergeCell walked to the cell with no further prev.
    pub fn first_vert_merge_cell(&self, mut row_index: usize, mut cell_index: usize) -> Option<(usize, usize)> {
        loop {
            if row_index == 0 {
                break;
            }
            let row = self.rows.get(row_index)?;
            if row.cells().get(cell_index)?.getVertMerge() == 3 {
                break;
            }
            let (left, right) = Self::cell_edges(row, cell_index);
            let Some(prev) = self.merge_cell_at(row_index - 1, left, right, |v| v == 1 || v == 3) else {
                break;
            };
            row_index -= 1;
            cell_index = prev;
        }
        Some((row_index, cell_index))
    }

    // Java: Tools.findValidFirstVMergeCell — walk prev until a cell with no further prev
    fn find_valid_first(&self, mut row_index: usize, mut cell_index: usize) -> bool {
        loop {
            if row_index == 0 {
                return false;
            }
            let Some(row) = self.rows.get(row_index) else { return false; };
            let (left, right) = Self::cell_edges(row, cell_index);
            let Some(prev) = self.merge_cell_at(row_index - 1, left, right, |v| v == 1 || v == 3) else {
                return false;
            };
            row_index -= 1;
            cell_index = prev;
            let Some(prev_row) = self.rows.get(row_index) else { return false; };
            let (pleft, pright) = Self::cell_edges(prev_row, cell_index);
            let restart = prev_row.cells().get(cell_index).is_some_and(|cell| cell.getVertMerge() == 3);
            let has_prev = !restart && row_index > 0
                && self.merge_cell_at(row_index - 1, pleft, pright, |v| v == 1 || v == 3).is_some();
            if !has_prev {
                return !prev_row.isRevisionDelete();
            }
        }
    }

    pub fn rows_mut(&mut self) -> &mut [RowInfoNode] {
        &mut self.rows
    }

    /// 格内 `CellInfoNode` 收成的嵌套 `TableInfoNode`，不含本表自己。
    pub fn nested_table_count(&self) -> i32 {
        let mut count = 0i32;
        for row in &self.rows {
            for cell in row.cells() {
                count += cell.tables().len() as i32;
                for nested in cell.tables() {
                    count += nested.nested_table_count();
                }
            }
        }
        count
    }

    // Java: IRowEntry init(IRowEntry rowEntry, int level, boolean scanNext, int maxCP)
    pub fn init(
        &mut self,
        rows: &[KTableRow],
        document: &dyn Document,
        level: i32,
        scan_next: bool,
        max_cp: i32,
        rebuild: *mut RebuildCache,
    ) -> usize {
        if rows.is_empty() {
            return 0;
        }
        // Java: scanNext advances with KTableManager.createRow until maxCP or
        // the first paragraph outside this table level. The caller's single
        // first row is not the complete input in updateInNormal/valid/tryAppend.
        let mut scanned = Vec::new();
        if scan_next {
            scanned.push(rows[0].clone());
            let manager = KTableManager::create(document);
            let mut end = rows[0].getEnd();
            while end < max_cp && manager.isInTable(end, level) {
                let next = manager.createRow(end, level, false);
                if !next.isComplete() || next.getEnd() <= end {
                    break;
                }
                end = next.getEnd();
                scanned.push(next);
            }
        }
        let rows = if scan_next { scanned.as_slice() } else { rows };
        // Java: this.mCache.getDocument()
        self.mDocument = Some(unsafe {
            std::mem::transmute::<&dyn Document, *const (dyn Document + 'static)>(document)
        });
        // Java: this.mLevel = level;
        self.mLevel = level;
        // Java: int oldTableEnd = rowEntry.getStart();
        let old_table_end = rows[0].getStart();
        // Java: int tableEnd = oldTableEnd;
        let mut table_end = old_table_end;
        // Java: int lastRowStart = lastRow.isHead() ? oldTableEnd : lastRow.getStartCP();
        let mut last_row_start = if self.row_count == 0 {
            old_table_end
        } else {
            self.last_row_start
        };
        let mut consumed = 0usize;
        for row in rows {
            // Java: int start = rowEntry.getStart(); int end = rowEntry.getEnd();
            let start = row.getStart();
            let end = row.getEnd();
            let is_head = self.row_count == 0 && consumed == 0;
            // Java: if (!lastRow.isHead() && (start != tableEnd || !TableTool.isInTheSameTable(...))) break;
            if !is_head
                && (start != table_end
                    || !TableTool::isInTheSameTable(
                        document,
                        last_row_start,
                        table_end,
                        start,
                        end,
                    ))
            {
                break;
            }
            // Java: tableEnd = end; lastRowStart = start;
            table_end = end;
            last_row_start = start;
            self.last_row_start = start;
            consumed += 1;
            self.row_count += 1;
            // Java: RowInfoNode newRow = (RowInfoNode)this.createSubNode(); newRow.init(rowEntry);
            self.rows.push(RowInfoNode::init(row, document, level, rebuild));
            // Java: if (!scanNext) rowEntry = rowEntry.getNext();
        }
        if self.row_count == consumed as i32 {
            self.start_cp = old_table_end;
        }
        // Java: if (this.mLength != 0 && lastRowEntry != null) this.invalid();
        // Java: this.mLength += tableEnd - oldTableEnd;
        self.mLength += table_end - old_table_end;
        consumed
    }

    // Java: TableInfoNode tryAppend()
    pub fn try_append(
        &mut self,
        manager: &crate::cn::wps::moffice::writer::data::table::KTableManager::KTableManager,
        document: &dyn Document,
        next_start: i32,
        rebuild: *mut RebuildCache,
    ) {
        // Java: int tableEnd = this.getEndCP();
        let mut table_end = self.getEndCP();
        // Java: if (tableManager.isInTable(tableEnd, this.mLevel))
        if !manager.isInTable(table_end, self.mLevel) {
            return;
        }
        // Java: if (tableEnd < nextTableStart) init(createRow(...), level, true, nextTableStart);
        while table_end < next_start && manager.isInTable(table_end, self.mLevel) {
            // Java: IRowEntry rowEntry = tableManager.createRow(tableEnd, this.mLevel);
            let row = manager.createRow(table_end, self.mLevel, false);
            if !row.isComplete() {
                break;
            }
            let used = self.init(std::slice::from_ref(&row), document, self.mLevel, true, next_start, rebuild);
            if used == 0 {
                break;
            }
            let next_end = self.getEndCP();
            if next_end <= table_end {
                break;
            }
            table_end = next_end;
        }
    }

    pub fn shift_cp(&mut self, delta: i32) {
        if delta == 0 {
            return;
        }
        self.start_cp += delta;
        for row in &mut self.rows {
            row.shift_cp(delta);
        }
    }

    pub fn set_length(&mut self, length: i32) {
        self.mLength = length;
    }

    // Java: int getGeneratioin()
    pub fn getGeneration(&self) -> i32 {
        self.mGeneration
    }

    pub fn row_len(&self) -> usize {
        self.rows.len()
    }

    pub fn cell_len(&self, row_index: usize) -> usize {
        self.rows.get(row_index).map(|row| row.cells().len()).unwrap_or(0)
    }

    pub fn nested_table_len(&self, row_index: usize, cell_index: usize) -> usize {
        self.rows
            .get(row_index)
            .and_then(|row| row.cells().get(cell_index))
            .map(|cell| cell.tables().len())
            .unwrap_or(0)
    }

    // java public int getStyleId()
    pub fn getStyleId(&mut self) -> i32 {
        // java this.valid(); return this.getData().getStyleId();
        self.valid();
        self.mStyleId
    }

    // Java: synchronized void valid()
    pub fn valid(&mut self) {
        // Java: if (!this.mValid) { this.mValid = true; tableTool.calcTableStyles(this); }
        if self.mValid {
            return;
        }
        self.mValid = true;
        // Java: tableData.clear()
        self.mPos = None;
        self.mShd = None;
        self.mStyleId = 4095;
        self.mTableLook = None;
        self.mTableBorders = None;
        self.mIsRTL = false;
        self.mWaitFitUpdate = false;
        let Some(document) = self.document() else {
            return;
        };
        let Some(end) = self.rows.first().map(|row| row.getEndCP()) else {
            return;
        };
        let calc = TableTool::calcTableStyles(document, end);
        self.mPos = calc.pos;
        self.mShd = calc.shd;
        self.mStyleId = calc.style_id;
        self.mTableLook = Some(calc.table_look);
        self.mTableBorders = calc.borders;
        self.mIsRTL = calc.is_rtl;
    }

    // Java: public GridCols getGrid()
    pub fn getGrid(&mut self) -> &GridColsImpl {
        // Java: if (data.mGridCols == null) { gridCols = new GridColsImpl(cache, this); gridCols.valid(); data.mGridCols = gridCols; }
        if self.mGridCols.is_none() {
            let mut grid = GridColsImpl::new();
            grid.valid(self);
            self.mGridCols = Some(grid);
        }
        self.mGridCols.as_ref().unwrap()
    }

    // Java: getGrid().calColFitInfo()
    pub fn calColFitInfo(&mut self) {
        let mut grid = match self.mGridCols.take() {
            Some(grid) => grid,
            None => {
                let mut grid = GridColsImpl::new();
                // Java: getGrid() calls valid() when the grid is first created
                grid.valid(self);
                grid
            }
        };
        grid.calColFitInfo(self);
        self.mGridCols = Some(grid);
    }

    // Java: getGrid().autoFitSize(typoCoreWidth). getGrid() valid()s a missing grid first.
    pub fn autoFitSize(&mut self, typo_core_width: i32) -> bool {
        let mut grid = match self.mGridCols.take() {
            Some(grid) => grid,
            None => {
                let mut grid = GridColsImpl::new();
                grid.valid(self);
                grid
            }
        };
        let changed = grid.autoFitSize(self, typo_core_width);
        self.mGridCols = Some(grid);
        changed
    }

    // Java: Document getDocument()
    pub fn getDocument(&self) -> Option<&dyn Document> {
        self.document()
    }

    // Java: getPos()
    pub fn getPos(&mut self) -> Option<&TablePos> {
        // Java: this.valid();
        self.valid();
        // Java: return this.getData().getPos();
        self.mPos.as_ref()
    }

    // Java: PropertySet getTableProperty() — first row getFinalProperty()
    pub fn table_property(&self) -> Option<PropertySet> {
        let document = self.document()?;
        let end = self.rows.first()?.getEndCP();
        Some(TableTool::getFinalRowProperty(document, end - 1))
    }

    // Java: TableTool.getTableFitMaxWidth(document, getRowByIndex(0))
    pub fn fit_max_core_width(&self) -> i32 {
        let document = self.getDocument().expect("table document");
        let row = &self.rows[0];
        let cp = row.getStartCP();
        let end = row.getEndCP();
        let last_cell_index = row.cells().len() as i32 - 1;
        let prop = TableTool::getFinalRowProperty(document, end - 1);
        TableTool::getTableFitMaxWidthAt(document, cp, &prop, last_cell_index)
    }

    pub fn row_rgdxa_mut(&mut self, row_index: usize) -> Option<&mut [i32]> {
        self.rows.get_mut(row_index)?.rgdxa_center_mut()
    }

    // Java: child.getGrid() then checkFitChanged(). getGrid() valid()s only when missing.
    pub fn nested_check_fit_changed(
        &mut self,
        row_index: usize,
        cell_index: usize,
        nested_index: usize,
    ) -> bool {
        let Some(nested) = self
            .rows
            .get_mut(row_index)
            .and_then(|row| row.cells_mut().get_mut(cell_index))
            .and_then(|cell| cell.tables_mut().get_mut(nested_index))
        else {
            return false;
        };
        let created = nested.mGridCols.is_none();
        let mut grid = nested.mGridCols.take().unwrap_or_else(GridColsImpl::new);
        if created {
            grid.valid(nested);
        }
        let changed = grid.checkFitChanged(nested);
        nested.mGridCols = Some(grid);
        changed
    }

    // Java: child.getGrid().getAutoSize().applyAutoFitChange()
    pub fn nested_apply_auto_fit(
        &mut self,
        row_index: usize,
        cell_index: usize,
        nested_index: usize,
    ) {
        let Some(nested) = self
            .rows
            .get_mut(row_index)
            .and_then(|row| row.cells_mut().get_mut(cell_index))
            .and_then(|cell| cell.tables_mut().get_mut(nested_index))
        else {
            return;
        };
        let created = nested.mGridCols.is_none();
        let mut grid = nested.mGridCols.take().unwrap_or_else(GridColsImpl::new);
        if created {
            grid.valid(nested);
        }
        grid.applyAutoFitChange(nested);
        nested.mGridCols = Some(grid);
    }

    // Java: child getGrid(), jc indent, fixed width, then doFit(adjust) — not autoFitSize
    pub fn nested_do_fit(
        &mut self,
        row_index: usize,
        cell_index: usize,
        nested_index: usize,
        cell_core_width: i32,
    ) {
        let Some(nested) = self
            .rows
            .get_mut(row_index)
            .and_then(|row| row.cells_mut().get_mut(cell_index))
            .and_then(|cell| cell.tables_mut().get_mut(nested_index))
        else {
            return;
        };
        let created = nested.mGridCols.is_none();
        let mut grid = nested.mGridCols.take().unwrap_or_else(GridColsImpl::new);
        if created {
            grid.valid(nested);
        }
        let jc = nested.getJC();
        let mut adjust = cell_core_width;
        if jc.jc == 0 {
            adjust = cell_core_width - jc.tableIndent;
        }
        let fix_width = nested.fixed_table_width();
        if fix_width > 0 {
            let real_best = grid.getTotalBestWidth(nested);
            if real_best <= fix_width && adjust < fix_width {
                adjust = fix_width;
            }
        }
        grid.doFit(nested, adjust);
        nested.mGridCols = Some(grid);
    }

    pub fn cell_mut(&mut self, row_index: usize, cell_index: usize) -> Option<&mut CellInfoNode> {
        self.rows
            .get_mut(row_index)
            .and_then(|row| row.cells_mut().get_mut(cell_index))
    }

    // Java: childTable.getGrid().calColFitInfo(), then jc == 0 uses tableIndent
    pub fn nested_auto_fit(
        &mut self,
        row_index: usize,
        cell_index: usize,
        nested_index: usize,
    ) -> Option<(i32, i32, i32, i32)> {
        let nested = self
            .rows
            .get_mut(row_index)
            .and_then(|row| row.cells_mut().get_mut(cell_index))
            .and_then(|cell| cell.tables_mut().get_mut(nested_index))?;
        let created = nested.mGridCols.is_none();
        let mut grid = nested.mGridCols.take().unwrap_or_else(GridColsImpl::new);
        if created {
            grid.valid(nested);
        }
        grid.calColFitInfo(nested);
        let jc = nested.getJC();
        let indent = if jc.jc == 0 { nested.getJC().tableIndent } else { 0 };
        let min_width = grid.getTotalMinWidth();
        let best_width = grid.getTotalBestWidth(nested);
        let para_width = grid.getParaTotalMax(nested);
        nested.mGridCols = Some(grid);
        Some((min_width, best_width, para_width, indent))
    }

    // Java: TableTool.getFixedTableWidth(this) via getTableProperty()
    pub fn fixed_table_width(&mut self) -> i32 {
        let Some(document) = self.document() else {
            return 0;
        };
        let Some(end) = self.rows.first().map(|row| row.getEndCP()) else {
            return 0;
        };
        if end <= 0 {
            return 0;
        }
        let prop = TableTool::getFinalRowProperty(document, end - 1);
        TableTool::getFixedTableWidth(&prop)
    }

    // Java: void invalid()
    pub fn invalid(&mut self) {
        // Java: this.invalid(true);
        self.invalid_with_grid(true);
    }

    // Java: void invalid(boolean invalidGrid)
    pub fn invalid_with_grid(&mut self, invalid_grid: bool) {
        // Java: this.mValid = false; this.getData().mTableJCData = null;
        self.mValid = false;
        // Java: this.getData().mTableJCData = null;
        self.mTableJCData = None;
        // Java: if (invalidGrid && this.getData().mGridCols != null) this.getData().mGridCols.invalid();
        if invalid_grid {
            if let Some(mut grid) = self.mGridCols.take() {
                grid.invalid(self);
                self.mGridCols = Some(grid);
            }
        }
        // Java: ++this.mGeneration; if (this.mGeneration < 0) reset row/cell generations
        self.mGeneration = self.mGeneration.wrapping_add(1);
        if self.mGeneration < 0 {
            self.mGeneration = 1;
            for row in &mut self.rows {
                row.set_generation(0);
                for cell in row.cells_mut() {
                    cell.set_generation(0);
                }
            }
        }
    }

    // Java: child table's mGridCols.invalid(), then invalidCellOriginWidths(child)
    pub fn invalidate_child_grid(&mut self, row_index: usize, cell_index: usize, nested_index: usize) {
        let Some(nested) = self
            .rows
            .get_mut(row_index)
            .and_then(|row| row.cells_mut().get_mut(cell_index))
            .and_then(|cell| cell.tables_mut().get_mut(nested_index))
        else {
            return;
        };
        if let Some(mut grid) = nested.mGridCols.take() {
            grid.invalid(nested);
            nested.mGridCols = Some(grid);
        }
        let text = nested.is_text_document();
        let level = nested.getLevel();
        let percent = nested.is_percent_width();
        if (text || level > 1) && percent {
            nested.clear_cell_widths();
        }
    }

    pub fn clear_cell_widths(&mut self) {
        for row in &mut self.rows {
            row.clear_cell_widths();
        }
    }

    pub fn is_text_document(&self) -> bool {
        let Some(document) = self.mDocument else {
            return false;
        };
        let document = unsafe { &*document };
        let ty = document.getType__on_IDocument();
        // Java: type == 5 || type == 6
        ty == 5 || ty == 6
    }

    pub fn is_percent_width(&mut self) -> bool {
        let Some(document) = self.document() else {
            return false;
        };
        let Some(end) = self.rows.first().map(|row| row.getEndCP()) else {
            return false;
        };
        let prop = TableTool::getFinalRowProperty(document, end - 1);
        TableTool::isTableWithPercent(&prop)
    }

    pub fn ensure_row_metrics(&mut self, row_index: usize) {
        let fresh = self
            .rows
            .get(row_index)
            .is_some_and(|row| row.generation() >= self.mGeneration && row.has_widths());
        if fresh {
            return;
        }
        self.ensure_row_geometry(row_index);
        // Java: delayInitCellWidths uses getParent().getJC()
        let jc = self.getJC();
        let Some(document) = self.document() else {
            return;
        };
        let end = self.rows[row_index].getEndCP();
        let start = self.rows[row_index].getStartCP();
        let prop = TableTool::getFinalRowProperty(document, end - 1);
        let table_prop = self
            .rows
            .first()
            .map(|row| TableTool::getFinalRowProperty(document, row.getEndCP() - 1))
            .unwrap_or_else(|| prop.clone());
        let margins: Vec<(i32, i32)> = self.rows[row_index]
            .cells()
            .iter()
            .map(|cell| (cell.margin_left(), cell.margin_right()))
            .collect();
        // Java: data.mCellWidths = TableTool.calcCellWidths(this, this.getFinalProperty(), this.getParent().getJC(), false);
        let widths = TableTool::calcCellWidths(
            document,
            self.mLevel,
            start,
            &prop,
            &table_prop,
            &margins,
            &jc,
            false,
            self.parent_fit,
        );
        if let Some(row) = self.rows.get_mut(row_index) {
            row.set_cell_widths(widths, self.mGeneration);
        }
    }

    // Java: row.valid() writes rgdxa and margins before delayInitCellWidths
    fn ensure_row_geometry(&mut self, row_index: usize) {
        if self.rows.get(row_index).is_none() {
            return;
        }
        self.valid();
        let borders = self.mTableBorders.clone();
        let Some(document) = self.document() else {
            return;
        };
        let end = self.rows[row_index].getEndCP();
        let cell_count = self.rows[row_index].cells().len() as i32;
        let calc = TableTool::calcRowStyle(
            document,
            row_index as i32,
            self.rows.len() as i32,
            self.mIsComplete,
            end,
            cell_count,
            borders.as_ref(),
        );
        if let Some(row) = self.rows.get_mut(row_index) {
            row.apply_row_calc(calc, self.mGeneration);
        }
        self.publish_parent_fit(row_index);
    }

    fn publish_parent_fit(&mut self, row_index: usize) {
        let cell_count = self.rows.get(row_index).map(|row| row.cells().len()).unwrap_or(0);
        for cell_index in 0..cell_count {
            let left = self.rows[row_index].rgdxa(cell_index as i32);
            let right = self.rows[row_index].rgdxa(cell_index as i32 + 1);
            let margin_left = self.rows[row_index].cells()[cell_index].margin_left();
            let margin_right = self.rows[row_index].cells()[cell_index].margin_right();
            let nested_len = self.rows[row_index].cells()[cell_index].tables().len();
            for nested_index in 0..nested_len {
                if let Some(nested) = self.rows[row_index]
                    .cells_mut()
                    .get_mut(cell_index)
                    .and_then(|cell| cell.tables_mut().get_mut(nested_index))
                {
                    nested.set_parent_cell_fit(left, right, margin_left, margin_right);
                }
            }
        }
    }

    // Java: Data.TableData.mIsRTL / TableInfo.isRTL()
    pub fn isRTL(&self) -> bool { self.mIsRTL }

    // Java: public synchronized TableJCData getJC()
    pub fn getJC(&mut self) -> TableJCData {
        if let Some(jc) = self.mTableJCData {
            return jc;
        }
        let row_count = self.rows.len();
        for index in 0..row_count {
            self.ensure_row_geometry(index);
        }
        let Some(document) = self.document() else {
            return TableJCData::default();
        };
        let Some(first) = self.rows.first() else {
            return TableJCData::default();
        };
        let first_end = first.getEndCP();
        let borders = self.mTableBorders.clone();
        let first_calc = TableTool::calcRowStyle(
            document,
            0,
            self.rows.len() as i32,
            self.mIsComplete,
            first_end,
            self.rows[0].cells().len() as i32,
            borders.as_ref(),
        );
        let inputs: Vec<RowJcInput> = self
            .rows
            .iter()
            .map(|row| RowJcInput {
                end_cp: row.getEndCP(),
                rgdxa_left: row.rgdxa(0),
                rgdxa_right: row.rgdxa(row.cells().len() as i32),
            })
            .collect();
        let jc = TableTool::calcJC(document, &first_calc.merged, &inputs);
        self.mTableJCData = Some(jc);
        jc
    }

    pub fn set_parent_cell_fit(&mut self, left: i32, right: i32, margin_left: i32, margin_right: i32) {
        let next = ParentCellFit {
            rgdxa_left: left,
            rgdxa_right: right,
            margin_left,
            margin_right,
        };
        if self.parent_fit.is_some_and(|fit| {
            fit.rgdxa_left == next.rgdxa_left
                && fit.rgdxa_right == next.rgdxa_right
                && fit.margin_left == next.margin_left
                && fit.margin_right == next.margin_right
        }) {
            return;
        }
        self.parent_fit = Some(next);
        self.mTableJCData = None;
        self.clear_cell_widths();
    }

    pub fn rgdxa_count(&mut self, row_index: usize) -> i32 {
        self.ensure_row_metrics(row_index);
        self.rows.get(row_index).map(|row| row.rgdxa_count()).unwrap_or(0)
    }

    pub fn rgdxa(&mut self, row_index: usize, index: i32) -> i32 {
        self.ensure_row_metrics(row_index);
        self.rows.get(row_index).map(|row| row.rgdxa(index)).unwrap_or(0)
    }

    pub fn cell_auto(&mut self, row_index: usize, cell_index: usize) -> bool {
        self.ensure_row_metrics(row_index);
        self.rows
            .get(row_index)
            .and_then(|row| row.cells().get(cell_index))
            .is_some_and(|cell| cell.is_auto())
    }

    pub fn cell_auto_at(&self, row_index: usize, cell_index: usize) -> bool {
        self.rows
            .get(row_index)
            .and_then(|row| row.cells().get(cell_index))
            .is_some_and(|cell| cell.is_auto())
    }

    pub fn cell_width_at(&self, row_index: usize, cell_index: usize) -> i32 {
        self.rows
            .get(row_index)
            .map(|row| row.cell_width(cell_index))
            .unwrap_or(0)
    }

    pub fn cell_width(&mut self, row_index: usize, cell_index: usize) -> i32 {
        self.ensure_row_metrics(row_index);
        self.rows
            .get(row_index)
            .map(|row| row.cell_width(cell_index))
            .unwrap_or(0)
    }

    pub fn width_before(&mut self, row_index: usize) -> i32 {
        self.ensure_row_metrics(row_index);
        self.rows.get(row_index).map(|row| row.width_before()).unwrap_or(0)
    }

    fn document(&self) -> Option<&dyn Document> {
        self.mDocument.map(|document| unsafe { &*document })
    }

    // Java: void positionFix(int offsetFromThis, int steps, int[] offsetBuf)
    pub fn positionFix(&mut self, offset_from_this: i32, steps: i32, marks: &mut Vec<i32>) {
        // Java: this.mLength += steps;
        self.mLength += steps;
        position_fix_rows(&mut self.rows, self.start_cp, offset_from_this, steps, marks);
    }

    // Java: boolean removeNodeInRange(int start, int end, int[] offsetBuf)
    pub fn removeNodeInRange(&mut self, start: i32, end: i32, rebuild: *mut RebuildCache) -> bool {
        // Java: boolean nodeChanged = super.removeNodeInRange(start, end, offsetBuf);
        let changed = remove_rows(&mut self.rows, self.start_cp, start, end, rebuild);
        // Java: if (nodeChanged) this.invalid();
        if changed {
            self.invalid();
        }
        changed
    }

    // Java: void valid(int start, int end)
    // Java: void valid(int start, int end)
    pub fn valid__int_int(
        &mut self,
        document: &dyn Document,
        mut start: i32,
        end: i32,
        rebuild: *mut RebuildCache,
    ) {
        let position = self.getStartCP();
        let manager = KTableManager::create(document);
        while start < end {
            // Java: IRowEntry rowEntry = this.getTableManager().createRow(position + start, this.mLevel);
            let row = manager.createRow(position + start, self.mLevel, false);
            if !row.isComplete() {
                break;
            }
            // Java: RowInfoNode newNode = (RowInfoNode)this.createSubNode(); newNode.init(rowEntry);
            let node = RowInfoNode::init(&row, document, self.mLevel, rebuild);
            let len = node.getEndCP() - node.getStartCP();
            if len <= 0 {
                break;
            }
            let at = node.getStartCP();
            let idx = self
                .rows
                .iter()
                .position(|row| row.getStartCP() >= at)
                .unwrap_or(self.rows.len());
            // Java: this.mTree.insertRight(lastNode, newNode);
            self.rows.insert(idx, node);
            start += len;
        }
    }

    // Java: protected void close()
    pub fn close(&mut self, cache: &mut RebuildCache) {
        // Java: for (subNode = leftmost; !subNode.isHead(); subNode = subNode.getNext()) subNode.close();
        for row in &mut self.rows {
            row.close(cache);
        }
    }
}

fn position_fix_rows(
    rows: &mut [RowInfoNode],
    parent_start: i32,
    offset_from_this: i32,
    steps: i32,
    marks: &mut Vec<i32>,
) {
    if rows.is_empty() {
        return;
    }
    let edit = parent_start + offset_from_this;
    let prev_i = rows.iter().rposition(|row| row.getStartCP() <= edit);
    if steps > 0 {
        if let Some(i) = prev_i {
            let within = edit - rows[i].getStartCP();
            let len = rows[i].getEndCP() - rows[i].getStartCP();
            // Java: if (!node.isHead() && offsetBuf[0] < node.mLength) node.positionFix(offsetBuf[0], steps);
            if within < len {
                rows[i].positionFix(within, steps, marks);
            }
        }
    } else {
        let deleted = -steps;
        if let Some(i) = prev_i {
            let within = edit - rows[i].getStartCP();
            let len = rows[i].getEndCP() - rows[i].getStartCP();
            if within < len {
                // Java: int subDeleted = Math.min(deleted, node.mLength - offsetBuf[0]);
                let sub_deleted = deleted.min(len - within);
                rows[i].positionFix(within, -sub_deleted, marks);
            }
        }
        if let Some(upper_i) = rows.iter().position(|row| row.getStartCP() > edit) {
            let upper_off = rows[upper_i].getStartCP() - parent_start;
            // Java: int upperDeleted = offsetFromThis + deleted - upper.offsetFromParent();
            let upper_deleted = offset_from_this + deleted - upper_off;
            if upper_deleted > 0 {
                // Java: this.mTree.moveRight(upper, upperDeleted); upper.positionFix(0, -upperDeleted);
                rows[upper_i].shift_cp(upper_deleted);
                rows[upper_i].positionFix(0, -upper_deleted, marks);
            }
        }
    }
    // Java: this.mTree.insertSpaceBefore(upper, steps);
    for row in rows.iter_mut() {
        if row.getStartCP() > edit {
            row.shift_cp(steps);
        }
    }
}

fn remove_rows(
    rows: &mut Vec<RowInfoNode>,
    parent_start: i32,
    start: i32,
    end: i32,
    rebuild: *mut RebuildCache,
) -> bool {
    if rows.is_empty() || end <= start {
        return false;
    }
    let range_lo = parent_start + start;
    let range_hi = parent_start + end;
    let mut changed = false;
    let mut i = 0;
    while i < rows.len() {
        let row_lo = rows[i].getStartCP();
        let row_hi = rows[i].getEndCP();
        if row_hi <= range_lo || row_lo >= range_hi {
            i += 1;
            continue;
        }
        // Java: subStart == 0 && subEnd == node.mLength → eraseSubNode
        if row_lo >= range_lo && row_hi <= range_hi {
            // Java: eraseSubNode → close() then erase
            let mut row = rows.remove(i);
            if !rebuild.is_null() {
                row.close(unsafe { &mut *rebuild });
            }
            changed = true;
            continue;
        }
        let sub_start = (range_lo - row_lo).max(0);
        let sub_end = (range_hi - row_lo).min(row_hi - row_lo);
        changed |= rows[i].removeNodeInRange(sub_start, sub_end, rebuild);
        i += 1;
    }
    changed
}
