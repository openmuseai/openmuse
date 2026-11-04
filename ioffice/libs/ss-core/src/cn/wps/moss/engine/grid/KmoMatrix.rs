// java package cn.wps.moss.engine.grid;

// java import cn.wps.et.ss.calcchain.CalcChain;
// java import cn.wps.memor4j.KmoAtom;
// java import cn.wps.memor4j.KmoAtomData;
// java import cn.wps.moss.app.KmoSheet;
// java import cn.wps.moss.app.KmoTransaction;
// java import cn.wps.moss.engine.grid.griditerator.KmoMatrixCellIterator;
// java import cn.wps.moss.engine.grid.griditerator.KmoMatrixColFirstCellIterator;
// java import cn.wps.moss.engine.grid.griditerator.KmoMatrixRowFirstCellIterator;
// java import cn.wps.moss.util.KmoRect;
// java import cn.wps.moss.util.objpool.SharedObjects;
// java import java.util.ArrayList;
// java import java.util.Arrays;
// java import java.util.HashSet;
// java import java.util.List;
// java import java.util.Set;
// java import org.apache.poi.ss.SpreadsheetVersion;
use crate::java_runtime::{RefCell, Cell, Weak};
use crate::cn::wps::moss::app::{KmoSheet::KmoSheet, KmoBook::KmoBook, KmoTransaction::KmoSheetTransactionListener};
use kfc::cn::wps::memor4j::AtomSnapshot::{SharedAtom, KmoAtom as SnapshotAtom, KmoAtomData as SnapshotData};
use std::collections::HashSet;
use crate::java_runtime::Rc;
use std::sync::Mutex;

use kfc::cn::wps::memor4j::KmoAtom::KmoAtom;
use kfc::cn::wps::memor4j::KmoAtomData::{KmoAtomData, KmoAtomDataSub};

use crate::cn::wps::moss::util::KmoRect::KmoRect;
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

use super::KmoMatrixToolkit::KmoMatrixToolkit;
use super::KmoMergeDelegate::KmoMergeDelegate;
use super::griditerator::KmoMatrixCellIterator::KmoMatrixCellIterator;
use super::griditerator::KmoMatrixColFirstCellIterator::KmoMatrixColFirstCellIterator;
use super::griditerator::KmoMatrixRowFirstCellIterator::KmoMatrixRowFirstCellIterator;

// java public static final int BYTES_PER_NODE = 7;
pub const BYTES_PER_NODE: i32 = 7;

fn deferred_kmo_sheet() -> ! {
    todo!("DEFERRED_NON_FUNCTION");
}

fn arraycopy(src: &[u8], src_pos: i32, dst: &mut [u8], dst_pos: i32, len: i32) {
    if src_pos < 0 || dst_pos < 0 || len < 0 {
        panic!("IndexOutOfBoundsException");
    }
    let s = src_pos as usize;
    let d = dst_pos as usize;
    let n = len as usize;
    if s.checked_add(n).map(|end| end > src.len()).unwrap_or(true)
        || d.checked_add(n).map(|end| end > dst.len()).unwrap_or(true)
    {
        panic!("IndexOutOfBoundsException");
    }
    dst[d..d + n].copy_from_slice(&src[s..s + n]);
}

// java public class KmoMatrix extends KmoMatrixToolkit
pub struct KmoMatrix {
    // java private final int maxCol;
    maxCol: i32,
    // java private final int maxRow;
    maxRow: i32,
    // java private boolean isInCmdMode = false;
    isInCmdMode: Rc<Cell<bool>>,
    // java private KmoSheet sheet;
    sheet_is_null: bool,
    // java private KmoChangeRange kmoChangeRange;
    kmoChangeRange: Option<Rc<KmoChangeRange>>,
    // rust: shared handles for Java block -> sheet -> matrix field access while a matrix write guard is held.
    binding: Option<BlockBinding>,
    // java List<KmoBlockRow> rowLst = new ArrayList();
    rowLst: Vec<Option<KmoBlockRow>>,
    // java final KmoRect usedRange;
    usedRange: Rc<RefCell<KmoRect>>,
}

// java public class KmoMatrix extends KmoMatrixToolkit
impl KmoMatrix {
    // java public KmoMatrix copy(KmoMatrix matrix)
    pub fn copy(&mut self, matrix: &KmoMatrix) -> &mut Self {
        // java int i = 0;
        let mut i: i32 = 0;
        // java for(int size = matrix.rowLst.size(); i < size; ++i) {
        let size = matrix.rowLst.len() as i32;
        while i < size {
            // java KmoBlockRow blkRow = (KmoBlockRow)matrix.rowLst.get(i);
            // java if (blkRow == null) {
            if matrix.rowLst[i as usize].is_none() {
                // java this.rowLst.add(null);
                self.rowLst.push(None);
            } else {
                // java this.rowLst.add(new KmoBlockRow(blkRow));
                let blkRow = matrix.rowLst[i as usize].as_ref().unwrap();
                self.rowLst.push(Some(KmoBlockRow::KmoBlockRow__from(blkRow)));
            }
            i += 1;
        }
        // java return this;
        self
    }

    // java public KmoMatrix mergeCopy(int offset, int ignoreNum, KmoRect extUsedRange, KmoMatrix matrix)
    pub fn mergeCopy(
        &mut self,
        offset: i32,
        ignoreNum: i32,
        extUsedRange: &KmoRect,
        matrix: &KmoMatrix,
    ) -> &mut Self {
        // java byte[] tmp = new byte[7];
        let mut tmp = [0u8; 7];
        // java for(int r = ignoreNum; r < extUsedRange.br.row; ++r) {
        let mut r = ignoreNum;
        while r < extUsedRange.br.row {
            // java for(int c = extUsedRange.tl.col; c < extUsedRange.br.col; ++c) {
            let mut c = extUsedRange.tl.col;
            while c < extUsedRange.br.col {
                // java if (matrix.get(r, c, tmp)) {
                if matrix.get(r, c, &mut tmp) {
                    // java this.set(r + offset, c, tmp);
                    self.set(r + offset, c, &tmp);
                }
                c += 1;
            }
            r += 1;
        }
        // java return this;
        self
    }

    // java public KmoMatrix(KmoRect usedRange)
    pub fn KmoMatrix__rect(usedRange: KmoRect) -> Self {
        // java this.usedRange = usedRange;
        let usedRange = Rc::new(RefCell::new(usedRange));
        // java this.maxCol = SpreadsheetVersion.EXCEL2007.getMaxColumns();
        let maxCol = SpreadsheetVersion::EXCEL2007.getMaxColumns();
        // java this.maxRow = SpreadsheetVersion.EXCEL2007.getMaxRows();
        let maxRow = SpreadsheetVersion::EXCEL2007.getMaxRows();
        Self {
            maxCol,
            maxRow,
            isInCmdMode: Rc::new(Cell::new(false)),
            sheet_is_null: true,
            kmoChangeRange: None,
            binding: None,
            rowLst: Vec::new(),
            usedRange,
        }
    }

    // java public KmoMatrix(KmoSheet sheet, KmoRect usedRange, SpreadsheetVersion version)
    pub fn KmoMatrix__sheet(
        sheet_is_null: bool,
        usedRange: Rc<RefCell<KmoRect>>,
        version: SpreadsheetVersion,
    ) -> Self {
        // java this.sheet = sheet;
        let sheet_is_null = sheet_is_null;
        // java this.usedRange = usedRange;
        let usedRange = usedRange;
        // java this.maxCol = version.getMaxColumns();
        let maxCol = version.getMaxColumns();
        // java this.maxRow = version.getMaxRows();
        let maxRow = version.getMaxRows();
        // java this.kmoChangeRange = new KmoChangeRange(sheet);
        let isInCmdMode = Rc::new(Cell::new(false));
        let kmoChangeRange = Some(Rc::new(KmoChangeRange::KmoChangeRange(isInCmdMode.clone())));
        Self {
            maxCol,
            maxRow,
            isInCmdMode,
            sheet_is_null,
            kmoChangeRange,
            binding: None,
            rowLst: Vec::new(),
            usedRange,
        }
    }

    // java public int getMaxCol()
    pub fn getMaxCol(&self) -> i32 {
        // java return this.maxCol;
        self.maxCol
    }

    // java public int getMaxRow()
    pub fn getMaxRow(&self) -> i32 {
        // java return this.maxRow;
        self.maxRow
    }

    // java public void startCmdMode()
    pub fn startCmdMode(&mut self) {
        // java this.isInCmdMode = true;
        self.isInCmdMode.set(true);
    }

    // java public void endCmdMode()
    pub fn endCmdMode(&mut self) {
        // java this.isInCmdMode = false;
        self.isInCmdMode.set(false);
    }

    // java private void adjustUsedRange(int row, int col)
    fn adjustUsedRange(&mut self, mut row: i32, mut col: i32) {
        // java row = row < 0 ? 0 : (row > this.maxRow - 1 ? this.maxRow - 1 : row);
        row = if row < 0 {
            0
        } else if row > self.maxRow - 1 {
            self.maxRow - 1
        } else {
            row
        };
        // java col = col < 0 ? 0 : (col > this.maxCol - 1 ? this.maxCol - 1 : col);
        col = if col < 0 {
            0
        } else if col > self.maxCol - 1 {
            self.maxCol - 1
        } else {
            col
        };
        // java if (this.usedRange.br.row == 0) {
        if self.usedRange.borrow().br.row == 0 {
            // java this.usedRange.copy(row, col, row + 1, col + 1);
            self.usedRange
                .borrow_mut()
                .copy__bounds(row, col, row + 1, col + 1);
        } else {
            // java this.usedRange.combineEx(row, col);
            self.usedRange.borrow_mut().combineEx__rc(row, col);
        }
    }

    // java public void fitUsedRange(KmoMergeDelegate mergeMgr)
    pub fn fitUsedRange(&mut self, _mergeMgr: &KmoMergeDelegate) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.fitUsedRange(mergeMgr, this.usedRange);
        */
    }

    // java public void fitUsedRange(KmoMergeDelegate mergeMgr, KmoRect usedRange)
    pub fn fitUsedRange__rect(&mut self, _mergeMgr: &KmoMergeDelegate, _usedRange: &KmoRect) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoMatrixUsedrgFitter.fitUsedRange(this, usedRange);
        // java KmoMatrixUsedrgFitter.fitUsedRangeWithMergeCell(usedRange, mergeMgr);
        */
    }

    // java public byte[] initFill(int row, int col, byte[] node)
    pub fn initFill(&mut self, row: i32, col: i32, node: &[u8]) {
        // java this.usedRange.combineEx(row, col);
        self.usedRange.borrow_mut().combineEx__rc(row, col);
        // java KmoBlock blk = this.allocRow(row / 32).allocCol(col / 16, this.maxRow, this.maxCol);
        let maxRow = self.maxRow;
        let maxCol = self.maxCol;
        let blk = self.allocRow(row / 32).allocCol(col / 16, maxRow, maxCol);
        // java return blk.initFill(row % 32, col % 16, node);
        blk.initFill(row % 32, col % 16, node);
    }

    // java public byte[] set(int row, int col, byte[] node)
    pub fn set(&mut self, row: i32, col: i32, node: &[u8]) {
        // java if (!isEmptyByte(node)) {
        if !Self::isEmptyByte(node) {
            // java this.adjustUsedRange(row, col);
            self.adjustUsedRange(row, col);
        }
        // java int blkRow = row / 32;
        let blkRow = row / 32;
        // java int blkCol = col / 16;
        let blkCol = col / 16;
        // java KmoBlock blk = this.allocRow(blkRow).allocCol(blkCol, this.maxRow, this.maxCol);
        let maxRow = self.maxRow;
        let maxCol = self.maxCol;
        let cmd = self.isInCmdMode.get();
        // java int rowInBlk = row % 32;
        let rowInBlk = row % 32;
        // java int colInBlk = col % 16;
        let colInBlk = col % 16;
        let blk = self.allocRow(blkRow).allocCol(blkCol, maxRow, maxCol);
        // java return blk.set(rowInBlk, colInBlk, node, this.isInCmdMode);
        blk.set(rowInBlk, colInBlk, node, cmd);
    }

    // java public byte[] setCalcResult(int row, int col, byte[] node)
    pub fn setCalcResult(&mut self, row: i32, col: i32, node: &[u8]) {
        // java if (!isEmptyByte(node)) {
        if !Self::isEmptyByte(node) {
            // java this.adjustUsedRange(row, col);
            self.adjustUsedRange(row, col);
        }
        // java int blkRow = row / 32;
        let blkRow = row / 32;
        // java int blkCol = col / 16;
        let blkCol = col / 16;
        // java KmoBlock blk = this.allocRow(blkRow).allocCol(blkCol, this.maxRow, this.maxCol);
        let maxRow = self.maxRow;
        let maxCol = self.maxCol;
        // java int rowInBlk = row % 32;
        let rowInBlk = row % 32;
        // java int colInBlk = col % 16;
        let colInBlk = col % 16;
        let blk = self.allocRow(blkRow).allocCol(blkCol, maxRow, maxCol);
        // java return blk.setCalcResult(rowInBlk, colInBlk, node);
        blk.setCalcResult(rowInBlk, colInBlk, node);
    }

    // java public KmoRect getUsedRange()
    pub fn getUsedRange(&self) -> Rc<RefCell<KmoRect>> {
        // java return this.usedRange;
        Rc::clone(&self.usedRange)
    }

    // java public KmoMatrixCellIterator getCellDataIterator(KmoRect rang, boolean rowFirst, boolean leftToRight, boolean topToBottom)
    pub fn getCellDataIterator(matrix: &Rc<RefCell<Self>>, rang: &KmoRect, rowFirst: bool, leftToRight: bool, topToBottom: bool) -> Box<dyn KmoMatrixCellIterator> {
        // java return (KmoMatrixCellIterator)(rowFirst ? new KmoMatrixRowFirstCellIterator(this, rang, leftToRight, topToBottom) : new KmoMatrixColFirstCellIterator(this, rang, leftToRight, topToBottom));
        if rowFirst {
            Box::new(KmoMatrixRowFirstCellIterator::KmoMatrixRowFirstCellIterator(Rc::clone(matrix), rang, leftToRight, topToBottom))
        } else {
            Box::new(KmoMatrixColFirstCellIterator::KmoMatrixColFirstCellIterator(Rc::clone(matrix), rang, leftToRight, topToBottom))
        }
    }

    // java public boolean isEmptyNode(int row, int col)
    pub fn isEmptyNode(&self, row: i32, col: i32) -> bool {
        // java int blkRow = row / 32;
        let blkRow = row / 32;
        // java int blkCol = col / 16;
        let blkCol = col / 16;
        // java if (blkRow >= this.rowLst.size()) {
        if blkRow >= self.rowLst.len() as i32 {
            // java return true;
            return true;
        } else {
            // java KmoBlockRow r = (KmoBlockRow)this.rowLst.get(blkRow);
            let row_ref = self.rowLst.get(blkRow as usize).and_then(|x| x.as_ref());
            // java if (r != null && blkCol < r.colLst.size()) {
            if row_ref.is_some() && blkCol < row_ref.unwrap().colLst.len() as i32 {
                // java int rowInBlk = row % 32;
                let rowInBlk = row % 32;
                // java int colInBlk = col % 16;
                let colInBlk = col % 16;
                // java byte[] node = SharedObjects.cellDataBuf.borrowObject();
                let mut node = [0u8; 7];
                // java boolean var10;
                let var10: bool;
                // java KmoBlock blk = (KmoBlock)r.colLst.get(blkCol);
                let blk = row_ref.unwrap().colLst.get(blkCol as usize).and_then(|b| b.as_ref());
                // java if (blk != null) {
                if let Some(blk) = blk {
                    // java blk.get(rowInBlk, colInBlk, node);
                    blk.get(rowInBlk, colInBlk, &mut node);
                    // java if (isEmptyByte(node)) {
                    if Self::isEmptyByte(&node) {
                        // java var10 = true;
                        var10 = true;
                        // java return var10;
                        return var10;
                    }
                    // java var10 = false;
                    var10 = false;
                    // java return var10;
                    return var10;
                }
                // java var10 = true;
                var10 = true;
                // java node = SharedObjects.cellDataBuf.returnObject(node);
                let _ = node;
                // java return var10;
                return var10;
            } else {
                // java return true;
                return true;
            }
        }
    }

    // java public boolean isBlankCell(int row, int col)
    pub fn isBlankCell(&self, row: i32, col: i32) -> bool {
        // java int blkRow = row / 32;
        let blkRow = row / 32;
        // java int blkCol = col / 16;
        let blkCol = col / 16;
        // java if (blkRow >= this.rowLst.size()) {
        if blkRow >= self.rowLst.len() as i32 {
            // java return true;
            return true;
        } else {
            // java KmoBlockRow r = (KmoBlockRow)this.rowLst.get(blkRow);
            let row_ref = self.rowLst.get(blkRow as usize).and_then(|x| x.as_ref());
            // java if (r != null && blkCol < r.colLst.size()) {
            if row_ref.is_some() && blkCol < row_ref.unwrap().colLst.len() as i32 {
                // java KmoBlock blk = (KmoBlock)r.colLst.get(blkCol);
                let blk = row_ref.unwrap().colLst.get(blkCol as usize).and_then(|b| b.as_ref());
                // java if (blk == null) {
                if blk.is_none() {
                    // java return true;
                    return true;
                } else {
                    // java int rowInBlk = row % 32;
                    let rowInBlk = row % 32;
                    // java int colInBlk = col % 16;
                    let colInBlk = col % 16;
                    // java return blk.isBlankCell(rowInBlk, colInBlk);
                    return blk.unwrap().isBlankCell(rowInBlk, colInBlk);
                }
            } else {
                // java return true;
                return true;
            }
        }
    }

    // java public boolean isEmptyRowBlock(int row)
    pub fn isEmptyRowBlock(&self, row: i32) -> bool {
        // java int blkRow = row / 32;
        let blkRow = row / 32;
        // java return blkRow >= this.rowLst.size() || this.rowLst.get(blkRow) == null;
        blkRow >= self.rowLst.len() as i32
            || self.rowLst.get(blkRow as usize).map(|r| r.is_none()).unwrap_or(true)
    }

    // java public boolean isEmptyColBlock(int row, int col)
    pub fn isEmptyColBlock(&self, row: i32, col: i32) -> bool {
        // java int blkRow = row / 32;
        let blkRow = row / 32;
        // java int blkCol = col / 16;
        let blkCol = col / 16;
        // java if (blkRow >= this.rowLst.size()) {
        if blkRow >= self.rowLst.len() as i32 {
            // java return true;
            return true;
        } else {
            // java KmoBlockRow r = (KmoBlockRow)this.rowLst.get(blkRow);
            let r = self.rowLst.get(blkRow as usize).and_then(|x| x.as_ref());
            // java return r == null || blkCol >= r.colLst.size() || r.colLst.get(blkCol) == null;
            r.is_none()
                || blkCol >= r.unwrap().colLst.len() as i32
                || r.unwrap().colLst.get(blkCol as usize).map(|b| b.is_none()).unwrap_or(true)
        }
    }

    // java public boolean get(int row, int col, byte[] node)
    pub fn get(&self, row: i32, col: i32, node: &mut [u8]) -> bool {
        // java int blkRow = row >> 5;
        let blkRow = row >> 5;
        // java int blkCol = col >> 4;
        let blkCol = col >> 4;
        // java if (blkRow >= 0 && blkRow < this.rowLst.size()) {
        if blkRow >= 0 && blkRow < self.rowLst.len() as i32 {
            // java KmoBlockRow r = (KmoBlockRow)this.rowLst.get(blkRow);
            let r = self.rowLst[blkRow as usize].as_ref();
            // java if (r != null && blkCol < r.colLst.size()) {
            if r.is_some() && blkCol >= 0 && blkCol < r.unwrap().colLst.len() as i32 {
                // java KmoBlock blk = (KmoBlock)r.colLst.get(blkCol);
                let blk = r.unwrap().colLst[blkCol as usize].as_ref();
                // java if (blk == null) {
                if blk.is_none() {
                    // java return false;
                    return false;
                } else {
                    // java int rowInBlk = row % 32;
                    let rowInBlk = row % 32;
                    // java int colInBlk = col % 16;
                    let colInBlk = col % 16;
                    // java blk.get(rowInBlk, colInBlk, node);
                    blk.unwrap().get(rowInBlk, colInBlk, node);
                    // java return true;
                    return true;
                }
            } else {
                // java return false;
                return false;
            }
        } else {
            // java return false;
            return false;
        }
    }

    // java public int getId(int row, int col)
    pub fn getId(&self, row: i32, col: i32) -> i32 {
        // java int blkRow = row >> 5;
        let blkRow = row >> 5;
        // java int blkCol = col >> 4;
        let blkCol = col >> 4;
        // java if (blkRow < this.rowLst.size() && blkRow >= 0) {
        if blkRow < self.rowLst.len() as i32 && blkRow >= 0 {
            // java KmoBlockRow r = (KmoBlockRow)this.rowLst.get(blkRow);
            let r = self.rowLst[blkRow as usize].as_ref();
            // java if (r != null && blkCol < r.colLst.size() && blkCol >= 0) {
            if r.is_some() && blkCol < r.unwrap().colLst.len() as i32 && blkCol >= 0 {
                // java KmoBlock blk = (KmoBlock)r.colLst.get(blkCol);
                let blk = r.unwrap().colLst[blkCol as usize].as_ref();
                // java if (blk == null) {
                if blk.is_none() {
                    // java return -1;
                    return -1;
                } else {
                    // java int rowInBlk = row % 32;
                    let rowInBlk = row % 32;
                    // java int colInBlk = col % 16;
                    let colInBlk = col % 16;
                    // java return blk.getId(rowInBlk, colInBlk);
                    return blk.unwrap().getId(rowInBlk, colInBlk);
                }
            } else {
                // java return -1;
                return -1;
            }
        } else {
            // java return -1;
            return -1;
        }
    }

    // java public int getOffset(int row, int col)
    pub fn getOffset(&self, row: i32, col: i32) -> i32 {
        // java int rowInBlk = row % 32;
        let rowInBlk = row % 32;
        // java int colInBlk = col % 16;
        let colInBlk = col % 16;
        // java return ((rowInBlk << 4) + colInBlk) * 7;
        ((rowInBlk << 4) + colInBlk) * 7
    }

    // java public final KmoBlock getBlk(int row, int col)
    pub fn getBlk(&self, row: i32, col: i32) -> Option<Rc<KmoBlock>> {
        // java int blkRow = row >> 5;
        let blkRow = row >> 5;
        // java int blkCol = col >> 4;
        let blkCol = col >> 4;
        // java if (blkRow < this.rowLst.size() && blkRow >= 0) {
        if blkRow < self.rowLst.len() as i32 && blkRow >= 0 {
            // java KmoBlockRow r = (KmoBlockRow)this.rowLst.get(blkRow);
            let r = self.rowLst[blkRow as usize].as_ref();
            // java if (r != null && blkCol < r.colLst.size() && blkCol >= 0) {
            if r.is_some() && blkCol < r.unwrap().colLst.len() as i32 && blkCol >= 0 {
                // java KmoBlock blk = (KmoBlock)r.colLst.get(blkCol);
                // java return blk;
                return r.unwrap().colLst[blkCol as usize].clone();
            } else {
                // java return null;
                return None;
            }
        } else {
            // java return null;
            return None;
        }
    }

    // java public KmoBlockRow getRow(int row)
    pub fn getRow(&self, row: i32) -> Option<&KmoBlockRow> {
        // java return row < this.rowLst.size() && row >= 0 ? (KmoBlockRow)this.rowLst.get(row) : null;
        if row < self.rowLst.len() as i32 && row >= 0 {
            self.rowLst[row as usize].as_ref()
        } else {
            None
        }
    }

    // java public void moveCol(int srcCol, int desCol, int firstRow, int lastRow)
    pub fn moveCol(&mut self, srcCol: i32, desCol: i32, firstRow: i32, lastRow: i32) {
        // java int firstRowBlk = firstRow / 32;
        let firstRowBlk = firstRow / 32;
        // java int lastRowBlk = lastRow / 32;
        let lastRowBlk = lastRow / 32;
        // java int srcColBlkIndex = srcCol / 16;
        let srcColBlkIndex = srcCol / 16;
        // java int desColBlkIndex = desCol / 16;
        let desColBlkIndex = desCol / 16;
        // java int srcColInBlock = srcCol % 16;
        let srcColInBlock = srcCol % 16;
        // java int desColInBlock = desCol % 16;
        let desColInBlock = desCol % 16;
        // java byte[] nodes = SharedObjects.cellDataBuf.borrowObject();
        let mut nodes = [0u8; 7];
        // java byte[] emptyNodes = newBuff();
        let mut emptyNodes = KmoMatrixToolkit::newBuff();
        // java for(int i = firstRowBlk; i <= lastRowBlk && i < this.rowLst.size(); ++i) {
        let mut i = firstRowBlk;
        let mut nodes_is_empty = false;
        while i <= lastRowBlk && (i as usize) < self.rowLst.len() {
            // java KmoBlockRow rowData = (KmoBlockRow)this.rowLst.get(i);
            let row_present = self.rowLst[i as usize].is_some();
            // java if (rowData != null) {
            if row_present {
                // java KmoBlock srcColBlk = rowData.getColBlock(srcColBlkIndex);
                let src_exists = self.col_block_exists(i, srcColBlkIndex);
                // java KmoBlock desColBlk = rowData.getColBlock(desColBlkIndex);
                let des_exists = self.col_block_exists(i, desColBlkIndex);
                // java if (srcColBlk != null || desColBlk != null) {
                if src_exists || des_exists {
                    // java int firstRowInBlock = i == firstRowBlk ? firstRow % 32 : 0;
                    let firstRowInBlock = if i == firstRowBlk { firstRow % 32 } else { 0 };
                    // java int lastRowInBlock = i == lastRowBlk ? lastRow % 32 : 31;
                    let lastRowInBlock = if i == lastRowBlk { lastRow % 32 } else { 31 };
                    // java desColBlk = rowData.allocCol(desColBlkIndex, this.maxRow, this.maxCol);
                    self.alloc_col_existing(i, desColBlkIndex);
                    // java for(int j = firstRowInBlock; j <= lastRowInBlock; ++j) {
                    let mut j = firstRowInBlock;
                    while j <= lastRowInBlock {
                        // java if (srcColBlk != null) {
                        if src_exists {
                            // java srcColBlk.get(j, srcColInBlock, nodes);
                            let got = self.read_cell(i, srcColBlkIndex, j, srcColInBlock);
                            if nodes_is_empty {
                                emptyNodes = got;
                            } else {
                                nodes = got;
                            }
                        } else {
                            // java nodes = emptyNodes;
                            nodes_is_empty = true;
                        }
                        // java desColBlk.set(j, desColInBlock, nodes, this.isInCmdMode);
                        let payload = if nodes_is_empty { emptyNodes } else { nodes };
                        let cmd = self.isInCmdMode.get();
                        self.write_cell(i, desColBlkIndex, j, desColInBlock, &payload, cmd);
                        // java if (srcColBlk != null) {
                        if src_exists {
                            // java srcColBlk.set(j, srcColInBlock, emptyNodes, this.isInCmdMode);
                            let cmd = self.isInCmdMode.get();
                            let clearing = emptyNodes;
                            self.write_cell(i, srcColBlkIndex, j, srcColInBlock, &clearing, cmd);
                        }
                        j += 1;
                    }
                }
            }
            i += 1;
        }
        // java nodes = SharedObjects.cellDataBuf.returnObject(nodes);
        let _ = nodes;
    }

    // java public void clearCol(int col, int firstRow, int lastRow)
    pub fn clearCol(&mut self, col: i32, firstRow: i32, lastRow: i32) {
        // java int firstRowBlk = firstRow / 32;
        let firstRowBlk = firstRow / 32;
        // java int lastRowBlk = lastRow / 32;
        let lastRowBlk = lastRow / 32;
        // java byte[] empty_node = SharedObjects.cellDataBuf.borrowObject();
        let empty_node = [0u8; 7];
        // java for(int i = firstRowBlk; i <= lastRowBlk && i < this.rowLst.size(); ++i) {
        let mut i = firstRowBlk;
        while i <= lastRowBlk && (i as usize) < self.rowLst.len() {
            // java KmoBlockRow rowData = (KmoBlockRow)this.rowLst.get(i);
            // java if (rowData != null) {
            if self.rowLst[i as usize].is_some() {
                // java KmoBlock colBlk = rowData.getColBlock(col / 16);
                let col_blk = col / 16;
                // java if (colBlk != null) {
                if self.col_block_exists(i, col_blk) {
                    // java int firstRowInBlock = i == firstRowBlk ? firstRow % 32 : 0;
                    let firstRowInBlock = if i == firstRowBlk { firstRow % 32 } else { 0 };
                    // java int lastRowInBlock = i == lastRowBlk ? lastRow % 32 : 31;
                    let lastRowInBlock = if i == lastRowBlk { lastRow % 32 } else { 31 };
                    // java int colInBlock = col % 16;
                    let colInBlock = col % 16;
                    // java for(int j = firstRowInBlock; j <= lastRowInBlock; ++j) {
                    let mut j = firstRowInBlock;
                    while j <= lastRowInBlock {
                        // java colBlk.set(j, colInBlock, empty_node, this.isInCmdMode);
                        let cmd = self.isInCmdMode.get();
                        self.write_cell(i, col_blk, j, colInBlock, &empty_node, cmd);
                        j += 1;
                    }
                }
            }
            i += 1;
        }
        // java empty_node = SharedObjects.cellDataBuf.returnObject(empty_node);
        let _ = empty_node;
    }

    // java public int moveRow(int srcRow, int desRow, int firstCol, int lastCol, boolean isClearSrcData)
    pub fn moveRow(
        &mut self,
        srcRow: i32,
        desRow: i32,
        firstCol: i32,
        lastCol: i32,
        isClearSrcData: bool,
    ) -> i32 {
        // java int srcRowBlk = srcRow / 32;
        let srcRowBlk = srcRow / 32;
        // java int desRowBlk = desRow / 32;
        let desRowBlk = desRow / 32;
        // java int srcRowIndexInBlk = srcRow % 32;
        let srcRowIndexInBlk = srcRow % 32;
        // java int desRowIndexInBlk = desRow % 32;
        let desRowIndexInBlk = desRow % 32;
        // java KmoBlockRow srcRowData = srcRowBlk >= this.rowLst.size() ? null : (KmoBlockRow)this.rowLst.get(srcRowBlk);
        let src_row_null = srcRowBlk < 0
            || srcRowBlk >= self.rowLst.len() as i32
            || self.rowLst[srcRowBlk as usize].is_none();
        // java KmoBlockRow desRowData = desRowBlk >= this.rowLst.size() ? null : (KmoBlockRow)this.rowLst.get(desRowBlk);
        let des_row_null = desRowBlk < 0
            || desRowBlk >= self.rowLst.len() as i32
            || self.rowLst[desRowBlk as usize].is_none();
        // java if (srcRowData == null && desRowData == null) {
        if src_row_null && des_row_null {
            // java return 1;
            return 1;
        } else {
            // java int firstColBlk = firstCol / 16;
            let firstColBlk = firstCol / 16;
            // java int lastColBlk = lastCol / 16;
            let lastColBlk = lastCol / 16;
            // java for(int i = firstColBlk; i <= lastColBlk; ++i) {
            let mut i = firstColBlk;
            while i <= lastColBlk {
                // java KmoBlock srcColBlock = srcRowData == null ? null : srcRowData.getColBlock(i);
                let src_exists = !src_row_null && self.col_block_exists(srcRowBlk, i);
                // java KmoBlock desColBlock = desRowData == null ? null : desRowData.getColBlock(i);
                let des_exists = self.row_is_present(desRowBlk) && self.col_block_exists(desRowBlk, i);
                // java if (srcColBlock != null || desColBlock != null) {
                if src_exists || des_exists {
                    // java int firstColInBlock = i == firstColBlk ? firstCol % 16 : 0;
                    let firstColInBlock = if i == firstColBlk { firstCol % 16 } else { 0 };
                    // java int lastColInBlock = i == lastColBlk ? lastCol % 16 : 15;
                    let lastColInBlock = if i == lastColBlk { lastCol % 16 } else { 15 };
                    // java if (srcColBlock == null) {
                    if !src_exists {
                        // java byte[] nodes = new byte[(lastColInBlock - firstColInBlock + 1) * 7];
                        let nodes = vec![0u8; ((lastColInBlock - firstColInBlock + 1) * 7) as usize];
                        // java desColBlock.setRangeBytes(desRow % 32, firstColInBlock, lastColInBlock, nodes, 0, this.isInCmdMode);
                        let cmd = self.isInCmdMode.get();
                        self.write_range(
                            desRowBlk,
                            i,
                            desRow % 32,
                            firstColInBlock,
                            lastColInBlock,
                            &nodes,
                            0,
                            cmd,
                        );
                    } else {
                        // java if (desRowData == null) {
                        if !self.row_is_present(desRowBlk) {
                            // java desRowData = this.allocRow(desRowBlk);
                            self.allocRow(desRowBlk);
                        }
                        // java desColBlock = desRowData.allocCol(i, this.maxRow, this.maxCol);
                        self.alloc_col_existing(desRowBlk, i);
                        // java byte[] nodes = new byte[(lastColInBlock - firstColInBlock + 1) * 7];
                        let mut nodes = vec![0u8; ((lastColInBlock - firstColInBlock + 1) * 7) as usize];
                        // java srcColBlock.getRangeBytes(srcRowIndexInBlk, firstColInBlock, lastColInBlock, nodes, 0);
                        self.read_range_into(
                            srcRowBlk,
                            i,
                            srcRowIndexInBlk,
                            firstColInBlock,
                            lastColInBlock,
                            &mut nodes,
                            0,
                        );
                        // java desColBlock.setRangeBytes(desRowIndexInBlk, firstColInBlock, lastColInBlock, nodes, 0, this.isInCmdMode);
                        let cmd = self.isInCmdMode.get();
                        self.write_range(
                            desRowBlk,
                            i,
                            desRowIndexInBlk,
                            firstColInBlock,
                            lastColInBlock,
                            &nodes,
                            0,
                            cmd,
                        );
                        // java if (isClearSrcData) {
                        if isClearSrcData {
                            // java srcColBlock.clearRangeBytes(srcRowIndexInBlk, firstColInBlock, lastColInBlock, this.isInCmdMode);
                            let cmd = self.isInCmdMode.get();
                            self.clear_range(
                                srcRowBlk,
                                i,
                                srcRowIndexInBlk,
                                firstColInBlock,
                                lastColInBlock,
                                cmd,
                                false,
                            );
                        }
                    }
                }
                i += 1;
            }
            // java return 1;
            return 1;
        }
    }

    // java public int clearRow(int row, int firstCol, int lastCol)
    pub fn clearRow(&mut self, row: i32, firstCol: i32, lastCol: i32) -> i32 {
        // java return this.clearRowAndUnlock(row, firstCol, lastCol, false);
        self.clearRowAndUnlock(row, firstCol, lastCol, false)
    }

    // java public int clearRowAndUnlock(int row, int firstCol, int lastCol, boolean unlock)
    pub fn clearRowAndUnlock(&mut self, row: i32, firstCol: i32, lastCol: i32, unlock: bool) -> i32 {
        // java int rowBlk = row / 32;
        let rowBlk = row / 32;
        // java KmoBlockRow rowData = rowBlk >= this.rowLst.size() ? null : (KmoBlockRow)this.rowLst.get(rowBlk);
        // java if (rowData == null) {
        if !self.row_is_present(rowBlk) {
            // java return 16;
            return 16;
        } else {
            // java int firstColBlk = firstCol / 16;
            let firstColBlk = firstCol / 16;
            // java int lastColBlk = lastCol / 16;
            let lastColBlk = lastCol / 16;
            // java for(int i = firstColBlk; i <= lastColBlk; ++i) {
            let mut i = firstColBlk;
            while i <= lastColBlk {
                // java int firstColInBlock = i == firstColBlk ? firstCol % 16 : 0;
                let firstColInBlock = if i == firstColBlk { firstCol % 16 } else { 0 };
                // java int lastColInBlock = i == lastColBlk ? lastCol % 16 : 15;
                let lastColInBlock = if i == lastColBlk { lastCol % 16 } else { 15 };
                // java KmoBlock block = rowData.getColBlock(i);
                // java if (block != null) {
                if self.col_block_exists(rowBlk, i) {
                    // java block.clearRangeBytesUnlock(row % 32, firstColInBlock, lastColInBlock, this.isInCmdMode, unlock);
                    let cmd = self.isInCmdMode.get();
                    self.clear_range(rowBlk, i, row % 32, firstColInBlock, lastColInBlock, cmd, unlock);
                }
                i += 1;
            }
            // java return 1;
            return 1;
        }
    }

    // java public void clearRowsMemory(int firstRow, int endRow)
    pub fn clearRowsMemory(&mut self, firstRow: i32, endRow: i32) {
        // java int firstRowBlk = firstRow / 32;
        let mut firstRowBlk = firstRow / 32;
        // java if (firstRowBlk * 32 < firstRow) {
        if firstRowBlk * 32 < firstRow {
            // java ++firstRowBlk;
            firstRowBlk += 1;
        }
        // java int endRowBlk = endRow / 32;
        let mut endRowBlk = endRow / 32;
        // java if ((endRowBlk + 1) * 32 - 1 > endRow) {
        if (endRowBlk + 1) * 32 - 1 > endRow {
            // java --endRowBlk;
            endRowBlk -= 1;
        }
        // java for(int rowBlk = firstRowBlk; rowBlk <= endRowBlk; ++rowBlk) {
        let mut rowBlk = firstRowBlk;
        while rowBlk <= endRowBlk {
            // java KmoBlockRow blkRow = (KmoBlockRow)this.rowLst.get(rowBlk);
            if rowBlk < 0 || rowBlk >= self.rowLst.len() as i32 {
                panic!("IndexOutOfBoundsException");
            }
            // java if (blkRow != null) {
            if self.rowLst[rowBlk as usize].is_some() {
                // java blkRow.clearBlockRowMemory();
                self.rowLst[rowBlk as usize]
                    .as_mut()
                    .unwrap()
                    .clearBlockRowMemory();
            }
            rowBlk += 1;
        }
    }

    // java public void clearAllRowsMemory()
    pub fn clearAllRowsMemory(&mut self) {
        // java this.rowLst.clear();
        self.rowLst.clear();
    }

    // java public static boolean isEmptyByte(byte[] buff)
    pub fn isEmptyByte(buff: &[u8]) -> bool {
        // java int i = 0;
        let mut i: i32 = 0;
        // java for(int len = buff.length; i < len; ++i) {
        let len = buff.len() as i32;
        while i < len {
            // java if (buff[i] != 0) {
            if buff[i as usize] != 0 {
                // java return false;
                return false;
            }
            i += 1;
        }
        // java return true;
        true
    }

    // java private KmoBlockRow allocRow(int row)
    fn allocRow(&mut self, row: i32) -> &mut KmoBlockRow {
        // java if (row < this.rowLst.size()) {
        if row < self.rowLst.len() as i32 {
            // java KmoBlockRow ret = (KmoBlockRow)this.rowLst.get(row);
            // java if (ret == null) {
            if self.rowLst[row as usize].is_none() {
                // java ret = new KmoBlockRow(row * 32, this.maxRow, this.sheet);
                let mut ret = KmoBlockRow::KmoBlockRow__new(row * 32, self.maxRow, self.sheet_is_null);
                ret.binding = self.binding.clone();
                // java this.rowLst.set(row, ret);
                self.rowLst[row as usize] = Some(ret);
            }
            // java return ret;
            return self.rowLst[row as usize].as_mut().unwrap();
        } else {
            // java for(int i = this.rowLst.size(); i < row; ++i) {
            // java this.rowLst.add(null);
            while (self.rowLst.len() as i32) < row {
                self.rowLst.push(None);
            }
            // java KmoBlockRow ret = new KmoBlockRow(row * 32, this.maxRow, this.sheet);
            let mut ret = KmoBlockRow::KmoBlockRow__new(row * 32, self.maxRow, self.sheet_is_null);
            ret.binding = self.binding.clone();
            // java this.rowLst.add(ret);
            self.rowLst.push(Some(ret));
            // java return ret;
            return self.rowLst.last_mut().unwrap().as_mut().unwrap();
        }
    }

    // java public List<KmoBlockRow> getRowList()
    pub fn getRowList(&self) -> &Vec<Option<KmoBlockRow>> {
        // java return this.rowLst;
        &self.rowLst
    }

    // java public void addChangeRange(KmoRect kmoRect)
    pub fn addChangeRange(&mut self, kmoRect: &KmoRect) {
        // java if (this.sheet.getBook().transaction().isTransStarted()) {
        let binding = self.binding.as_ref().expect("NullPointerException sheet");
        let sheet = binding.sheet.upgrade().expect("KmoSheet");
        if sheet.borrow().getBook().transaction().isTransStarted() {
        // java this.kmoChangeRange.addRangeOuter(kmoRect);
            binding.range.addRangeOuter(kmoRect);
        }
    }

    // rust: complete Java constructor self-reference binding before the sheet is published to readers.
    pub(crate) fn bindSheet(&mut self, sheet: Weak<RefCell<KmoSheet>>, book: &KmoBook, uid: i32) {
        let range = self.kmoChangeRange.as_ref().expect("KmoChangeRange").clone();
        *range.sheet.borrow_mut() = sheet.clone();
        self.binding = Some(BlockBinding { sheet, range: range.clone() });
        self.sheet_is_null = false;
        // java sheet.getBook().transaction().addTransactionListener(sheet.getSheetUid(), this);
        book.transaction().addTransactionListener(uid, Some(range));
    }

    fn row_is_present(&self, row_blk: i32) -> bool {
        row_blk >= 0
            && (row_blk as usize) < self.rowLst.len()
            && self.rowLst[row_blk as usize].is_some()
    }

    fn col_block_exists(&self, row_blk: i32, col_blk: i32) -> bool {
        if !self.row_is_present(row_blk) {
            return false;
        }
        self.rowLst[row_blk as usize]
            .as_ref()
            .unwrap()
            .getColBlock(col_blk)
            .is_some()
    }

    fn read_cell(&self, row_blk: i32, col_blk: i32, row_in: i32, col_in: i32) -> [u8; 7] {
        let mut node = [0u8; 7];
        self.rowLst[row_blk as usize]
            .as_ref()
            .unwrap()
            .getColBlock(col_blk)
            .unwrap()
            .get(row_in, col_in, &mut node);
        node
    }

    fn write_cell(
        &mut self,
        row_blk: i32,
        col_blk: i32,
        row_in: i32,
        col_in: i32,
        node: &[u8],
        cmd: bool,
    ) {
        self.rowLst[row_blk as usize]
            .as_mut()
            .unwrap()
            .col_mut(col_blk)
            .set(row_in, col_in, node, cmd);
    }

    fn alloc_col_existing(&mut self, row_blk: i32, col_blk: i32) {
        let maxRow = self.maxRow;
        let maxCol = self.maxCol;
        self.rowLst[row_blk as usize]
            .as_mut()
            .unwrap()
            .allocCol(col_blk, maxRow, maxCol);
    }

    fn read_range_into(
        &self,
        row_blk: i32,
        col_blk: i32,
        row_in: i32,
        first_col: i32,
        last_col: i32,
        nodes: &mut [u8],
        des_pos: i32,
    ) {
        self.rowLst[row_blk as usize]
            .as_ref()
            .unwrap()
            .getColBlock(col_blk)
            .unwrap()
            .getRangeBytes(row_in, first_col, last_col, nodes, des_pos);
    }

    fn write_range(
        &mut self,
        row_blk: i32,
        col_blk: i32,
        row_in: i32,
        first_col: i32,
        last_col: i32,
        nodes: &[u8],
        src_pos: i32,
        cmd: bool,
    ) {
        self.rowLst[row_blk as usize]
            .as_mut()
            .unwrap()
            .col_mut(col_blk)
            .setRangeBytes(row_in, first_col, last_col, nodes, src_pos, cmd);
    }

    fn clear_range(
        &mut self,
        row_blk: i32,
        col_blk: i32,
        row_in: i32,
        first_col: i32,
        last_col: i32,
        cmd: bool,
        unlock: bool,
    ) {
        self.rowLst[row_blk as usize]
            .as_mut()
            .unwrap()
            .col_mut(col_blk)
            .clearRangeBytesUnlock(row_in, first_col, last_col, cmd, unlock);
    }
}

// java public static class KmoBlockRow
pub struct KmoBlockRow {
    // java List<KmoBlock> colLst = new ArrayList();
    colLst: Vec<Option<Rc<KmoBlock>>>,
    // java int topRow;
    topRow: i32,
    // java KmoSheet sheet;
    sheet_is_null: bool,
    binding: Option<BlockBinding>,
}

// java public static class KmoBlockRow
impl KmoBlockRow {
    // java public KmoBlockRow copy(KmoBlockRow rowBlock)
    pub fn copy(&mut self, rowBlock: &KmoBlockRow) -> &mut Self {
        // java this.topRow = rowBlock.topRow;
        self.topRow = rowBlock.topRow;
        // java this.sheet = rowBlock.sheet;
        self.sheet_is_null = rowBlock.sheet_is_null;
        self.binding = rowBlock.binding.clone();
        // java int i = 0;
        let mut i: i32 = 0;
        // java for(int size = rowBlock.colLst.size(); i < size; ++i) {
        let size = rowBlock.colLst.len() as i32;
        while i < size {
            // java KmoBlock blk = (KmoBlock)rowBlock.colLst.get(i);
            // java if (blk == null) {
            if rowBlock.colLst[i as usize].is_none() {
                // java this.colLst.add(null);
                self.colLst.push(None);
            } else {
                // java this.colLst.add(new KmoBlock(blk));
                let blk = rowBlock.colLst[i as usize].as_ref().unwrap();
                self.colLst.push(Some(Rc::new(KmoBlock::KmoBlock__from(blk))));
            }
            i += 1;
        }
        // java return this;
        self
    }

    // java public KmoBlockRow(KmoBlockRow rowBlock)
    pub fn KmoBlockRow__from(rowBlock: &KmoBlockRow) -> Self {
        // java this.copy(rowBlock);
        let mut row = Self::KmoBlockRow__new(0, 0, true);
        row.copy(rowBlock);
        row
    }

    // java public KmoBlockRow(int topRow, int maxRow, KmoSheet sheet)
    pub fn KmoBlockRow__new(topRow: i32, _maxRow: i32, sheet_is_null: bool) -> Self {
        // java this.topRow = topRow;
        let topRow = topRow;
        // java this.sheet = sheet;
        let sheet_is_null = sheet_is_null;
        Self {
            colLst: Vec::new(),
            topRow,
            sheet_is_null,
            binding: None,
        }
    }

    // java public KmoBlock allocCol(int col, int maxRow, int maxCol)
    pub fn allocCol(&mut self, col: i32, maxRow: i32, maxCol: i32) -> Rc<KmoBlock> {
        // java if (col < this.colLst.size()) {
        if col < self.colLst.len() as i32 {
            // java KmoBlock ret = (KmoBlock)this.colLst.get(col);
            // java if (ret == null) {
            if self.colLst[col as usize].is_none() {
                // java ret = KmoMatrix.KmoBlock.newAncestorAtom(this.topRow, (short)(col * 16), maxRow, maxCol, this.sheet);
                let mut ret = KmoBlock::newAncestorAtom(
                    self.topRow,
                    col.wrapping_mul(16) as i16,
                    maxRow,
                    maxCol,
                    self.sheet_is_null,
                );
                *ret.binding.borrow_mut() = self.binding.clone();
                // java this.colLst.set(col, ret);
                self.colLst[col as usize] = Some(Rc::new(ret));
            }
            // java return ret;
            return self.colLst[col as usize].as_ref().unwrap().clone();
        } else {
            // java for(int i = this.colLst.size(); i < col; ++i) {
            // java this.colLst.add(null);
            while (self.colLst.len() as i32) < col {
                self.colLst.push(None);
            }
            // java KmoBlock ret = KmoMatrix.KmoBlock.newAncestorAtom(this.topRow, (short)(col * 16), maxRow, maxCol, this.sheet);
            let mut ret = KmoBlock::newAncestorAtom(
                self.topRow,
                col.wrapping_mul(16) as i16,
                maxRow,
                maxCol,
                self.sheet_is_null,
            );
            *ret.binding.borrow_mut() = self.binding.clone();
            // java this.colLst.add(ret);
            self.colLst.push(Some(Rc::new(ret)));
            // java return ret;
            return self.colLst.last().unwrap().as_ref().unwrap().clone();
        }
    }

    // java public List<KmoBlock> getColLst()
    pub fn getColLst(&self) -> &Vec<Option<Rc<KmoBlock>>> {
        // java return this.colLst;
        &self.colLst
    }

    // java public KmoBlock getColBlock(int colBlk)
    pub fn getColBlock(&self, colBlk: i32) -> Option<Rc<KmoBlock>> {
        // java return colBlk >= this.colLst.size() ? null : (KmoBlock)this.colLst.get(colBlk);
        if colBlk >= self.colLst.len() as i32 {
            None
        } else if colBlk < 0 {
            panic!("IndexOutOfBoundsException");
        } else {
            self.colLst[colBlk as usize].clone()
        }
    }

    // java public void clearBlockRowMemory()
    pub fn clearBlockRowMemory(&mut self) {
        // java this.colLst.clear();
        self.colLst.clear();
    }

    fn col_mut(&mut self, col_blk: i32) -> Rc<KmoBlock> {
        self.colLst[col_blk as usize].as_ref().unwrap().clone()
    }
}

// java private static class KmoChangeRangeData extends KmoAtomData
struct KmoChangeRangeData {
    base: KmoAtomData,
    // java private Set<KmoRect> changeRange;
    changeRange: HashSet<KmoRect>,
}

impl KmoAtomDataSub for KmoChangeRangeData {
    fn as_base(&self) -> &KmoAtomData {
        &self.base
    }
    fn as_base_mut(&mut self) -> &mut KmoAtomData {
        &mut self.base
    }
    // java public KmoAtomData copy()
    fn copy(&self) -> Self {
        // java KmoChangeRangeData cpy = new KmoChangeRangeData();
        let mut cpy = KmoChangeRangeData::KmoChangeRangeData();
        // java cpy.changeRange = new HashSet(this.changeRange);
        cpy.changeRange = self.changeRange.clone();
        // java return cpy;
        cpy
    }
}

impl KmoChangeRangeData {
    // java private KmoChangeRangeData()
    fn KmoChangeRangeData() -> Self {
        // java this.changeRange = new HashSet();
        Self {
            base: KmoAtomData::new(),
            changeRange: HashSet::new(),
        }
    }
}

// java private static class KmoChangeRange extends KmoAtom<KmoChangeRangeData> implements KmoTransaction.KmoSheetTransactionListener
#[derive(Clone)]
struct BlockBinding { sheet: Weak<RefCell<KmoSheet>>, range: Rc<KmoChangeRange> }

struct KmoChangeRange {
    atom: Rc<RefCell<KmoAtom<KmoChangeRangeData>>>,
    _owner: SharedAtom,
    // java private final KmoSheet sheet;
    sheet: Rc<RefCell<Weak<RefCell<KmoSheet>>>>,
    // rust: same matrix command-mode field, without reacquiring its write guard.
    commandMode: Rc<Cell<bool>>,
    // java private boolean outerRangeAdd = false;
    outerRangeAdd: Cell<bool>,
    // java private boolean needWrite = false;
    needWrite: Cell<bool>,
}

impl KmoChangeRange {
    // java protected KmoChangeRange(KmoSheet sheet)
    fn KmoChangeRange(commandMode: Rc<Cell<bool>>) -> Self {
        // java super(new KmoChangeRangeData());
        let sheet = Rc::new(RefCell::new(Weak::new()));
        let (atom, owner) = bindChangeRange(KmoChangeRangeData::KmoChangeRangeData(), sheet.clone());
        // java this.sheet = sheet;
        // java if (sheet != null) {
        // java sheet.getBook().transaction().addTransactionListener(sheet.getSheetUid(), this);
        // rust: bindSheet registers the exact Rc owner once Java's constructor `this` is published.
        Self {
            atom, _owner: owner, sheet, commandMode,
            outerRangeAdd: Cell::new(false),
            needWrite: Cell::new(false),
        }
    }

    // java public void addRangeOuter(KmoRect kmoRect)
    pub fn addRangeOuter(&self, kmoRect: &KmoRect) {
        // java this.outerRangeAdd = true;
        self.outerRangeAdd.set(true);
        // java this.needWrite = true;
        self.needWrite.set(true);
        // java ((KmoChangeRangeData)this.rawData()).changeRange.add(kmoRect);
        self.atom.borrow_mut().rawData_mut().changeRange.insert(kmoRect.clone());
    }

    // java public void addRange(KmoRect kmoRect)
    pub fn addRange(&self, kmoRect: &KmoRect) {
        // java this.needWrite = true;
        self.needWrite.set(true);
        // java ((KmoChangeRangeData)this.rawData()).changeRange.add(kmoRect);
        self.atom.borrow_mut().rawData_mut().changeRange.insert(kmoRect.clone());
    }

    // java public void willSwap(boolean isUndo)
    pub fn willSwap(&self, isUndo: bool) {
        // java if (isUndo) {
        if isUndo {
        // java this.reverseOp(isUndo, (KmoChangeRangeData)this.rawData());
            Self::reverseOp(&self.sheet, isUndo, &self.atom.borrow().rawData().changeRange);
        }
    }

    // java public void didFinishSwapping(boolean isUndo)
    pub fn didFinishSwapping(&self, isUndo: bool) {
        // java if (!isUndo) {
        if !isUndo {
        // java this.reverseOp(isUndo, (KmoChangeRangeData)this.rawData());
            Self::reverseOp(&self.sheet, isUndo, &self.atom.borrow().rawData().changeRange);
        }
    }

    // java private void reverseOp(boolean isUndo, KmoChangeRangeData data)
    fn reverseOp(sheet: &RefCell<Weak<RefCell<KmoSheet>>>, _isUndo: bool, changeRange: &HashSet<KmoRect>) {
        // java CalcChain.recalcLock.lock();
        let _lock = crate::cn::wps::et::ss::calcchain::recalcLock.lock();
        // java if (this.sheet != null) {
        if let Some(sheet) = sheet.borrow().upgrade() {
        // java List<KmoRect> list = new ArrayList(data.changeRange);
            let mut list: Vec<_> = changeRange.iter().cloned().collect();
        // java for(KmoRect rect : list) {
        // java if (rect != null) {
        // java this.sheet.addInUTransactionList(rect);
            for rect in &list { sheet.borrow().addInUTransactionList(rect); }
        // java list.clear();
            list.clear();
        }
        // java CalcChain.recalcLock.unlock();
        // rust: finally releases the original shared reentrant lock even on unwind.
    }

    // java public void onCommit()
    pub fn onCommit(&self) {
        // java if (!this.isInCmdMode() && this.needWrite) {
        if !self.isInCmdMode() && self.needWrite.get() {
            // java this.beforeWrite();
            self.atom.borrow_mut().beforeWrite();
        }
        // java this.needWrite = false;
        self.needWrite.set(false);
        // java this.outerRangeAdd = false;
        self.outerRangeAdd.set(false);
    }

    // java private boolean isInCmdMode()
    fn isInCmdMode(&self) -> bool {
        // java return this.sheet != null && this.sheet.getGrid().getMatrix().isInCmdMode;
        self.sheet.borrow().upgrade().is_some() && self.commandMode.get()
    }
}

impl KmoSheetTransactionListener for KmoChangeRange { fn onCommit(&self) { Self::onCommit(self); } }

// rust: Java's AtomImage retains the same raw-data owner; hooks dispatch the
// original KmoChangeRange before/after-swap branches, without a matrix lock.
struct ChangeRangeSnapshotOwner { atom: Rc<RefCell<KmoAtom<KmoChangeRangeData>>>, sheet: Rc<RefCell<Weak<RefCell<KmoSheet>>>> }
impl SnapshotAtom for ChangeRangeSnapshotOwner {
    fn willSwap(&mut self, undo: bool) { if undo { KmoChangeRange::reverseOp(&self.sheet, undo, &self.atom.borrow().rawData().changeRange); } }
    fn swapData(&mut self, data: &mut Box<dyn SnapshotData>) { self.atom.borrow_mut().swapSnapshotData(data); }
    fn didFinishSwapping(&mut self, undo: bool) { if !undo { KmoChangeRange::reverseOp(&self.sheet, undo, &self.atom.borrow().rawData().changeRange); } }
    fn isNeedSupportNonSequence(&self) -> bool { self.atom.borrow().isNeedSupportNonSequence() }
    fn cloneAtom(&self) -> SharedAtom { bindChangeRange(self.atom.borrow().rawData().copy(), self.sheet.clone()).1 }
}
fn bindChangeRange(data: KmoChangeRangeData, sheet: Rc<RefCell<Weak<RefCell<KmoSheet>>>>) -> (Rc<RefCell<KmoAtom<KmoChangeRangeData>>>, SharedAtom) {
    let atom=Rc::new(RefCell::new(KmoAtom::new(data)));
    let owner:SharedAtom=std::sync::Arc::new(Mutex::new(Box::new(ChangeRangeSnapshotOwner{atom:atom.clone(),sheet})));
    atom.borrow_mut().bindSnapshotOwner(&owner);(atom,owner)
}

// java private static final int BYTES_OF_DATA = 3584;
const BYTES_OF_DATA: i32 = 3584;

// java public static class KmoBlockData extends KmoAtomData
pub struct KmoBlockData {
    base: KmoAtomData,
    // java public byte[] data = new byte[3584];
    pub data: [u8; 3584],
}

impl KmoAtomDataSub for KmoBlockData {
    fn as_base(&self) -> &KmoAtomData {
        &self.base
    }
    fn as_base_mut(&mut self) -> &mut KmoAtomData {
        &mut self.base
    }
    // java public KmoAtomData copy()
    fn copy(&self) -> Self {
        // java KmoBlockData cpy = new KmoBlockData();
        let mut cpy = KmoBlockData::KmoBlockData();
        // java System.arraycopy(this.data, 0, cpy.data, 0, this.data.length);
        cpy.data.copy_from_slice(&self.data);
        let _ = BYTES_OF_DATA;
        // java return cpy;
        cpy
    }
}

impl KmoBlockData {
    fn KmoBlockData() -> Self {
        Self {
            base: KmoAtomData::new(),
            data: [0u8; 3584],
        }
    }
}

// java public static final int W = 16;
pub const W: i32 = 16;
// java public static final int H = 32;
pub const H: i32 = 32;
// java public static final int RAWW = 4;
pub const RAWW: i32 = 4;
// java public static final int RAWH = 5;
pub const RAWH: i32 = 5;
// java public static final int BYTES_PER_NODE = 7;
pub const BLOCK_BYTES_PER_NODE: i32 = 7;
// java private static final byte[] nullNode = new byte[7];
static nullNode: [u8; 7] = [0u8; 7];
// java private static final byte[] buffNode = new byte[7];
static buffNode: Mutex<[u8; 7]> = Mutex::new([0u8; 7]);

// java public static class KmoBlock extends KmoAtom<KmoBlockData>
pub struct KmoBlock {
    // rust: stable Java object identity shared by the matrix and AtomSnapshot.
    atom: Rc<RefCell<KmoAtom<KmoBlockData>>>,
    _snapshotOwner: kfc::cn::wps::memor4j::AtomSnapshot::SharedAtom,
    // java private int topRow;
    topRow: Cell<i32>,
    // java private int leftCol;
    leftCol: Cell<i32>,
    // java private KmoSheet sheet;
    sheet_is_null: Cell<bool>,
    binding: RefCell<Option<BlockBinding>>,
}

// java public static class KmoBlock extends KmoAtom<KmoBlockData>
impl KmoBlock {
    // java public KmoBlock copy(KmoBlock block)
    pub fn copy(&self, block: &KmoBlock) -> &Self {
        // java this.topRow = block.topRow;
        self.topRow.set(block.topRow.get());
        // java this.leftCol = block.leftCol;
        self.leftCol.set(block.leftCol.get());
        // java this.sheet = block.sheet;
        self.sheet_is_null.set(block.sheet_is_null.get());
        *self.binding.borrow_mut() = block.binding.borrow().clone();
        // java System.arraycopy(((KmoBlockData)block.rawData()).data, 0, ((KmoBlockData)this.rawData()).data, 0, ((KmoBlockData)block.rawData()).data.length);
        let src = block.atom.borrow().rawData().data;
        self.atom.borrow_mut().rawData_mut().data.copy_from_slice(&src);
        // java return this;
        self
    }

    // java KmoBlock(KmoBlock block)
    pub fn KmoBlock__from(block: &KmoBlock) -> Self {
        // java super(new KmoBlockData());
        let mut blk = Self::KmoBlock__coords(0, 0, 0, 0, true);
        // java this.copy(block);
        blk.copy(block);
        blk
    }

    // java KmoBlock(int topRow, short leftCol, int maxRow, int maxCol, KmoSheet sheet)
    pub fn KmoBlock__coords(
        topRow: i32,
        leftCol: i16,
        maxRow: i32,
        maxCol: i32,
        sheet_is_null: bool,
    ) -> Self {
        // java super(new KmoBlockData());
        let (atom, snapshotOwner) = crate::cn::wps::moss::app::xf::snapshot::bind(KmoBlockData::KmoBlockData());
        // java this.topRow = topRow;
        let topRow = topRow;
        // java this.leftCol = leftCol;
        let leftCol = leftCol as i32;
        // java this.sheet = sheet;
        let sheet_is_null = sheet_is_null;
        let _ = (maxRow, maxCol);
        Self {
            atom,
            _snapshotOwner: snapshotOwner,
            topRow:Cell::new(topRow),
            leftCol:Cell::new(leftCol),
            sheet_is_null:Cell::new(sheet_is_null),
            binding: RefCell::new(None),
        }
    }

    // java public byte[] initFill(int row, int col, byte[] node)
    pub fn initFill<'a>(&self, row: i32, col: i32, node: &'a [u8]) -> &'a [u8] {
        // java System.arraycopy(node, 0, ((KmoBlockData)this.rawData()).data, (row * 16 + col) * 7, 7);
        let off = (row * 16 + col) * 7;
        arraycopy(node, 0, &mut self.atom.borrow_mut().rawData_mut().data, off, 7);
        // java return node;
        node
    }

    // java public byte[] set(int row, int col, byte[] node, boolean isInCmdMode)
    pub fn set<'a>(&self, row: i32, col: i32, node: &'a [u8], isInCmdMode: bool) -> &'a [u8] {
        // java if (!isInCmdMode) {
        if !isInCmdMode {
            // java if (this.sheet != null) {
            if let Some(binding) = self.binding.borrow().clone() {
                // java KmoMatrix matrix = this.sheet.getGrid().getMatrix();
                // java if (!matrix.kmoChangeRange.outerRangeAdd && this.sheet.getBook().transaction().isTransStarted()) {
                let sheet = binding.sheet.upgrade().expect("KmoSheet");
                if !binding.range.outerRangeAdd.get() && sheet.borrow().getBook().transaction().isTransStarted() {
                // java KmoRect rect = new KmoRect(row + this.topRow, col + this.leftCol, row + this.topRow, col + this.leftCol);
                    let r = row.wrapping_add(self.topRow.get()); let c = col.wrapping_add(self.leftCol.get());
                    let rect = KmoRect::KmoRect__bounds(r, c, r, c);
                // java matrix.kmoChangeRange.addRange(rect);
                    binding.range.addRange(&rect);
                }
            } else if !self.sheet_is_null.get() { panic!("unbound non-null KmoSheet");
            }
            // java this.beforeWrite();
            self.atom.borrow_mut().beforeWrite();
        }
        // java int offset = (row * 16 + col) * 7;
        let offset = (row * 16 + col) * 7;
        // java System.arraycopy(node, 0, ((KmoBlockData)this.rawData()).data, offset, 7);
        arraycopy(node, 0, &mut self.atom.borrow_mut().rawData_mut().data, offset, 7);
        // java if (this.sheet != null) {
        if let Some(binding) = self.binding.borrow().clone() {
            // java this.sheet.notifyCellChange(row, col, false);
            binding.sheet.upgrade().expect("KmoSheet").borrow().notifyCellChange(row, col, false);
        }
        // java return node;
        node
    }

    // java public byte[] setCalcResult(int row, int col, byte[] node)
    pub fn setCalcResult<'a>(&self, row: i32, col: i32, node: &'a [u8]) -> &'a [u8] {
        // java int offset = (row * 16 + col) * 7;
        let offset = (row * 16 + col) * 7;
        // java System.arraycopy(node, 0, ((KmoBlockData)this.rawData()).data, offset, 7);
        arraycopy(node, 0, &mut self.atom.borrow_mut().rawData_mut().data, offset, 7);
        // java if (this.sheet != null) {
        if let Some(binding) = self.binding.borrow().clone() {
            // java this.sheet.notifyCellChange(row, col, true);
            binding.sheet.upgrade().expect("KmoSheet").borrow().notifyCellChange(row, col, true);
        }
        // java return node;
        node
    }

    // java public byte[] setRangeBytes(int row, int firstCol, int lastCol, byte[] nodes, int srcPos, boolean isInCmdMode)
    pub fn setRangeBytes<'a>(
        &self,
        row: i32,
        firstCol: i32,
        lastCol: i32,
        nodes: &'a [u8],
        srcPos: i32,
        isInCmdMode: bool,
    ) -> &'a [u8] {
        // java if (!isInCmdMode) {
        if !isInCmdMode {
            // java if (this.sheet != null && !this.sheet.getGrid().getMatrix().kmoChangeRange.outerRangeAdd) {
            if let Some(binding) = self.binding.borrow().clone().filter(|b| !b.range.outerRangeAdd.get()) {
                // java KmoRect rect = new KmoRect(row + this.topRow, firstCol + this.leftCol, row + this.topRow, lastCol + this.leftCol);
                let r = row.wrapping_add(self.topRow.get());
                let rect = KmoRect::KmoRect__bounds(r, firstCol.wrapping_add(self.leftCol.get()), r, lastCol.wrapping_add(self.leftCol.get()));
                // java this.sheet.getGrid().getMatrix().kmoChangeRange.addRange(rect);
                binding.range.addRange(&rect);
            }
            // java this.beforeWrite();
            self.atom.borrow_mut().beforeWrite();
        }
        // java int startOffset = (row * 16 + firstCol) * 7;
        let startOffset = (row * 16 + firstCol) * 7;
        // java int offsetLength = (lastCol - firstCol + 1) * 7;
        let offsetLength = (lastCol - firstCol + 1) * 7;
        // java System.arraycopy(nodes, srcPos, ((KmoBlockData)this.rawData()).data, startOffset, offsetLength);
        arraycopy(
            nodes,
            srcPos,
            &mut self.atom.borrow_mut().rawData_mut().data,
            startOffset,
            offsetLength,
        );
        // java return nodes;
        nodes
    }

    // java public void clearRangeBytes(int row, int firstCol, int lastCol, boolean isInCmdMode)
    pub fn clearRangeBytes(&self, row: i32, firstCol: i32, lastCol: i32, isInCmdMode: bool) {
        // java this.clearRangeBytesUnlock(row, firstCol, lastCol, isInCmdMode, false);
        self.clearRangeBytesUnlock(row, firstCol, lastCol, isInCmdMode, false);
    }

    // java public void clearRangeBytesUnlock(int row, int firstCol, int lastCol, boolean isInCmdMode, boolean unlock)
    pub fn clearRangeBytesUnlock(
        &self,
        row: i32,
        firstCol: i32,
        lastCol: i32,
        isInCmdMode: bool,
        unlock: bool,
    ) {
        // java if (!isInCmdMode) {
        if !isInCmdMode {
            // java if (this.sheet != null && !this.sheet.getGrid().getMatrix().kmoChangeRange.outerRangeAdd) {
            if let Some(binding) = self.binding.borrow().clone().filter(|b| !b.range.outerRangeAdd.get()) {
                // java KmoRect rect = new KmoRect(row + this.topRow, firstCol + this.leftCol, row + this.topRow, lastCol + this.leftCol);
                let r = row.wrapping_add(self.topRow.get());
                let rect = KmoRect::KmoRect__bounds(r, firstCol.wrapping_add(self.leftCol.get()), r, lastCol.wrapping_add(self.leftCol.get()));
                // java this.sheet.getGrid().getMatrix().kmoChangeRange.addRange(rect);
                binding.range.addRange(&rect);
            }
            // java this.beforeWrite();
            self.atom.borrow_mut().beforeWrite();
        }
        // java int startOffset = (row * 16 + firstCol) * 7;
        let startOffset = (row * 16 + firstCol) * 7;
        // java int endOffset = (row * 16 + lastCol + 1) * 7;
        let endOffset = (row * 16 + lastCol + 1) * 7;
        // java Arrays.fill(((KmoBlockData)this.rawData()).data, startOffset, endOffset, (byte)0);
        let mut atom = self.atom.borrow_mut();
        let mut raw = atom.rawData_mut();
        let data = &mut raw.data;
        if startOffset < 0 || endOffset < startOffset || endOffset as usize > data.len() {
            panic!("IndexOutOfBoundsException");
        }
        data[startOffset as usize..endOffset as usize].fill(0);
        // java if (unlock) {
        if unlock {
            // java for(int i = firstCol; i <= lastCol; ++i) {
            let mut i = firstCol;
            while i <= lastCol {
                // java this.setIXF(((KmoBlockData)this.rawData()).data, (row * 16 + i) * 7, 21);
                let off = (row * 16 + i) * 7;
                let data = &mut raw.data;
                Self::setIXF(data, off, 21);
                i += 1;
            }
        }
    }

    // java private void setIXF(byte[] buff, int offset, int ixf)
    fn setIXF(buff: &mut [u8], offset: i32, ixf: i32) {
        // java buff[offset] = (byte)ixf;
        buff[offset as usize] = ixf as u8;
        // java buff[offset + 1] = (byte)(ixf >> 8);
        buff[offset as usize + 1] = (ixf >> 8) as u8;
    }

    // java public byte[] get(int row, int col, byte[] node)
    pub fn get<'a>(&self, row: i32, col: i32, node: &'a mut [u8]) -> &'a mut [u8] {
        // java int offset = ((row << 4) + col) * 7;
        let offset = ((row << 4) + col) * 7;
        // java System.arraycopy(((KmoBlockData)this.rawData()).data, offset, node, 0, 7);
        arraycopy(&self.atom.borrow().rawData().data, offset, node, 0, 7);
        // java return node;
        node
    }

    // java public int getId(int row, int col)
    pub fn getId(&self, row: i32, col: i32) -> i32 {
        // java int offset = ((row << 4) + col) * 7;
        let offset = ((row << 4) + col) * 7;
        // java return ((KmoBlockData)this.rawData()).data[offset + 3] & 255 | (((KmoBlockData)this.rawData()).data[offset + 4] & 255) << 8 | (((KmoBlockData)this.rawData()).data[offset + 5] & 255) << 16 | (((KmoBlockData)this.rawData()).data[offset + 6] & 255) << 24;
        let atom = self.atom.borrow();
        let data = &atom.rawData().data;
        let o = offset as usize;
        ((data[o + 3] as u32)
            | ((data[o + 4] as u32) << 8)
            | ((data[o + 5] as u32) << 16)
            | ((data[o + 6] as u32) << 24)) as i32
    }

    // java public final int getOffset(int row, int col)
    pub fn getOffset(&self, row: i32, col: i32) -> i32 {
        // java return ((row << 4) + col) * 7;
        ((row << 4) + col) * 7
    }

    // java public byte[] getRawData()
    pub fn getRawData(&self) -> KmoBlockByteView {
        // java return ((KmoBlockData)this.rawData()).data;
        KmoBlockByteView(self.atom.borrow().rawData())
    }

    // java public boolean isBlankCell(int row, int col)
    pub fn isBlankCell(&self, row: i32, col: i32) -> bool {
        // java int offset = (row * 16 + col) * 7;
        let offset = (row * 16 + col) * 7;
        // java byte[] bytes = ((KmoBlockData)this.rawData()).data;
        let atom = self.atom.borrow();
        let bytes = &atom.rawData().data;
        // java return bytes[offset + 3] == 0 && bytes[offset + 4] == 0 && bytes[offset + 5] == 0 && bytes[offset + 6] == 0;
        let o = offset as usize;
        bytes[o + 3] == 0 && bytes[o + 4] == 0 && bytes[o + 5] == 0 && bytes[o + 6] == 0
    }

    // java public byte[] getRangeBytes(int row, int firstCol, int lastCol, byte[] nodes, int desPos)
    pub fn getRangeBytes<'a>(
        &self,
        row: i32,
        firstCol: i32,
        lastCol: i32,
        nodes: &'a mut [u8],
        desPos: i32,
    ) -> &'a mut [u8] {
        // java int startOffset = (row * 16 + firstCol) * 7;
        let startOffset = (row * 16 + firstCol) * 7;
        // java int offsetLength = (lastCol - firstCol + 1) * 7;
        let offsetLength = (lastCol - firstCol + 1) * 7;
        // java System.arraycopy(((KmoBlockData)this.rawData()).data, startOffset, nodes, desPos, offsetLength);
        arraycopy(
            &self.atom.borrow().rawData().data,
            startOffset,
            nodes,
            desPos,
            offsetLength,
        );
        // java return nodes;
        nodes
    }

    // java public boolean isNull(int row, int col)
    pub fn isNull(&self, row: i32, col: i32) -> bool {
        // java this.get(row, col, buffNode);
        let mut buf = buffNode.lock().unwrap();
        self.get(row, col, buf.as_mut());
        // java return Arrays.equals(buffNode, nullNode);
        *buf == nullNode
    }

    // java public static KmoBlock newAncestorAtom(int topRow, short leftCol, int maxRow, int maxCol, KmoSheet sheet)
    pub fn newAncestorAtom(
        topRow: i32,
        leftCol: i16,
        maxRow: i32,
        maxCol: i32,
        sheet_is_null: bool,
    ) -> Self {
        // java KmoBlock blk = new KmoBlock(topRow, leftCol, maxRow, maxCol, sheet);
        let mut blk = Self::KmoBlock__coords(topRow, leftCol, maxRow, maxCol, sheet_is_null);
        // java ((KmoBlockData)blk.rawData()).__forceOldest();
        blk.atom.borrow_mut().rawData_mut().as_base_mut().__forceOldest();
        // java return blk;
        blk
    }

    // java public byte[] rawByteData()
    pub fn rawByteData(&self) -> KmoBlockByteView {
        // java return ((KmoBlockData)this.rawData()).data;
        KmoBlockByteView(self.atom.borrow().rawData())
    }
}

// Native retained view of the same Java byte array, without copying it.
pub struct KmoBlockByteView(kfc::cn::wps::memor4j::KmoAtom::AtomDataRead<KmoBlockData>);
impl std::ops::Deref for KmoBlockByteView{type Target=[u8];fn deref(&self)->&[u8]{&self.0.data}}

// java public static class KmoRangeBlockInfo
pub struct KmoRangeBlockInfo {
    // java public final int firstBlkRow;
    pub firstBlkRow: i32,
    // java public final int lastBlkRow;
    pub lastBlkRow: i32,
    // java public final int firstBlkCol;
    pub firstBlkCol: i32,
    // java public final int lastBlkCol;
    pub lastBlkCol: i32,
    // java public final int firstRowInBlk;
    pub firstRowInBlk: i32,
    // java public final int lastRowInBlk;
    pub lastRowInBlk: i32,
    // java public final int firstColInBlk;
    pub firstColInBlk: i32,
    // java public final int lastColInBlk;
    pub lastColInBlk: i32,
}

impl KmoRangeBlockInfo {
    // java public KmoRangeBlockInfo(KmoRect range)
    pub fn KmoRangeBlockInfo(range: &KmoRect) -> Self {
        // java this.firstBlkRow = range.tl.row / 32;
        let firstBlkRow = range.tl.row / 32;
        // java this.lastBlkRow = range.br.row / 32;
        let lastBlkRow = range.br.row / 32;
        // java this.firstBlkCol = range.tl.col / 16;
        let firstBlkCol = range.tl.col / 16;
        // java this.lastBlkCol = range.br.col / 16;
        let lastBlkCol = range.br.col / 16;
        // java this.firstRowInBlk = range.tl.row % 32;
        let firstRowInBlk = range.tl.row % 32;
        // java this.lastRowInBlk = range.br.row % 32;
        let lastRowInBlk = range.br.row % 32;
        // java this.firstColInBlk = range.tl.col % 16;
        let firstColInBlk = range.tl.col % 16;
        // java this.lastColInBlk = range.br.col % 16;
        let lastColInBlk = range.br.col % 16;
        Self {
            firstBlkRow,
            lastBlkRow,
            firstBlkCol,
            lastBlkCol,
            firstRowInBlk,
            lastRowInBlk,
            firstColInBlk,
            lastColInBlk,
        }
    }
}
