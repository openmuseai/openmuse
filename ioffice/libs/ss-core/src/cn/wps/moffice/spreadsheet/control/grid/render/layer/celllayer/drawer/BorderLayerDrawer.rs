use super::super::CellsRenderLayer::CellsRenderLayer;
use super::BaseCellLayerDrawer::{BaseCellLayerDrawer, CellLayerRenderer};
use crate::cn::wps::moffice::spreadsheet::control::grid;
use crate::java_runtime::{Cell, Rc, RefCell, Weak};
use grid::render::{cache::BorderPath::BorderPath, data::BorderData::BorderData};
use grid::{
    core::{GridRange::GridRange, GridSheetLayoutInfo::GridSheetLayoutInfo},
    render::{
        data::GridClientData::{ClientRect, ClientRects, GridClientData},
        graphics::{Canvas, Paint, Path, Rect},
    },
};
pub struct BorderLayerDrawer {
    base: BaseCellLayerDrawer,
    mBorderData: BorderData,
    mBorderPath: BorderPath,
}
impl BorderLayerDrawer {
    pub fn new(p: Weak<CellsRenderLayer>) -> Self {
        Self {
            base: BaseCellLayerDrawer::new(p),
            mBorderData: BorderData::default(),
            mBorderPath: BorderPath::default(),
        }
    }
}
impl CellLayerRenderer for BorderLayerDrawer {
    fn base(&mut self) -> &mut BaseCellLayerDrawer {
        &mut self.base
    }
    fn name(&self) -> &'static str {
        "BorderLayerDrawer"
    }
    // java protected void doRenderer(Canvas canvas, Paint paint, GridRange range, GridSheetLayoutInfo gridLayoutInfo) {
    // java int firstRow = range.mFirstRow;
    // java int firstColumn = range.mFirstColumn;
    // java int lastRow = range.mLastRow;
    // java int lastColumn = range.mLastColumn;
    // java int startColumnX = this.mParent.mRangeFirstColX;
    // java int startRowY = this.mParent.mRangeFirstRowY;
    // java this.mBorderPath.reset(this.mParent.isStrokeWidthScale(), this.mParent.strokeWidthScale());
    // java if (firstRow > 0 && this.mParent.mCurClientRect.top == startRowY) {
    // java --firstRow;
    // java startRowY -= gridLayoutInfo.rowHeightInPixels(firstRow);
    // java }
    // java
    // java if (firstColumn > 0 && this.mParent.mCurClientRect.left == startColumnX) {
    // java --firstColumn;
    // java startColumnX -= gridLayoutInfo.columnWidthInPixels(firstColumn);
    // java }
    // java
    // java this.mRecycleRect.top = startRowY;
    // java this.mRecycleRect.bottom = startRowY;
    // java TableStyle shareTableStyle = this.mParent.mShareTableStyle;
    // java MergedRangeL2Cache mergedCache = this.mParent.mMergedRangeCache;
    // java CellXFCache cellXfCache = this.mParent.mCellXfCache;
    // java IGridSheet gridSheet = gridLayoutInfo.mGridSheet;
    // java Rect preClientRect = this.mParent.mPreClientRect;
    // java Rect additionalRect = this.mParent.mAdditionalRect;
    // java boolean isEmpty = additionalRect.isEmpty();
    // java boolean isRightToLeft = gridLayoutInfo.isSheetRightToLeft();
    // java
    // java for(int i = firstRow; i <= lastRow; ++i) {
    // java int height = gridLayoutInfo.rowHeightInPixels(i);
    // java if (height != 0) {
    // java this.mRecycleRect.left = startColumnX;
    // java this.mRecycleRect.right = startColumnX;
    // java Rect var10000 = this.mRecycleRect;
    // java var10000.bottom += height;
    // java
    // java for(int j = firstColumn; j <= lastColumn; ++j) {
    // java int width = gridLayoutInfo.columnWidthInPixels(j);
    // java if (width != 0) {
    // java var10000 = this.mRecycleRect;
    // java var10000.right += width;
    // java boolean needDraw = isEmpty ? !contains(preClientRect, this.mRecycleRect) : intersects(additionalRect, this.mRecycleRect);
    // java if (needDraw && this.mBorderData.reset(gridSheet, i, j, mergedCache, cellXfCache, shareTableStyle)) {
    // java this.mBorderPath.checkData(this.mBorderData, this.mRecycleRect, canvas, paint, isRightToLeft);
    // java }
    // java
    // java this.mRecycleRect.left = this.mRecycleRect.right;
    // java }
    // java }
    // java
    // java this.mRecycleRect.top = this.mRecycleRect.bottom;
    // java }
    // java }
    // java
    // java this.mBorderPath.draw(canvas, paint);
    // java }
    fn doRenderer(
        &mut self,
        c: &mut Canvas,
        p: &mut Paint,
        r: &GridRange,
        l: &GridSheetLayoutInfo,
    ) {
        let parent = self.base.parent();
        let (mut firstRow, mut firstCol) = (r.mFirstRow, r.mFirstColumn);
        let (mut startX, mut startY) = (parent.mRangeFirstColX.get(), parent.mRangeFirstRowY.get());
        self.mBorderPath
            .reset(parent.isStrokeWidthScale(), parent.strokeWidthScale());
        let cur = parent.curClientRect();
        if firstRow > 0 && cur.top == startY {
            firstRow -= 1;
            startY -= l.rowHeightInPixels(firstRow);
        }
        if firstCol > 0 && cur.left == startX {
            firstCol -= 1;
            startX -= l.columnWidthInPixels(firstCol);
        }
        let pre = parent.preClientRect();
        let additional = *parent.mAdditionalRect.borrow();
        let empty = additional.isEmpty();
        let merged = parent.mMergedRangeCache.borrow();
        self.base.mRecycleRect.top = startY;
        self.base.mRecycleRect.bottom = startY;
        for row in firstRow..=r.mLastRow {
            let height = l.rowHeightInPixels(row);
            if height != 0 {
                self.base.mRecycleRect.left = startX;
                self.base.mRecycleRect.right = startX;
                self.base.mRecycleRect.bottom += height;
                for col in firstCol..=r.mLastColumn {
                    let width = l.columnWidthInPixels(col);
                    if width != 0 {
                        self.base.mRecycleRect.right += width;
                        let rect = self.base.mRecycleRect;
                        let contains = pre.left < rect.left
                            && pre.top < rect.top
                            && pre.right > rect.right
                            && pre.bottom > rect.bottom;
                        let intersects = additional.left <= rect.right
                            && rect.left <= additional.right
                            && additional.top <= rect.bottom
                            && rect.top <= additional.bottom;
                        let need = if empty { !contains } else { intersects };
                        if need
                            && self.mBorderData.reset(
                                l.gridSheet().as_ref(),
                                row,
                                col,
                                Some(&merged),
                                Some(&parent.mCellXfCache),
                                parent.mShareTableStyle.as_ref(),
                            )
                        {
                            self.mBorderPath.checkData(
                                &self.mBorderData,
                                &rect,
                                c,
                                p,
                                l.isSheetRightToLeft(),
                            );
                        }
                        self.base.mRecycleRect.left = self.base.mRecycleRect.right;
                    }
                }
                self.base.mRecycleRect.top = self.base.mRecycleRect.bottom;
            }
        }
        self.mBorderPath.draw(c, p);
    }
}
