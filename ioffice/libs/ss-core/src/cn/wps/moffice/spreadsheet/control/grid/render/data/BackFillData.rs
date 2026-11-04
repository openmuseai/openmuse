use super::super::super::{
    layout::cache::CellXFCache::CellXFCache,
    TableStyle::{isAutomaticColor, TableStyle},
};
use super::super::graphics::{Canvas, Paint, Rect};
use crate::cn::wps::moffice::spreadsheet::utils::{
    CellDrawerUtil::CellDrawerUtil, CellGradientUtil::CellGradientUtil,
};
use crate::{
    cn::wps::moss::{
        app::{xf::KmoXF::KmoXF, IGridSheet::IGridSheet, KmoSheet::KmoSheet},
        core::book::KmoPalette::KmoPalette,
        filefmt::biff8::record::common::XFGradient::XFGradient,
        util::KmoColor::KmoColor,
    },
    java_runtime::Rc,
};
// java public class BackFillData
pub struct BackFillData {
    pub mIsValid: bool,
    pub mBackColor: i32,
    pub mForceColor: i32,
    pub mFillStyle: i16,
    pub mGradient: Option<XFGradient>,
    mCondfmtValid: bool,
}
impl Default for BackFillData {
    fn default() -> Self {
        Self::new(true)
    }
}
impl BackFillData {
    // java public BackFillData(boolean condfmtValid) {
    // java this.mIsValid = false;
    // java this.mCondfmtValid = condfmtValid;
    // java }
    pub fn new(valid: bool) -> Self {
        Self {
            mIsValid: false,
            mBackColor: 0,
            mForceColor: 0,
            mFillStyle: 0,
            mGradient: None,
            mCondfmtValid: valid,
        }
    }
    // java public KmoXF mergePatternFormat(IGridSheet gridSheet, KmoXF xf, int rowIndex, int colIndex) {
    // java KmoSheet kmoSheet = gridSheet.coreSheet();
    // java KmoTableManager tableManager = kmoSheet.getTableManager();
    // java KmoXF newXf = xf;
    // java if (null != tableManager) {
    // java newXf = tableManager.getMergeXF(rowIndex, colIndex, xf, (byte)0);
    // java }
    // java
    // java if (!this.mCondfmtValid) {
    // java return newXf;
    // java } else {
    // java KmoConditionFormatter cfFormatter = kmoSheet.getCFormatter();
    // java return cfFormatter.isEmpty() ? newXf : cfFormatter.getMergeXF(rowIndex, colIndex, newXf, (byte)0);
    // java }
    // java }
    pub fn mergePatternFormat(
        &self,
        g: &dyn IGridSheet,
        xf: Rc<KmoXF>,
        row: i32,
        col: i32,
    ) -> Rc<KmoXF> {
        let core = g.coreSheet();
        let s = core.borrow();
        if !self.mCondfmtValid || s.getCFormatter().isEmpty() {
            xf
        } else {
            s.getCFormatter().getMergeXF(row, col, xf, 0)
        }
    }
    // java public boolean reset(IGridSheet gridSheet, int rowIndex, int colIndex, CellXFCache cellXfCache, TableStyle shareTableStyle) {
    // java this.clear();
    // java if (this.reset(rowIndex, colIndex, shareTableStyle)) {
    // java return true;
    // java } else {
    // java KmoXF xf = cellXfCache != null ? cellXfCache.get(gridSheet, rowIndex, colIndex) : gridSheet.getCellXF(rowIndex, colIndex);
    // java xf = this.mergePatternFormat(gridSheet, xf, rowIndex, colIndex);
    // java if (xf == null) {
    // java return false;
    // java } else {
    // java this.mFillStyle = xf.getAdtlFillPattern();
    // java if (this.mFillStyle == 0) {
    // java return false;
    // java } else {
    // java int colorIdx = 0;
    // java if (this.mFillStyle != 1 && this.mFillStyle != 32) {
    // java colorIdx = xf.getFillBackground();
    // java if (colorIdx != 64) {
    // java if (ColorToolKit.isPaletteIndex(colorIdx)) {
    // java this.mBackColor = gridSheet.getPalette().getRGBColor((short)colorIdx);
    // java } else {
    // java this.mBackColor = colorIdx;
    // java }
    // java }
    // java }
    // java
    // java colorIdx = xf.getFillForeground();
    // java if (colorIdx != 64) {
    // java CfColorScaleValue colorScale = gridSheet.coreSheet().getCFormatter().getColorScale(rowIndex, colIndex);
    // java if (colorScale != null && gridSheet.coreSheet().getCFormatter().getGenerals(rowIndex, colIndex) == null) {
    // java this.mBackColor = colorScale.getColor();
    // java }
    // java
    // java if (ColorToolKit.isPaletteIndex(colorIdx)) {
    // java this.mForceColor = gridSheet.getPalette().getRGBColor((short)colorIdx);
    // java } else {
    // java this.mForceColor = colorIdx;
    // java }
    // java }
    // java
    // java if (this.mFillStyle == 32) {
    // java this.mGradient = xf.getXFGradient().copy();
    // java
    // java for(int i = 0; i < this.mGradient.colorStops.size(); ++i) {
    // java XFGradient.ColorStop stop = (XFGradient.ColorStop)this.mGradient.colorStops.get(i);
    // java colorIdx = stop.color;
    // java if (colorIdx != 64) {
    // java if (ColorToolKit.isPaletteIndex(colorIdx)) {
    // java colorIdx = gridSheet.getPalette().getRGBColor((short)colorIdx);
    // java } else {
    // java colorIdx = colorIdx;
    // java }
    // java
    // java stop.color = colorIdx;
    // java }
    // java }
    // java }
    // java
    // java this.mIsValid = true;
    // java return this.mIsValid;
    // java }
    // java }
    // java }
    // java }
    pub fn reset(
        &mut self,
        g: &dyn IGridSheet,
        row: i32,
        col: i32,
        cache: Option<&CellXFCache>,
        table: &dyn TableStyle,
    ) -> bool {
        self.clear();
        if self.reset__table(row, col, table) {
            return true;
        }
        let xf = cache
            .and_then(|c| c.get(g, row, col))
            .or_else(|| g.getCellXF(row, col));
        let Some(xf) = xf else {
            return false;
        };
        let xf = self.mergePatternFormat(g, xf, row, col);
        let core = g.coreSheet();
        let s = core.borrow();
        self.reset__xf_fields(&xf, &g.getPalette(), &s, row, col, true)
    }
    // java private boolean reset(int rowIndex, int colIndex, TableStyle shareTableStyle) {
    // java Integer fillColor = shareTableStyle.fillColor(rowIndex, colIndex);
    // java if (TableStyle.isAutomaticColor(fillColor)) {
    // java return false;
    // java } else {
    // java this.mFillStyle = 1;
    // java this.mBackColor = this.mForceColor = fillColor;
    // java this.mGradient = null;
    // java this.mIsValid = true;
    // java return this.mIsValid;
    // java }
    // java }
    fn reset__table(&mut self, row: i32, col: i32, table: &dyn TableStyle) -> bool {
        let color = table.fillColor(row, col);
        if isAutomaticColor(color) {
            false
        } else {
            self.mFillStyle = 1;
            self.mBackColor = color.unwrap();
            self.mForceColor = self.mBackColor;
            self.mGradient = None;
            self.mIsValid = true;
            true
        }
    }
    // java public boolean reset(KmoXF xf, KmoPalette palette, KmoSheet sheet, int rowIndex, int colIndex, TableStyle shareTableStyle) {
    // java this.clear();
    // java if (this.reset(rowIndex, colIndex, shareTableStyle)) {
    // java return true;
    // java } else if (xf == null) {
    // java return false;
    // java } else {
    // java this.mFillStyle = xf.getAdtlFillPattern();
    // java if (this.mFillStyle == 0) {
    // java return false;
    // java } else {
    // java int colorIdx = 0;
    // java if (this.mFillStyle != 1 && this.mFillStyle != 32) {
    // java colorIdx = xf.getFillBackground();
    // java if (colorIdx != 64) {
    // java if (ColorToolKit.isPaletteIndex(colorIdx)) {
    // java this.mBackColor = palette.getRGBColor((short)colorIdx);
    // java } else {
    // java this.mBackColor = colorIdx;
    // java }
    // java }
    // java }
    // java
    // java colorIdx = xf.getFillForeground();
    // java if (colorIdx != 64) {
    // java CfColorScaleValue colorScale = sheet.getCFormatter().getColorScale(rowIndex, colIndex);
    // java if (colorScale != null) {
    // java this.mBackColor = colorScale.getColor();
    // java }
    // java
    // java if (ColorToolKit.isPaletteIndex(colorIdx)) {
    // java this.mForceColor = palette.getRGBColor((short)colorIdx);
    // java } else {
    // java this.mForceColor = colorIdx;
    // java }
    // java }
    // java
    // java if (this.mFillStyle == 32) {
    // java this.mGradient = xf.getXFGradient().copy();
    // java
    // java for(int i = 0; i < this.mGradient.colorStops.size(); ++i) {
    // java XFGradient.ColorStop stop = (XFGradient.ColorStop)this.mGradient.colorStops.get(i);
    // java colorIdx = stop.color;
    // java if (colorIdx != 64) {
    // java if (ColorToolKit.isPaletteIndex(colorIdx)) {
    // java colorIdx = palette.getRGBColor((short)colorIdx);
    // java } else {
    // java colorIdx = colorIdx;
    // java }
    // java
    // java stop.color = colorIdx;
    // java }
    // java }
    // java }
    // java
    // java this.mIsValid = true;
    // java return this.mIsValid;
    // java }
    // java }
    // java }
    pub fn reset__xf(
        &mut self,
        xf: Option<&KmoXF>,
        palette: &KmoPalette,
        s: &KmoSheet,
        row: i32,
        col: i32,
        table: &dyn TableStyle,
    ) -> bool {
        self.clear();
        if self.reset__table(row, col, table) {
            return true;
        }
        let Some(xf) = xf else {
            return false;
        };
        self.reset__xf_fields(xf, palette, s, row, col, false)
    }
    // rust: Shared literal body of the two Java reset overloads; only the original getGenerals guard differs.
    fn reset__xf_fields(
        &mut self,
        xf: &KmoXF,
        palette: &KmoPalette,
        s: &KmoSheet,
        row: i32,
        col: i32,
        generalGuard: bool,
    ) -> bool {
        self.mFillStyle = xf.getAdtlFillPattern();
        if self.mFillStyle == 0 {
            return false;
        }
        if self.mFillStyle != 1 && self.mFillStyle != 32 {
            let color = xf.getFillBackground();
            if color != 64 {
                self.mBackColor = if color & -16777216 == 0 {
                    palette.getRGBColor(color as i16)
                } else {
                    color
                };
            }
        }
        let color = xf.getFillForeground();
        if color != 64 {
            if let Some(scale) = s.getCFormatter().getColorScale(row, col) {
                if !generalGuard || s.getCFormatter().getGenerals(row, col).is_none() {
                    self.mBackColor = scale.getColor();
                }
            }
            self.mForceColor = if color & -16777216 == 0 {
                palette.getRGBColor(color as i16)
            } else {
                color
            };
        }
        if self.mFillStyle == 32 {
            let gradient = xf.getXFGradient().expect("gradient");
            let gradient = gradient.borrow().copy();
            for stop in &gradient.colorStops {
                let mut stop = stop.borrow_mut();
                let color = stop.color;
                if color != 64 && color & -16777216 == 0 {
                    stop.color = palette.getRGBColor(color as i16);
                }
            }
            self.mGradient = Some(gradient);
        }
        self.mIsValid = true;
        true
    }
    // java public void drawFill(Canvas canvas, Paint paint, Rect rect) {
    // java if (!rect.isEmpty()) {
    // java if (!KmoColor.isAutomaticColor(this.mBackColor)) {
    // java paint.setStyle(Style.FILL);
    // java paint.setColor(this.mBackColor);
    // java canvas.drawRect(rect, paint);
    // java }
    // java
    // java if (0 != this.mFillStyle) {
    // java if (this.mFillStyle == 32 && this.mGradient != null) {
    // java paint.reset();
    // java paint.setStyle(Style.FILL);
    // java CellGradientUtil.drawGradient(canvas, paint, rect, this.mGradient);
    // java paint.reset();
    // java return;
    // java }
    // java
    // java int color = this.mForceColor;
    // java if (KmoColor.isAutomaticColor(color)) {
    // java color = this.mFillStyle == 1 ? -1 : -16777216;
    // java }
    // java
    // java if (this.mFillStyle == 1 && color == -1) {
    // java paint.setStyle(Style.FILL);
    // java paint.setColor(-1);
    // java canvas.drawRect((float)rect.left, (float)rect.top, (float)(rect.right + 1), (float)(rect.bottom + 1), paint);
    // java }
    // java
    // java CellDrawerUtil.drawPatternStyle(color, this.mFillStyle, canvas, paint, rect);
    // java }
    // java
    // java }
    // java }
    pub fn drawFill(&self, c: &mut Canvas, p: &mut Paint, r: &Rect) {
        if r.isEmpty() {
            return;
        }
        if !KmoColor::isAutomaticColor(self.mBackColor) {
            p.style = 0;
            p.color = self.mBackColor;
            c.drawRect(r, p);
        }
        if self.mFillStyle != 0 {
            if self.mFillStyle == 32 {
                if let Some(g) = &self.mGradient {
                    *p = Paint::default();
                    p.style = 0;
                    CellGradientUtil::drawGradient(c, p, r, g);
                    *p = Paint::default();
                    return;
                }
            }
            let mut color = self.mForceColor;
            if KmoColor::isAutomaticColor(color) {
                color = if self.mFillStyle == 1 { -1 } else { -16777216 };
            }
            if self.mFillStyle == 1 && color == -1 {
                p.style = 0;
                p.color = -1;
                c.drawRect__float(
                    r.left as f32,
                    r.top as f32,
                    (r.right + 1) as f32,
                    (r.bottom + 1) as f32,
                    p,
                );
            }
            CellDrawerUtil::drawPatternStyle(color, self.mFillStyle, c, p, r);
        }
    }
    // java public final void clear() {
    // java this.mIsValid = false;
    // java this.mBackColor = 16777215;
    // java this.mForceColor = 16777215;
    // java this.mFillStyle = 0;
    // java this.mGradient = null;
    // java }
    pub fn clear(&mut self) {
        self.mIsValid = false;
        self.mBackColor = 16777215;
        self.mForceColor = 16777215;
        self.mFillStyle = 0;
        self.mGradient = None;
    }
    // java public final void copyFrom(BackFillData source) {
    // java this.mIsValid = source.mIsValid;
    // java this.mBackColor = source.mBackColor;
    // java this.mForceColor = source.mForceColor;
    // java this.mFillStyle = source.mFillStyle;
    // java this.mGradient = null;
    // java }
    pub fn copyFrom(&mut self, s: &Self) {
        self.mIsValid = s.mIsValid;
        self.mBackColor = s.mBackColor;
        self.mForceColor = s.mForceColor;
        self.mFillStyle = s.mFillStyle;
        self.mGradient = None;
    }
}
