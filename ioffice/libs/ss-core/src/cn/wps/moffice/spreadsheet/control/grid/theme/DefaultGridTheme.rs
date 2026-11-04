use super::super::{layout::typo::PaintFontHelper::PaintFontHelper, render::native::Paint};
use crate::cn::wps::moss::app::{IGridSheet::IGridSheet, UnitsConverter::UnitsConverter};
pub static USE_PHONE_THEME: crate::java_runtime::Cell<bool> = crate::java_runtime::Cell::new(false);
pub static IS_NIGHT_MODE: crate::java_runtime::Cell<bool> = crate::java_runtime::Cell::new(false);
// java public class DefaultGridTheme implements IGridTheme
pub struct DefaultGridTheme {
    pub mHeaderTextPaint: Paint,
    pub mNormalTextPaint: Paint,
    pub mNormalTextPaintUnScale: Paint,
    pub mNormalFontSize: f32,
    pub usePhone: bool,
    pub mHeaderTextColor: i32,
    pub mGridlineColor: i32,
}
impl DefaultGridTheme {
    // java public static void setThemeStyle(boolean usePhone) {
    // java USE_PHONE_THEME = usePhone;
    // java }
    pub fn setThemeStyle(v: bool) {
        USE_PHONE_THEME.set(v);
    }
    // java public static void setIsNightMode(boolean isNightMode) {
    // java mIsNightMode = isNightMode;
    // java }
    pub fn setIsNightMode(v: bool) {
        IS_NIGHT_MODE.set(v);
    }

    // java public DefaultGridTheme(UnitsConverter aUnitsConverter) {
    // java this.mNormalFontSize = aUnitsConverter.PointsToPixels(10.0F);
    // java this.resetThemeColor(USE_PHONE_THEME);
    // java }
    pub fn new(c: &UnitsConverter) -> Self {
        Self {
            mHeaderTextPaint: Paint::default(),
            mNormalTextPaint: Paint::default(),
            mNormalTextPaintUnScale: Paint::default(),
            mNormalFontSize: c.PointsToPixels(10.0),
            usePhone: USE_PHONE_THEME.get(),
            mHeaderTextColor: if USE_PHONE_THEME.get() {
                -10395295
            } else if IS_NIGHT_MODE.get() {
                -856294913
            } else {
                -13420480
            },
            mGridlineColor: -1973791,
        }
    }
    // java protected void setNormalText(IGridSheet gridSheet, UnitsConverter converter) {
    // java this.mNormalFontSize = converter.PointsToPixels(10.0F);
    // java this.mNormalTextPaint.reset();
    // java this.mNormalTextPaint.setFlags(1);
    // java this.mNormalTextPaint.setTypeface(this.defaultFont());
    // java this.mNormalTextPaint.setTextSize(this.mNormalFontSize);
    // java this.mNormalTextPaint.setColor(-16777216);
    // java this.setNormalTextUnScale(gridSheet, converter);
    // java }
    pub fn setNormalText(&mut self, _grid: &dyn IGridSheet, c: &UnitsConverter) {
        self.mNormalFontSize = c.PointsToPixels(10.0);
        self.mNormalTextPaint = Paint {
            flags: 1,
            textSize: self.mNormalFontSize,
            ..Paint::default()
        };
        self.setNormalTextUnScale(c);
    }
    // java private void setNormalTextUnScale(IGridSheet gridSheet, UnitsConverter converter) {
    // java float fontSize = converter.PointsToPixelsUnScale(10.0F);
    // java this.mNormalTextPaintUnScale.reset();
    // java this.mNormalTextPaintUnScale.setFlags(1);
    // java this.mNormalTextPaintUnScale.setTypeface(this.defaultFont());
    // java this.mNormalTextPaintUnScale.setTextSize(fontSize);
    // java this.mNormalTextPaintUnScale.setColor(-16777216);
    // java }
    fn setNormalTextUnScale(&mut self, c: &UnitsConverter) {
        self.mNormalTextPaintUnScale = Paint {
            flags: 1,
            textSize: c.PointsToPixelsUnScale(10.0),
            ..Paint::default()
        };
    }
    // java protected void setHeaderText(float zoom) {
    // java this.mHeaderTextPaint.set(this.mNormalTextPaint);
    // java this.mHeaderTextPaint.setTextAlign(Align.LEFT);
    // java if (zoom > 1.5F || zoom < 0.3F) {
    // java float font = this.mHeaderTextPaint.getTextSize();
    // java font = font / zoom * (zoom > 1.5F ? 1.5F : 0.3F);
    // java this.mHeaderTextPaint.setTextSize(font);
    // java }
    // java
    // java this.mHeaderTextPaint.setColor(this.mHeaderTextColor);
    // java this.mHeaderFontInfo.measure(this.mHeaderTextPaint);
    // java }
    pub fn setHeaderText(&mut self, zoom: f32) {
        self.mHeaderTextPaint = self.mNormalTextPaint.clone();
        self.mHeaderTextPaint.textAlign = 0;
        if zoom > 1.5 || zoom < 0.3 {
            self.mHeaderTextPaint.textSize =
                self.mHeaderTextPaint.textSize / zoom * if zoom > 1.5 { 1.5 } else { 0.3 };
        }
        self.mHeaderTextPaint.color = self.mHeaderTextColor;
    }
    // java public float getNormalCharWidth() {
    // java return PaintFontHelper.getCharWidth(this.mNormalTextPaint);
    // java }
    pub fn getNormalCharWidth(&self) -> f32 {
        PaintFontHelper::getCharWidth(&self.mNormalTextPaint)
    }
    // java public float getNormalCharWidthUnScale() {
    // java return PaintFontHelper.getCharWidth(this.mNormalTextPaintUnScale);
    // java }
    pub fn getNormalCharWidthUnScale(&self) -> f32 {
        PaintFontHelper::getCharWidth(&self.mNormalTextPaintUnScale)
    }
    // java public float getNormalCharHeight() {
    // java return this.getCharHeight(this.mNormalTextPaint);
    // java }
    pub fn getNormalCharHeight(&self) -> f32 {
        Self::getCharHeight(&self.mNormalTextPaint)
    }
    // java public int getHeaderColWidth() {
    // java return (int)(PaintFontHelper.getCharWidth(this.mHeaderTextPaint) * HEADER_COL_WIDTH_RIDIO);
    // java }
    pub fn getHeaderColWidth(&self) -> i32 {
        (PaintFontHelper::getCharWidth(&self.mHeaderTextPaint)
            * if self.usePhone { 5.2 } else { 4.0 }) as i32
    }
    // java public int getHeaderRowHeight() {
    // java return (int)(this.getCharHeight(this.mHeaderTextPaint) * HEADER_ROW_HEIGHT_RIDIO);
    // java }
    pub fn getHeaderRowHeight(&self) -> i32 {
        (Self::getCharHeight(&self.mHeaderTextPaint) * if self.usePhone { 1.5 } else { 1.0 }) as i32
    }
    // java private float getCharHeight(Paint paint) {
    // java paint.getFontMetrics(this.mFontMetrics);
    // java return this.mFontMetrics.bottom - this.mFontMetrics.top;
    // java }
    fn getCharHeight(p: &Paint) -> f32 {
        let fm = p.getFontMetrics();
        fm.bottom - fm.top
    }
    // java public void setEmptyCellFillStyle(Paint paint) {
    // java paint.reset();
    // java paint.setStyle(Style.FILL);
    // java paint.setColor(-1);
    // java }
    pub fn setEmptyCellFillStyle(&self, p: &mut Paint) {
        *p = Paint::default();
        p.color = -1;
    }
    // java public void setBackgroundFillStyle(Paint paint) {
    // java paint.reset();
    // java paint.setStyle(Style.FILL);
    // java paint.setColor(-5395027);
    // java }
    pub fn setBackgroundFillStyle(&self, p: &mut Paint) {
        *p = Paint::default();
        p.color = -5395027;
    }
    // java public void setCellBorderStyle(Paint paint) {
    // java paint.reset();
    // java paint.setStyle(Style.STROKE);
    // java paint.setStrokeWidth(1.0F);
    // java paint.setColor(Color.rgb(208, 215, 229));
    // java }
    pub fn setCellBorderStyle(&self, p: &mut Paint) {
        *p = Paint::default();
        p.style = 1;
        p.strokeWidth = 1.0;
        p.color = -3090459;
    }
    // java public void setGridlineStyle(Paint paint) {
    // java paint.setColor(this.mGridlineColor);
    // java }
    pub fn setGridlineStyle(&self, p: &mut Paint) {
        p.color = self.mGridlineColor;
    }
}
