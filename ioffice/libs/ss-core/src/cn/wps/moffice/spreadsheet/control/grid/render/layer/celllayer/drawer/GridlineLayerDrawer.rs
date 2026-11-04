use super::super::CellsRenderLayer::CellsRenderLayer;
use super::BaseCellLayerDrawer::{BaseCellLayerDrawer, CellLayerRenderer};
use crate::cn::wps::moffice::spreadsheet::control::grid;
use crate::java_runtime::{Cell, Rc, RefCell, Weak};
use grid::{
    core::{GridRange::GridRange, GridSheetLayoutInfo::GridSheetLayoutInfo},
    render::{
        data::GridClientData::{ClientRect, ClientRects, GridClientData},
        graphics::{Canvas, Paint, Path, Rect},
    },
};
pub struct GridlineLayerDrawer {
    base: BaseCellLayerDrawer,
    mPath: Path,
    mGridlinePaint: Paint,
}
impl GridlineLayerDrawer {
    // java public GridlineLayerDrawer(CellsRenderLayer parent) {
    // java super(parent);
    // java this.mGridlinePaint.reset();
    // java this.mGridlinePaint.setStyle(Style.STROKE);
    // java if (DefaultGridTheme.USE_PHONE_THEME) {
    // java this.mGridlinePaint.setStrokeWidth(0.0F);
    // java this.mGridlinePaint.setColor(Color.rgb(208, 215, 229));
    // java } else {
    // java this.mGridlinePaint.setStrokeWidth((float)UnitsConverter.dp2pix(1.0F));
    // java this.mGridlinePaint.setColor(437326615);
    // java }
    // java
    // java this.mGridlinePaint.setAntiAlias(true);
    // java }
    pub fn new(parent: Weak<CellsRenderLayer>) -> Self {
        let phone = grid::theme::DefaultGridTheme::USE_PHONE_THEME.get();
        let density = grid::render::native::host(serde_json::json!({"op":"platformMetrics"}))
            ["density"]
            .as_f64()
            .expect("native density") as f32;
        let mut p = Paint {
            style: 1,
            ..Paint::default()
        };
        if phone {
            p.strokeWidth = 0.0;
            p.color = -3090459;
        } else {
            p.strokeWidth =
                crate::cn::wps::moss::app::UnitsConverter::UnitsConverter::dp2pix(1.0, density)
                    as f32;
            p.color = 437326615;
        }
        p.flags |= 1;
        Self {
            base: BaseCellLayerDrawer::new(parent),
            mPath: Path::default(),
            mGridlinePaint: p,
        }
    }
    // java private void calcHLinePath(GridRange aGridRange, GridSheetLayoutInfo gridLayoutInfo) {
    // java int startColX = this.mParent.mCurClientRect.left < this.mParent.mRangeFirstColX ? this.mParent.mRangeFirstColX : this.mParent.mCurClientRect.left;
    // java int stopColX = this.mParent.mCurClientRect.right;
    // java int startRowY = this.mParent.mRangeFirstRowY;
    // java int height = 0;
    // java this.mPath.moveTo((float)startColX, (float)startRowY);
    // java this.mPath.lineTo((float)stopColX, (float)startRowY);
    // java int i = aGridRange.mFirstRow;
    // java
    // java for(int lastRow = aGridRange.mLastRow; i <= lastRow; ++i) {
    // java height = gridLayoutInfo.rowHeightInPixels(i);
    // java if (height > 0) {
    // java startRowY += height;
    // java this.mPath.moveTo((float)startColX, (float)startRowY);
    // java this.mPath.lineTo((float)stopColX, (float)startRowY);
    // java }
    // java }
    // java
    // java }
    fn calcHLinePath(&mut self, r: &GridRange, l: &GridSheetLayoutInfo) {
        let parent = self.base.parent();
        let x = parent
            .curClientRect()
            .left
            .max(parent.mRangeFirstColX.get());
        let right = parent.curClientRect().right;
        let mut y = parent.mRangeFirstRowY.get();
        self.mPath.moveTo(x as f32, y as f32);
        self.mPath.lineTo(right as f32, y as f32);
        for row in r.mFirstRow..=r.mLastRow {
            let height = l.rowHeightInPixels(row);
            if height > 0 {
                y += height;
                self.mPath.moveTo(x as f32, y as f32);
                self.mPath.lineTo(right as f32, y as f32);
            }
        }
    }
    // java private void calcVLinePath(GridRange aGridRange, GridSheetLayoutInfo gridLayoutInfo) {
    // java int startColX = this.mParent.mRangeFirstColX;
    // java int startRowY = this.mParent.mCurClientRect.top < this.mParent.mRangeFirstRowY ? this.mParent.mRangeFirstRowY : this.mParent.mCurClientRect.top;
    // java int stopRowY = this.mParent.mCurClientRect.bottom;
    // java int width = 0;
    // java this.mPath.moveTo((float)startColX, (float)startRowY);
    // java this.mPath.lineTo((float)startColX, (float)stopRowY);
    // java int j = aGridRange.mFirstColumn;
    // java
    // java for(int lastCol = aGridRange.mLastColumn; j <= lastCol; ++j) {
    // java width = gridLayoutInfo.columnWidthInPixels(j);
    // java if (width > 0) {
    // java startColX += width;
    // java this.mPath.moveTo((float)startColX, (float)startRowY);
    // java this.mPath.lineTo((float)startColX, (float)stopRowY);
    // java }
    // java }
    // java
    // java }
    fn calcVLinePath(&mut self, r: &GridRange, l: &GridSheetLayoutInfo) {
        let parent = self.base.parent();
        let mut x = parent.mRangeFirstColX.get();
        let y = parent.curClientRect().top.max(parent.mRangeFirstRowY.get());
        let bottom = parent.curClientRect().bottom;
        self.mPath.moveTo(x as f32, y as f32);
        self.mPath.lineTo(x as f32, bottom as f32);
        for col in r.mFirstColumn..=r.mLastColumn {
            let width = l.columnWidthInPixels(col);
            if width > 0 {
                x += width;
                self.mPath.moveTo(x as f32, y as f32);
                self.mPath.lineTo(x as f32, bottom as f32);
            }
        }
    }
}
impl CellLayerRenderer for GridlineLayerDrawer {
    fn base(&mut self) -> &mut BaseCellLayerDrawer {
        &mut self.base
    }
    fn name(&self) -> &'static str {
        "GridlineLayerDrawer"
    }
    // java protected void doRenderer(Canvas canvas, Paint paint, GridRange range, GridSheetLayoutInfo gridLayoutInfo) {
    // java if (gridLayoutInfo.isDrawGridline()) {
    // java gridLayoutInfo.mGridTheme.setGridlineStyle(this.mGridlinePaint);
    // java float oldStroke = this.mGridlinePaint.getStrokeWidth();
    // java if (this.mParent.isStrokeWidthScale()) {
    // java float singleLine = 1.001F / this.mParent.strokeWidthScale();
    // java this.mGridlinePaint.setStrokeWidth(Math.max(oldStroke, singleLine));
    // java }
    // java
    // java this.mPath.rewind();
    // java this.calcHLinePath(range, gridLayoutInfo);
    // java this.calcVLinePath(range, gridLayoutInfo);
    // java canvas.drawPath(this.mPath, this.mGridlinePaint);
    // java if (this.mParent.isStrokeWidthScale()) {
    // java this.mGridlinePaint.setStrokeWidth(oldStroke);
    // java }
    // java
    // java }
    // java }
    fn doRenderer(
        &mut self,
        c: &mut Canvas,
        _p: &mut Paint,
        r: &GridRange,
        l: &GridSheetLayoutInfo,
    ) {
        if l.isDrawGridline() {
            let parent = self.base.parent();
            l.mGridTheme.setGridlineStyle(&mut self.mGridlinePaint);
            let old = self.mGridlinePaint.strokeWidth;
            if parent.isStrokeWidthScale() {
                let single = 1.001 / parent.strokeWidthScale();
                self.mGridlinePaint.strokeWidth = old.max(single);
            }
            self.mPath.reset();
            self.calcHLinePath(r, l);
            self.calcVLinePath(r, l);
            c.drawPath(&self.mPath, &self.mGridlinePaint);
            if parent.isStrokeWidthScale() {
                self.mGridlinePaint.strokeWidth = old;
            }
        }
    }
}
