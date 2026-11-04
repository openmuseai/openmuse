use super::super::super::{
    core::GridSheetLayoutInfo::GridSheetLayoutInfo,
    render::graphics::{Canvas, FontMetricsInt, Paint, PointF, Rect},
};
use super::super::{
    data::{CellTextData::CellTextData, ParaInfo::ParaInfo},
    ICellTextLayout::ICellTextLayout,
};
use super::BaseCellTextLayout::BaseCellTextLayout;
use super::PaintFontHelper::PaintFontHelper;
#[derive(Default)]
pub struct SimpleTextLayout {
    pub base: BaseCellTextLayout,
    pub mFontMetrics: FontMetricsInt,
}
impl std::ops::Deref for SimpleTextLayout {
    type Target = BaseCellTextLayout;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl std::ops::DerefMut for SimpleTextLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl SimpleTextLayout {
    // java protected float calcHorizAlign(ParaInfo paraInfo, Paint paint, float cellWidth, float typoWidth) {
    // java float offset = 0.0F;
    // java switch (paraInfo.mHAlignment) {
    // java case 0:
    // java if (!this.rtl) {
    // java paint.setTextAlign(Align.LEFT);
    // java offset = (float)(2 + paraInfo.mIndent);
    // java } else {
    // java paint.setTextAlign(Align.LEFT);
    // java offset = cellWidth - typoWidth - (float)paraInfo.mIndent - 2.0F;
    // java }
    // java break;
    // java case 1:
    // java case 5:
    // java paint.setTextAlign(Align.LEFT);
    // java offset = (float)(2 + paraInfo.mIndent);
    // java break;
    // java case 2:
    // java case 6:
    // java case 7:
    // java paint.setTextAlign(Align.LEFT);
    // java if (cellWidth > typoWidth) {
    // java offset = (cellWidth - typoWidth) * 0.5F;
    // java }
    // java break;
    // java case 3:
    // java paint.setTextAlign(Align.LEFT);
    // java offset = cellWidth - typoWidth - (float)paraInfo.mIndent - 2.0F;
    // java break;
    // java case 4:
    // java paint.setTextAlign(Align.LEFT);
    // java offset = 0.0F;
    // java break;
    // java default:
    // java paint.setTextAlign(Align.LEFT);
    // java }
    // java
    // java return offset;
    // java }
    fn calcHorizAlign(&mut self, p: &ParaInfo, w: f32, tw: f32) -> f32 {
        self.mTextPaint.textAlign = 0;
        match p.mHAlignment {
            0 => {
                if self.rtl {
                    w - tw - p.mIndent as f32 - 2.0
                } else {
                    (2 + p.mIndent as i32) as f32
                }
            }
            1 | 5 => (2 + p.mIndent as i32) as f32,
            2 | 6 | 7 => {
                if w > tw {
                    (w - tw) * 0.5
                } else {
                    0.0
                }
            }
            3 => w - tw - p.mIndent as f32 - 2.0,
            _ => 0.0,
        }
    }
    // java protected float calcVertAlign(ParaInfo paraInfo, Paint paint, float cellHeight, float typoHeight) {
    // java float offset = 0.0F;
    // java switch (paraInfo.mVAlignment) {
    // java case 0:
    // java case 3:
    // java paint.getFontMetricsInt(this.mFontMetrics);
    // java offset = (float)(0 - this.mFontMetrics.top);
    // java break;
    // java case 1:
    // java case 4:
    // java offset = cellHeight * 0.5F;
    // java if (this.mTextData.mFirstSpan.mIsSuperScript) {
    // java offset -= typoHeight * 0.05F;
    // java if (offset < 0.0F) {
    // java offset = 0.0F;
    // java }
    // java } else if (this.mTextData.mFirstSpan.mIsSubScript) {
    // java offset += typoHeight * 0.5F;
    // java } else {
    // java offset += typoHeight * 0.3F;
    // java }
    // java break;
    // java case 2:
    // java paint.getFontMetricsInt(this.mFontMetrics);
    // java offset += cellHeight - (float)this.mFontMetrics.bottom;
    // java }
    // java
    // java return offset;
    // java }
    fn calcVertAlign(&mut self, p: &ParaInfo, h: f32, th: f32, d: &CellTextData) -> f32 {
        match p.mVAlignment {
            0 | 3 => {
                self.mFontMetrics = self.mTextPaint.getFontMetricsInt();
                -self.mFontMetrics.top as f32
            }
            1 | 4 => {
                let mut o = h * 0.5;
                if d.mFirstSpan.mIsSuperScript {
                    o -= th * 0.05;
                    if o < 0.0 {
                        o = 0.0;
                    }
                } else if d.mFirstSpan.mIsSubScript {
                    o += th * 0.5;
                } else {
                    o += th * 0.3;
                }
                o
            }
            2 => {
                self.mFontMetrics = self.mTextPaint.getFontMetricsInt();
                h - self.mFontMetrics.bottom as f32
            }
            _ => 0.0,
        }
    }
}
impl ICellTextLayout for SimpleTextLayout {
    // java public void typo(GridSheetLayoutInfo layoutInfo, CellTextData data, int width, int height, boolean forceTypo) {
    // java super.typo(layoutInfo, data, width, height, forceTypo);
    // java PaintFontHelper.setPaintForText(data.mFirstSpan, this.mTextPaint, false);
    // java if (data.mPara.mIsShrinkToFit) {
    // java int rectWidth = this.mTextRect.width();
    // java float textWidth = this.mTextPaint.measureText(data.mSourceText);
    // java if (textWidth > (float)rectWidth) {
    // java float textSize = data.mFirstSpan.mFontSize * (float)rectWidth / textWidth;
    // java this.mTextPaint.setTextSize(textSize);
    // java }
    // java }
    // java
    // java this.mTextPaint.getFontMetricsInt(this.mFontMetrics);
    // java float typoWidth = this.mTextPaint.measureText(data.mSourceText);
    // java float italicFontSize = data.italicFontSize();
    // java if (italicFontSize > 0.0F) {
    // java typoWidth += italicFontSize * 0.2F;
    // java }
    // java
    // java this.setTextAlignment(data.mPara, this.mTextRect, this.mTextPaint, typoWidth, (float)(this.mFontMetrics.bottom - this.mFontMetrics.top));
    // java this.mUnderlineCache.calcUnderline(this.mFontMetrics, typoWidth, data);
    // java }
    fn typo(&mut self, l: &GridSheetLayoutInfo, d: &mut CellTextData, w: i32, h: i32, force: bool) {
        self.base.typo(l, d, w, h, force);
        PaintFontHelper::setPaintForText(&d.mFirstSpan, &mut self.base.mTextPaint, false);
        if d.mPara.mIsShrinkToFit {
            let rw = self.mTextRect.width();
            let tw = self.mTextPaint.measureText(d.source());
            if tw > rw as f32 {
                self.base.mTextPaint.textSize = d.mFirstSpan.mFontSize * rw as f32 / tw;
            }
        }
        self.mFontMetrics = self.mTextPaint.getFontMetricsInt();
        let mut tw = self.mTextPaint.measureText(d.source());
        let italic = d.italicFontSize();
        if italic > 0.0 {
            tw += italic * 0.2;
        }
        let x = self.mTextRect.left as f32
            + self.calcHorizAlign(&d.mPara, self.mTextRect.width() as f32, tw);
        let y = self.mTextRect.top as f32
            + self.calcVertAlign(
                &d.mPara,
                self.mTextRect.height() as f32,
                (self.mFontMetrics.bottom - self.mFontMetrics.top) as f32,
                d,
            );
        self.base.mTypoOffset = PointF { x, y };
        let fm = self.mFontMetrics.clone();
        self.base.mUnderlineCache.calcUnderline(&fm, tw, d);
    }
    // java public void renderer(Canvas canvas) {
    // java canvas.save();
    // java canvas.clipRect(this.mTextRect);
    // java canvas.translate(this.mTypoOffset.x, this.mTypoOffset.y);
    // java canvas.drawText(this.mTextData.mSourceText, 0.0F, 0.0F, this.mTextPaint);
    // java this.mUnderlineCache.draw(canvas);
    // java canvas.restore();
    // java }
    fn renderer(&mut self, c: &mut Canvas) {
        c.save();
        c.clipRect(&self.mTextRect);
        c.translate(self.mTypoOffset.x, self.mTypoOffset.y);
        c.drawText(
            self.mTextData.as_ref().expect("text").source(),
            0.0,
            0.0,
            &self.mTextPaint,
        );
        self.base.mUnderlineCache.draw(c);
        c.restore();
    }
    fn getTextRect(&self) -> Rect {
        self.mTextRect
    }
    fn getTypoOffset(&self) -> PointF {
        self.mTypoOffset
    }
    fn clear(&mut self) {
        self.base.clear();
    }
    fn className(&self) -> &'static str {
        "SimpleTextLayout"
    }
}
