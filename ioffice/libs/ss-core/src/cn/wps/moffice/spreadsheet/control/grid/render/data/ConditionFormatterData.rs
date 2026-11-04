use super::super::graphics::{Canvas, Paint, Path, Rect};
use super::{ColorScaleData::ColorScaleData, DataBarsData::DataBarsData, IconSetData::IconSetData};
use crate::cn::wps::moss::{app::IGridSheet::IGridSheet, util::KmoColor::KmoColor};
use serde_json::json;
#[derive(Default)]
pub struct ConditionFormatterData {
    mDataBars: DataBarsData,
    mColorScale: ColorScaleData,
    mIconSet: IconSetData,
}
impl ConditionFormatterData {
    // java public void drawConFmt(IGridSheet sheet, Canvas canvas, Paint paint, Rect rect, int row, int col) {
    // java if (this.mColorScale.reset(sheet, row, col)) {
    // java this.mColorScale.drawFill(canvas, paint, rect);
    // java }
    // java
    // java if (this.mDataBars.reset(sheet, row, col)) {
    // java this.mDataBars.draw(canvas, paint, rect);
    // java }
    // java
    // java if (this.mIconSet.reset(sheet, row, col)) {
    // java this.mIconSet.drawIcon(canvas, paint, rect);
    // java }
    // java
    // java paint.reset();
    // java }
    pub fn drawConFmt(
        &mut self,
        g: &dyn IGridSheet,
        c: &mut Canvas,
        p: &mut Paint,
        r: &Rect,
        row: i32,
        col: i32,
    ) {
        if self.mColorScale.reset(g, row, col) {
            self.mColorScale.drawFill(c, p, r);
        }
        if self.mDataBars.reset(g, row, col) {
            self.mDataBars.draw(c, p, r);
        }
        if self.mIconSet.reset(g, row, col) {
            self.mIconSet.drawIcon(c, p, r);
        }
        *p = Paint::default();
    }
}
