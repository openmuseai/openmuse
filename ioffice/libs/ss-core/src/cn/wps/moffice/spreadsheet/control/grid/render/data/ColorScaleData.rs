use super::super::graphics::{Canvas, Paint, Path, Rect};
use crate::cn::wps::moss::{app::IGridSheet::IGridSheet, util::KmoColor::KmoColor};
use serde_json::json;
#[derive(Default)]
pub struct ColorScaleData {
    pub mBackColor: i32,
}
impl ColorScaleData {
    // java public boolean reset(IGridSheet gridSheet, int rowIndex, int colIndex) {
    // java CfColorScaleValue colorScaleValue = this.getColorScale(gridSheet, rowIndex, colIndex);
    // java if (colorScaleValue == null) {
    // java return false;
    // java } else {
    // java int colorIndex = gridSheet.getCellXF(rowIndex, colIndex).getFillBackground();
    // java if (colorIndex != 64) {
    // java return false;
    // java } else {
    // java this.mBackColor = colorScaleValue.getColor();
    // java List<KmoCfRule> generals = gridSheet.coreSheet().getCFormatter().getGenerals(rowIndex, colIndex);
    // java if (generals != null) {
    // java int i = 0;
    // java
    // java for(int len = generals.size(); i < len; ++i) {
    // java Rule rule = ((KmoCfRule)generals.get(i)).getRule();
    // java if (rule.getPriority() < colorScaleValue.getPriority() && rule.getPattern() != null) {
    // java this.mBackColor = rule.getPattern().getFillBackgroundColor();
    // java if (this.mBackColor == 0) {
    // java this.mBackColor = 64;
    // java }
    // java
    // java return true;
    // java }
    // java }
    // java }
    // java
    // java return true;
    // java }
    // java }
    // java }
    pub fn reset(&mut self, g: &dyn IGridSheet, row: i32, col: i32) -> bool {
        let core = g.coreSheet();
        let sheet = core.borrow();
        let Some(scale) = sheet.getCFormatter().getColorScale(row, col) else {
            return false;
        };
        if g.getCellXF(row, col).expect("cell XF").getFillBackground() != 64 {
            return false;
        }
        self.mBackColor = scale.getColor();
        if let Some(generals) = sheet.getCFormatter().getGenerals(row, col) {
            for cf in generals {
                let rule = cf.getRule();
                if rule.getPriority() < scale.getPriority() {
                    if let Some(pattern) = rule.getPattern() {
                        self.mBackColor = pattern.borrow().getFillBackgroundColor();
                        if self.mBackColor == 0 {
                            self.mBackColor = 64;
                        }
                        return true;
                    }
                }
            }
        }
        true
    }
    // java public void drawFill(Canvas canvas, Paint paint, Rect rect) {
    // java if (!rect.isEmpty() && !KmoColor.isAutomaticColor(this.mBackColor)) {
    // java paint.reset();
    // java paint.setStyle(Style.FILL);
    // java paint.setColor(this.mBackColor);
    // java canvas.drawRect(rect, paint);
    // java }
    // java }
    pub fn drawFill(&self, c: &mut Canvas, p: &mut Paint, r: &Rect) {
        if !r.isEmpty() && !KmoColor::isAutomaticColor(self.mBackColor) {
            *p = Paint::default();
            p.style = 0;
            p.color = self.mBackColor;
            c.drawRect(r, p);
        }
    }
}
