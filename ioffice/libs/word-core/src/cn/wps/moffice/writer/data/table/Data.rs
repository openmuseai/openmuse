// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.moffice.drawing.IPos
use drawing::cn::wps::moffice::drawing::IPos::*;
// Java: import cn.wps.moffice.drawing.graphics.SHD
use drawing::cn::wps::moffice::drawing::graphics::SHD::*;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::*;
// Java: import java.lang.Math
use stubs::java::lang::Math::Math;
// Java: import cn.wps.moffice.writer.data.table.grid.GridCols
use crate::cn::wps::moffice::writer::data::table::grid::GridCols::*;
// Java: import cn.wps.moffice.writer.data.value.BRC
use crate::cn::wps::moffice::writer::data::value::BRC::*;
// Java: import cn.wps.moffice.writer.data.value.TableBorders
use crate::cn::wps::moffice::writer::data::value::TableBorders::TableBorders;
// Java: import cn.wps.moffice.writer.data.value.TableCell
use crate::cn::wps::moffice::writer::data::value::TableCell::*;
// Java: import cn.wps.moffice.writer.data.value.TableLook
use crate::cn::wps::moffice::writer::data::value::TableLook::*;
// Java: import java.util.Arrays
// Java: import java.util.HashMap
// Java: import java.util.Map
// Java: import cn.wps.moffice.writer.data.table.TableJCData
use crate::cn::wps::moffice::writer::data::table::TableJCData::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
// Java: import cn.wps.moffice.writer.data.table.CellInfo
use crate::cn::wps::moffice::writer::data::table::CellInfo::*;
use std::collections::{HashMap};

type CellInfoHandle = Box<dyn CellInfo>;

// java public static class TableData
#[derive(Default)]
pub struct TableData {
    // java public TableJCData mTableJCData = null;
    pub(crate) mTableJCData: Option<TableJCData>,
    // java public IPos mPos = null;
    pub(crate) mPos: Option<Box<dyn IPos>>,
    // java public SHD mShd = null;
    pub(crate) mShd: Option<SHD>,
    // java public BRC[] mBRCs = new BRC[4];
    pub(crate) mBRCs: Vec<Option<BRC>>,
    // java public int mStyleId = 4095;
    pub(crate) mStyleId: i32,
    // java public TableLook mTableLook = null;
    pub(crate) mTableLook: Option<TableLook>,
    // java public TableBorders mTableBorders = null;
    pub(crate) mTableBorders: Option<TableBorders>,
    // java public boolean mIsRTL = false;
    pub(crate) mIsRTL: bool,
    // java public GridCols mGridCols = null;
    pub(crate) mGridCols: Option<Box<dyn GridCols>>,
    // java public boolean mWaitFitUpdate = false;
    pub(crate) mWaitFitUpdate: bool,
}

// java public static class RowData
#[derive(Default)]
pub struct RowData {
    // java public int mRowHeight = 0;
    pub(crate) mRowHeight: i32,
    // java public int[] mRgdxaCenter = null;
    pub(crate) mRgdxaCenter: Option<Vec<i32>>,
    // java public int[] mCellWidths = null;
    pub(crate) mCellWidths: Option<Vec<i32>>,
    // java public int mWidthBefore = 0;
    pub(crate) mWidthBefore: i32,
    // java public boolean mCantSplit = false;
    pub(crate) mCantSplit: bool,
    // java public boolean mIsRevisionDelete = false;
    pub(crate) mIsRevisionDelete: bool,
    // java public boolean mIsRevisionInsert = false;
    pub(crate) mIsRevisionInsert: bool,
    // java public boolean mIsFormatChange = false;
    pub(crate) mIsFormatChange: bool,
}

// java public static class CellData
#[derive(Default)]
pub struct CellData {
    // java public TableCell mTableCell = null;
    pub(crate) mTableCell: Option<TableCell>,
    // java public CellInfo mNextVertMergeCell = null;
    pub(crate) mNextVertMergeCell: Option<CellInfoHandle>,
    // java public CellInfo mPrevVertMergeCell = null;
    pub(crate) mPrevVertMergeCell: Option<CellInfoHandle>,
    // java public SHD mShd = null;
    pub(crate) mShd: Option<SHD>,
    // java public BRC mBrcLeft = null;
    pub(crate) mBrcLeft: Option<BRC>,
    // java public BRC mBrcTop = null;
    pub(crate) mBrcTop: Option<BRC>,
    // java public BRC mBrcRight = null;
    pub(crate) mBrcRight: Option<BRC>,
    // java public BRC mBrcBottom = null;
    pub(crate) mBrcBottom: Option<BRC>,
    // java public BRC mBrcFromTopLeftToBottomRight = null;
    pub(crate) mBrcFromTopLeftToBottomRight: Option<BRC>,
    // java public BRC mBrcFromTopRightToBottomLeft = null;
    pub(crate) mBrcFromTopRightToBottomLeft: Option<BRC>,
    // java public AutoFitInfo mAutoFitInfo = null;
    pub(crate) mAutoFitInfo: Option<AutoFitInfo>,
    // java public int mMarginTop = 0;
    pub(crate) mMarginTop: i32,
    // java public int mMarginLeft = 0;
    pub(crate) mMarginLeft: i32,
    // java public int mMarginBottom = 0;
    pub(crate) mMarginBottom: i32,
    // java public int mMarginRight = 0;
    pub(crate) mMarginRight: i32,
}

// java public static class AutoFitInfo
#[derive(Default, Clone)]
pub struct AutoFitInfo {
    // java private int mMaxWidth = 0;
    mMaxWidth: i32,
    // java private int mMinWidth = 0;
    mMinWidth: i32,
    // java private int mParaTotalMax = 0;
    mParaTotalMax: i32,
    // java private boolean mComplete = false;
    mComplete: bool,
    // java public AutoFitInfo mChildBestFitInfo = null;
    pub(crate) mChildBestFitInfo: Option<Box<AutoFitInfo>>,
}

// java public static class RebuildCache
#[derive(Default)]
pub struct RebuildCache {
    // java private Map<Long, AutoFitInfo> mMap = new HashMap();
    mMap: HashMap<i64, AutoFitInfo>,
    // java private boolean mRebuilding = false;
    mRebuilding: bool,
    // java private long mInvalidRange = 0L;
    mInvalidRange: i64,
    // java private int mInvalidCount = 0;
    mInvalidCount: i32,
}

// java public static class GridColData
#[derive(Default)]
pub struct GridColData {
    // java public int mLeftPos = 0;
    pub(crate) mLeftPos: i32,
    // java public int mWidth = 0;
    pub(crate) mWidth: i32,
    // java public int mOriWidth = 0;
    pub(crate) mOriWidth: i32,
    // java public boolean mIsAutoWidth = true;
    pub(crate) mIsAutoWidth: bool,
    // java public int mMinWidth = 0;
    pub(crate) mMinWidth: i32,
    // java public int mBestWidth = 0;
    pub(crate) mBestWidth: i32,
    // java public int mParaTotalMax = 0;
    pub(crate) mParaTotalMax: i32,
    // java public int mFitWidth = 0;
    pub(crate) mFitWidth: i32,
    // java public boolean mTooSmallToMerged;
    pub(crate) mTooSmallToMerged: bool,
}

// java public interface Data
pub trait Data {
}

// java public static class TableData
impl TableData {
    // java public void clear()
    pub fn clear(&mut self) {
      // java this.mPos = null;
      self.mPos = None;
      // java this.mShd = null;
      self.mShd = None;
      // java Arrays.fill(this.mBRCs, (Object)null);
      for brc in self.mBRCs.iter_mut() {
        *brc = None;
      }
      // java this.mStyleId = 4095;
      self.mStyleId = 4095;
      // java this.mTableLook = null;
      self.mTableLook = None;
      // java this.mTableBorders = null;
      self.mTableBorders = None;
      // java this.mIsRTL = false;
      self.mIsRTL = false;
      // java this.mWaitFitUpdate = false;
      self.mWaitFitUpdate = false;
    }

    // java public TableJCData getJC()
    pub fn getJC(&self) -> Option<&TableJCData> {
      // java return this.mTableJCData;
      return self.mTableJCData.as_ref();
    }

    // java public IPos getPos()
    pub fn getPos(&self) -> Option<&dyn IPos> {
      // java return this.mPos;
      return self.mPos.as_deref();
    }

    // java public SHD getSHD()
    pub fn getSHD(&self) -> Option<&SHD> {
      // java return this.mShd;
      return self.mShd.as_ref();
    }

    // java public TableLook getTableLook()
    pub fn getTableLook(&self) -> Option<&TableLook> {
      // java return this.mTableLook;
      return self.mTableLook.as_ref();
    }

    // java public TableBorders getTableBorders()
    pub fn getTableBorders(&self) -> Option<&TableBorders> {
      // java return this.mTableBorders;
      return self.mTableBorders.as_ref();
    }

    // java public int getStyleId()
    pub fn getStyleId(&self) -> i32 {
      // java return this.mStyleId;
      return self.mStyleId;
    }

    // java public BRC getBrcTop()
    pub fn getBrcTop(&self) -> Option<&BRC> {
      // java return this.mBRCs[0];
      return self.mBRCs[0usize].as_ref();
    }

    // java public BRC getBrcLeft()
    pub fn getBrcLeft(&self) -> Option<&BRC> {
      // java return this.mBRCs[1];
      return self.mBRCs[1usize].as_ref();
    }

    // java public BRC getBrcBottom()
    pub fn getBrcBottom(&self) -> Option<&BRC> {
      // java return this.mBRCs[2];
      return self.mBRCs[2usize].as_ref();
    }

    // java public BRC getBrcRight()
    pub fn getBrcRight(&self) -> Option<&BRC> {
      // java return this.mBRCs[3];
      return self.mBRCs[3usize].as_ref();
    }

    // java public boolean isRTL()
    pub fn isRTL(&self) -> bool {
      // java return this.mIsRTL;
      return self.mIsRTL;
    }

    // java public boolean isWaitFitUpdate()
    pub fn isWaitFitUpdate(&self) -> bool {
      // java return this.mWaitFitUpdate;
      return self.mWaitFitUpdate;
    }

    // java public void setWaitFitUpdate(boolean value)
    pub fn setWaitFitUpdate(&mut self, value: bool) {
      // java this.mWaitFitUpdate = value;
      self.mWaitFitUpdate = value;
    }

}

// java public static class RowData
impl RowData {
    // java public void clear()
    pub fn clear(&mut self) {
      // java this.mRowHeight = 0;
      self.mRowHeight = 0;
      // java this.mRgdxaCenter = null;
      self.mRgdxaCenter = None;
      // java this.mCellWidths = null;
      self.mCellWidths = None;
      // java this.mWidthBefore = 0;
      self.mWidthBefore = 0;
      // java this.mCantSplit = false;
      self.mCantSplit = false;
      // java this.mIsRevisionDelete = false;
      self.mIsRevisionDelete = false;
      // java this.mIsRevisionInsert = false;
      self.mIsRevisionInsert = false;
      // java this.mIsFormatChange = false;
      self.mIsFormatChange = false;
    }

    // java public int getRowHeight()
    pub fn getRowHeight(&self) -> i32 {
      // java return this.mRowHeight;
      return self.mRowHeight;
    }

    // java public int getWidthBefore()
    pub fn getWidthBefore(&self) -> i32 {
      // java return this.mWidthBefore;
      return self.mWidthBefore;
    }

    // java public int getRgdxaCount()
    pub fn getRgdxaCount(&self) -> i32 {
      // java return null == this.mRgdxaCenter ? 0 : this.mRgdxaCenter.length;
      return (if self.mRgdxaCenter.is_none() { 0 } else { (self.mRgdxaCenter.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) });
    }

    // java public int getRgdxa(int index)
    pub fn getRgdxa(&self, index: i32) -> i32 {
      // java return 0 <= index && null != this.mRgdxaCenter && index < this.mRgdxaCenter.length ? this.mRgdxaCenter[index] : 0;
      return (if 0 <= index && self.mRgdxaCenter.is_some() && index < (self.mRgdxaCenter.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) { self.mRgdxaCenter.as_ref().unwrap()[(index) as usize] } else { 0 });
    }

    // java public int getCellWidth(int index)
    pub fn getCellWidth(&self, index: i32) -> i32 {
      // java return 0 <= index && null != this.mCellWidths && index < this.mCellWidths.length ? this.mCellWidths[index] : 0;
      return (if 0 <= index && self.mCellWidths.is_some() && index < (self.mCellWidths.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) { self.mCellWidths.as_ref().unwrap()[(index) as usize] } else { 0 });
    }

    // java public boolean cantSplit()
    pub fn cantSplit(&self) -> bool {
      // java return this.mCantSplit;
      return self.mCantSplit;
    }

    // java public boolean isRevisionDelete()
    pub fn isRevisionDelete(&self) -> bool {
      // java return this.mIsRevisionDelete;
      return self.mIsRevisionDelete;
    }

    // java public boolean isRevisionInsert()
    pub fn isRevisionInsert(&self) -> bool {
      // java return this.mIsRevisionInsert;
      return self.mIsRevisionInsert;
    }

    // java public boolean isFormatChange()
    pub fn isFormatChange(&self) -> bool {
      // java return this.mIsFormatChange;
      return self.mIsFormatChange;
    }

}

// java public static class CellData
impl CellData {
    // java public void clear()
    pub fn clear(&mut self) {
      // java this.mTableCell = null;
      self.mTableCell = None;
      // java this.mShd = null;
      self.mShd = None;
      // java this.mBrcLeft = null;
      self.mBrcLeft = None;
      // java this.mBrcTop = null;
      self.mBrcTop = None;
      // java this.mBrcRight = null;
      self.mBrcRight = None;
      // java this.mBrcBottom = null;
      self.mBrcBottom = None;
      // java this.mBrcFromTopLeftToBottomRight = null;
      self.mBrcFromTopLeftToBottomRight = None;
      // java this.mBrcFromTopRightToBottomLeft = null;
      self.mBrcFromTopRightToBottomLeft = None;
      // java this.mMarginTop = 0;
      self.mMarginTop = 0;
      // java this.mMarginLeft = 0;
      self.mMarginLeft = 0;
      // java this.mMarginBottom = 0;
      self.mMarginBottom = 0;
      // java this.mMarginRight = 0;
      self.mMarginRight = 0;
    }

    // java public BRC getBrcTop()
    pub fn getBrcTop(&self) -> Option<&BRC> {
      // java return this.mBrcTop;
      return self.mBrcTop.as_ref();
    }

    // java public void setBrcTop(BRC brcTop)
    pub fn setBrcTop(&mut self, brcTop: Option<BRC>) {
      // java this.mBrcTop = brcTop;
      self.mBrcTop = brcTop;
    }

    // java public BRC getBrcLeft()
    pub fn getBrcLeft(&self) -> Option<&BRC> {
      // java return this.mBrcLeft;
      return self.mBrcLeft.as_ref();
    }

    // java public void setBrcLeft(BRC brcLeft)
    pub fn setBrcLeft(&mut self, brcLeft: Option<BRC>) {
      // java this.mBrcLeft = brcLeft;
      self.mBrcLeft = brcLeft;
    }

    // java public BRC getBrcBottom()
    pub fn getBrcBottom(&self) -> Option<&BRC> {
      // java return this.mBrcBottom;
      return self.mBrcBottom.as_ref();
    }

    // java public void setBrcBottom(BRC brcBottom)
    pub fn setBrcBottom(&mut self, brcBottom: Option<BRC>) {
      // java this.mBrcBottom = brcBottom;
      self.mBrcBottom = brcBottom;
    }

    // java public BRC getBrcRight()
    pub fn getBrcRight(&self) -> Option<&BRC> {
      // java return this.mBrcRight;
      return self.mBrcRight.as_ref();
    }

    // java public void setBrcRight(BRC brcRight)
    pub fn setBrcRight(&mut self, brcRight: Option<BRC>) {
      // java this.mBrcRight = brcRight;
      self.mBrcRight = brcRight;
    }

    // java public BRC getBrcFromTopLeftToBottomRight()
    pub fn getBrcFromTopLeftToBottomRight(&self) -> Option<&BRC> {
      // java return this.mBrcFromTopLeftToBottomRight;
      return self.mBrcFromTopLeftToBottomRight.as_ref();
    }

    // java public void setBrcFromTopLeftToBottomRight(BRC brcTopLeftToBottomRight)
    pub fn setBrcFromTopLeftToBottomRight(&mut self, brcTopLeftToBottomRight: Option<BRC>) {
      // java this.mBrcFromTopLeftToBottomRight = brcTopLeftToBottomRight;
      self.mBrcFromTopLeftToBottomRight = brcTopLeftToBottomRight;
    }

    // java public BRC getBrcFromTopRightToBottomLeft()
    pub fn getBrcFromTopRightToBottomLeft(&self) -> Option<&BRC> {
      // java return this.mBrcFromTopRightToBottomLeft;
      return self.mBrcFromTopRightToBottomLeft.as_ref();
    }

    // java public void setBrcFromTopRightToBottomLeft(BRC brcTopRightToBottomLeft)
    pub fn setBrcFromTopRightToBottomLeft(&mut self, brcTopRightToBottomLeft: Option<BRC>) {
      // java this.mBrcFromTopRightToBottomLeft = brcTopRightToBottomLeft;
      self.mBrcFromTopRightToBottomLeft = brcTopRightToBottomLeft;
    }

    // java public boolean isVertMergeCell()
    pub fn isVertMergeCell(&self) -> bool {
      // java return null != this.mNextVertMergeCell || null != this.mPrevVertMergeCell;
      return self.mNextVertMergeCell.is_some() || self.mPrevVertMergeCell.is_some();
    }

    // java public boolean isFirstVertMergeCell()
    pub fn isFirstVertMergeCell(&self) -> bool {
      // java return null == this.mPrevVertMergeCell && this.isVertMergeCell();
      return self.mPrevVertMergeCell.is_none() && self.isVertMergeCell();
    }

    // java public boolean isLastVertMergeCell()
    pub fn isLastVertMergeCell(&self) -> bool {
      // java return null == this.mNextVertMergeCell && this.isVertMergeCell();
      return self.mNextVertMergeCell.is_none() && self.isVertMergeCell();
    }

    // java public CellInfo getPrevVertMergeCell()
    pub fn getPrevVertMergeCell(&self) -> Option<&CellInfoHandle> {
      // java return this.mPrevVertMergeCell;
      return self.mPrevVertMergeCell.as_ref();
    }

    // java public CellInfo getNextVertMergeCell()
    pub fn getNextVertMergeCell(&self) -> Option<&CellInfoHandle> {
      // java return this.mNextVertMergeCell;
      return self.mNextVertMergeCell.as_ref();
    }

    // java public int getHoriMerge()
    pub fn getHoriMerge(&self) -> i32 {
      // java Assert.assertNotNull(this.mTableCell);
      Assert::assertNotNull(self.mTableCell.as_ref());
      // java return null == this.mTableCell ? 0 : this.mTableCell.getHoriMerge();
      return (if self.mTableCell.is_none() { 0 } else { self.mTableCell.as_ref().unwrap().getHoriMerge() });
    }

    // java public int getVertMerge()
    pub fn getVertMerge(&self) -> i32 {
      // java Assert.assertNotNull(this.mTableCell);
      Assert::assertNotNull(self.mTableCell.as_ref());
      // java if (null == this.mTableCell)
      if self.mTableCell.is_none() {
        // java return 0;
        return 0;
      }
      // java else
      else {
        // java int verMerge = this.mTableCell.getVertMerge();
        let mut verMerge: i32 = self.mTableCell.as_ref().unwrap().getVertMerge();
        // java if (0 == verMerge)
        if 0 == verMerge {
          // java return verMerge;
          return verMerge;
        }
        // java else if (!this.isVertMergeCell())
        else if !self.isVertMergeCell() {
          // java return 0;
          return 0;
        }
        // java else
        else {
          // java return this.isFirstVertMergeCell() ? 3 : 1;
          return (if self.isFirstVertMergeCell() { 3 } else { 1 });
        }
      }
    }

    // java public int getVertAlign()
    pub fn getVertAlign(&self) -> i32 {
      // java Assert.assertNotNull(this.mTableCell);
      Assert::assertNotNull(self.mTableCell.as_ref());
      // java return null == this.mTableCell ? 0 : this.mTableCell.getVertAlign();
      return (if self.mTableCell.is_none() { 0 } else { self.mTableCell.as_ref().unwrap().getVertAlign() });
    }

    // java public int getTextFlow()
    pub fn getTextFlow(&self) -> i32 {
      // java Assert.assertNotNull(this.mTableCell);
      Assert::assertNotNull(self.mTableCell.as_ref());
      // java return null == this.mTableCell ? 0 : this.mTableCell.getTextFlow();
      return (if self.mTableCell.is_none() { 0 } else { self.mTableCell.as_ref().unwrap().getTextFlow() });
    }

    // java public boolean getFitText()
    pub fn getFitText(&self) -> bool {
      // java Assert.assertNotNull(this.mTableCell);
      Assert::assertNotNull(self.mTableCell.as_ref());
      // java return null == this.mTableCell ? true : this.mTableCell.getFitText();
      return (if self.mTableCell.is_none() { true } else { self.mTableCell.as_ref().unwrap().getFitText() });
    }

    // java public boolean isAutoWidth()
    pub fn isAutoWidth(&self) -> bool {
      // java Assert.assertNotNull(this.mTableCell);
      Assert::assertNotNull(self.mTableCell.as_ref());
      // java if (null == this.mTableCell)
      if self.mTableCell.is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java return this.mTableCell.getWidthType() == 1;
        return self.mTableCell.as_ref().unwrap().getWidthType() == 1;
      }
    }

    // java public boolean getNoWrap()
    pub fn getNoWrap(&self) -> bool {
      // java Assert.assertNotNull(this.mTableCell);
      Assert::assertNotNull(self.mTableCell.as_ref());
      // java return null == this.mTableCell ? false : this.mTableCell.getNoWrap();
      return (if self.mTableCell.is_none() { false } else { self.mTableCell.as_ref().unwrap().getNoWrap() });
    }

    // java public SHD getSHD()
    pub fn getSHD(&self) -> Option<&SHD> {
      // java return this.mShd;
      return self.mShd.as_ref();
    }

    // java public int getMarginTop()
    pub fn getMarginTop(&self) -> i32 {
      // java return this.mMarginTop;
      return self.mMarginTop;
    }

    // java public int getMarginLeft()
    pub fn getMarginLeft(&self) -> i32 {
      // java return this.mMarginLeft;
      return self.mMarginLeft;
    }

    // java public int getMarginBottom()
    pub fn getMarginBottom(&self) -> i32 {
      // java return this.mMarginBottom;
      return self.mMarginBottom;
    }

    // java public int getMarginRight()
    pub fn getMarginRight(&self) -> i32 {
      // java return this.mMarginRight;
      return self.mMarginRight;
    }

    // java public int getMinWidth()
    pub fn getMinWidth(&mut self) -> i32 {
      // java return this.mMarginLeft + this.mMarginRight + this.getAutoFitInfo().getMinWidth();
      return self.mMarginLeft + self.mMarginRight + self.getAutoFitInfo().getMinWidth();
    }

    // java public int getMaxWidth()
    pub fn getMaxWidth(&mut self) -> i32 {
      // java return this.mMarginLeft + this.mMarginRight + this.getAutoFitInfo().getMaxWidth();
      return self.mMarginLeft + self.mMarginRight + self.getAutoFitInfo().getMaxWidth();
    }

    // java public int getParaTotalMax()
    pub fn getParaTotalMax(&mut self) -> i32 {
      // java int paraTotal = this.getAutoFitInfo().getParaTotalMax();
      let mut paraTotal: i32 = self.getAutoFitInfo().getParaTotalMax();
      // java return paraTotal == 0 ? 0 : this.mMarginLeft + this.mMarginRight + paraTotal;
      return (if paraTotal == 0 { 0 } else { self.mMarginLeft + self.mMarginRight + paraTotal });
    }

    // java public AutoFitInfo getAutoFitInfo()
    pub fn getAutoFitInfo(&mut self) -> &mut AutoFitInfo {
      // java if (null == this.mAutoFitInfo)
      if self.mAutoFitInfo.is_none() {
        // java this.mAutoFitInfo = new AutoFitInfo();
        self.mAutoFitInfo = Some(AutoFitInfo::default());
      }
      // java return this.mAutoFitInfo;
      return self.mAutoFitInfo.as_mut().unwrap();
    }

}

// java public static class AutoFitInfo
impl AutoFitInfo {
    // java public boolean isComplete()
    pub fn isComplete(&self) -> bool {
      // java return this.mComplete;
      return self.mComplete;
    }

    // java public void setComplete(boolean complete)
    pub fn setComplete(&mut self, complete: bool) {
      // java this.mComplete = complete;
      self.mComplete = complete;
    }

    // java public int getMinWidth()
    pub fn getMinWidth(&self) -> i32 {
      // java return this.mChildBestFitInfo != null ? Math.max(this.mMinWidth, this.mChildBestFitInfo.mMinWidth) : this.mMinWidth;
      return (if let Some(child) = self.mChildBestFitInfo.as_ref() { std::cmp::max(self.mMinWidth, child.mMinWidth) } else { self.mMinWidth });
    }

    // java public int getMaxWidth()
    pub fn getMaxWidth(&self) -> i32 {
      // java return this.mChildBestFitInfo != null ? Math.max(this.mMaxWidth, this.mChildBestFitInfo.mMaxWidth) : this.mMaxWidth;
      return (if let Some(child) = self.mChildBestFitInfo.as_ref() { std::cmp::max(self.mMaxWidth, child.mMaxWidth) } else { self.mMaxWidth });
    }

    // java public int getParaTotalMax()
    pub fn getParaTotalMax(&self) -> i32 {
      // java return this.mChildBestFitInfo != null ? Math.max(this.mParaTotalMax, this.mChildBestFitInfo.mParaTotalMax) : this.mParaTotalMax;
      return (if let Some(child) = self.mChildBestFitInfo.as_ref() { std::cmp::max(self.mParaTotalMax, child.mParaTotalMax) } else { self.mParaTotalMax });
    }

    // java public void clear()
    pub fn clear(&mut self) {
      // java this.mMaxWidth = 0;
      self.mMaxWidth = 0;
      // java this.mMinWidth = 0;
      self.mMinWidth = 0;
      // java this.mParaTotalMax = 0;
      self.mParaTotalMax = 0;
      // java this.mComplete = false;
      self.mComplete = false;
      // java if (this.mChildBestFitInfo != null)
      if let Some(mChildBestFitInfo) = self.mChildBestFitInfo.as_mut() {
        // java this.mChildBestFitInfo.clear();
        mChildBestFitInfo.clear();
      }
    }

    // java public boolean updateFrom(AutoFitInfo src)
    pub fn updateFrom(&mut self, src: &AutoFitInfo) -> bool {
      // java boolean updated = false;
      let mut updated: bool = false;
      // java if (this.mComplete != src.mComplete)
      if self.mComplete != src.mComplete {
        // java this.mComplete = src.mComplete;
        self.mComplete = src.mComplete;
        // java updated = true;
        updated = true;
      }
      // java if (this.mMinWidth != src.mMinWidth)
      if self.mMinWidth != src.mMinWidth {
        // java this.mMinWidth = src.mMinWidth;
        self.mMinWidth = src.mMinWidth;
        // java updated = true;
        updated = true;
      }
      // java if (this.mMaxWidth != src.mMaxWidth)
      if self.mMaxWidth != src.mMaxWidth {
        // java this.mMaxWidth = src.mMaxWidth;
        self.mMaxWidth = src.mMaxWidth;
        // java updated = true;
        updated = true;
      }
      // java if (src.mChildBestFitInfo != null)
      if let Some(src_mChildBestFitInfo) = src.mChildBestFitInfo.as_ref() {
        // java if (this.mChildBestFitInfo == null)
        if self.mChildBestFitInfo.is_none() {
          // java this.mChildBestFitInfo = new AutoFitInfo();
          self.mChildBestFitInfo = Some(Box::new(AutoFitInfo::default()));
        }
        // java updated |= this.mChildBestFitInfo.updateFrom(src.mChildBestFitInfo);
        updated = updated | self.mChildBestFitInfo.as_mut().unwrap().updateFrom(src_mChildBestFitInfo);
      }
      // java if (this.mParaTotalMax != src.mParaTotalMax)
      if self.mParaTotalMax != src.mParaTotalMax {
        // java this.mParaTotalMax = src.mParaTotalMax;
        self.mParaTotalMax = src.mParaTotalMax;
        // java updated = true;
        updated = true;
      }
      // java return updated;
      return updated;
    }

    // java public boolean commitNewInfo(int minWidth, int maxWidth, int paraTotalMax)
    pub fn commitNewInfo(&mut self, minWidth: i32, maxWidth: i32, paraTotalMax: i32) -> bool {
      // java if (minWidth > this.mMinWidth)
      if minWidth > self.mMinWidth {
        // java this.mMinWidth = minWidth;
        self.mMinWidth = minWidth;
      }
      // java boolean maxUpdate = false;
      let mut maxUpdate: bool = false;
      // java if (maxWidth > this.mMaxWidth)
      if maxWidth > self.mMaxWidth {
        // java this.mMaxWidth = maxWidth;
        self.mMaxWidth = maxWidth;
        // java maxUpdate = true;
        maxUpdate = true;
      }
      // java if (paraTotalMax > this.mParaTotalMax)
      if paraTotalMax > self.mParaTotalMax {
        // java this.mParaTotalMax = paraTotalMax;
        self.mParaTotalMax = paraTotalMax;
        // java maxUpdate = true;
        maxUpdate = true;
      }
      // java this.mComplete = true;
      self.mComplete = true;
      // java return maxUpdate;
      return maxUpdate;
    }

    // java public void addNewMaxInfo(int minWidth, int maxWidth)
    pub fn addNewMaxInfo(&mut self, minWidth: i32, maxWidth: i32) {
      // java if (minWidth > this.mMinWidth)
      if minWidth > self.mMinWidth {
        // java this.mMinWidth = minWidth;
        self.mMinWidth = minWidth;
      }
      // java this.mMaxWidth += maxWidth;
      self.mMaxWidth = self.mMaxWidth + maxWidth;
      // java this.mComplete = true;
      self.mComplete = true;
    }

    // java public void commitNewChild(int minWidth, int maxWidth, int paraTotalMax)
    pub fn commitNewChild(&mut self, minWidth: i32, maxWidth: i32, paraTotalMax: i32) {
      // java if (this.mChildBestFitInfo == null)
      if self.mChildBestFitInfo.is_none() {
        // java this.mChildBestFitInfo = new AutoFitInfo();
        self.mChildBestFitInfo = Some(Box::new(AutoFitInfo::default()));
      }
      // java this.mChildBestFitInfo.commitNewInfo(minWidth, maxWidth, paraTotalMax);
      self.mChildBestFitInfo.as_mut().unwrap().commitNewInfo(minWidth, maxWidth, paraTotalMax);
      // java this.mComplete = true;
      self.mComplete = true;
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java StringBuilder buf = new StringBuilder();
      let mut buf: String = String::new();
      // java buf.append("autoFit:");
      buf.push_str("autoFit:");
      // java buf.append("--min " + this.mMinWidth);
      buf.push_str(&format!("--min {}", self.mMinWidth));
      // java buf.append("--max " + this.mMaxWidth);
      buf.push_str(&format!("--max {}", self.mMaxWidth));
      // java buf.append("--ParaMax " + this.mParaTotalMax);
      buf.push_str(&format!("--ParaMax {}", self.mParaTotalMax));
      // java return buf.toString();
      return buf;
    }

}

// java public static class RebuildCache
impl RebuildCache {
    // java public boolean isRebuilding()
    pub fn isRebuilding(&self) -> bool {
      // java return this.mRebuilding;
      return self.mRebuilding;
    }

    // java public void start()
    pub fn start(&mut self) {
      // java this.mRebuilding = true;
      self.mRebuilding = true;
      // java this.mInvalidRange = 0L;
      self.mInvalidRange = 0;
      // java this.mInvalidCount = 0;
      self.mInvalidCount = 0;
      // java this.mMap.clear();
      self.mMap.clear();
    }

    // java public void end()
    pub fn end(&mut self) {
      // java this.mRebuilding = false;
      self.mRebuilding = false;
      // java this.mMap.clear();
      self.mMap.clear();
    }

    // java public void backup(int level, int cellStartCp, int length, AutoFitInfo info)
    pub fn backup(&mut self, level: i32, cellStartCp: i32, length: i32, info: AutoFitInfo) {
      // java long key = this.getKey(level, cellStartCp, length);
      let mut key: i64 = self.getKey(level, cellStartCp, length);
      // java this.mMap.put(key, info);
      self.mMap.insert(key, info);
    }

    // java public AutoFitInfo get(int level, int cellStartCp, int length)
    pub fn get(&self, level: i32, cellStartCp: i32, length: i32) -> Option<&AutoFitInfo> {
      // java long key = this.getKey(level, cellStartCp, length);
      let mut key: i64 = self.getKey(level, cellStartCp, length);
      // java return (AutoFitInfo)this.mMap.get(key);
      return self.mMap.get(&key);
    }

    // java public void addInvalidRange(long invalidRange)
    pub fn addInvalidRange(&mut self, invalidRange: i64) {
      // java ++this.mInvalidCount;
      { let _tmp = self.mInvalidCount; self.mInvalidCount += 1; _tmp };
      // java this.mInvalidRange = RANGEFactory.include(this.mInvalidRange, invalidRange);
      self.mInvalidRange = RANGEFactory::include__long_long(self.mInvalidRange, invalidRange);
    }

    // java public long getInvalidRange()
    pub fn getInvalidRange(&self) -> i64 {
      // java return this.mInvalidRange;
      return self.mInvalidRange;
    }

    // java public long getInvalidCount()
    pub fn getInvalidCount(&self) -> i64 {
      // java return (long)this.mInvalidCount;
      return ((self.mInvalidCount) as i64);
    }

    // java private long getKey(int level, int cellStartCp, int length)
    pub fn getKey(&self, level: i32, cellStartCp: i32, length: i32) -> i64 {
      // java int hight = level << 24 | length;
      let mut hight: i32 = level << 24 | length;
      // java return (long)hight << 32 | (long)cellStartCp;
      return ((hight) as i64) << 32 | ((cellStartCp) as i64);
    }

}

// java public static class GridColData
impl GridColData {
    // java public void clear()
    pub fn clear(&mut self) {
      // java this.mLeftPos = 0;
      self.mLeftPos = 0;
      // java this.mWidth = 0;
      self.mWidth = 0;
      // java this.mOriWidth = 0;
      self.mOriWidth = 0;
      // java this.mIsAutoWidth = true;
      self.mIsAutoWidth = true;
      // java this.clearFitInfo();
      self.clearFitInfo();
    }

    // java public void setOriWidth(int w)
    pub fn setOriWidth(&mut self, w: i32) {
      // java if (w >= 0)
      if w >= 0 {
        // java this.mOriWidth = w;
        self.mOriWidth = w;
      }
      // java else if (this.mWidth > 0)
      else if self.mWidth > 0 {
        // java this.mOriWidth = this.mWidth;
        self.mOriWidth = self.mWidth;
      }
    }

    // java public void clearFitInfo()
    pub fn clearFitInfo(&mut self) {
      // java this.mMinWidth = 0;
      self.mMinWidth = 0;
      // java this.mBestWidth = 0;
      self.mBestWidth = 0;
      // java this.mParaTotalMax = 0;
      self.mParaTotalMax = 0;
      // java this.mFitWidth = 0;
      self.mFitWidth = 0;
      // java this.mTooSmallToMerged = false;
      self.mTooSmallToMerged = false;
    }

    // java public int getLeftPos()
    pub fn getLeftPos(&self) -> i32 {
      // java return this.mLeftPos;
      return self.mLeftPos;
    }

    // java public int getWidth()
    pub fn getWidth(&self) -> i32 {
      // java return this.mWidth;
      return self.mWidth;
    }

    // java public int getRightPos()
    pub fn getRightPos(&self) -> i32 {
      // java return this.mLeftPos + this.mWidth;
      return self.mLeftPos + self.mWidth;
    }

    // java public int getBestAdd()
    pub fn getBestAdd(&self) -> i32 {
      // java return this.mBestWidth - this.mOriWidth;
      return self.mBestWidth - self.mOriWidth;
    }

    // java public int getParaMaxAddNeed()
    pub fn getParaMaxAddNeed(&self) -> i32 {
      // java return this.mParaTotalMax > this.mBestWidth ? this.mParaTotalMax - this.mBestWidth : 0;
      return (if self.mParaTotalMax > self.mBestWidth { self.mParaTotalMax - self.mBestWidth } else { 0 });
    }

    // java public boolean checkFitChanged()
    pub fn checkFitChanged(&self) -> bool {
      // java return Math.abs(this.mFitWidth - this.mWidth) > 2;
      return Math::abs(self.mFitWidth - self.mWidth) > 2;
    }

    // java public void applyAutoFitChange(int leftPos)
    pub fn applyAutoFitChange(&mut self, leftPos: i32) {
      // java this.mLeftPos = leftPos;
      self.mLeftPos = leftPos;
      // java if (this.mWidth > this.mFitWidth && this.mFitWidth < 40)
      if self.mWidth > self.mFitWidth && self.mFitWidth < 40 {
        // java this.mTooSmallToMerged = true;
        self.mTooSmallToMerged = true;
      }
      // java this.mWidth = this.mFitWidth;
      self.mWidth = self.mFitWidth;
    }

}
