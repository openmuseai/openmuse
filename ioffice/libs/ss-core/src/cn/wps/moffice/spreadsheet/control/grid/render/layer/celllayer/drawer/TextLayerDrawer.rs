use super::{
    super::CellsRenderLayer::CellsRenderLayer,
    BaseCellLayerDrawer::{BaseCellLayerDrawer, CellLayerRenderer},
};
use crate::cn::wps::moffice::spreadsheet::control::grid;
use crate::cn::wps::moss::util::KmoRect::KmoRect;
use crate::java_runtime::{Rc, Weak};
use grid::{
    core::{GridRange::GridRange, GridSheetLayoutInfo::GridSheetLayoutInfo},
    layout::CellTextLayout::CellTextLayout,
    render::{
        data::{
            BackFillData::BackFillData, BorderData::BorderData,
            ConditionFormatterData::ConditionFormatterData,
        },
        graphics::{Canvas, Paint, Rect},
    },
};
// java public class TextLayerDrawer extends BaseCellLayerDrawer
pub struct TextLayerDrawer {
    base: BaseCellLayerDrawer,
    mRangeRect: Rect,
    mMergedFill: BackFillData,
    mConFmtData: ConditionFormatterData,
    mMergedBorder: BorderData,
    mMergedCells: Vec<KmoRect>,
    mCellTextLayout: CellTextLayout,
    mVergedCellInfo: VergeCellInfo,
    mHasCFRule: bool,
}
impl TextLayerDrawer {
    // java public TextLayerDrawer(CellsRenderLayer parent) {
    // java super(parent);
    // java this.cellsRenderLayer = parent;
    // java this.mPicInCellDrawer = new PicInCellDrawer();
    // java }
    pub fn new(parent: Weak<CellsRenderLayer>) -> Self {
        Self {
            base: BaseCellLayerDrawer::new(parent),
            mRangeRect: Rect::default(),
            mMergedFill: BackFillData::default(),
            mConFmtData: ConditionFormatterData::default(),
            mMergedBorder: BorderData::default(),
            mMergedCells: vec![],
            mCellTextLayout: CellTextLayout::default(),
            mVergedCellInfo: VergeCellInfo::default(),
            mHasCFRule: false,
        }
    }
    // java protected boolean processMergedCell(GridRange range, int rowIdx, int colIdx, Canvas canvas, Paint paint, GridSheetLayoutInfo gridLayoutInfo, CellXFCache cellXfCache, MergedRangeL2Cache mergedCellCache, TableStyle shareTableStyle) {
    // java KmoRect mergedRange = mergedCellCache.getMergedRange(rowIdx, colIdx);
    // java if (mergedRange == null) {
    // java return false;
    // java } else if (this.mMergedCells.indexOf(mergedRange) != -1) {
    // java return true;
    // java } else {
    // java this.mMergedCells.add(mergedRange);
    // java int firstMergeRow = mergedRange.tl.row;
    // java int firstMergeCol = mergedRange.tl.col;
    // java int lastMergeRow = mergedRange.br.row;
    // java int lastMergeCol = mergedRange.br.col;
    // java this.mRangeRect.left = gridLayoutInfo.logicalColXPos(firstMergeCol);
    // java this.mRangeRect.top = gridLayoutInfo.logicalRowYPos(firstMergeRow);
    // java this.mRangeRect.right = gridLayoutInfo.logicalColXPos(lastMergeCol + 1);
    // java this.mRangeRect.bottom = gridLayoutInfo.logicalRowYPos(lastMergeRow + 1);
    // java if (this.mRangeRect.isEmpty()) {
    // java return true;
    // java } else {
    // java IGridSheet sheet = gridLayoutInfo.mGridSheet;
    // java KmoXF xf = cellXfCache.get(sheet, firstMergeRow, firstMergeCol);
    // java xf = this.mMergedFill.mergePatternFormat(sheet, xf, rowIdx, colIdx);
    // java KmoPalette palette = sheet.getPalette();
    // java int lineOffset = 1;
    // java paint.reset();
    // java paint.setStyle(Style.FILL);
    // java paint.setColor(-1);
    // java canvas.drawRect((float)this.mRangeRect.left, (float)this.mRangeRect.top, (float)this.mRangeRect.right, (float)this.mRangeRect.bottom, paint);
    // java if (gridLayoutInfo.getBgPicture() != null) {
    // java Rect clipRect = new Rect(this.mRangeRect.left + lineOffset, this.mRangeRect.top + lineOffset, this.mRangeRect.right - lineOffset, this.mRangeRect.bottom - lineOffset);
    // java BackgroundLayerDrawer.drawClipBGImage(canvas, paint, clipRect, gridLayoutInfo);
    // java Object var24 = null;
    // java }
    // java
    // java KmoSheet kmoSheet = gridLayoutInfo.mGridSheet.coreSheet();
    // java if (kmoSheet.isValidWoProps() && this.cellsRenderLayer.hasWoPropsLayer() && WoPropsLayerDrawer.colorList.contains(kmoSheet.getWoSheetPropsItem().getInterlineColor())) {
    // java paint.reset();
    // java paint.setColor(kmoSheet.getWoSheetPropsItem().getInterlineColor());
    // java paint.setAlpha(255);
    // java paint.setStyle(Style.FILL);
    // java this.mMergeCellRect.left = this.mRangeRect.left;
    // java this.mMergeCellRect.right = this.mRangeRect.right;
    // java this.mMergeCellRect.top = this.mRangeRect.top;
    // java this.mMergeCellRect.bottom = this.mMergeCellRect.top;
    // java int hiddenCount = 0;
    // java if (firstMergeRow != 0) {
    // java hiddenCount += kmoSheet.getHiddenRowsManager().getHiddenRowsCount(firstMergeRow);
    // java }
    // java
    // java for(int i = firstMergeRow; i <= lastMergeRow; ++i) {
    // java int height = gridLayoutInfo.rowHeightInPixels(i);
    // java if (height == 0) {
    // java if (kmoSheet.getHiddenRowsManager().isHidden(i)) {
    // java ++hiddenCount;
    // java }
    // java } else {
    // java Rect var10000 = this.mMergeCellRect;
    // java var10000.bottom += height;
    // java if ((i + hiddenCount) % 2 != 0) {
    // java this.mMergeCellRect.top = this.mMergeCellRect.bottom;
    // java } else {
    // java canvas.drawRect(this.mMergeCellRect, paint);
    // java this.mMergeCellRect.top = this.mMergeCellRect.bottom;
    // java }
    // java }
    // java }
    // java }
    // java
    // java if (this.mMergedFill.reset(xf, palette, sheet.coreSheet(), rowIdx, colIdx, shareTableStyle)) {
    // java this.mMergedFill.drawFill(canvas, paint, this.mRangeRect);
    // java } else if (gridLayoutInfo.isDrawGridline()) {
    // java gridLayoutInfo.mGridTheme.setCellBorderStyle(paint);
    // java canvas.drawRect(this.mRangeRect, paint);
    // java }
    // java
    // java if (this.mHasCFRule) {
    // java this.mConFmtData.drawConFmt(sheet, canvas, paint, this.mRangeRect, rowIdx, colIdx);
    // java }
    // java
    // java KmoSparklineManager sparklineMgr = gridLayoutInfo.mGridSheet.coreSheet().sparklineMgr();
    // java KSlSparkline sparkline = sparklineMgr.findSparkline(firstMergeRow, firstMergeCol);
    // java if (sparkline != null) {
    // java this.mSparklineData.drawSparkline(canvas, sparkline, gridLayoutInfo, this.mRangeRect);
    // java }
    // java
    // java if (this.mMergedBorder.resetDiagonal(xf, palette)) {
    // java this.mMergedBorder.drawBorder((short)48, canvas, paint, this.mRangeRect);
    // java }
    // java
    // java if (!gridLayoutInfo.mGridSheet.coreSheet().getCellData(firstMergeRow, firstMergeCol).isPicInCell || !this.drawPicInCell(gridLayoutInfo, canvas, firstMergeRow, firstMergeCol, sheet.coreSheet(), paint)) {
    // java TableStyle.FontStyle fontStyle = shareTableStyle.fontStyle(firstMergeRow, firstMergeCol);
    // java this.mCellTextLayout.layout(gridLayoutInfo, canvas, this.mRangeRect, firstMergeRow, firstMergeCol, true, this.mHasCFRule, cellXfCache, fontStyle);
    // java }
    // java
    // java return true;
    // java }
    // java }
    // java }
    fn processMergedCell(
        &mut self,
        row: i32,
        col: i32,
        c: &mut Canvas,
        p: &mut Paint,
        l: &GridSheetLayoutInfo,
    ) -> bool {
        let parent = self.base.parent();
        let merged = parent
            .mMergedRangeCache
            .borrow()
            .getMergedRange(row, col)
            .cloned();
        let Some(m) = merged else {
            return false;
        };
        if self.mMergedCells.contains(&m) {
            return true;
        }
        self.mMergedCells.push(m.clone());
        let (fr, fc, lr, lc) = (m.tl.row, m.tl.col, m.br.row, m.br.col);
        self.mRangeRect = Rect::new(
            l.logicalColXPos(fc),
            l.logicalRowYPos(fr),
            l.logicalColXPos(lc + 1),
            l.logicalRowYPos(lr + 1),
        );
        if self.mRangeRect.isEmpty() {
            return true;
        }
        let grid = l.gridSheet();
        let xf = parent.mCellXfCache.get(grid.as_ref(), fr, fc).map(|xf| {
            self.mMergedFill
                .mergePatternFormat(grid.as_ref(), xf, row, col)
        });
        let palette = grid.getPalette();
        *p = Paint::default();
        p.style = 0;
        p.color = -1;
        c.drawRect(&self.mRangeRect, p);
        // rust: Background bitmaps, WoProps and sparklines have no reader state in this read closure; their original branch bodies require the corresponding reader ports.
        let core = grid.coreSheet();
        if self.mMergedFill.reset__xf(
            xf.as_deref(),
            &palette,
            &core.borrow(),
            row,
            col,
            parent.mShareTableStyle.as_ref(),
        ) {
            self.mMergedFill.drawFill(c, p, &self.mRangeRect);
        } else if l.isDrawGridline() {
            l.mGridTheme.setCellBorderStyle(p);
            c.drawRect(&self.mRangeRect, p);
        }
        if self.mHasCFRule {
            self.mConFmtData
                .drawConFmt(grid.as_ref(), c, p, &self.mRangeRect, row, col);
        }
        if self.mMergedBorder.resetDiagonal(xf.as_deref(), &palette) {
            self.mMergedBorder.drawBorder(48, c, p, &self.mRangeRect);
        }
        self.mCellTextLayout.layout(
            l,
            c,
            &self.mRangeRect,
            fr,
            fc,
            true,
            self.mHasCFRule,
            Some(&parent.mCellXfCache),
            parent.mShareTableStyle.fontStyle(fr, fc),
        );
        true
    }
    // java protected void processVergeCell(int rowIdx, int vergeCol, int end, Canvas canvas, GridSheetLayoutInfo gridLayoutInfo, CellXFCache cellXfCache, TableStyle shareTableStyle) {
    // java KmoSheet sheet = gridLayoutInfo.mGridSheet.coreSheet();
    // java
    // java for(int colIdx = vergeCol - 1; colIdx >= 0; --colIdx) {
    // java int width = gridLayoutInfo.columnWidthInPixels(colIdx);
    // java if (width != 0) {
    // java boolean isBlank = sheet.isBlankCellValue(rowIdx, colIdx);
    // java if (!isBlank) {
    // java if (sheet.getMergedRange(rowIdx, colIdx) == null && !this.mVergedCellInfo.contians(rowIdx, colIdx) && !sheet.getCellData(rowIdx, colIdx).isPicInCell) {
    // java this.mVergedCellInfo.mLRowIdx = rowIdx;
    // java this.mVergedCellInfo.mLColIdx = colIdx;
    // java this.mRangeRect.left = gridLayoutInfo.logicalColXPos(colIdx);
    // java this.mRangeRect.top = this.mRecycleRect.top;
    // java this.mRangeRect.right = this.mRangeRect.left + width;
    // java this.mRangeRect.bottom = this.mRecycleRect.bottom;
    // java TableStyle.FontStyle fontStyle = shareTableStyle.fontStyle(rowIdx, colIdx);
    // java this.mCellTextLayout.layout(gridLayoutInfo, canvas, this.mRangeRect, rowIdx, colIdx, false, this.mHasCFRule, (CellXFCache)null, fontStyle);
    // java }
    // java break;
    // java }
    // java }
    // java }
    // java
    // java for(int colIdx = vergeCol + 1; colIdx < gridLayoutInfo.mMaxColumns; ++colIdx) {
    // java int width = gridLayoutInfo.columnWidthInPixels(colIdx);
    // java if (width != 0) {
    // java boolean isBlank = sheet.isBlankCellValue(rowIdx, colIdx);
    // java if (!isBlank) {
    // java if (sheet.getMergedRange(rowIdx, colIdx) == null && !this.mVergedCellInfo.contians(rowIdx, colIdx) && !sheet.getCellData(rowIdx, colIdx).isPicInCell) {
    // java this.mVergedCellInfo.mRRowIdx = rowIdx;
    // java this.mVergedCellInfo.mRColIdx = colIdx;
    // java int cellType = sheet.getCellValueType(rowIdx, colIdx, this.mCellTypeInfo);
    // java if (cellType == 2) {
    // java this.mRangeRect.left = gridLayoutInfo.logicalColXPos(colIdx);
    // java this.mRangeRect.top = this.mRecycleRect.top;
    // java this.mRangeRect.right = this.mRangeRect.left + width;
    // java this.mRangeRect.bottom = this.mRecycleRect.bottom;
    // java TableStyle.FontStyle fontStyle = shareTableStyle.fontStyle(rowIdx, colIdx);
    // java this.mCellTextLayout.layout(gridLayoutInfo, canvas, this.mRangeRect, rowIdx, colIdx, false, this.mHasCFRule, cellXfCache, fontStyle);
    // java }
    // java }
    // java break;
    // java }
    // java }
    // java }
    // java
    // java }
    fn processVergeCell(
        &mut self,
        row: i32,
        verge: i32,
        _end: i32,
        c: &mut Canvas,
        l: &GridSheetLayoutInfo,
    ) {
        let parent = self.base.parent();
        let core = l.gridSheet().coreSheet();
        let sheet = core.borrow();
        for col in (0..verge).rev() {
            let width = l.columnWidthInPixels(col);
            if width == 0 {
                continue;
            }
            if !sheet.isBlankCellValue(row, col) {
                if sheet.getMergedRange(row, col).is_none()
                    && !self.mVergedCellInfo.contians(row, col)
                {
                    self.mVergedCellInfo.mLRowIdx = row;
                    self.mVergedCellInfo.mLColIdx = col;
                    self.mRangeRect = Rect::new(
                        l.logicalColXPos(col),
                        self.base.mRecycleRect.top,
                        l.logicalColXPos(col) + width,
                        self.base.mRecycleRect.bottom,
                    );
                    drop(sheet);
                    self.mCellTextLayout.layout(
                        l,
                        c,
                        &self.mRangeRect,
                        row,
                        col,
                        false,
                        self.mHasCFRule,
                        None,
                        parent.mShareTableStyle.fontStyle(row, col),
                    );
                    return self.processRightVerge(row, verge, c, l);
                }
                break;
            }
        }
        drop(sheet);
        self.processRightVerge(row, verge, c, l);
    }
    // rust: The second Java for-loop is split to release the borrowed KmoSheet before CellTextLayout queries it.
    fn processRightVerge(&mut self, row: i32, verge: i32, c: &mut Canvas, l: &GridSheetLayoutInfo) {
        let parent = self.base.parent();
        let core = l.gridSheet().coreSheet();
        let sheet = core.borrow();
        for col in verge + 1..l.mMaxColumns {
            let width = l.columnWidthInPixels(col);
            if width == 0 {
                continue;
            }
            if !sheet.isBlankCellValue(row, col) {
                if sheet.getMergedRange(row, col).is_none()
                    && !self.mVergedCellInfo.contians(row, col)
                {
                    self.mVergedCellInfo.mRRowIdx = row;
                    self.mVergedCellInfo.mRColIdx = col;
                    if sheet.getCellValueType__2(row, col) == 2 {
                        self.mRangeRect = Rect::new(
                            l.logicalColXPos(col),
                            self.base.mRecycleRect.top,
                            l.logicalColXPos(col) + width,
                            self.base.mRecycleRect.bottom,
                        );
                        drop(sheet);
                        self.mCellTextLayout.layout(
                            l,
                            c,
                            &self.mRangeRect,
                            row,
                            col,
                            false,
                            self.mHasCFRule,
                            Some(&parent.mCellXfCache),
                            parent.mShareTableStyle.fontStyle(row, col),
                        );
                        return;
                    }
                }
                break;
            }
        }
    }
}
impl CellLayerRenderer for TextLayerDrawer {
    fn base(&mut self) -> &mut BaseCellLayerDrawer {
        &mut self.base
    }
    fn name(&self) -> &'static str {
        "TextLayerDrawer"
    }
    // java protected void doRenderer(Canvas canvas, Paint paint, GridRange range, GridSheetLayoutInfo gridLayoutInfo) {
    // java int startRowY = this.mParent.mRangeFirstRowY;
    // java int startColumnX = this.mParent.mRangeFirstColX;
    // java int lastValidCol = Math.min(range.mLastColumn, gridLayoutInfo.mMaxColumns);
    // java int lastValidRow = Math.min(range.mLastRow, gridLayoutInfo.mMaxRows);
    // java boolean isForward = this.mParent.mPreClientRect.left < this.mParent.mCurClientRect.left;
    // java boolean isVerticalScroll = this.mParent.mPreClientRect.left == this.mParent.mCurClientRect.left;
    // java boolean isEmptyClient = this.mParent.mPreClientRect.isEmpty();
    // java MergedRangeL2Cache mergedCellCache = this.mParent.mMergedRangeCache;
    // java boolean hasMergeCell = !mergedCellCache.isEmpty();
    // java this.mCellTypeInfo = (KmoSheet.CellTypeInfo)this.mLocalCellTypeInfo.get();
    // java this.mRecycleRect.top = startRowY;
    // java this.mRecycleRect.bottom = startRowY;
    // java this.mVergedCellInfo.reset(isEmptyClient, range.mFirstColumn, range.mLastColumn);
    // java Rect preClientRect = this.mParent.mPreClientRect;
    // java Rect additionalRect = this.mParent.mAdditionalRect;
    // java boolean isEmpty = additionalRect.isEmpty();
    // java KmoRect kmoRect = SharedObjects.kmoRectPool.borrowObject();
    // java kmoRect.set(range.mFirstRow, range.mFirstColumn, range.mLastRow, range.mLastColumn);
    // java this.mHasCFRule = gridLayoutInfo.mGridSheet.coreSheet().getCFormatter().hasRuleIn(kmoRect);
    // java kmoRect = SharedObjects.kmoRectPool.returnObject(kmoRect);
    // java CellXFCache cellXfCache = this.mParent.mCellXfCache;
    // java TableStyle shareTableStyle = this.mParent.mShareTableStyle;
    // java
    // java for(int i = range.mFirstRow; i <= lastValidRow; ++i) {
    // java int height = gridLayoutInfo.rowHeightInPixels(i);
    // java if (height != 0) {
    // java this.mRecycleRect.left = startColumnX;
    // java this.mRecycleRect.right = startColumnX;
    // java Rect var10000 = this.mRecycleRect;
    // java var10000.bottom += height;
    // java boolean firstColCheck = true;
    // java boolean hasLayout = false;
    // java
    // java for(int j = range.mFirstColumn; j <= lastValidCol; ++j) {
    // java int width = gridLayoutInfo.columnWidthInPixels(j);
    // java if (width != 0) {
    // java var10000 = this.mRecycleRect;
    // java var10000.right += width;
    // java boolean needDraw;
    // java if (isEmpty) {
    // java needDraw = !preClientRect.contains(this.mRecycleRect);
    // java } else {
    // java needDraw = Rect.intersects(additionalRect, this.mRecycleRect);
    // java if (!needDraw && isVerticalScroll && j == range.mFirstColumn) {
    // java firstColCheck = false;
    // java }
    // java }
    // java
    // java if (needDraw) {
    // java IGridSheet gridSheet = gridLayoutInfo.mGridSheet;
    // java KmoSheet sheet = gridSheet.coreSheet();
    // java hasLayout = hasMergeCell ? this.processMergedCell(range, i, j, canvas, paint, gridLayoutInfo, cellXfCache, mergedCellCache, shareTableStyle) : false;
    // java if (!hasLayout) {
    // java this.mRangeRect.set(this.mRecycleRect);
    // java if (!sheet.getCellData(i, j).isPicInCell || !this.drawPicInCell(gridLayoutInfo, canvas, i, j, sheet, paint)) {
    // java hasLayout = this.mCellTextLayout.layout(gridLayoutInfo, canvas, this.mRangeRect, i, j, true, this.mHasCFRule, cellXfCache, shareTableStyle.fontStyle(i, j));
    // java }
    // java }
    // java } else {
    // java hasLayout = false;
    // java }
    // java
    // java if (j == range.mFirstColumn && hasLayout) {
    // java firstColCheck = false;
    // java }
    // java
    // java this.mRecycleRect.left = this.mRecycleRect.right;
    // java }
    // java }
    // java
    // java if (firstColCheck && (!isForward || isEmptyClient && range.mFirstColumn > 0)) {
    // java this.processVergeCell(i, range.mFirstColumn, range.mLastColumn, canvas, gridLayoutInfo, cellXfCache, shareTableStyle);
    // java }
    // java
    // java if (!hasLayout && isForward) {
    // java this.processVergeCell(i, range.mLastColumn, range.mFirstColumn, canvas, gridLayoutInfo, cellXfCache, shareTableStyle);
    // java }
    // java
    // java this.mRecycleRect.top = this.mRecycleRect.bottom;
    // java }
    // java }
    // java
    // java }
    fn doRenderer(
        &mut self,
        c: &mut Canvas,
        p: &mut Paint,
        r: &GridRange,
        l: &GridSheetLayoutInfo,
    ) {
        let parent = self.base.parent();
        let startY = parent.mRangeFirstRowY.get();
        let startX = parent.mRangeFirstColX.get();
        let lastCol = r.mLastColumn.min(l.mMaxColumns);
        let lastRow = r.mLastRow.min(l.mMaxRows);
        let pre = parent.preClientRect();
        let cur = parent.curClientRect();
        let forward = pre.left < cur.left;
        let vertical = pre.left == cur.left;
        let emptyClient = pre.isEmpty();
        let hasMerged = !parent.mMergedRangeCache.borrow().isEmpty();
        self.base.mRecycleRect.top = startY;
        self.base.mRecycleRect.bottom = startY;
        self.mVergedCellInfo
            .reset(emptyClient, r.mFirstColumn, r.mLastColumn);
        let additional = *parent.mAdditionalRect.borrow();
        let empty = additional.isEmpty();
        self.mHasCFRule = l
            .gridSheet()
            .coreSheet()
            .borrow()
            .getCFormatter()
            .hasRuleIn(&KmoRect::KmoRect__bounds(
                r.mFirstRow,
                r.mFirstColumn,
                r.mLastRow,
                r.mLastColumn,
            ));
        for row in r.mFirstRow..=lastRow {
            let height = l.rowHeightInPixels(row);
            if height == 0 {
                continue;
            }
            self.base.mRecycleRect.left = startX;
            self.base.mRecycleRect.right = startX;
            self.base.mRecycleRect.bottom += height;
            let mut firstCheck = true;
            let mut hasLayout = false;
            for col in r.mFirstColumn..=lastCol {
                let width = l.columnWidthInPixels(col);
                if width == 0 {
                    continue;
                }
                self.base.mRecycleRect.right += width;
                let rect = self.base.mRecycleRect;
                let need = if empty {
                    !pre.contains(&rect)
                } else {
                    let need = Rect::intersects(&additional, &rect);
                    if !need && vertical && col == r.mFirstColumn {
                        firstCheck = false;
                    }
                    need
                };
                if need {
                    hasLayout = hasMerged && self.processMergedCell(row, col, c, p, l);
                    if !hasLayout {
                        self.mRangeRect = rect;
                        hasLayout = self.mCellTextLayout.layout(
                            l,
                            c,
                            &self.mRangeRect,
                            row,
                            col,
                            true,
                            self.mHasCFRule,
                            Some(&parent.mCellXfCache),
                            parent.mShareTableStyle.fontStyle(row, col),
                        );
                    }
                } else {
                    hasLayout = false;
                }
                if col == r.mFirstColumn && hasLayout {
                    firstCheck = false;
                }
                self.base.mRecycleRect.left = self.base.mRecycleRect.right;
            }
            if firstCheck && (!forward || emptyClient && r.mFirstColumn > 0) {
                self.processVergeCell(row, r.mFirstColumn, r.mLastColumn, c, l);
            }
            if !hasLayout && forward {
                self.processVergeCell(row, r.mLastColumn, r.mFirstColumn, c, l);
            }
            self.base.mRecycleRect.top = self.base.mRecycleRect.bottom;
        }
    }
    // java protected void cleanUp() {
    // java super.cleanUp();
    // java this.mMergedCells.clear();
    // java this.mCellTypeInfo = null;
    // java }
    fn cleanUp(&mut self) {
        self.base.cleanUp();
        self.mMergedCells.clear();
    }
}
#[derive(Default)]
struct VergeCellInfo {
    mLRowIdx: i32,
    mLColIdx: i32,
    mRRowIdx: i32,
    mRColIdx: i32,
    mStartCol: i32,
    mEndCol: i32,
    mIsEmptyClient: bool,
}
impl VergeCellInfo {
    // java final void reset(boolean isEmpty, int startCol, int endCol) {
    // java this.mLRowIdx = this.mLColIdx = this.mRRowIdx = this.mRColIdx = -1;
    // java this.mStartCol = startCol;
    // java this.mEndCol = endCol;
    // java this.mIsEmptyClient = isEmpty;
    // java }
    fn reset(&mut self, empty: bool, start: i32, end: i32) {
        self.mLRowIdx = -1;
        self.mLColIdx = -1;
        self.mRRowIdx = -1;
        self.mRColIdx = -1;
        self.mStartCol = start;
        self.mEndCol = end;
        self.mIsEmptyClient = empty;
    }
    // java final boolean contians(int row, int col) {
    // java return this.mIsEmptyClient && col >= this.mStartCol && col <= this.mEndCol || this.mLRowIdx == row && this.mLColIdx == col || this.mRRowIdx == row && this.mRColIdx == col;
    // java }
    fn contians(&self, row: i32, col: i32) -> bool {
        self.mIsEmptyClient && col >= self.mStartCol && col <= self.mEndCol
            || self.mLRowIdx == row && self.mLColIdx == col
            || self.mRRowIdx == row && self.mRColIdx == col
    }
}
