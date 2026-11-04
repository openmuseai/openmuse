use super::super::super::render::graphics::{Paint, Rect};
use super::super::{data::TextData::TextData, layouter::BidiTools::BidiTools};
use super::PaintFontHelper::PaintFontHelper;
use crate::cn::wps::text::layout::{
    layouter::WpsLayout::WpsLayout, typo::layout::Layout::Alignment,
};
pub struct TypoLayoutGen;
impl TypoLayoutGen {
    // java public Layout getLayout(Rect rect, TextPaint textPaint, TextData data, boolean forceTypo) {
    // java CharSequence cellText = null;
    // java short angle = data.mPara.mAngle;
    // java Object var10;
    // java if (!data.mIsRichText) {
    // java var10 = new SpannableStringBuilder(data.mSourceText);
    // java PaintFontHelper.setPaintForText(data.mFirstSpan, textPaint, false);
    // java WpsTypefaceSpan typefaceSpan = new WpsTypefaceSpan("", data.mFirstSpan.mFontName, data.mFirstSpan.mIsBold, data.mFirstSpan.mIsItalic, textPaint.getTypeface(), false);
    // java ((SpannableStringBuilder)var10).setSpan(typefaceSpan, 0, ((CharSequence)var10).length(), 33);
    // java } else {
    // java textPaint.set(PaintFontHelper.INIT_TEXT_PAINT);
    // java textPaint.setTypeface(Typeface.DEFAULT);
    // java textPaint.setFlags(1);
    // java textPaint.setTextScaleX(1.0F);
    // java textPaint.setTextSkewX(0.0F);
    // java var10 = this.createSpannable(data, false);
    // java }
    // java
    // java int[] positions = Thai.getWordPositions(((CharSequence)var10).toString().toCharArray());
    // java if (null != positions) {
    // java WordSplitSpan splitSpan = new WordSplitSpan();
    // java splitSpan.positions = positions;
    // java ((SpannableStringBuilder)var10).setSpan(splitSpan, 0, ((CharSequence)var10).length(), 33);
    // java }
    // java
    // java Layout.Alignment alignment = getLayoutAlignment(data.mPara.mHAlignment, data.mPara.mIsRightToLeft);
    // java int width = angle == 0 ? rect.width() : rect.height();
    // java if (!data.mPara.mIsWrapText && angle == 0) {
    // java if (BidiTools.needRtl(data.mSourceText)) {
    // java PaintFontHelper.measureBoringMetrics(data.mSourceText, this.mBoringMetrics, data.mFirstSpan, data.mOtherSpans);
    // java width = Math.max(width, this.mBoringMetrics.width);
    // java } else {
    // java width = (int)PaintFontHelper.measureCharWidth(data, 0, data.mSourceText.length());
    // java }
    // java }
    // java
    // java return ANDROID_VERSION != 16 ? createAutoWrapLayout((CharSequence)var10, width, data, alignment, textPaint, angle) : createAutoWrapLayoutFor4_1((CharSequence)var10, data, width, alignment, textPaint, angle);
    // java }
    pub fn getLayout(rect: &Rect, p: &mut Paint, d: &TextData, _force: bool) -> WpsLayout {
        if !d.mIsRichText {
            PaintFontHelper::setPaintForText(&d.mFirstSpan, p, false);
        } else {
            *p = Paint {
                flags: 1,
                scaleX: 1.0,
                skewX: 0.0,
                ..Paint::default()
            };
        }
        let angle = d.mPara.mAngle;
        let mut width = if angle == 0 {
            rect.width()
        } else {
            rect.height()
        };
        if !d.mPara.mIsWrapText && angle == 0 {
            if BidiTools::needRtl(d.source()) {
                let fm = PaintFontHelper::measureBoringMetrics(
                    d.source(),
                    &d.mFirstSpan,
                    &d.mOtherSpans,
                );
                width = width.max(fm.width);
            } else {
                width = PaintFontHelper::measureCharWidth(d, 0, d.source().len()) as i32;
            }
        }
        let alignment =
            Self::getLayoutAlignment(d.mPara.mHAlignment as i32, d.mPara.mIsRightToLeft);
        Self::createAutoWrapLayout(d, width, alignment, p.clone())
    }
    // java private static Layout.Alignment getLayoutAlignment(int align, boolean isRightToLeft) {
    // java Layout.Alignment alignment = Alignment.ALIGN_NORMAL;
    // java if (!isRightToLeft) {
    // java switch (align) {
    // java case 2:
    // java case 6:
    // java case 7:
    // java alignment = Alignment.ALIGN_CENTER;
    // java break;
    // java case 3:
    // java alignment = Alignment.ALIGN_OPPOSITE;
    // java case 4:
    // java case 5:
    // java }
    // java } else {
    // java switch (align) {
    // java case 1:
    // java alignment = Alignment.ALIGN_LEFT;
    // java break;
    // java case 2:
    // java case 6:
    // java case 7:
    // java alignment = Alignment.ALIGN_CENTER;
    // java break;
    // java case 3:
    // java alignment = Alignment.ALIGN_OPPOSITE;
    // java case 4:
    // java case 5:
    // java }
    // java }
    // java
    // java return alignment;
    // java }
    pub fn getLayoutAlignment(align: i32, rtl: bool) -> Alignment {
        match align {
            2 | 6 | 7 => Alignment::ALIGN_CENTER,
            3 => Alignment::ALIGN_OPPOSITE,
            1 if rtl => Alignment::ALIGN_LEFT,
            _ => Alignment::ALIGN_NORMAL,
        }
    }
    // java private static Layout createAutoWrapLayout(CharSequence text, int layoutWidth, TextData data, Layout.Alignment alignment, TextPaint textPaint, short angle) {
    // java boolean singleLine = !data.mPara.mIsWrapText;
    // java
    // java try {
    // java CharSequence source = processRightToLeftText(text);
    // java return new WpsLayout(source, 0, source.length(), textPaint, (float)Math.round((float)layoutWidth), alignment, 1.0F, 0.0F, (TextUtils.TruncateAt)null, 0, 0, singleLine, (ParaInfo)null, 0, angle);
    // java } catch (Exception e) {
    // java e.printStackTrace();
    // java return null;
    // java }
    // java }
    fn createAutoWrapLayout(d: &TextData, w: i32, a: Alignment, p: Paint) -> WpsLayout {
        WpsLayout::new(d, w, a, p)
    }
}
