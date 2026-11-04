// java package cn.wps.moss.engine.grid.griditerator;

use crate::java_runtime::RefCell;
use crate::java_runtime::Rc;

// java import cn.wps.moss.engine.grid.KmoMatrix;
use crate::cn::wps::moss::engine::grid::KmoMatrix::KmoMatrix;
// java import cn.wps.moss.util.KmoRect;
use crate::cn::wps::moss::util::KmoRect::KmoRect;

use super::KmoMatrixCellIterator::KmoMatrixCellIterator;

// java public final class KmoMatrixRowFirstCellIterator implements KmoMatrixCellIterator
pub struct KmoMatrixRowFirstCellIterator {
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

impl KmoMatrixRowFirstCellIterator {
    // java public KmoMatrixRowFirstCellIterator(KmoMatrix matrix, KmoRect range)
    pub fn KmoMatrixRowFirstCellIterator__2(matrix: Rc<RefCell<KmoMatrix>>, range: &KmoRect) -> Self {
        // java this(matrix, range, true, true);
        Self::KmoMatrixRowFirstCellIterator(matrix, range, true, true)
    }

    // java public KmoMatrixRowFirstCellIterator(KmoMatrix matrix, KmoRect range, boolean leftToRight, boolean topToBottom)
    pub fn KmoMatrixRowFirstCellIterator(matrix: Rc<RefCell<KmoMatrix>>, range: &KmoRect, leftToRight: bool, topToBottom: bool) -> Self {
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
        // java this.nextRow = topToBottom ? this.firstRow : this.lastRow;
        let nextRow = if valid { if topToBottom { firstRow } else { lastRow } } else { lastRow + 1 };
        // java this.nextCol = leftToRight ? this.firstCol - 1 : this.lastCol + 1;
        let nextCol = if leftToRight { firstCol - 1 } else { lastCol + 1 };
        let mut iter = Self { matrix, firstRow, lastRow, firstCol, lastCol, leftToRight, topToBottom,
            currentRow: if valid { -1 } else { 0 }, currentCol: if valid { -1 } else { 0 },
            currentData: [0; 7], nextRow, nextCol, nextData: [0; 7] };
        // java if (valid) this.advance();
        if valid { iter.advance(); }
        iter
    }

    // java private void advance()
    fn advance(&mut self) {
        // java if (this.leftToRight) ++this.nextCol; else --this.nextCol;
        self.nextCol += if self.leftToRight { 1 } else { -1 };
        // java while (true)
        loop {
            // java if (this.nextCol < this.firstCol || this.nextCol > this.lastCol)
            if self.nextCol < self.firstCol || self.nextCol > self.lastCol {
                self.nextRow += if self.topToBottom { 1 } else { -1 };
                self.nextCol = if self.leftToRight { self.firstCol } else { self.lastCol };
            }
            // java if (this.nextRow < this.firstRow || this.nextRow > this.lastRow) break;
            if self.nextRow < self.firstRow || self.nextRow > self.lastRow { break; }
            // java int blockRowIndex = this.nextRow / 32;
            let blockRowIndex = self.nextRow / 32;
            let matrix = self.matrix.borrow();
            // java if (blockRowIndex >= this.matrix.getRowList().size())
            if blockRowIndex >= matrix.getRowList().len() as i32 {
                if self.topToBottom {
                    self.nextRow = self.lastRow + 1;
                    break;
                }
                self.nextRow = matrix.getRowList().len() as i32 * 32 - 1;
                self.nextCol = if self.leftToRight { self.firstCol } else { self.lastCol };
            } else if let Some(br) = matrix.getRow(blockRowIndex) {
                // java int blockColIndex = this.nextCol / 16;
                let blockColIndex = self.nextCol / 16;
                // java int blockColSize = br.getColLst().size();
                let blockColSize = br.getColLst().len() as i32;
                // java if (blockColIndex >= blockColSize)
                if blockColIndex >= blockColSize {
                    if blockColSize * 16 <= self.firstCol {
                        self.nextRow = blockRowIndex * 32 + if self.topToBottom { 32 } else { -1 };
                        self.nextCol = if self.leftToRight { self.firstCol } else { self.lastCol };
                    } else if self.leftToRight {
                        self.nextRow += if self.topToBottom { 1 } else { -1 };
                        self.nextCol = self.firstCol;
                    } else {
                        self.nextCol = blockColSize * 16 - 1;
                    }
                } else if let Some(block) = br.getColBlock(blockColIndex) {
                    // java b.get(this.nextRow % 32, this.nextCol % 16, this.nextData);
                    block.get(self.nextRow % 32, self.nextCol % 16, &mut self.nextData);
                    // java boolean nullnode = true; for (...) if (this.nextData[i] != 0) ...
                    if self.nextData.iter().any(|byte| *byte != 0) { break; }
                    // java this.nextCol += this.leftToRight ? 1 : -1;
                    self.nextCol += if self.leftToRight { 1 } else { -1 };
                } else {
                    // java this.nextCol = blockColIndex * 16 + (this.leftToRight ? 16 : -1);
                    self.nextCol = blockColIndex * 16 + if self.leftToRight { 16 } else { -1 };
                }
            } else {
                // java this.nextRow = blockRowIndex * 32 + (this.topToBottom ? 32 : -1);
                self.nextRow = blockRowIndex * 32 + if self.topToBottom { 32 } else { -1 };
                // java this.nextCol = this.leftToRight ? this.firstCol : this.lastCol;
                self.nextCol = if self.leftToRight { self.firstCol } else { self.lastCol };
            }
        }
    }
}

impl KmoMatrixCellIterator for KmoMatrixRowFirstCellIterator {
    // java public boolean hasNext()
    fn hasNext(&self) -> bool { self.firstRow <= self.nextRow && self.nextRow <= self.lastRow }
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
