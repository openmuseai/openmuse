use super::super::super::{
    core::GridSheetLayoutInfo::GridSheetLayoutInfo,
    render::graphics::{Canvas, PointF, Rect},
};
use super::super::{data::CellTextData::CellTextData, ICellTextLayout::ICellTextLayout};
use super::CellRichTextLayout::CellRichTextLayout;
use crate::cn::wps::text::layout::typo::layout::Layout::Layout;
#[derive(Default)]
pub struct AngledTextLayout2 {
    pub base: CellRichTextLayout,
    mAngle: i16,
    mRadiansAngle: f32,
    sin: f32,
    cos: f32,
}
impl AngledTextLayout2 {
    // java private float calcTranslateX() {
    // java if (this.mAngle > 0 && this.mAngle <= 90) {
    // java return this.mTypoOffset.x + (Float)this.mTypoCache.getLineHeights().get(0) * this.sin;
    // java } else if (this.mAngle >= -90 && this.mAngle < 0) {
    // java return this.mTypoOffset.x - ((Float)this.mTypoCache.getLineWidths().get(0) - ((float)this.mTypoCache.getAngleRectWidth() - (Float)this.mTypoCache.getLineHeights().get(0) * this.sin));
    // java } else {
    // java throw new IllegalArgumentException("mAngle should in [-90,90]");
    // java }
    // java }
    fn calcTranslateX(&self) -> f32 {
        let cache = self.base.mTypoCache.as_ref().unwrap();
        if self.mAngle > 0 && self.mAngle <= 90 {
            self.base.mTypoOffset.x + cache.mLineHeights[0] * self.sin
        } else if self.mAngle >= -90 && self.mAngle < 0 {
            self.base.mTypoOffset.x
                - (cache.mLineWidths[0]
                    - (cache.mAngleRectWidth as f32 - cache.mLineHeights[0] * self.sin))
        } else {
            panic!("mAngle should in [-90,90]")
        }
    }
    // java private float calcRotateX() {
    // java if (this.mAngle > 0 && this.mAngle <= 90) {
    // java return 0.0F;
    // java } else if (this.mAngle >= -90 && this.mAngle < 0) {
    // java return (Float)this.mTypoCache.getLineWidths().get(0);
    // java } else {
    // java throw new IllegalArgumentException("mAngle should in [-90,90]");
    // java }
    // java }
    fn calcRotateX(&self) -> f32 {
        if self.mAngle > 0 && self.mAngle <= 90 {
            0.0
        } else if self.mAngle >= -90 && self.mAngle < 0 {
            self.base.mTypoCache.as_ref().unwrap().mLineWidths[0]
        } else {
            panic!("mAngle should in [-90,90]")
        }
    }
}
impl ICellTextLayout for AngledTextLayout2 {
    // java public void typo(GridSheetLayoutInfo layoutInfo, CellTextData data, int width, int height, boolean forceTypo) {
    // java this.mAngle = data.mPara.mAngle;
    // java if (this.mAngle == 0) {
    // java super.typo(layoutInfo, data, width, height, forceTypo);
    // java } else {
    // java this.mRadiansAngle = (float)Math.toRadians((double)MathUtil.abs(this.mAngle));
    // java this.sin = (float)Math.sin((double)this.mRadiansAngle);
    // java this.cos = (float)Math.cos((double)this.mRadiansAngle);
    // java super.typo(layoutInfo, data, width, height, forceTypo);
    // java }
    // java }
    fn typo(&mut self, l: &GridSheetLayoutInfo, d: &mut CellTextData, w: i32, h: i32, force: bool) {
        self.mAngle = d.mPara.mAngle;
        if self.mAngle != 0 {
            self.mRadiansAngle = ((self.mAngle as f64).abs() * 0.017453292519943295) as f32;
            self.sin = (self.mRadiansAngle as f64).sin() as f32;
            self.cos = (self.mRadiansAngle as f64).cos() as f32;
        }
        self.base.typo(l, d, w, h, force);
    }
    // java public void renderer(Canvas canvas) {
    // java if (this.mAngle == 0) {
    // java super.renderer(canvas);
    // java } else {
    // java canvas.save();
    // java
    // java try {
    // java float dx = this.calcTranslateX();
    // java float dy = this.mTypoOffset.y + (float)this.mTypoCache.getAngleRectHeight() - (Float)this.mTypoCache.getLineHeights().get(0);
    // java float rotateX = this.calcRotateX();
    // java float rotateY = (Float)this.mTypoCache.getLineHeights().get(0);
    // java canvas.translate(dx, dy);
    // java canvas.rotate((float)(-this.mAngle), rotateX, rotateY);
    // java this.mTypoCache.draw(canvas, (Path)null, (Paint)null, 0);
    // java this.mUnderlineCache.draw(canvas, this.mTypoCache, this.mAngle);
    // java } catch (Exception e) {
    // java e.printStackTrace();
    // java }
    // java
    // java canvas.restore();
    // java }
    // java }
    fn renderer(&mut self, c: &mut Canvas) {
        if self.mAngle == 0 {
            self.base.renderer(c);
        } else {
            c.save();
            let l = self.base.mTypoCache.as_ref().unwrap();
            let dx = self.calcTranslateX();
            let dy = self.base.mTypoOffset.y + l.mAngleRectHeight as f32 - l.mLineHeights[0];
            c.translate(dx, dy);
            c.rotate__pivot(-self.mAngle as f32, self.calcRotateX(), l.mLineHeights[0]);
            l.draw(c);
            self.base
                .base
                .mUnderlineCache
                .draw__layout(c, Some(l), self.mAngle);
            c.restore();
        }
    }
    fn getTextRect(&self) -> Rect {
        self.base.getTextRect()
    }
    fn getTypoOffset(&self) -> PointF {
        self.base.getTypoOffset()
    }
    fn clear(&mut self) {
        self.base.clear();
    }
    fn className(&self) -> &'static str {
        "AngledTextLayout2"
    }
}
