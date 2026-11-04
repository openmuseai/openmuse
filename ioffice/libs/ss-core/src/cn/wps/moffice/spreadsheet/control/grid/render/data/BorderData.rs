use super::super::super::{
    layout::cache::CellXFCache::CellXFCache,
    TableStyle::{isAutomaticColor, TableStyle},
};
use super::super::{
    cache::MergedRangeL2Cache::MergedRangeL2Cache,
    graphics::{Canvas, Paint, Rect},
};
use crate::cn::wps::moffice::spreadsheet::utils::CellDrawerUtil::CellDrawerUtil;
use crate::{
    cn::wps::moss::{
        app::{xf::KmoXF::KmoXF, IGridSheet::IGridSheet},
        core::book::KmoPalette::KmoPalette,
    },
    java_runtime::Rc,
};
// java public class BorderData
#[derive(Clone)]
pub struct BorderData {
    pub mIsValid: bool,
    mCondfmtValid: bool,
    pub mLeftStyle: i16,
    pub mRightStyle: i16,
    pub mTopStyle: i16,
    pub mBottomStyle: i16,
    pub mDiagonalStyle: i16,
    pub mRevDiagonalStyle: i16,
    pub mInsideHorizStyle: i16,
    pub mInsideVertStyle: i16,
    pub mLeftColor: i32,
    pub mRightColor: i32,
    pub mTopColor: i32,
    pub mBottomColor: i32,
    pub mDiagonalColor: i32,
    pub mInsideHorizColor: i32,
    pub mInsideVertColor: i32,
}
impl Default for BorderData {
    fn default() -> Self {
        Self::new(true)
    }
}
impl BorderData {
    // java public BorderData(boolean condfmtValid) {
    // java this.mPoints = new float[4];
    // java this.mCondfmtValid = condfmtValid;
    // java }
    pub fn new(valid: bool) -> Self {
        Self {
            mIsValid: false,
            mCondfmtValid: valid,
            mLeftStyle: 0,
            mRightStyle: 0,
            mTopStyle: 0,
            mBottomStyle: 0,
            mDiagonalStyle: 0,
            mRevDiagonalStyle: 0,
            mInsideHorizStyle: 0,
            mInsideVertStyle: 0,
            mLeftColor: 0,
            mRightColor: 0,
            mTopColor: 0,
            mBottomColor: 0,
            mDiagonalColor: 0,
            mInsideHorizColor: 0,
            mInsideVertColor: 0,
        }
    }
    // java private KmoXF mergeBorderFormat(IGridSheet sheet, int rowIndex, int colIndex, KmoXF xf) {
    // java KmoSheet kmoSheet = sheet.coreSheet();
    // java KmoTableManager kmoTableManager = kmoSheet.getTableManager();
    // java KmoXF newXF = xf;
    // java if (null != kmoTableManager) {
    // java newXF = kmoTableManager.getMergeXF(rowIndex, colIndex, xf, (byte)1);
    // java }
    // java
    // java if (!this.mCondfmtValid) {
    // java return newXF;
    // java } else {
    // java KmoConditionFormatter cfFormatter = kmoSheet.getCFormatter();
    // java return cfFormatter.isEmpty() ? newXF : cfFormatter.getMergeXF(rowIndex, colIndex, newXF, (byte)1);
    // java }
    // java }
    fn mergeBorderFormat(
        &self,
        g: &dyn IGridSheet,
        row: i32,
        col: i32,
        xf: Rc<KmoXF>,
    ) -> Rc<KmoXF> {
        let core = g.coreSheet();
        let sheet = core.borrow();
        if !self.mCondfmtValid || sheet.getCFormatter().isEmpty() {
            xf
        } else {
            sheet.getCFormatter().getMergeXF(row, col, xf, 1)
        }
    }
    // java public boolean reset(IGridSheet sheet, int rowIndex, int colIndex, MergedRangeL2Cache mergedCache, CellXFCache cellXfCache, TableStyle shareTableStyle) {
    // java if (this.reset(rowIndex, colIndex, mergedCache, shareTableStyle)) {
    // java return true;
    // java } else {
    // java this.mIsValid = false;
    // java KmoXF xf = cellXfCache != null ? cellXfCache.get(sheet, rowIndex, colIndex) : sheet.getCellXF(rowIndex, colIndex);
    // java xf = this.mergeBorderFormat(sheet, rowIndex, colIndex, xf);
    // java if (xf != null && (xf.hasBorder() || xf.getAdtlDiagLineStyle() != 0)) {
    // java this.mLeftStyle = xf.getBorderLeft();
    // java this.mRightStyle = xf.getBorderRight();
    // java this.mTopStyle = xf.getBorderTop();
    // java this.mBottomStyle = xf.getBorderBottom();
    // java KmoRect address = mergedCache != null ? mergedCache.getMergedRange(rowIndex, colIndex) : null;
    // java if (address != null) {
    // java boolean isFirstRow = address.tl.row == rowIndex;
    // java boolean isLastRow = address.br.row == rowIndex;
    // java boolean isFirstCol = address.tl.col == colIndex;
    // java boolean isLastCol = address.br.col == colIndex;
    // java if (!isFirstRow) {
    // java this.mTopStyle = 0;
    // java }
    // java
    // java if (!isLastRow) {
    // java this.mBottomStyle = 0;
    // java }
    // java
    // java if (!isFirstCol) {
    // java this.mLeftStyle = 0;
    // java }
    // java
    // java if (!isLastCol) {
    // java this.mRightStyle = 0;
    // java }
    // java
    // java this.mDiagonalStyle = 0;
    // java this.mRevDiagonalStyle = 0;
    // java } else {
    // java this.mDiagonalStyle = this.getDianonalBorder(xf, false);
    // java this.mRevDiagonalStyle = this.getDianonalBorder(xf, true);
    // java }
    // java
    // java KmoPalette palette = sheet.getPalette();
    // java if (this.isValidBorder(this.mLeftStyle)) {
    // java this.mLeftColor = this.getRGBColor(palette, xf.getLeftBorderPaletteIdx());
    // java this.mIsValid = true;
    // java }
    // java
    // java if (this.isValidBorder(this.mRightStyle)) {
    // java this.mRightColor = this.getRGBColor(palette, xf.getRightBorderPaletteIdx());
    // java this.mIsValid = true;
    // java }
    // java
    // java if (this.isValidBorder(this.mTopStyle)) {
    // java this.mTopColor = this.getRGBColor(palette, xf.getTopBorderPaletteIdx());
    // java this.mIsValid = true;
    // java }
    // java
    // java if (this.isValidBorder(this.mBottomStyle)) {
    // java this.mBottomColor = this.getRGBColor(palette, xf.getBottomBorderPaletteIdx());
    // java this.mIsValid = true;
    // java }
    // java
    // java if (this.isValidBorder(this.mDiagonalStyle) || this.isValidBorder(this.mRevDiagonalStyle)) {
    // java this.mDiagonalColor = this.getRGBColor(palette, xf.getAdtlDiag());
    // java this.mIsValid = true;
    // java }
    // java
    // java return this.mIsValid;
    // java } else {
    // java return false;
    // java }
    // java }
    // java }
    pub fn reset(
        &mut self,
        g: &dyn IGridSheet,
        row: i32,
        col: i32,
        merged: Option<&MergedRangeL2Cache>,
        cache: Option<&CellXFCache>,
        table: &dyn TableStyle,
    ) -> bool {
        if self.reset__table(row, col, merged, table) {
            return true;
        }
        self.mIsValid = false;
        let xf = if let Some(c) = cache {
            c.get(g, row, col)
        } else {
            g.getCellXF(row, col)
        };
        let Some(xf) = xf else {
            return false;
        };
        let xf = self.mergeBorderFormat(g, row, col, xf);
        if !xf.hasBorder() && xf.getAdtlDiagLineStyle() == 0 {
            return false;
        }
        self.mLeftStyle = xf.getBorderLeft();
        self.mRightStyle = xf.getBorderRight();
        self.mTopStyle = xf.getBorderTop();
        self.mBottomStyle = xf.getBorderBottom();
        if let Some(m) = merged.and_then(|m| m.getMergedRange(row, col)) {
            if m.tl.row != row {
                self.mTopStyle = 0;
            }
            if m.br.row != row {
                self.mBottomStyle = 0;
            }
            if m.tl.col != col {
                self.mLeftStyle = 0;
            }
            if m.br.col != col {
                self.mRightStyle = 0;
            }
            self.mDiagonalStyle = 0;
            self.mRevDiagonalStyle = 0;
        } else {
            self.mDiagonalStyle = Self::getDianonalBorder(&xf, false);
            self.mRevDiagonalStyle = Self::getDianonalBorder(&xf, true);
        }
        let palette = g.getPalette();
        if self.mLeftStyle != 0 {
            self.mLeftColor = Self::getRGBColor(&palette, xf.getLeftBorderPaletteIdx());
            self.mIsValid = true;
        }
        if self.mRightStyle != 0 {
            self.mRightColor = Self::getRGBColor(&palette, xf.getRightBorderPaletteIdx());
            self.mIsValid = true;
        }
        if self.mTopStyle != 0 {
            self.mTopColor = Self::getRGBColor(&palette, xf.getTopBorderPaletteIdx());
            self.mIsValid = true;
        }
        if self.mBottomStyle != 0 {
            self.mBottomColor = Self::getRGBColor(&palette, xf.getBottomBorderPaletteIdx());
            self.mIsValid = true;
        }
        if self.mDiagonalStyle != 0 || self.mRevDiagonalStyle != 0 {
            self.mDiagonalColor = Self::getRGBColor(&palette, xf.getAdtlDiag());
            self.mIsValid = true;
        }
        self.mIsValid
    }
    // java private boolean reset(int rowIndex, int colIndex, MergedRangeL2Cache mergedCache, TableStyle tableStyle) {
    // java Integer borderColor = tableStyle.borderColor(rowIndex, colIndex);
    // java if (TableStyle.isAutomaticColor(borderColor)) {
    // java return false;
    // java } else {
    // java this.mIsValid = false;
    // java this.mLeftStyle = 1;
    // java this.mRightStyle = 1;
    // java this.mTopStyle = 1;
    // java this.mBottomStyle = 1;
    // java KmoRect address = mergedCache != null ? mergedCache.getMergedRange(rowIndex, colIndex) : null;
    // java if (address != null) {
    // java boolean isFirstRow = address.tl.row == rowIndex;
    // java boolean isLastRow = address.br.row == rowIndex;
    // java boolean isFirstCol = address.tl.col == colIndex;
    // java boolean isLastCol = address.br.col == colIndex;
    // java if (!isFirstRow) {
    // java this.mTopStyle = 0;
    // java }
    // java
    // java if (!isLastRow) {
    // java this.mBottomStyle = 0;
    // java }
    // java
    // java if (!isFirstCol) {
    // java this.mLeftStyle = 0;
    // java }
    // java
    // java if (!isLastCol) {
    // java this.mRightStyle = 0;
    // java }
    // java
    // java this.mDiagonalStyle = 0;
    // java this.mRevDiagonalStyle = 0;
    // java } else {
    // java this.mDiagonalStyle = 0;
    // java this.mRevDiagonalStyle = 0;
    // java }
    // java
    // java this.mLeftColor = borderColor;
    // java this.mRightColor = borderColor;
    // java this.mTopColor = borderColor;
    // java this.mBottomColor = borderColor;
    // java this.mIsValid = true;
    // java return this.mIsValid;
    // java }
    // java }
    fn reset__table(
        &mut self,
        row: i32,
        col: i32,
        merged: Option<&MergedRangeL2Cache>,
        table: &dyn TableStyle,
    ) -> bool {
        let color = table.borderColor(row, col);
        if isAutomaticColor(color) {
            return false;
        }
        self.mIsValid = false;
        self.mLeftStyle = 1;
        self.mRightStyle = 1;
        self.mTopStyle = 1;
        self.mBottomStyle = 1;
        if let Some(m) = merged.and_then(|m| m.getMergedRange(row, col)) {
            if m.tl.row != row {
                self.mTopStyle = 0;
            }
            if m.br.row != row {
                self.mBottomStyle = 0;
            }
            if m.tl.col != col {
                self.mLeftStyle = 0;
            }
            if m.br.col != col {
                self.mRightStyle = 0;
            }
        }
        self.mDiagonalStyle = 0;
        self.mRevDiagonalStyle = 0;
        self.mLeftColor = color.unwrap();
        self.mRightColor = color.unwrap();
        self.mTopColor = color.unwrap();
        self.mBottomColor = color.unwrap();
        self.mIsValid = true;
        true
    }
    // java public boolean resetDiagonal(KmoXF xf, KmoPalette palette) {
    // java if (null == xf) {
    // java return false;
    // java } else {
    // java this.mDiagonalStyle = this.getDianonalBorder(xf, false);
    // java this.mRevDiagonalStyle = this.getDianonalBorder(xf, true);
    // java if (!this.isValidBorder(this.mDiagonalStyle) && !this.isValidBorder(this.mRevDiagonalStyle)) {
    // java return false;
    // java } else {
    // java this.mDiagonalColor = this.getRGBColor(palette, xf.getAdtlDiag());
    // java return true;
    // java }
    // java }
    // java }
    pub fn resetDiagonal(&mut self, xf: Option<&KmoXF>, p: &KmoPalette) -> bool {
        let Some(xf) = xf else {
            return false;
        };
        self.mDiagonalStyle = Self::getDianonalBorder(xf, false);
        self.mRevDiagonalStyle = Self::getDianonalBorder(xf, true);
        if self.mDiagonalStyle == 0 && self.mRevDiagonalStyle == 0 {
            false
        } else {
            self.mDiagonalColor = Self::getRGBColor(p, xf.getAdtlDiag());
            true
        }
    }
    // java private final short getDianonalBorder(KmoXF kmoXF, boolean isReverse) {
    // java short border = (short)kmoXF.getDiag();
    // java short borderStyle = kmoXF.getAdtlDiagLineStyle();
    // java if (isReverse) {
    // java return (border & 1) == 0 ? 0 : borderStyle;
    // java } else {
    // java return (border & 2) == 0 ? 0 : borderStyle;
    // java }
    // java }
    fn getDianonalBorder(xf: &KmoXF, reverse: bool) -> i16 {
        let border = xf.getDiag() as i16;
        let style = xf.getAdtlDiagLineStyle();
        if reverse {
            if border & 1 == 0 {
                0
            } else {
                style
            }
        } else {
            if border & 2 == 0 {
                0
            } else {
                style
            }
        }
    }
    // java public int getDrawOffset(int style) {
    // java int offset = 0;
    // java return style != 2 && style != 5 ? offset : 1;
    // java }
    pub fn getDrawOffset(&self, style: i32) -> i32 {
        if style == 2 || style == 5 {
            1
        } else {
            0
        }
    }
    // java private int getRGBColor(KmoPalette palette, int color) {
    // java return ColorToolKit.isPaletteIndex(color) ? palette.getRGBColor((short)color) : color;
    // java }
    fn getRGBColor(p: &KmoPalette, color: i32) -> i32 {
        if color & -16777216 == 0 {
            p.getRGBColor(color as i16)
        } else {
            color
        }
    }
    // java public void copyFrom(BorderData src) {
    // java this.mIsValid = src.mIsValid;
    // java this.mLeftStyle = src.mLeftStyle;
    // java this.mRightStyle = src.mRightStyle;
    // java this.mTopStyle = src.mTopStyle;
    // java this.mBottomStyle = src.mBottomStyle;
    // java this.mDiagonalStyle = src.mDiagonalStyle;
    // java this.mRevDiagonalStyle = src.mRevDiagonalStyle;
    // java this.mInsideHorizStyle = src.mInsideHorizStyle;
    // java this.mInsideVertStyle = src.mInsideVertStyle;
    // java this.mLeftColor = src.mLeftColor;
    // java this.mRightColor = src.mRightColor;
    // java this.mTopColor = src.mTopColor;
    // java this.mBottomColor = src.mBottomColor;
    // java this.mDiagonalColor = src.mDiagonalColor;
    // java this.mInsideHorizColor = src.mInsideHorizColor;
    // java this.mInsideVertColor = src.mInsideVertColor;
    // java }
    pub fn copyFrom(&mut self, s: &Self) {
        let valid = self.mCondfmtValid;
        *self = s.clone();
        self.mCondfmtValid = valid;
    }
    // java public boolean equals(Object o) {
    // java if (!(o instanceof BorderData)) {
    // java return false;
    // java } else {
    // java BorderData renderer = (BorderData)o;
    // java return this.mTopStyle == renderer.mTopStyle && this.mTopColor == renderer.mTopColor && this.mBottomStyle == renderer.mBottomStyle && this.mBottomColor == renderer.mBottomColor && this.mLeftStyle == renderer.mLeftStyle && this.mLeftColor == renderer.mLeftColor && this.mRightStyle == renderer.mRightStyle && this.mRightColor == renderer.mRightColor && this.mDiagonalStyle == renderer.mDiagonalStyle && this.mDiagonalColor == renderer.mDiagonalColor && this.mRevDiagonalStyle == renderer.mRevDiagonalStyle;
    // java }
    // java }
    pub fn equals(&self, s: &Self) -> bool {
        self.mTopStyle == s.mTopStyle
            && self.mTopColor == s.mTopColor
            && self.mBottomStyle == s.mBottomStyle
            && self.mBottomColor == s.mBottomColor
            && self.mLeftStyle == s.mLeftStyle
            && self.mLeftColor == s.mLeftColor
            && self.mRightStyle == s.mRightStyle
            && self.mRightColor == s.mRightColor
            && self.mDiagonalStyle == s.mDiagonalStyle
            && self.mDiagonalColor == s.mDiagonalColor
            && self.mRevDiagonalStyle == s.mRevDiagonalStyle
    }
    // java public void drawBorder(short border, Canvas canvas, Paint paint, Rect rect) {
    // java int offset = 0;
    // java if ((border & 1) != 0 && this.isValidBorder(this.mLeftStyle)) {
    // java offset = this.getDrawOffset(this.mLeftStyle);
    // java this.mPoints[0] = (float)rect.left;
    // java this.mPoints[1] = (float)(rect.bottom - offset);
    // java this.mPoints[2] = (float)rect.left;
    // java this.mPoints[3] = (float)(rect.top + offset);
    // java CellDrawerUtil.drawBorderLine(this.mLeftStyle, canvas, paint, this.mLeftColor, this.mPoints);
    // java }
    // java
    // java if ((border & 2) != 0 && this.isValidBorder(this.mRightStyle)) {
    // java offset = this.getDrawOffset(this.mRightStyle);
    // java this.mPoints[0] = (float)rect.right;
    // java this.mPoints[1] = (float)(rect.bottom - offset);
    // java this.mPoints[2] = (float)rect.right;
    // java this.mPoints[3] = (float)(rect.top + offset);
    // java CellDrawerUtil.drawBorderLine(this.mRightStyle, canvas, paint, this.mRightColor, this.mPoints);
    // java }
    // java
    // java if ((border & 4) != 0 && this.isValidBorder(this.mTopStyle)) {
    // java offset = this.getDrawOffset(this.mTopStyle);
    // java this.mPoints[0] = (float)(rect.left + offset);
    // java this.mPoints[1] = (float)rect.top;
    // java this.mPoints[2] = (float)(rect.right - offset);
    // java this.mPoints[3] = (float)rect.top;
    // java CellDrawerUtil.drawBorderLine(this.mTopStyle, canvas, paint, this.mTopColor, this.mPoints);
    // java }
    // java
    // java if ((border & 8) != 0 && this.isValidBorder(this.mBottomStyle)) {
    // java offset = this.getDrawOffset(this.mBottomStyle);
    // java this.mPoints[0] = (float)(rect.left + offset);
    // java this.mPoints[1] = (float)rect.bottom;
    // java this.mPoints[2] = (float)(rect.right - offset);
    // java this.mPoints[3] = (float)rect.bottom;
    // java CellDrawerUtil.drawBorderLine(this.mBottomStyle, canvas, paint, this.mBottomColor, this.mPoints);
    // java }
    // java
    // java if ((border & 16) != 0 && this.isValidBorder(this.mDiagonalStyle)) {
    // java this.mPoints[0] = (float)rect.right;
    // java this.mPoints[1] = (float)rect.top;
    // java this.mPoints[2] = (float)rect.left;
    // java this.mPoints[3] = (float)rect.bottom;
    // java CellDrawerUtil.drawBorderLine(this.mDiagonalStyle, canvas, paint, this.mDiagonalColor, this.mPoints);
    // java }
    // java
    // java if ((border & 32) != 0 && this.isValidBorder(this.mRevDiagonalStyle)) {
    // java this.mPoints[0] = (float)rect.right;
    // java this.mPoints[1] = (float)rect.bottom;
    // java this.mPoints[2] = (float)rect.left;
    // java this.mPoints[3] = (float)rect.top;
    // java CellDrawerUtil.drawBorderLine(this.mRevDiagonalStyle, canvas, paint, this.mDiagonalColor, this.mPoints);
    // java }
    // java
    // java if ((border & 64) != 0 && this.isValidBorder(this.mInsideHorizStyle)) {
    // java this.mPoints[0] = (float)rect.right;
    // java this.mPoints[1] = rect.exactCenterY();
    // java this.mPoints[2] = (float)rect.left;
    // java this.mPoints[3] = rect.exactCenterY();
    // java CellDrawerUtil.drawBorderLine(this.mInsideHorizStyle, canvas, paint, this.mInsideHorizColor, this.mPoints);
    // java }
    // java
    // java if ((border & 128) != 0 && this.isValidBorder(this.mInsideVertStyle)) {
    // java this.mPoints[0] = rect.exactCenterX();
    // java this.mPoints[1] = (float)rect.bottom;
    // java this.mPoints[2] = rect.exactCenterX();
    // java this.mPoints[3] = (float)rect.top;
    // java CellDrawerUtil.drawBorderLine(this.mInsideVertStyle, canvas, paint, this.mInsideVertColor, this.mPoints);
    // java }
    // java
    // java }
    pub fn drawBorder(&self, border: i16, c: &mut Canvas, p: &mut Paint, r: &Rect) {
        if border & 1 != 0 && self.mLeftStyle != 0 {
            let offset = self.getDrawOffset(self.mLeftStyle as i32);
            let mut points = [
                r.left as f32,
                (r.bottom - offset) as f32,
                r.left as f32,
                (r.top + offset) as f32,
            ];
            CellDrawerUtil::drawBorderLine(self.mLeftStyle, c, p, self.mLeftColor, &mut points);
        }
        if border & 2 != 0 && self.mRightStyle != 0 {
            let offset = self.getDrawOffset(self.mRightStyle as i32);
            let mut points = [
                r.right as f32,
                (r.bottom - offset) as f32,
                r.right as f32,
                (r.top + offset) as f32,
            ];
            CellDrawerUtil::drawBorderLine(self.mRightStyle, c, p, self.mRightColor, &mut points);
        }
        if border & 4 != 0 && self.mTopStyle != 0 {
            let offset = self.getDrawOffset(self.mTopStyle as i32);
            let mut points = [
                (r.left + offset) as f32,
                r.top as f32,
                (r.right - offset) as f32,
                r.top as f32,
            ];
            CellDrawerUtil::drawBorderLine(self.mTopStyle, c, p, self.mTopColor, &mut points);
        }
        if border & 8 != 0 && self.mBottomStyle != 0 {
            let offset = self.getDrawOffset(self.mBottomStyle as i32);
            let mut points = [
                (r.left + offset) as f32,
                r.bottom as f32,
                (r.right - offset) as f32,
                r.bottom as f32,
            ];
            CellDrawerUtil::drawBorderLine(self.mBottomStyle, c, p, self.mBottomColor, &mut points);
        }
        if border & 16 != 0 && self.mDiagonalStyle != 0 {
            let offset = self.getDrawOffset(self.mDiagonalStyle as i32);
            let mut points = [r.right as f32, r.top as f32, r.left as f32, r.bottom as f32];
            CellDrawerUtil::drawBorderLine(
                self.mDiagonalStyle,
                c,
                p,
                self.mDiagonalColor,
                &mut points,
            );
        }
        if border & 32 != 0 && self.mRevDiagonalStyle != 0 {
            let offset = self.getDrawOffset(self.mRevDiagonalStyle as i32);
            let mut points = [r.right as f32, r.bottom as f32, r.left as f32, r.top as f32];
            CellDrawerUtil::drawBorderLine(
                self.mRevDiagonalStyle,
                c,
                p,
                self.mDiagonalColor,
                &mut points,
            );
        }
        if border & 64 != 0 && self.mInsideHorizStyle != 0 {
            let offset = self.getDrawOffset(self.mInsideHorizStyle as i32);
            let mut points = [
                r.right as f32,
                (r.top + r.bottom) as f32 * 0.5,
                r.left as f32,
                (r.top + r.bottom) as f32 * 0.5,
            ];
            CellDrawerUtil::drawBorderLine(
                self.mInsideHorizStyle,
                c,
                p,
                self.mInsideHorizColor,
                &mut points,
            );
        }
        if border & 128 != 0 && self.mInsideVertStyle != 0 {
            let offset = self.getDrawOffset(self.mInsideVertStyle as i32);
            let mut points = [
                (r.left + r.right) as f32 * 0.5,
                r.bottom as f32,
                (r.left + r.right) as f32 * 0.5,
                r.top as f32,
            ];
            CellDrawerUtil::drawBorderLine(
                self.mInsideVertStyle,
                c,
                p,
                self.mInsideVertColor,
                &mut points,
            );
        }
    }
}
