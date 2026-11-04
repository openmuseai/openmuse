#![allow(non_snake_case)]

use super::GridCellImpl::GridCellImpl;

// Java: public class GridColImpl implements GridCol
#[derive(Clone, Debug, Default)]
pub struct GridColImpl {
    // Java field: Data.GridColData.mLeftPos
    pub mLeftPos: i32,
    // Java field: Data.GridColData.mWidth
    pub mWidth: i32,
    // Java field: Data.GridColData.mOriWidth
    pub mOriWidth: i32,
    // Java field: Data.GridColData.mIsAutoWidth = true
    pub mIsAutoWidth: bool,
    // Java field: Data.GridColData.mMinWidth
    pub mMinWidth: i32,
    // Java field: Data.GridColData.mBestWidth
    pub mBestWidth: i32,
    // Java field: Data.GridColData.mParaTotalMax
    pub mParaTotalMax: i32,
    // Java field: Data.GridColData.mFitWidth
    pub mFitWidth: i32,
    // Java field: Data.GridColData.mTooSmallToMerged
    pub mTooSmallToMerged: bool,
    // Java field: ArrayList<GridCell> mCells, indexed by row
    pub mCells: Vec<Option<GridCellImpl>>,
}

impl GridColImpl {
    pub fn new(left: i32, width: i32) -> Self {
        Self {
            mLeftPos: left,
            mWidth: width,
            mOriWidth: 0,
            mIsAutoWidth: true,
            mMinWidth: 0,
            mBestWidth: 0,
            mParaTotalMax: 0,
            mFitWidth: 0,
            mTooSmallToMerged: false,
            mCells: Vec::new(),
        }
    }

    // Java: Data.GridColData.clearFitInfo()
    pub fn clearFitInfo(&mut self) {
        // Java: this.mMinWidth = 0; this.mBestWidth = 0; this.mParaTotalMax = 0; this.mFitWidth = 0; this.mTooSmallToMerged = false;
        self.mMinWidth = 0;
        self.mBestWidth = 0;
        self.mParaTotalMax = 0;
        self.mFitWidth = 0;
        self.mTooSmallToMerged = false;
    }

    // Java: int getLeftPos()
    pub fn getLeftPos(&self) -> i32 {
        self.mLeftPos
    }

    // Java: int getWidth()
    pub fn getWidth(&self) -> i32 {
        self.mWidth
    }

    // Java: int getRightPos()
    pub fn getRightPos(&self) -> i32 {
        self.mLeftPos + self.mWidth
    }

    // Java: void setCell(int rowIndex, GridCell cellInfo)
    pub fn setCell(&mut self, row_index: usize, cell: GridCellImpl) {
        if row_index >= self.mCells.len() {
            self.mCells.resize_with(row_index + 1, || None);
        }
        self.mCells[row_index] = Some(cell);
    }

    // Java: GridCell getCell(int rowIndex)
    pub fn getCell(&self, row_index: usize) -> Option<&GridCellImpl> {
        self.mCells.get(row_index).and_then(|cell| cell.as_ref())
    }

    pub fn getCellMut(&mut self, row_index: usize) -> Option<&mut GridCellImpl> {
        self.mCells.get_mut(row_index).and_then(|cell| cell.as_mut())
    }

    // Java: public int getBestAdd()
    pub fn getBestAdd(&self) -> i32 {
        // Java: return this.mBestWidth - this.mOriWidth;
        self.mBestWidth - self.mOriWidth
    }

    // Java: public int getParaMaxAddNeed()
    pub fn getParaMaxAddNeed(&self) -> i32 {
        // Java: return this.mParaTotalMax > this.mBestWidth ? this.mParaTotalMax - this.mBestWidth : 0;
        if self.mParaTotalMax > self.mBestWidth {
            self.mParaTotalMax - self.mBestWidth
        } else {
            0
        }
    }

    // Java: public boolean checkFitChanged()
    pub fn checkFitChanged(&self) -> bool {
        // Java: return Math.abs(this.mFitWidth - this.mWidth) > 2;
        (self.mFitWidth - self.mWidth).abs() > 2
    }

    // Java: public void applyAutoFitChange(int leftPos)
    pub fn applyAutoFitChange(&mut self, left_pos: i32) {
        // Java: this.mLeftPos = leftPos;
        self.mLeftPos = left_pos;
        // Java: if (this.mWidth > this.mFitWidth && this.mFitWidth < 40)
        if self.mWidth > self.mFitWidth && self.mFitWidth < 40 {
            // Java: this.mTooSmallToMerged = true;
            self.mTooSmallToMerged = true;
        }
        // Java: this.mWidth = this.mFitWidth;
        self.mWidth = self.mFitWidth;
    }
}
