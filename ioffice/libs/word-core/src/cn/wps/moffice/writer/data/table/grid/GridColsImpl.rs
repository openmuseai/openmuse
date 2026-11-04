#![allow(non_snake_case)]

// Java: import cn.wps.moffice.writer.core.table.Tools
use crate::cn::wps::moffice::writer::core::table::Tools::Tools;
// Java: import cn.wps.moffice.writer.data.table.TableTool
use crate::cn::wps::moffice::writer::data::table::TableTool::TableTool;
// Java: import cn.wps.moffice.writer.data.table.tree.TableInfoNode
use crate::cn::wps::moffice::writer::data::table::tree::TableInfoNode::TableInfoNode;
// Java: import cn.wps.moffice.writer.data.value.TableLength
use crate::cn::wps::moffice::writer::data::value::TableLength::TableLength;
use super::GridCellImpl::GridCellImpl;
use super::GridColImpl::GridColImpl;

// Java: public class GridColsImpl implements GridCols, GridAutoSize
#[derive(Clone, Default)]
pub struct GridColsImpl {
    // Java field: int mGeneration = 0
    mGeneration: i32,
    // Java field: boolean mForceFit = false
    mForceFit: bool,
    // Java field: TableWidthType mTableWidthType
    mTableWidthType: TableWidthType,
    // Java field: ArrayList<GridCol> mIndexedColums
    mIndexedColums: Vec<GridColImpl>,
    // Java field: ArrayList<GridCell> mSpanedCells — (row, first column)
    mSpanedCells: Vec<(usize, usize)>,
    // Java field: ArrayList<GridCell> mCacheSpanedCell — (column, row)
    mCacheSpanedCell: Vec<(usize, usize)>,
}

impl GridColsImpl {
    // Java: GridColsImpl(TreeTableCache cache, TableInfoNode tableInfo)
    pub fn new() -> Self {
        Self {
            mGeneration: 0,
            mForceFit: false,
            mTableWidthType: TableWidthType::default(),
            mIndexedColums: Vec::new(),
            mSpanedCells: Vec::new(),
            mCacheSpanedCell: Vec::new(),
        }
    }

    // Java: public synchronized void valid()
    pub fn valid(&mut self, table: &mut TableInfoNode) {
        // Java: if (this.mGeneration < this.mTableInfo.getGeneratioin())
        if self.mGeneration < table.getGeneration() {
            self.mGeneration = table.getGeneration();
            self.init(table);
        } else if self.invalidCellOriginWidths(table) {
            // Java: this.calcWidthForGridCell();
            self.calcWidthForGridCell(table);
        }
    }

    // Java: public synchronized void invalid()
    pub fn invalid(&mut self, table: &mut TableInfoNode) {
        // Java: this.mGeneration = 0; this.mForceFit = false;
        self.mGeneration = 0;
        self.mForceFit = false;
        let row_count = table.row_len();
        for row_index in 0..row_count {
            let cell_count = table.cell_len(row_index);
            for cell_index in 0..cell_count {
                let nested = table.nested_table_len(row_index, cell_index);
                for nested_index in 0..nested {
                    // Java: tableData.mGridCols.invalid(); this.invalidCellOriginWidths(tableInfo);
                    table.invalidate_child_grid(row_index, cell_index, nested_index);
                }
            }
        }
    }

    // Java: private void init()
    fn init(&mut self, table: &mut TableInfoNode) {
        // Java: this.disposeColumns(); this.rebuildCulums(); this.associateOnCells(); this.calcWidthForGridCell();
        self.mIndexedColums.clear();
        self.rebuildCulums(table);
        self.associateOnCells(table);
        self.calcWidthForGridCell(table);
    }

    // Java: private void rebuildCulums()
    fn rebuildCulums(&mut self, table: &mut TableInfoNode) {
        let row_count = table.row_len();
        for row_index in 0..row_count {
            table.ensure_row_metrics(row_index);
            let cell_size = table.cell_len(row_index) as i32;
            let rgdx_count = table.rgdxa_count(row_index);
            for i in 0..rgdx_count {
                let rgdx = table.rgdxa(row_index, i);
                let cell_width = if i == cell_size {
                    rgdx - table.rgdxa(row_index, i - 1)
                } else {
                    table.rgdxa(row_index, i + 1) - rgdx
                };
                let residual = TableTool::getResidual(cell_width);
                if self.col_at_offset(rgdx).is_none() {
                    if self.mIndexedColums.is_empty() {
                        let width = table.rgdxa(row_index, i + 1) - rgdx;
                        self.addColToColunms(rgdx, width);
                    } else {
                        let left_pos = self.mIndexedColums[0].getLeftPos();
                        let left_offset = left_pos - rgdx;
                        if left_offset > 0 {
                            if left_offset > residual {
                                self.addColToColunms(rgdx, left_offset);
                            }
                        } else {
                            let right = self.mIndexedColums.last().unwrap().getRightPos();
                            if rgdx < right + residual {
                                if i < cell_size {
                                    let width = table.rgdxa(row_index, i + 1) - rgdx;
                                    self.addColToColunms(rgdx, width);
                                }
                            } else {
                                self.addColToColunms(right, rgdx - right);
                            }
                        }
                    }
                } else if let Some(col_index) = self.col_at_offset(rgdx) {
                    let left_offset = rgdx - self.mIndexedColums[col_index].getLeftPos();
                    let right_offset = self.mIndexedColums[col_index].getRightPos() - rgdx;
                    if right_offset > residual && left_offset > residual {
                        self.mIndexedColums[col_index].mWidth = left_offset;
                        self.addColToColunms(rgdx, right_offset);
                    } else if right_offset > residual && i > 0 {
                        let left_residual = TableTool::getResidual(
                            rgdx - table.rgdxa(row_index, i - 1),
                        ) as f32;
                        if (left_offset as f32) > left_residual {
                            self.mIndexedColums[col_index].mWidth = left_offset;
                            self.addColToColunms(rgdx, right_offset);
                        }
                    }
                }
            }
        }
    }

    // Java: private void addColToColunms(int leftPos, int width)
    fn addColToColunms(&mut self, left_pos: i32, width: i32) {
        let col = GridColImpl::new(left_pos, width);
        let size = self.mIndexedColums.len();
        if size == 0 {
            self.mIndexedColums.push(col);
            return;
        }
        let index = self.binarySearch(left_pos as f32);
        if index < 0 {
            let index = (-index - 1) as usize;
            if index >= size {
                self.mIndexedColums.push(col);
            } else {
                self.mIndexedColums.insert(index, col);
            }
        }
    }

    // Java: private void associateOnCells()
    fn associateOnCells(&mut self, table: &mut TableInfoNode) {
        let row_count = table.row_len();
        for row_index in 0..row_count {
            table.ensure_row_metrics(row_index);
            let cell_size = table.cell_len(row_index);
            for cell_index in 0..cell_size {
                let left = table.rgdxa(row_index, cell_index as i32);
                let right = table.rgdxa(row_index, cell_index as i32 + 1);
                let auto = self.mForceFit || table.cell_auto(row_index, cell_index);
                self.setCellForOverlayCols(row_index, cell_index as i32, left, right, auto);
            }
        }
    }

    // Java: private void setCellForOverlayCols(CellInfo cellInfo, int rowIndex)
    fn setCellForOverlayCols(
        &mut self,
        row_index: usize,
        cell_index: i32,
        left_pos: i32,
        right_pos: i32,
        auto: bool,
    ) {
        let mut pre_col: Option<usize> = None;
        let size = self.mIndexedColums.len();
        for i in 0..size {
            if !Self::insidePosRange(left_pos, right_pos, &self.mIndexedColums[i]) {
                continue;
            }
            if !auto {
                self.mIndexedColums[i].mIsAutoWidth = false;
            }
            let mut cell = GridCellImpl::new(cell_index);
            cell.pre_col = pre_col;
            if let Some(prev) = pre_col {
                if let Some(prev_cell) = self.mIndexedColums[prev].getCellMut(row_index) {
                    prev_cell.next_col = Some(i);
                }
            }
            self.mIndexedColums[i].setCell(row_index, cell);
            pre_col = Some(i);
        }
    }

    // Java: private boolean insidePosRange(int cellLeft, int cellRight, GridCol col)
    fn insidePosRange(cell_left: i32, cell_right: i32, col: &GridColImpl) -> bool {
        let col_left = col.getLeftPos();
        let col_right = col.getRightPos();
        if col_right >= cell_left && col_left <= cell_right {
            let left = cell_left.max(col_left);
            let right = cell_right.min(col_right);
            let residual = TableTool::getResidual(cell_right - cell_left);
            right - left > residual
        } else {
            false
        }
    }

    // Java: public int columSize()
    pub fn columSize(&self) -> i32 {
        self.mIndexedColums.len() as i32
    }

    // Java: GridCol getColByOffset(float offset)
    fn col_at_offset(&self, offset: i32) -> Option<usize> {
        let size = self.mIndexedColums.len();
        if size == 0 {
            return None;
        }
        if size <= 4 {
            for (i, col) in self.mIndexedColums.iter().enumerate() {
                if offset >= col.getLeftPos() && offset < col.getRightPos() {
                    return Some(i);
                }
            }
            return None;
        }
        let index = self.binarySearch(offset as f32);
        if index >= 0 {
            return Some(index as usize);
        }
        let index = (-index - 1) as usize;
        if index == 0 {
            None
        } else if index == size {
            let last = &self.mIndexedColums[size - 1];
            if offset < last.getRightPos() {
                Some(size - 1)
            } else {
                None
            }
        } else {
            Some(index - 1)
        }
    }

    // Java: private int binarySearch(float value) — returns ~lo when absent
    fn binarySearch(&self, value: f32) -> i32 {
        let size = self.mIndexedColums.len();
        let mut lo = 0i32;
        let mut hi = size as i32 - 1;
        while lo <= hi {
            let mid = (lo + hi) >> 1;
            let mid_val = self.mIndexedColums[mid as usize].getLeftPos() as f32;
            if mid_val < value {
                lo = mid + 1;
            } else if mid_val > value {
                hi = mid - 1;
            } else {
                return mid;
            }
        }
        !lo
    }

    // Java: private void calcWidthForGridCell()
    fn calcWidthForGridCell(&mut self, table: &mut TableInfoNode) {
        // Java: this.mSpanedCells.clear();
        self.mSpanedCells.clear();
        let col_size = self.mIndexedColums.len();
        for col_index in 0..col_size {
            if let Some((row_index, cell)) = self.longest_cell_no_span(table, col_index) {
                let width = table.cell_width(row_index, cell.cell_index as usize);
                self.mIndexedColums[col_index].mOriWidth = width;
                let auto = table.cell_auto(row_index, cell.cell_index as usize) || self.mForceFit;
                if auto {
                    self.mIndexedColums[col_index].mIsAutoWidth = true;
                    if let Some((span_row, span_col)) = self.first_not_auto(table, col_index) {
                        if self.span_count(span_col, span_row) > 1 {
                            if let Some(first_col) = self.first_span_col(span_col, span_row) {
                                if !self.mSpanedCells.contains(&(span_row, first_col)) {
                                    self.mSpanedCells.push((span_row, first_col));
                                }
                            }
                        }
                    }
                }
            } else if let Some((row_index, cell)) = self.first_cell_at_span_end(col_index) {
                let width = table.cell_width(row_index, cell.cell_index as usize);
                let offset = self.ori_offset_for_span(table, col_index, row_index);
                self.mIndexedColums[col_index].mOriWidth = width - offset;
            } else if col_index + 1 < col_size {
                if let Some((row_index, cell)) = self.first_cell_at_span_begin(col_index + 1) {
                    if cell.cell_index == 0 {
                        let width_before = table.width_before(row_index);
                        let offset = self.ori_offset_for_table(col_index);
                        self.mIndexedColums[col_index].mOriWidth = width_before - offset;
                    } else if let Some(pre_col) = self.pre_col_index(col_index + 1) {
                        if let Some(pre_cell) = self.mIndexedColums[pre_col].getCell(row_index) {
                            let width = table.cell_width(row_index, pre_cell.cell_index as usize);
                            let offset = self.ori_offset_for_span(table, pre_col, row_index);
                            self.mIndexedColums[col_index].mOriWidth = width - offset;
                        }
                    }
                } else {
                    self.mIndexedColums[col_index].mOriWidth = 0;
                }
            }
        }
        self.adjustAutoColForOriWidth(table);
    }

    // Java: private void adjustAutoColForOriWidth()
    fn adjustAutoColForOriWidth(&mut self, table: &TableInfoNode) {
        let spanned = std::mem::take(&mut self.mSpanedCells);
        for (row_index, first_col) in spanned {
            let Some(cell) = self.mIndexedColums[first_col].getCell(row_index) else {
                continue;
            };
            // Java: float extraSplitWidth = (float)cellInfo.getWidth();
            let mut extra = table.cell_width_at(row_index, cell.cell_index as usize) as f32;
            let last = first_col + self.span_count(first_col, row_index) - 1;
            let mut total_weight = 0.0f32;
            for col_index in first_col..=last {
                let data = &self.mIndexedColums[col_index];
                extra -= data.mOriWidth as f32;
                // Java: auto columns weight mParaTotalMax when it exceeds mOriWidth, otherwise mOriWidth
                if data.mIsAutoWidth {
                    if data.mParaTotalMax > data.mOriWidth {
                        total_weight += data.mParaTotalMax as f32;
                    } else {
                        total_weight += data.mOriWidth as f32;
                    }
                }
            }
            if extra > 0.0 && total_weight > 0.0 {
                for col_index in first_col..=last {
                    let data = &mut self.mIndexedColums[col_index];
                    if data.mIsAutoWidth {
                        if data.mParaTotalMax > data.mOriWidth {
                            // Java: mOriWidth += extraSplitWidth * mParaTotalMax / totalWeight
                            data.mOriWidth = (data.mOriWidth as f32
                                + extra * data.mParaTotalMax as f32 / total_weight)
                                as i32;
                        } else {
                            // Java: mOriWidth += extraSplitWidth * mOriWidth / totalWeight
                            data.mOriWidth = (data.mOriWidth as f32
                                + extra * data.mOriWidth as f32 / total_weight)
                                as i32;
                        }
                    }
                }
            }
        }
    }

    fn longest_cell_no_span(
        &self,
        table: &TableInfoNode,
        col_index: usize,
    ) -> Option<(usize, GridCellImpl)> {
        let col = &self.mIndexedColums[col_index];
        let mut auto: Option<(usize, GridCellImpl)> = None;
        let mut auto_width = i32::MIN;
        let mut fix: Option<(usize, GridCellImpl)> = None;
        let mut fix_width = i32::MIN;
        for (row_index, cell) in col.mCells.iter().enumerate() {
            let Some(cell) = cell else { continue };
            if self.span_count(col_index, row_index) != 1 {
                continue;
            }
            let width = table.cell_width_at(row_index, cell.cell_index as usize);
            if table.cell_auto_at(row_index, cell.cell_index as usize) {
                if auto.is_none() || width > auto_width {
                    auto_width = width;
                    auto = Some((row_index, cell.clone()));
                }
            } else if fix.is_none() || width > fix_width {
                fix_width = width;
                fix = Some((row_index, cell.clone()));
            }
        }
        fix.or(auto)
    }

    fn first_not_auto(&self, table: &TableInfoNode, col_index: usize) -> Option<(usize, usize)> {
        let col = &self.mIndexedColums[col_index];
        for (row_index, cell) in col.mCells.iter().enumerate() {
            let Some(cell) = cell else { continue };
            if !table.cell_auto_at(row_index, cell.cell_index as usize) {
                return Some((row_index, col_index));
            }
        }
        None
    }

    fn first_cell_at_span_end(&self, col_index: usize) -> Option<(usize, GridCellImpl)> {
        let col = &self.mIndexedColums[col_index];
        for (row_index, cell) in col.mCells.iter().enumerate() {
            let Some(cell) = cell.clone() else { continue };
            let count = self.span_count(col_index, row_index);
            if self.span_index(col_index, row_index) == count - 1 {
                return Some((row_index, cell));
            }
        }
        None
    }

    fn first_cell_at_span_begin(&self, col_index: usize) -> Option<(usize, GridCellImpl)> {
        let col = &self.mIndexedColums[col_index];
        for (row_index, cell) in col.mCells.iter().enumerate() {
            let Some(cell) = cell.clone() else { continue };
            if self.span_index(col_index, row_index) == 0 {
                return Some((row_index, cell));
            }
        }
        None
    }

    fn span_index(&self, col_index: usize, row_index: usize) -> usize {
        let mut index = 0usize;
        let mut cursor = self.mIndexedColums[col_index].getCell(row_index).and_then(|c| c.pre_col);
        while let Some(prev) = cursor {
            index += 1;
            cursor = self.mIndexedColums[prev].getCell(row_index).and_then(|c| c.pre_col);
        }
        index
    }

    fn span_count(&self, col_index: usize, row_index: usize) -> usize {
        self.span_index(col_index, row_index)
            + 1
            + {
                let mut count = 0usize;
                let mut cursor = self.mIndexedColums[col_index]
                    .getCell(row_index)
                    .and_then(|c| c.next_col);
                while let Some(next) = cursor {
                    count += 1;
                    cursor = self.mIndexedColums[next].getCell(row_index).and_then(|c| c.next_col);
                }
                count
            }
    }

    fn first_span_col(&self, col_index: usize, row_index: usize) -> Option<usize> {
        let mut cursor = col_index;
        while let Some(prev) = self.mIndexedColums[cursor].getCell(row_index).and_then(|c| c.pre_col)
        {
            cursor = prev;
        }
        Some(cursor)
    }

    fn pre_col_index(&self, col_index: usize) -> Option<usize> {
        if col_index > 0 { Some(col_index - 1) } else { None }
    }

    // Java: calcOriOffsetForSpandCell — sum of earlier span columns' ori widths
    fn ori_offset_for_span(&self, _table: &TableInfoNode, col_index: usize, row_index: usize) -> i32 {
        let mut offset = 0i32;
        let mut cursor = self.mIndexedColums[col_index].getCell(row_index).and_then(|c| c.pre_col);
        while let Some(prev) = cursor {
            offset += self.mIndexedColums[prev].mOriWidth;
            cursor = self.mIndexedColums[prev].getCell(row_index).and_then(|c| c.pre_col);
        }
        offset
    }

    // Java: calcoriOffsetForTable
    fn ori_offset_for_table(&self, col_index: usize) -> i32 {
        let mut offset = 0i32;
        for col in self.mIndexedColums.iter().take(col_index) {
            offset += col.mOriWidth;
        }
        offset
    }

    // Java: private boolean invalidCellOriginWidths(TableInfo tableInfo)
    fn invalidCellOriginWidths(&mut self, table: &mut TableInfoNode) -> bool {
        let text = table.is_text_document();
        let level = table.getLevel();
        if (text || level > 1) && table.is_percent_width() {
            table.clear_cell_widths();
            true
        } else {
            false
        }
    }

    pub fn columns(&self) -> &[GridColImpl] {
        &self.mIndexedColums
    }

    // Java: public void calColFitInfo()
    pub fn calColFitInfo(&mut self, table: &mut TableInfoNode) {
        // Java: this.valid();
        self.valid(table);
        self.mCacheSpanedCell.clear();
        let col_size = self.mIndexedColums.len();
        for col_index in 0..col_size {
            // Java: colData.clearFitInfo();
            self.mIndexedColums[col_index].clearFitInfo();
            let row_count = self.mIndexedColums[col_index].mCells.len();
            for row_index in 0..row_count {
                let Some(cell_index) = self.mIndexedColums[col_index]
                    .getCell(row_index)
                    .map(|cell| cell.cell_index as usize)
                else {
                    continue;
                };
                let span_count = self.span_count(col_index, row_index);
                let span_index = self.span_index(col_index, row_index);
                if span_count == 1 {
                    // Java: calcCellFitInfo; fitCellInfoToColInfo
                    self.calcCellFitInfo(table, row_index, cell_index);
                    self.fitCellInfoToColInfo(table, row_index, cell_index, col_index);
                } else if span_index == 0 {
                    self.calcCellFitInfo(table, row_index, cell_index);
                    let margin_left = table
                        .cell_mut(row_index, cell_index)
                        .map(|cell| cell.margin_left())
                        .unwrap_or(0);
                    // Java: if (cellData.getMarginLeft() > colData.mMinWidth) colData.mMinWidth = marginLeft;
                    if margin_left > self.mIndexedColums[col_index].mMinWidth {
                        self.mIndexedColums[col_index].mMinWidth = margin_left;
                    }
                    // Java: this.mCacheSpanedCell.add(gridCell);
                    self.mCacheSpanedCell.push((col_index, row_index));
                } else if span_index + 1 == span_count {
                    let margin_right = table
                        .cell_mut(row_index, cell_index)
                        .map(|cell| cell.margin_right())
                        .unwrap_or(0);
                    // Java: if (cellData.getMarginRight() > colData.mMinWidth) colData.mMinWidth = marginRight;
                    if margin_right > self.mIndexedColums[col_index].mMinWidth {
                        self.mIndexedColums[col_index].mMinWidth = margin_right;
                    }
                }
            }
        }
        // Java: this.adjustAutoColForOriWidth(); uses mSpanedCells from valid(), then clears it
        self.adjustAutoColForOriWidth(table);
        let spanned = std::mem::take(&mut self.mCacheSpanedCell);
        for (col_index, row_index) in spanned {
            let Some(cell_index) = self.mIndexedColums[col_index]
                .getCell(row_index)
                .map(|cell| cell.cell_index as usize)
            else {
                continue;
            };
            let span = self.span_count(col_index, row_index);
            let (max_width, para_max) = {
                let Some(cell) = table.cell_mut(row_index, cell_index) else {
                    continue;
                };
                let max_width = cell.getMaxWidth();
                // Java: cellInfo.isAutoWidth() ? cellData.getParaTotalMax() : 0
                let para_max = if cell.isAutoWidth() {
                    cell.getParaTotalMax()
                } else {
                    0
                };
                (max_width, para_max)
            };
            // Java: splitWidthToChildColumn(..., col.index(), col.index() + spanCount - 1)
            self.splitWidthToChildColumn(max_width, para_max, col_index, col_index + span - 1);
        }
    }

    // Java: private void calcCellFitInfo(CellInfo cellInfo)
    fn calcCellFitInfo(&mut self, table: &mut TableInfoNode, row_index: usize, cell_index: usize) {
        if let Some(cell) = table.cell_mut(row_index, cell_index) {
            cell.clear_child_fit();
        }
        let child_count = table.nested_table_len(row_index, cell_index);
        let is_auto = table
            .cell_mut(row_index, cell_index)
            .is_some_and(|cell| cell.isAutoWidth());
        for nested_index in 0..child_count {
            // Java: childTable.getGrid().getAutoSize().calColFitInfo(); indent when jc == 0
            let Some((min_width, best_width, para_width, indent)) =
                table.nested_auto_fit(row_index, cell_index, nested_index)
            else {
                continue;
            };
            let para = if is_auto { para_width + indent } else { 0 };
            if let Some(cell) = table.cell_mut(row_index, cell_index) {
                // Java: autoFitInfo.commitNewChild(min + indent, best + indent, para)
                cell.getAutoFitInfo()
                    .commitNewChild(min_width + indent, best_width + indent, para);
            }
        }
    }

    // Java: private void fitCellInfoToColInfo(CellInfo cellInfo, Data.GridColData colData)
    fn fitCellInfoToColInfo(
        &mut self,
        table: &mut TableInfoNode,
        row_index: usize,
        cell_index: usize,
        col_index: usize,
    ) {
        let Some(cell) = table.cell_mut(row_index, cell_index) else {
            return;
        };
        let min = cell.getMinWidth();
        let max = cell.getMaxWidth();
        let para = cell.getParaTotalMax();
        let auto = cell.isAutoWidth();
        let col = &mut self.mIndexedColums[col_index];
        // Java: if (min > colData.mMinWidth) colData.mMinWidth = min;
        if min > col.mMinWidth {
            col.mMinWidth = min;
        }
        // Java: if (max > colData.mBestWidth) colData.mBestWidth = max;
        if max > col.mBestWidth {
            col.mBestWidth = max;
        }
        // Java: if (cellData.isAutoWidth() || this.mForceFit)
        if auto || self.mForceFit {
            if para > col.mParaTotalMax {
                col.mParaTotalMax = para;
            }
        }
    }

    // Java: private void splitWidthToChildColumn(int bestWidth, int paraMax, int startIndex, int endIndex)
    fn splitWidthToChildColumn(&mut self, best_width: i32, para_max: i32, start_index: usize, end_index: usize) {
        let mut total_ori = 0i32;
        let mut total_best = 0i32;
        let mut total_para = 0i32;
        let mut any_best_zero = false;
        let mut any_para_zero = false;
        for index in start_index..=end_index {
            let col = &self.mIndexedColums[index];
            total_ori += col.mOriWidth;
            any_best_zero |= col.mBestWidth == 0;
            total_best += col.mBestWidth;
            if para_max > 0 {
                total_para += col.mParaTotalMax;
                any_para_zero |= col.mParaTotalMax == 0;
            }
        }
        if best_width - total_best > 0 {
            if any_best_zero {
                let mut leave_best = best_width;
                let mut leave_ori = total_ori;
                for index in start_index..=end_index {
                    let col = &self.mIndexedColums[index];
                    let split = (best_width as f32 * col.mOriWidth as f32 / total_ori as f32) as i32;
                    if split < col.mBestWidth {
                        leave_best -= col.mBestWidth;
                        leave_ori -= col.mOriWidth;
                    }
                }
                if leave_best > 0 {
                    for index in start_index..=end_index {
                        let split = (best_width as f32
                            * self.mIndexedColums[index].mOriWidth as f32
                            / total_ori as f32) as i32;
                        if split > self.mIndexedColums[index].mBestWidth {
                            let ori = self.mIndexedColums[index].mOriWidth;
                            self.mIndexedColums[index].mBestWidth =
                                (leave_best as f32 * ori as f32 / leave_ori as f32) as i32;
                        }
                    }
                }
            } else {
                let mut leave_total_best = total_best as f32;
                for index in start_index..=end_index {
                    let col = &self.mIndexedColums[index];
                    let split = best_width as f32 * col.mBestWidth as f32 / total_best as f32;
                    if split < col.mBestWidth as f32 {
                        leave_total_best -= col.mBestWidth as f32;
                    }
                }
                if leave_total_best > 0.0 {
                    let leave_best = best_width as f32 - (total_best as f32 - leave_total_best);
                    for index in start_index..=end_index {
                        let best = self.mIndexedColums[index].mBestWidth;
                        let split = best_width as f32 * best as f32 / total_best as f32;
                        if split > best as f32 {
                            self.mIndexedColums[index].mBestWidth =
                                (leave_best * best as f32 / leave_total_best) as i32;
                        }
                    }
                }
            }
        }
        if para_max > 0 && para_max - total_para > 0 {
            if any_para_zero {
                let mut leave_para = para_max;
                let mut leave_ori = total_ori;
                for index in start_index..=end_index {
                    let col = &self.mIndexedColums[index];
                    if col.mIsAutoWidth {
                        let split = para_max * col.mOriWidth / total_ori;
                        if split < col.mParaTotalMax {
                            leave_para -= col.mParaTotalMax;
                            leave_ori -= col.mOriWidth;
                        }
                    }
                }
                if leave_para > 0 {
                    for index in start_index..=end_index {
                        let auto = self.mIndexedColums[index].mIsAutoWidth;
                        let ori = self.mIndexedColums[index].mOriWidth;
                        let para = self.mIndexedColums[index].mParaTotalMax;
                        if auto {
                            let split = para_max * ori / total_ori;
                            if split > para {
                                self.mIndexedColums[index].mParaTotalMax =
                                    leave_para * ori / leave_ori;
                            }
                        }
                    }
                }
            } else {
                let mut leave_total_para = total_para;
                for index in start_index..=end_index {
                    let col = &self.mIndexedColums[index];
                    if col.mIsAutoWidth {
                        let split = para_max * col.mParaTotalMax / total_para;
                        if split < col.mParaTotalMax {
                            leave_total_para -= col.mParaTotalMax;
                        }
                    }
                }
                if leave_total_para > 0 {
                    let leave_para = para_max - (total_para - leave_total_para);
                    for index in start_index..=end_index {
                        let auto = self.mIndexedColums[index].mIsAutoWidth;
                        let para = self.mIndexedColums[index].mParaTotalMax;
                        if auto {
                            let split = para_max * para / total_para;
                            if split > para {
                                self.mIndexedColums[index].mParaTotalMax =
                                    leave_para * para / leave_total_para;
                            }
                        }
                    }
                }
            }
        }
    }

    // Java: public int getTotalMinWidth()
    pub fn getTotalMinWidth(&self) -> i32 {
        let mut min_width = 0;
        for col in &self.mIndexedColums {
            min_width += col.mMinWidth;
        }
        min_width
    }

    // Java: public int getTotalBestWidth()
    pub fn getTotalBestWidth(&self, table: &mut TableInfoNode) -> i32 {
        let mut best_width = 0;
        for col in &self.mIndexedColums {
            best_width += col.mBestWidth;
        }
        // Java: if (this.mTableInfo.getLevel() > 1) bestWidth = max(bestWidth, getFixedTableWidth)
        if table.getLevel() > 1 {
            let fix = table.fixed_table_width();
            if fix > best_width {
                best_width = fix;
            }
        }
        best_width
    }

    // Java: public int getParaTotalMax()
    pub fn getParaTotalMax(&self, table: &mut TableInfoNode) -> i32 {
        let mut para_total = 0;
        for col in &self.mIndexedColums {
            // Java: mParaTotalMax == 0 contributes mBestWidth, otherwise mParaTotalMax
            if col.mParaTotalMax == 0 {
                para_total += col.mBestWidth;
            } else {
                para_total += col.mParaTotalMax;
            }
        }
        if table.getLevel() > 1 {
            let fix = table.fixed_table_width();
            if fix > para_total {
                para_total = fix;
            }
        }
        para_total
    }

    // Java: public int getTotalGridWidth()
    pub fn getTotalGridWidth(&self) -> i32 {
        let mut grid_width = 0;
        for col in &self.mIndexedColums {
            // Java: gridWidth += data.getWidth();
            grid_width += col.getWidth();
        }
        grid_width
    }

    // Java: public int getTotalOriWidth()
    pub fn getTotalOriWidth(&self) -> i32 {
        let mut ori_width = 0;
        for col in &self.mIndexedColums {
            ori_width += col.mOriWidth;
        }
        ori_width
    }

    // Java: public int getTotalFitWidth()
    pub fn getTotalFitWidth(&self) -> i32 {
        let mut fit_width = 0;
        for col in &self.mIndexedColums {
            fit_width += col.mFitWidth;
        }
        fit_width
    }

    // Java: public boolean isForceAdapted()
    pub fn isForceAdapted(&self) -> bool {
        self.mForceFit
    }

    // Java: public synchronized boolean autoFitSize(int typoCoreWidth)
    pub fn autoFitSize(&mut self, table: &mut TableInfoNode, mut typo_core_width: i32) -> bool {
        // Java: if (this.mTableInfo.getLevel() != 1) return false;
        if table.getLevel() != 1 {
            return false;
        }
        // Java: this.mForceFit = typoCoreWidth > 0;
        self.mForceFit = typo_core_width > 0;
        // Java: this.calColFitInfo();
        self.calColFitInfo(table);
        // Java: if (typoCoreWidth == -1) typoCoreWidth = TableTool.getTableFitMaxWidth(...)
        if typo_core_width == -1 {
            typo_core_width = table.fit_max_core_width();
        }
        // Java: this.doFit(typoCoreWidth);
        self.doFit(table, typo_core_width);
        // Java: if (this.checkFitChanged()) { this.applyAutoFitChange(); return true; }
        if self.checkFitChanged(table) {
            self.applyAutoFitChange(table);
            return true;
        }
        false
    }

    // Java: public void doFit(int typoCoreWidth)
    pub fn doFit(&mut self, table: &mut TableInfoNode, typo_core_width: i32) {
        self.doFitCurrentGrid(table, typo_core_width);
        self.doFitChildGrid(table);
    }

    // Java: private boolean isGrowAutoFit()
    fn isGrowAutoFit(&self, table: &TableInfoNode) -> bool {
        let Some(document) = table.getDocument() else {
            return false;
        };
        // Java: if (textDocument.getCompatibilityMode().isBefore2013Mode())
        if !Tools::compatibility_mode(document).isBefore2013Mode() {
            return false;
        }
        // Java: growAutoFit = textDocument.getProperty().getBoolean(524, false);
        let mut grow_auto_fit = Tools::document_property_boolean(document, 524, false);
        // Java: growAutoFit |= this.mTableInfo.getPos() != null;
        let mut table_view = table.clone();
        grow_auto_fit |= table_view.getPos().is_some();
        grow_auto_fit
    }

    // Java: private TableWidthType getTableWidthType()
    fn getTableWidthType(&mut self, table: &TableInfoNode) -> TableWidthType {
        self.mTableWidthType.update(self.mForceFit, table);
        self.mTableWidthType
    }

    // Java: public boolean checkFitChanged()
    pub fn checkFitChanged(&mut self, table: &mut TableInfoNode) -> bool {
        for col in &self.mIndexedColums {
            if col.checkFitChanged() {
                return true;
            }
        }
        let row_count = table.row_len();
        for row_index in 0..row_count {
            let cell_count = table.cell_len(row_index);
            for cell_index in 0..cell_count {
                let child_count = table.nested_table_len(row_index, cell_index);
                for nested_index in 0..child_count {
                    // Java: cell.getTableByIndex(i).getGrid().getAutoSize().checkFitChanged()
                    if table.nested_check_fit_changed(row_index, cell_index, nested_index) {
                        return true;
                    }
                }
            }
        }
        false
    }

    // Java: public void applyAutoFitChange()
    pub fn applyAutoFitChange(&mut self, table: &mut TableInfoNode) {
        let grid_cell_count = self.mIndexedColums.len();
        let mut col_pos = 0;
        for i in 0..grid_cell_count {
            if i == 0 {
                col_pos = self.mIndexedColums[i].getLeftPos();
            }
            self.mIndexedColums[i].applyAutoFitChange(col_pos);
            col_pos = self.mIndexedColums[i].getRightPos();
        }
        let row_count = table.row_len();
        for row_index in 0..row_count {
            self.write_row_rgdxa(table, row_index, grid_cell_count);
            let cell_count = table.cell_len(row_index);
            for cell_index in 0..cell_count {
                let child_count = table.nested_table_len(row_index, cell_index);
                for nested_index in 0..child_count {
                    // Java: child.getGrid().getAutoSize().applyAutoFitChange()
                    table.nested_apply_auto_fit(row_index, cell_index, nested_index);
                }
            }
        }
    }

    // Java: rewrite row.getData().mRgdxaCenter from column left/right and span skips
    fn write_row_rgdxa(&self, table: &mut TableInfoNode, row_index: usize, grid_cell_count: usize) {
        let Some(rgdxa_center) = table.row_rgdxa_mut(row_index) else {
            return;
        };
        let len = rgdxa_center.len();
        let mut rgdxa_index = 0usize;
        let mut cell_index = 0isize;
        let grid_cell_count = grid_cell_count as isize;
        while cell_index <= grid_cell_count {
            if cell_index == grid_cell_count {
                if rgdxa_index < len {
                    rgdxa_center[rgdxa_index] =
                        self.mIndexedColums[cell_index as usize - 1].getRightPos();
                }
            } else {
                let col = cell_index as usize;
                if self.mIndexedColums[col].getCell(row_index).is_some() {
                    if rgdxa_index < len {
                        rgdxa_center[rgdxa_index] = self.mIndexedColums[col].getLeftPos();
                    }
                    rgdxa_index += 1;
                    // Java: cellIndex += gridCell.getSpanCount() - 1; the for-loop then adds 1
                    cell_index += self.span_count(col, row_index) as isize - 1;
                } else if rgdxa_index > 0 && rgdxa_index < len {
                    rgdxa_center[rgdxa_index] = self.mIndexedColums[col - 1].getRightPos();
                    break;
                }
            }
            cell_index += 1;
        }
    }

    // Java: private void doFitCurrentGrid(int typoCoreWidth)
    fn doFitCurrentGrid(&mut self, table: &mut TableInfoNode, mut typo_core_width: i32) {
        let mut add_needed = 0i32;
        let mut best_total = 0i32;
        let mut normal_reduce_allow = 0i32;
        let mut auto_col_add_needed_total = 0i32;
        for col in &self.mIndexedColums {
            let add = col.getBestAdd();
            if add >= 0 {
                add_needed += add;
            } else {
                normal_reduce_allow += -add;
            }
            best_total += col.mBestWidth;
            auto_col_add_needed_total += col.getParaMaxAddNeed();
        }
        let total_ori = self.getTotalOriWidth();
        let total_grid_width = self.getTotalGridWidth();
        let overflow_width = total_grid_width - typo_core_width;
        let mut layout_leave = typo_core_width - total_ori;
        let is_in_text_box = table.is_text_document();
        let grow_auto_fit = self.isGrowAutoFit(table) && table.getLevel() == 1;
        let width_type = self.getTableWidthType(table);
        if overflow_width > 40 {
            if table.getLevel() > 1 {
                let reduce_need = add_needed + -layout_leave;
                if reduce_need <= normal_reduce_allow {
                    self.fit_reduceSpaceColumn(normal_reduce_allow, reduce_need);
                    self.fit_adjustForAutoCols(
                        width_type,
                        auto_col_add_needed_total,
                        normal_reduce_allow - reduce_need,
                        layout_leave,
                    );
                } else {
                    self.fit_fourceReduceAllColumns(best_total, typo_core_width);
                }
            } else if best_total <= total_ori
                || best_total >= total_grid_width
                || !width_type.allowOverFlow() && !grow_auto_fit
            {
                if width_type.allowOverFlow() {
                    typo_core_width = total_grid_width;
                    layout_leave = total_grid_width - total_ori;
                }
                if layout_leave >= add_needed {
                    self.fit_expandTableWidth();
                    self.fit_adjustForAutoCols(
                        width_type,
                        auto_col_add_needed_total,
                        normal_reduce_allow,
                        layout_leave - add_needed,
                    );
                } else if add_needed - layout_leave < normal_reduce_allow {
                    self.fit_reduceSpaceColumn(normal_reduce_allow, add_needed - layout_leave);
                    self.fit_adjustForAutoCols(
                        width_type,
                        auto_col_add_needed_total,
                        normal_reduce_allow - add_needed,
                        layout_leave,
                    );
                } else if !grow_auto_fit
                    && !width_type.allowOverFlow()
                    && !is_in_text_box
                    && !self.mForceFit
                {
                    self.fit_fourceReduceAllColumns(best_total, typo_core_width);
                } else {
                    self.fit_gaveBestWidth();
                }
            } else {
                self.fit_gaveBestWidth();
                self.fit_adjustForAutoCols(
                    width_type,
                    auto_col_add_needed_total,
                    normal_reduce_allow - add_needed,
                    layout_leave,
                );
            }
        } else if layout_leave <= 0 {
            let reduce_need = add_needed - layout_leave;
            if table.getLevel() > 1 {
                if reduce_need <= normal_reduce_allow {
                    self.fit_reduceSpaceColumn(normal_reduce_allow, reduce_need);
                } else {
                    self.fit_fourceReduceAllColumns(best_total, typo_core_width);
                }
            } else if reduce_need <= normal_reduce_allow {
                self.fit_reduceSpaceColumn(normal_reduce_allow, reduce_need);
            } else if !is_in_text_box && !grow_auto_fit {
                self.fit_fourceReduceAllColumns(best_total, typo_core_width);
            } else {
                self.fit_gaveBestWidth();
            }
        } else if is_in_text_box && table.getLevel() == 1 {
            if add_needed <= normal_reduce_allow {
                self.fit_reduceSpaceColumn(normal_reduce_allow, add_needed);
            } else {
                self.fit_gaveBestWidth();
            }
            self.fit_adjustForAutoCols(
                width_type,
                auto_col_add_needed_total,
                normal_reduce_allow - add_needed,
                layout_leave,
            );
        } else if width_type.reduceSpaceFirst() {
            if add_needed <= normal_reduce_allow {
                self.fit_reduceSpaceColumn(normal_reduce_allow, add_needed);
                self.fit_adjustForAutoCols(
                    width_type,
                    auto_col_add_needed_total,
                    normal_reduce_allow - add_needed,
                    layout_leave,
                );
            } else if layout_leave <= add_needed - normal_reduce_allow && !grow_auto_fit {
                self.fit_fourceReduceAllColumns(best_total, typo_core_width);
            } else {
                self.fit_gaveBestWidth();
                self.fit_adjustForAutoCols(
                    width_type,
                    auto_col_add_needed_total,
                    normal_reduce_allow - add_needed,
                    layout_leave,
                );
            }
        } else if layout_leave >= add_needed {
            self.fit_expandTableWidth();
            self.fit_adjustForAutoCols(
                width_type,
                auto_col_add_needed_total,
                -add_needed,
                layout_leave,
            );
        } else if add_needed - layout_leave < normal_reduce_allow {
            self.fit_reduceSpaceColumn(normal_reduce_allow, add_needed - layout_leave);
        } else if grow_auto_fit {
            self.fit_gaveBestWidth();
        } else {
            self.fit_fourceReduceAllColumns(best_total, typo_core_width);
        }
    }

    // Java: private void fit_adjustForAutoCols(...)
    fn fit_adjustForAutoCols(
        &mut self,
        width_type: TableWidthType,
        auto_col_add_needed_total: i32,
        mut cols_leave: i32,
        mut layout_leave: i32,
    ) {
        if auto_col_add_needed_total <= 0 {
            return;
        }
        if width_type.reduceSpaceFirst() {
            if cols_leave > 0 {
                cols_leave = 0;
                for col in &self.mIndexedColums {
                    if col.mIsAutoWidth {
                        let leave = (col.mFitWidth - col.mBestWidth) as f32;
                        if leave > 0.0 {
                            cols_leave = (cols_leave as f32 + leave) as i32;
                        }
                    }
                }
            }
            if cols_leave > 0 {
                for col in &mut self.mIndexedColums {
                    if col.mIsAutoWidth {
                        let auto_col_add_needed = col.getParaMaxAddNeed();
                        col.mFitWidth = col.mBestWidth
                            + cols_leave * auto_col_add_needed / auto_col_add_needed_total;
                    }
                }
            }
        } else {
            if cols_leave < 0 {
                layout_leave += cols_leave;
            }
            if layout_leave > 0 {
                let col_size = self.mIndexedColums.len();
                if layout_leave > auto_col_add_needed_total {
                    for i in 0..col_size {
                        if self.mIndexedColums[i].mIsAutoWidth {
                            let auto_col_add_needed = self.mIndexedColums[i].getParaMaxAddNeed();
                            let fit_to_best_least =
                                self.mIndexedColums[i].mFitWidth - self.mIndexedColums[i].mBestWidth;
                            if fit_to_best_least > 0 {
                                if auto_col_add_needed > fit_to_best_least {
                                    self.mIndexedColums[i].mFitWidth =
                                        self.mIndexedColums[i].mBestWidth + auto_col_add_needed;
                                }
                            } else {
                                self.mIndexedColums[i].mFitWidth += auto_col_add_needed;
                            }
                        }
                    }
                } else {
                    let mut leave_width = 0i32;
                    let mut leave_total_para_needed = 0i32;
                    let mut leave_cols = Vec::new();
                    for i in 0..col_size {
                        if !self.mIndexedColums[i].mIsAutoWidth {
                            continue;
                        }
                        let auto_col_add_needed = self.mIndexedColums[i].getParaMaxAddNeed();
                        let split_width =
                            layout_leave * auto_col_add_needed / auto_col_add_needed_total;
                        let fit_to_best_least =
                            self.mIndexedColums[i].mFitWidth - self.mIndexedColums[i].mBestWidth;
                        if fit_to_best_least > 0 {
                            let max_to = self.mIndexedColums[i].mOriWidth + split_width;
                            let needed_to = self.mIndexedColums[i].mBestWidth + auto_col_add_needed;
                            if needed_to > self.mIndexedColums[i].mFitWidth {
                                self.mIndexedColums[i].mFitWidth = std::cmp::min(max_to, needed_to);
                            }
                            leave_width += self.mIndexedColums[i].mOriWidth + split_width
                                - self.mIndexedColums[i].mFitWidth;
                        } else {
                            leave_cols.push(i);
                            leave_width += split_width;
                            leave_total_para_needed += auto_col_add_needed;
                        }
                    }
                    if !leave_cols.is_empty() && leave_total_para_needed > 0 {
                        for i in leave_cols {
                            let auto_col_add_needed = self.mIndexedColums[i].getParaMaxAddNeed();
                            let split_width =
                                leave_width * auto_col_add_needed / leave_total_para_needed;
                            self.mIndexedColums[i].mFitWidth +=
                                std::cmp::min(split_width, auto_col_add_needed);
                        }
                    }
                }
            }
        }
    }

    // Java: private void doFitChildGrid()
    fn doFitChildGrid(&mut self, table: &mut TableInfoNode) {
        let col_size = self.mIndexedColums.len();
        for col_index in 0..col_size {
            let cell_size = self.mIndexedColums[col_index].mCells.len();
            for row_index in 0..cell_size {
                let Some(cell_index) = self.mIndexedColums[col_index]
                    .getCell(row_index)
                    .map(|cell| cell.cell_index)
                else {
                    continue;
                };
                if cell_index < 0 {
                    continue;
                }
                let cell_index = cell_index as usize;
                let child_count = table.nested_table_len(row_index, cell_index);
                if child_count == 0 || self.span_index(col_index, row_index) != 0 {
                    continue;
                }
                let span_count = self.span_count(col_index, row_index);
                let mut cell_core_width = 0i32;
                if span_count <= 1 {
                    cell_core_width = self.mIndexedColums[col_index].mFitWidth;
                } else {
                    let mut cursor = Some(col_index);
                    while let Some(link) = cursor {
                        cell_core_width += self.mIndexedColums[link].mFitWidth;
                        cursor = self.mIndexedColums[link]
                            .getCell(row_index)
                            .and_then(|cell| cell.next_col);
                    }
                }
                let (margin_left, margin_right) = table
                    .cell_mut(row_index, cell_index)
                    .map(|cell| (cell.margin_left(), cell.margin_right()))
                    .unwrap_or((0, 0));
                cell_core_width -= margin_left + margin_right;
                for nested_index in 0..child_count {
                    // Java: ((GridAutoSize)gridCols).doFit(adjustTypeWidth), not autoFitSize
                    table.nested_do_fit(row_index, cell_index, nested_index, cell_core_width);
                }
            }
        }
    }

    // Java: private void fit_gaveBestWidth()
    fn fit_gaveBestWidth(&mut self) {
        for col in &mut self.mIndexedColums {
            col.mFitWidth = col.mBestWidth;
        }
    }

    // Java: private void fit_reserveOriWidth()
    fn fit_reserveOriWidth(&mut self) {
        for col in &mut self.mIndexedColums {
            col.mFitWidth = col.getWidth();
        }
    }

    // Java: private void fit_expandTableWidth()
    fn fit_expandTableWidth(&mut self) {
        for col in &mut self.mIndexedColums {
            if col.mBestWidth > col.mOriWidth {
                col.mFitWidth = col.mBestWidth;
            } else {
                col.mFitWidth = col.mOriWidth;
            }
        }
    }

    // Java: private void fit_reduceSpaceColumn(int normalReduceAllow, int reduceNeed)
    fn fit_reduceSpaceColumn(&mut self, normal_reduce_allow: i32, reduce_need: i32) {
        for col in &mut self.mIndexedColums {
            let add = col.getBestAdd();
            if add < 0 {
                col.mFitWidth = col.mOriWidth + add * reduce_need / normal_reduce_allow;
            } else {
                col.mFitWidth = col.mBestWidth;
            }
        }
    }

    // Java: private void fit_fourceReduceAllColumns(int tatalBestWidth, int maxTableWidth)
    fn fit_fourceReduceAllColumns(&mut self, mut total_best_width: i32, mut max_table_width: i32) {
        let mut min_width_total = 0i32;
        for col in &self.mIndexedColums {
            min_width_total += col.mMinWidth;
        }
        if min_width_total < total_best_width && min_width_total < max_table_width {
            total_best_width -= min_width_total;
            max_table_width -= min_width_total;
            for col in &mut self.mIndexedColums {
                col.mFitWidth = col.mMinWidth
                    + (max_table_width as f32 * (col.mBestWidth - col.mMinWidth) as f32
                        / total_best_width as f32) as i32;
            }
        } else {
            for col in &mut self.mIndexedColums {
                col.mFitWidth = col.mMinWidth;
            }
        }
    }
}

// Java: private class TableWidthType
#[derive(Clone, Copy, Default)]
struct TableWidthType {
    mAllowOverFlow: bool,
    mReduceSpaceFirst: bool,
}

impl TableWidthType {
    // Java: void update()
    fn update(&mut self, force_fit: bool, table: &TableInfoNode) {
        self.mAllowOverFlow = false;
        self.mReduceSpaceFirst = false;
        // Java: if (!GridColsImpl.this.mForceFit)
        if force_fit {
            return;
        }
        let Some(final_prop) = table.table_property() else {
            return;
        };
        // Java: if (finalProp.contains(295))
        if !final_prop.contains(295) {
            return;
        }
        let Some(table_length) = final_prop.getValue(295).and_then(|value| {
            value.as_any().downcast_ref::<TableLength>().cloned()
        }) else {
            return;
        };
        match table_length.getType() {
            // Java: case 0, 3, 19 fall through to break after setting both flags
            0 | 3 | 19 => {
                self.mReduceSpaceFirst = true;
                self.mAllowOverFlow = true;
            }
            // Java: case 2 sets reduceSpaceFirst, and allowOverFlow when value/5000 > 1
            2 => {
                self.mReduceSpaceFirst = true;
                let pct = table_length.getValue() as f32 / 5000.0;
                if pct > 1.0 {
                    self.mAllowOverFlow = true;
                }
            }
            _ => {}
        }
    }

    fn allowOverFlow(self) -> bool {
        self.mAllowOverFlow
    }

    fn reduceSpaceFirst(self) -> bool {
        self.mReduceSpaceFirst
    }
}
