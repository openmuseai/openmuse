use crate::cn::wps::moffice::spreadsheet::control::grid::{
    layout::data::{SpanInfo::SpanInfo, TextData::TextData},
    render::native::{FontMetricsInt, Paint},
};
// java public class MeasuredText
pub struct MeasuredText {
    pub mWidths: Vec<f32>,
    pub mChars: Vec<u16>,
    pub mPos: usize,
    pub mTextStart: usize,
    pub mSpacing: Vec<f32>,
    pub spacingmult: f32,
}
impl MeasuredText {
    pub fn new() -> Self {
        Self {
            mWidths: vec![],
            mChars: vec![],
            mPos: 0,
            mTextStart: 0,
            mSpacing: vec![],
            spacingmult: 1.0,
        }
    }
    // java void setPara(CharSequence text, int start, int end) {
    // java this.mText = text;
    // java this.mTextStart = start;
    // java int len = end - start;
    // java this.mLen = len;
    // java this.mPos = 0;
    // java if (this.mWidths == null || this.mWidths.length < len) {
    // java this.mWidths = new float[ArrayUtils.idealFloatArraySize(len)];
    // java }
    // java
    // java if (this.mChars == null || this.mChars.length < len) {
    // java this.mChars = new char[ArrayUtils.idealCharArraySize(len)];
    // java }
    // java
    // java TextUtils.getChars(text, start, end, this.mChars, 0);
    // java this.mDir = 1;
    // java this.mEasy = true;
    // java }
    pub fn setPara(&mut self, text: &[u16], start: usize, end: usize) {
        self.mChars = text[start..end].to_vec();
        self.mWidths = vec![0.0; end - start];
        self.mTextStart = start;
        self.mPos = 0;
    }
    // java void setPos(int pos) {
    // java this.mPos = pos - this.mTextStart;
    // java }
    pub fn setPos(&mut self, pos: usize) {
        self.mPos = pos - self.mTextStart;
    }
    // java float addStyleRun(TextPaint paint, int len, Paint.FontMetricsInt fm, ITypeface iTf) {
    // java if (fm != null) {
    // java paint.getFontMetricsInt(fm);
    // java }
    // java
    // java float fontSize = paint.getTextSize();
    // java boolean isBold = paint.isFakeBoldText();
    // java int p = this.mPos;
    // java this.mPos = p + len;
    // java float[] widths = new float[len];
    // java Paint measurePaint = new Paint(paint);
    // java measurePaint.setTextSize(fontSize);
    // java measurePaint.getTextWidths(this.mChars, p, len, widths);
    // java
    // java for(int i = 1; i < len; ++i) {
    // java char cur = this.mChars[p + i];
    // java if (widths[i] == 0.0F && Arabic.charIsArabic(cur)) {
    // java widths[i - 1] = measurePaint.measureText(this.mChars, p + i - 1, 1);
    // java widths[i] = measurePaint.measureText(this.mChars, p + i, 1);
    // java }
    // java }
    // java
    // java if (this.mText instanceof Spanned) {
    // java Spanned spanned = (Spanned)this.mText;
    // java SpacingSpan[] spacingSpans = (SpacingSpan[])spanned.getSpans(p, p + len, SpacingSpan.class);
    // java if (spacingSpans.length > 0) {
    // java for(SpacingSpan spacingSpan : spacingSpans) {
    // java int start = spanned.getSpanStart(spacingSpan);
    // java int end = spanned.getSpanEnd(spacingSpan);
    // java Arrays.fill(this.mSpacing, start, end, spacingSpan.getSpacing());
    // java }
    // java }
    // java }
    // java
    // java for(int i = 0; i < len; ++i) {
    // java float width = widths[i];
    // java if (width != 0.0F) {
    // java width += this.mSpacing[i + p];
    // java if (this.mSpacing[i + p] < 0.0F && width <= 0.0F) {
    // java width = 0.01F;
    // java }
    // java }
    // java
    // java this.mWidths[p + i] = width;
    // java }
    // java
    // java return (float)(fm.descent - fm.ascent);
    // java }
    pub fn addStyleRun(&mut self, paint: &Paint, len: usize, fm: &mut FontMetricsInt) -> f32 {
        *fm = paint.getFontMetricsInt();
        let p = self.mPos;
        self.mPos = p + len;
        let mut widths = paint.getTextWidths(&self.mChars[p..p + len]);
        for i in 0..len {
            let mut w = widths[i];
            if w != 0.0 {
                w += self.mSpacing[p + i];
                if self.mSpacing[p + i] < 0.0 && w <= 0.0 {
                    w = 0.01;
                }
            }
            self.mWidths[p + i] = w;
        }
        (fm.descent - fm.ascent) as f32
    }
    // java float addStyleRun(TextPaint paint, MetricAffectingSpan[] spans, int len, Paint.FontMetricsInt fm, boolean isVert) {
    // java TextPaint workPaint = this.mWorkPaint;
    // java workPaint.set(paint);
    // java workPaint.baselineShift = 0;
    // java boolean needScale = false;
    // java Float scale = 1.0F;
    // java boolean isTopBottomFont = false;
    // java WpsTypefaceSpan wpsTypefaceSpan = null;
    // java float height = 0.0F;
    // java
    // java for(int i = 0; i < spans.length; ++i) {
    // java MetricAffectingSpan span = spans[i];
    // java span.updateMeasureState(workPaint);
    // java if (span instanceof WpsTypefaceSpan) {
    // java wpsTypefaceSpan = (WpsTypefaceSpan)span;
    // java }
    // java }
    // java
    // java ITypeface iTf = null;
    // java if (wpsTypefaceSpan != null) {
    // java if (!wpsTypefaceSpan.getFontName().equals("serif")) {
    // java iTf = PaintFontHelper.getITypeface(wpsTypefaceSpan.getFontName(), wpsTypefaceSpan.isBold(), wpsTypefaceSpan.isItalic());
    // java if (iTf.isCJKFont() && !isNotNeedHeightPercentFont(wpsTypefaceSpan.getFontName()) || isVert) {
    // java height = isVert ? iTf.getWppLayoutStakckedHeight() : iTf.getWppLayoutHeight();
    // java height = height * workPaint.getTextSize() / iTf.getWriterLayoutEMSize();
    // java height = (float)Math.floor((double)height);
    // java }
    // java }
    // java
    // java isTopBottomFont = wpsTypefaceSpan.isTopBottomFont();
    // java String fontName = wpsTypefaceSpan.getFontName();
    // java Float fontScale = (Float)fontScaleMap.get(fontName);
    // java if (fontScale != null) {
    // java scale = fontScale;
    // java needScale = true;
    // java }
    // java
    // java fontScale = (Float)fontBigSizeMap.get(fontName);
    // java if (fontScale != null) {
    // java if (workPaint.getTextSize() >= 60.0F) {
    // java scale = scale * 0.6F;
    // java }
    // java
    // java needScale = true;
    // java }
    // java
    // java if (this.spacingmult < 1.0F) {
    // java Float lineScale = (Float)lineScaleMap.get(fontName);
    // java if (lineScale != null) {
    // java scale = scale * lineScale;
    // java needScale = true;
    // java }
    // java }
    // java }
    // java
    // java this.addStyleRun(workPaint, len, fm, iTf);
    // java if (fm != null) {
    // java if (workPaint.baselineShift < 0) {
    // java fm.ascent += workPaint.baselineShift;
    // java fm.top += workPaint.baselineShift;
    // java } else {
    // java fm.descent += workPaint.baselineShift;
    // java fm.bottom += workPaint.baselineShift;
    // java }
    // java
    // java if (needScale && !isVert) {
    // java if (isTopBottomFont) {
    // java fm.descent = (int)((float)fm.bottom * 0.92F);
    // java fm.ascent = (int)((float)fm.top * 0.92F);
    // java } else if (scale != null) {
    // java fm.descent = (int)((float)fm.descent * scale);
    // java fm.ascent = (int)((float)fm.ascent * scale);
    // java fm.top = (int)((float)fm.top * scale);
    // java fm.bottom = (int)((float)fm.bottom * scale);
    // java }
    // java }
    // java }
    // java
    // java if (height == 0.0F) {
    // java height = (float)(fm.descent - fm.ascent) / 1.2F;
    // java }
    // java
    // java return height;
    // java }
    pub fn addStyleRun__span(
        &mut self,
        paint: &Paint,
        span: &SpanInfo,
        rich: bool,
        len: usize,
        fm: &mut FontMetricsInt,
    ) -> f32 {
        let mut p = paint.clone();
        let family = crate::cn::wps::font::FontHost::FontHost::getFontTable()
            .borrow()
            .get(&span.mFontName, true, false)
            .unwrap();
        let name = if rich {
            family.mFamilyName.as_str()
        } else {
            span.mFontName.as_str()
        };
        let mut scale = 1.0f32;
        let mut needScale = false;
        match name {
            "Lucida Sans Unicode" => {
                scale = 0.79;
                needScale = true;
            }
            "Segoe Print" => {
                scale = 0.67;
                needScale = true;
            }
            "宋体" => {
                scale = 1.07;
                needScale = true;
            }
            "华文中宋" => {
                scale = 0.92;
                needScale = true;
            }
            "黑体" => {
                scale = 1.09;
                needScale = true;
            }
            "Times New Roman" => {
                scale = 1.05;
                needScale = true;
            }
            "微软雅黑" => {
                scale = 0.96;
                needScale = true;
            }
            "Arial" => {
                scale = 1.06;
                needScale = true;
            }
            "Arial Unicode MS" => {
                scale = 0.9;
                needScale = true;
            }
            "Arial Narrow" => {
                scale = 1.06;
                needScale = true;
            }
            "Verdana" => {
                scale = 0.9;
                needScale = true;
            }
            "楷体_GB2312" => {
                scale = 1.06;
                needScale = true;
            }
            "MS Gothic" => {
                scale = 1.01;
                needScale = true;
            }
            "MS PGothic" => {
                scale = 1.01;
                needScale = true;
            }
            "华文新魏" => {
                scale = 1.12;
                needScale = true;
            }
            "隶书" => {
                scale = 1.1;
                needScale = true;
            }
            "华文楷体" => {
                scale = 0.93;
                needScale = true;
            }
            "苹方-简" => {
                scale = 0.88;
                needScale = true;
            }
            "经典特宋简" => {
                scale = 1.02;
                needScale = true;
            }
            "Tahoma" => {
                scale = 1.0;
                needScale = true;
            }
            "Cambria" => {
                scale = 1.065;
                needScale = true;
            }
            "Microsoft YaHei UI" => {
                scale = 0.9;
                needScale = true;
            }
            "Microsoft YaHei UI Light" => {
                scale = 1.04;
                needScale = true;
            }
            "华文细黑" => {
                scale = 0.85;
                needScale = true;
            }
            "Garamond" => {
                scale = 1.05;
                needScale = true;
            }
            "Vijaya" => {
                scale = 1.15;
                needScale = true;
            }
            "Tw Cen MT" => {
                scale = 1.1;
                needScale = true;
            }
            "Comic Sans MS" => {
                scale = 0.9;
                needScale = true;
            }
            "Calibri" => {
                scale = 1.2;
                needScale = true;
            }
            _ => {}
        }
        if name == "Times New Roman" {
            if p.textSize >= 60.0 {
                scale *= 0.6;
            }
            needScale = true;
        }
        self.addStyleRun(&p, len, fm);
        // rust: ET passes no paragraph spacing and the demo's native FontHost resolves to serif; non-default registered font metadata remains a separate native adapter input.
        if p.baselineShift < 0 {
            fm.ascent += p.baselineShift;
            fm.top += p.baselineShift;
        } else {
            fm.descent += p.baselineShift;
            fm.bottom += p.baselineShift;
        }
        if needScale {
            fm.descent = (fm.descent as f32 * scale) as i32;
            fm.ascent = (fm.ascent as f32 * scale) as i32;
            fm.top = (fm.top as f32 * scale) as i32;
            fm.bottom = (fm.bottom as f32 * scale) as i32;
        }
        (fm.descent - fm.ascent) as f32 / 1.2
    }
}
