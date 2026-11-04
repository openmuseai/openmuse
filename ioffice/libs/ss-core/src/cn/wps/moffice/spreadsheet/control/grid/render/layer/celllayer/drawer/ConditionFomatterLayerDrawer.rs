use super::super::CellsRenderLayer::CellsRenderLayer;
use super::BaseCellLayerDrawer::{BaseCellLayerDrawer, CellLayerRenderer};
use crate::cn::wps::moffice::spreadsheet::control::grid;
use crate::java_runtime::{Cell, Rc, RefCell, Weak};
use grid::render::data::ConditionFormatterData::ConditionFormatterData;
use grid::{
    core::{GridRange::GridRange, GridSheetLayoutInfo::GridSheetLayoutInfo},
    render::{
        data::GridClientData::{ClientRect, ClientRects, GridClientData},
        graphics::{Canvas, Paint, Path, Rect},
    },
};
pub struct ConditionFomatterLayerDrawer {
    base: BaseCellLayerDrawer,
    data: ConditionFormatterData,
}
impl ConditionFomatterLayerDrawer {
    pub fn new(p: Weak<CellsRenderLayer>) -> Self {
        Self {
            base: BaseCellLayerDrawer::new(p),
            data: ConditionFormatterData::default(),
        }
    }
}
impl CellLayerRenderer for ConditionFomatterLayerDrawer {
    fn base(&mut self) -> &mut BaseCellLayerDrawer {
        &mut self.base
    }
    fn name(&self) -> &'static str {
        "ConditionFomatterLayerDrawer"
    }
    // java protected void doRenderer(Canvas canvas, Paint paint, GridRange range, GridSheetLayoutInfo gridLayoutInfo) {
    // java KmoConditionFormatter cfFormatter = gridLayoutInfo.mGridSheet.coreSheet().getCFormatter();
    // java if (!cfFormatter.isEmpty()) {
    // java MRULongKeyCache<KmoConditionFormatter.CfCacheItem> cfResultCache = cfFormatter.getResultCache();
    // java if (cfResultCache.size() > 0) {
    // java int firstColumn = range.mFirstColumn;
    // java int firstRow = range.mFirstRow;
    // java int startColumnX = this.mParent.mRangeFirstColX;
    // java int startRowY = this.mParent.mRangeFirstRowY;
    // java this.mRecycleRect.top = startRowY;
    // java this.mRecycleRect.bottom = startRowY;
    // java IGridSheet sheet = gridLayoutInfo.mGridSheet;
    // java Rect preClientRect = this.mParent.mPreClientRect;
    // java Rect additionalRect = this.mParent.mAdditionalRect;
    // java boolean isEmpty = additionalRect.isEmpty();
    // java int i = firstRow;
    // java
    // java for(int lastRow = range.mLastRow; i <= lastRow; ++i) {
    // java int height = gridLayoutInfo.rowHeightInPixels(i);
    // java if (height != 0) {
    // java this.mRecycleRect.left = startColumnX;
    // java this.mRecycleRect.right = startColumnX;
    // java Rect var10000 = this.mRecycleRect;
    // java var10000.bottom += height;
    // java int j = firstColumn;
    // java
    // java for(int lastCol = range.mLastColumn; j <= lastCol; ++j) {
    // java int width = gridLayoutInfo.columnWidthInPixels(j);
    // java if (width != 0) {
    // java var10000 = this.mRecycleRect;
    // java var10000.right += width;
    // java boolean needDraw = isEmpty ? !preClientRect.contains(this.mRecycleRect) : Rect.intersects(additionalRect, this.mRecycleRect);
    // java if (needDraw) {
    // java KmoConditionFormatter.CfCacheItem item = cfResultCache.get(AppLogicHelper.rc2long(i, j));
    // java if (item != null && item.rules != null && item.rules.contains07Result()) {
    // java this.getCondFmtData().drawConFmt(sheet, canvas, paint, this.mRecycleRect, i, j);
    // java }
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
    // java }
    // java }
    // java }
    fn doRenderer(
        &mut self,
        c: &mut Canvas,
        p: &mut Paint,
        r: &GridRange,
        l: &GridSheetLayoutInfo,
    ) {
        let parent = self.base.parent();
        let grid = l.gridSheet();
        let core = grid.coreSheet();
        let sheet = core.borrow();
        let cf = sheet.getCFormatter();
        if cf.isEmpty() || cf.getResultCache().size() <= 0 {
            return;
        }
        let pre = parent.preClientRect();
        let additional = *parent.mAdditionalRect.borrow();
        let empty = additional.isEmpty();
        let mut offset = 0;
        let startX = parent.mRangeFirstColX.get();
        let startY = parent.mRangeFirstRowY.get();
        self.base.mRecycleRect.top = startY - offset;
        self.base.mRecycleRect.bottom = startY;
        for row in r.mFirstRow..=r.mLastRow {
            let height = l.rowHeightInPixels(row);
            if height != 0 {
                self.base.mRecycleRect.left = startX - offset;
                self.base.mRecycleRect.right = startX;
                self.base.mRecycleRect.bottom += height;
                for col in r.mFirstColumn..=r.mLastColumn {
                    let width = l.columnWidthInPixels(col);
                    if width != 0 {
                        self.base.mRecycleRect.right += width;
                        let rect = self.base.mRecycleRect;
                        let need = if empty {
                            !pre.contains(&rect)
                        } else {
                            Rect::intersects(&additional, &rect)
                        };
                        if need {
                            if let Some(item) = cf.getResultCache().get(
                                crate::cn::wps::moss::util::AppLogicHelper::AppLogicHelper::rc2long(
                                    row, col,
                                ),
                            ) {
                                if item.rules.borrow().contains07Result() {
                                    self.data.drawConFmt(grid.as_ref(), c, p, &rect, row, col);
                                }
                            }
                        }
                        self.base.mRecycleRect.left = self.base.mRecycleRect.right;
                    }
                }
                self.base.mRecycleRect.top = self.base.mRecycleRect.bottom;
            }
        }
    }
}
