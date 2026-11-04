// java package cn.wps.moss.engine.grid.griditerator;

use crate::java_runtime::RefCell;
use crate::java_runtime::Rc;

// java import cn.wps.moss.engine.grid.KmoMatrix;
use crate::cn::wps::moss::engine::grid::KmoMatrix::KmoMatrix;
// java import cn.wps.moss.util.KmoRect;
use crate::cn::wps::moss::util::KmoRect::KmoRect;

use super::KmoMatrixCellIterator::KmoMatrixCellIterator;

// java public final class KmoMatrixColFirstCellIterator implements KmoMatrixCellIterator
pub struct KmoMatrixColFirstCellIterator {
    matrix: Rc<RefCell<KmoMatrix>>,
    firstRow: i32,
    lastRow: i32,
    firstCol: i32,
    lastCol: i32,
    leftToRight: bool,
    topToBottom: bool,
    currentRow: i32,
    currentCol: i32,
    currentData: [u8; 7],
    nextRow: i32,
    nextCol: i32,
    nextData: [u8; 7],
}

impl KmoMatrixColFirstCellIterator {
    // java public KmoMatrixColFirstCellIterator(KmoMatrix matrix, KmoRect range)
    pub fn KmoMatrixColFirstCellIterator__2(matrix: Rc<RefCell<KmoMatrix>>, range: &KmoRect) -> Self {
        // java this(matrix, range, true, true);
        Self::KmoMatrixColFirstCellIterator(matrix, range, true, true)
    }

    // java public KmoMatrixColFirstCellIterator(KmoMatrix matrix, KmoRect range, boolean leftToRight, boolean topToBottom)
    pub fn KmoMatrixColFirstCellIterator(matrix: Rc<RefCell<KmoMatrix>>, range: &KmoRect, leftToRight: bool, topToBottom: bool) -> Self {
        // java this.currentData = new byte[7]; this.nextData = new byte[7];
        // java KmoRect usedRange = matrix.getUsedRange();
        let used = matrix.borrow().getUsedRange();
        let used = used.borrow();
        // java this.firstRow = Math.max(range.tl.row, usedRange.tl.row);
        let firstRow = range.tl.row.max(used.tl.row);
        // java this.lastRow = Math.min(range.br.row, usedRange.br.row);
        let lastRow = range.br.row.min(used.br.row);
        // java this.firstCol = Math.max(range.tl.col, usedRange.tl.col);
        let firstCol = range.tl.col.max(used.tl.col);
        // java this.lastCol = Math.min(range.br.col, usedRange.br.col);
        let lastCol = range.br.col.min(used.br.col);
        drop(used);
        // java this.leftToRight = leftToRight; this.topToBottom = topToBottom;
        let valid = 0 <= firstRow && firstRow <= lastRow && 0 <= firstCol && firstCol <= lastCol;
        // java this.nextRow = topToBottom ? this.firstRow - 1 : this.lastRow + 1;
        let nextRow = if topToBottom { firstRow - 1 } else { lastRow + 1 };
        // java this.nextCol = leftToRight ? this.firstCol : this.lastCol;
        let nextCol = if valid { if leftToRight { firstCol } else { lastCol } } else { lastCol + 1 };
        let mut iter = Self { matrix, firstRow, lastRow, firstCol, lastCol, leftToRight, topToBottom,
            currentRow: if valid { -1 } else { 0 }, currentCol: if valid { -1 } else { 0 },
            currentData: [0; 7], nextRow, nextCol, nextData: [0; 7] };
        // java if (valid) this.advance();
        if valid { iter.advance(); }
        iter
    }

    // java private void advance()
    fn advance(&mut self) {
        // java if (this.topToBottom) ++this.nextRow; else --this.nextRow;
        self.nextRow += if self.topToBottom { 1 } else { -1 };
        // java while (true)
        loop {
            // java if (this.nextRow < this.firstRow || this.nextRow > this.lastRow)
            if self.nextRow < self.firstRow || self.nextRow > self.lastRow {
                self.nextCol += if self.leftToRight { 1 } else { -1 };
                self.nextRow = if self.topToBottom { self.firstRow } else { self.lastRow };
            }
            // java if (this.nextCol < this.firstCol || this.nextCol > this.lastCol) break;
            if self.nextCol < self.firstCol || self.nextCol > self.lastCol { break; }
            // java int blockRowIndex = this.nextRow / 32;
            let blockRowIndex = self.nextRow / 32;
            let matrix = self.matrix.borrow();
            // java int blockRowSize = this.matrix.getRowList().size();
            let blockRowSize = matrix.getRowList().len() as i32;
            // java if (blockRowIndex >= blockRowSize)
            if blockRowIndex >= blockRowSize {
                if self.topToBottom {
                    self.nextCol += if self.leftToRight { 1 } else { -1 };
                    self.nextRow = self.firstRow;
                } else {
                    self.nextRow = blockRowSize * 32 - 1;
                }
            } else {
                // java KmoMatrix.KmoBlockRow br = this.matrix.getRow(blockRowIndex);
                let br = matrix.getRow(blockRowIndex);
                // java int blockColIndex = this.nextCol / 16;
                let blockColIndex = self.nextCol / 16;
                // java KmoMatrix.KmoBlock b = null; if (br != null && blockColIndex < br.getColLst().size()) b = br.getColBlock(blockColIndex);
                let block = br.and_then(|row| if blockColIndex < row.getColLst().len() as i32 { row.getColBlock(blockColIndex) } else { None });
                if let Some(block) = block {
                    // java b.get(this.nextRow % 32, this.nextCol % 16, this.nextData);
                    block.get(self.nextRow % 32, self.nextCol % 16, &mut self.nextData);
                    // java boolean nullnode = true; for (...) if (this.nextData[i] != 0) ...
                    if self.nextData.iter().any(|byte| *byte != 0) { break; }
                    // java this.nextRow += this.topToBottom ? 1 : -1;
                    self.nextRow += if self.topToBottom { 1 } else { -1 };
                } else {
                    // java this.nextRow = blockRowIndex * 32 + (this.topToBottom ? 32 : -1);
                    self.nextRow = blockRowIndex * 32 + if self.topToBottom { 32 } else { -1 };
                }
            }
        }
    }
}

impl KmoMatrixCellIterator for KmoMatrixColFirstCellIterator {
    // java public boolean hasNext()
    fn hasNext(&self) -> bool { self.firstCol <= self.nextCol && self.nextCol <= self.lastCol }
    // java public void next()
    fn next(&mut self) {
        // java if (!this.hasNext()) throw new IllegalStateException();
        if !self.hasNext() { panic!("IllegalStateException"); }
        // java this.currentRow = this.nextRow; this.currentCol = this.nextCol;
        self.currentRow = self.nextRow;
        self.currentCol = self.nextCol;
        // java byte[] tmp = this.currentData; this.currentData = this.nextData; this.nextData = tmp;
        std::mem::swap(&mut self.currentData, &mut self.nextData);
        // java this.advance();
        self.advance();
    }
    // java public int row()
    fn row(&self) -> i32 { self.currentRow }
    // java public int col()
    fn col(&self) -> i32 { self.currentCol }
    // java public byte[] data()
    fn data(&self) -> &[u8] { &self.currentData }
}
