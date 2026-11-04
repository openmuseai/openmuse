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
pub struct BackgroundLayerDrawer {
    base: BaseCellLayerDrawer,
}
impl BackgroundLayerDrawer {
    pub fn new(p: Weak<CellsRenderLayer>) -> Self {
        Self {
            base: BaseCellLayerDrawer::new(p),
        }
    }
}
impl CellLayerRenderer for BackgroundLayerDrawer {
    fn base(&mut self) -> &mut BaseCellLayerDrawer {
        &mut self.base
    }
    fn name(&self) -> &'static str {
        "BackgroundLayerDrawer"
    }
    // java protected void doRenderer(Canvas canvas, Paint paint, GridRange range, GridSheetLayoutInfo gridLayoutInfo) {
    // java Bitmap bitmap = gridLayoutInfo.getBgPicture();
    // java if (bitmap != null) {
    // java canvas.save();
    // java BitmapShader shader = new BitmapShader(bitmap, TileMode.REPEAT, TileMode.REPEAT);
    // java BitmapDrawable drawable = new BitmapDrawable(bitmap);
    // java drawable.setFilterBitmap(true);
    // java int nOffsetX = gridLayoutInfo.clientOriginX();
    // java int nOffsetY = gridLayoutInfo.clientOriginY();
    // java this.mRecycleRect.set(this.mParent.mCurClientRect);
    // java canvas.translate((float)nOffsetX, (float)nOffsetY);
    // java this.mRecycleRect.offset(-nOffsetX, -nOffsetY);
    // java dealWithTooFarTile(this.mRecycleRect, canvas, bitmap);
    // java drawable.setBounds(this.mRecycleRect);
    // java drawable.getPaint().setShader(shader);
    // java drawable.draw(canvas);
    // java BitmapShader var10 = null;
    // java BitmapDrawable var11 = null;
    // java canvas.restore();
    // java }
    // java
    // java }
    fn doRenderer(
        &mut self,
        _c: &mut Canvas,
        _p: &mut Paint,
        _r: &GridRange,
        _l: &GridSheetLayoutInfo,
    ) {
        // rust: Workbook background bitmap/media import is outside the M1–M3 ordinary-grid read closure. No background exists in this closure.
    }
}
