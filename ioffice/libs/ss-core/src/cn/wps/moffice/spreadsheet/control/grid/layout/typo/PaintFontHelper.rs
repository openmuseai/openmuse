use super::super::super::render::native::{FontMetricsInt, Paint};
use super::super::data::SpanInfo::SpanInfo;
// java private static final LocalTextPaint sTextPaint = new LocalTextPaint();
// java protected TextPaint initialValue() { return new TextPaint(); }
thread_local! {static TEXT_PAINT:std::cell::RefCell<Paint>=std::cell::RefCell::new(Paint::default());}
// java public class PaintFontHelper
pub struct PaintFontHelper;
impl PaintFontHelper {
    // java public static void setPaintForText(SpanInfo spanInfo, Paint paint, boolean setUnderline) {
    // java paint.reset();
    // java paint.setFlags(1);
    // java paint.setStyle(Style.FILL);
    // java paint.setColor(spanInfo.mFontColor);
    // java paint.setTextSize(spanInfo.mFontSize);
    // java paint.setStrikeThruText(spanInfo.mIsStrikeout);
    // java paint.setUnderlineText(setUnderline && spanInfo.hasUnderline());
    // java paint.setTypeface(getTypeface(spanInfo.mFontName, spanInfo.mIsBold, spanInfo.mIsItalic));
    // java paint.setFakeBoldText(spanInfo.mIsBold);
    // java paint.setTextSkewX(spanInfo.mIsItalic ? -0.25F : 0.0F);
    // java paint.setTextScaleX(1.0F);
    // java }
    pub fn setPaintForText(s: &SpanInfo, p: &mut Paint, setUnderline: bool) {
        *p = Paint::default();
        p.useFontHost = true;
        p.flags = 1;
        p.style = 0;
        p.color = s.mFontColor;
        p.textSize = s.mFontSize;
        p.strike = s.mIsStrikeout;
        p.underline = setUnderline && s.hasUnderline();
        p.fontName = s.mFontName.clone();
        p.bold = s.mIsBold;
        p.italic = s.mIsItalic;
        p.fakeBold = s.mIsBold;
        p.skewX = if s.mIsItalic { -0.25 } else { 0.0 };
        p.scaleX = 1.0;
        p.setTypeface(&Self::getTypeface(&s.mFontName, s.mIsBold, s.mIsItalic));
    }
    // java private static final void setMeasurePaint(SpanInfo spanInfo, Paint paint) {
    // java paint.setFlags(1);
    // java paint.setTextSize(spanInfo.mFontSize);
    // java paint.setTypeface(getTypeface(spanInfo.mFontName, spanInfo.mIsBold, spanInfo.mIsItalic));
    // java }
    pub fn setMeasurePaint(s: &SpanInfo, p: &mut Paint) {
        p.useFontHost = true;
        p.flags = 1;
        p.textSize = s.mFontSize;
        p.fontName = s.mFontName.clone();
        p.bold = s.mIsBold;
        p.italic = s.mIsItalic;
        p.setTypeface(&Self::getTypeface(&s.mFontName, s.mIsBold, s.mIsItalic));
    }
    // java public static float getCharWidth(Paint paint) {
    // java paint.getTextWidths("0123456789", mMeasureCharWidths);
    // java float maxWidth = 0.0F;
    // java
    // java for(int i = 0; i < mMeasureCharWidths.length; ++i) {
    // java if (maxWidth < mMeasureCharWidths[i]) {
    // java maxWidth = mMeasureCharWidths[i];
    // java }
    // java }
    // java
    // java return maxWidth;
    // java }
    pub fn getCharWidth(p: &Paint) -> f32 {
        let mut max = 0.0;
        for w in p.getTextWidths(&"0123456789".encode_utf16().collect::<Vec<_>>()) {
            if max < w {
                max = w;
            }
        }
        max
    }
    // java public static float getCharWidth(String text, String fontName, float fontSize, boolean bold, boolean italic) {
    // java Typeface typeface = getTypeface(fontName, bold, italic);
    // java TextPaint textPaint = (TextPaint)sTextPaint.get();
    // java textPaint.setTextSize(fontSize * 100.0F);
    // java textPaint.setTypeface(typeface);
    // java int len = text.length();
    // java float[] widths = len == 1 ? (float[])m1LenFloatLocal.get() : new float[len];
    // java textPaint.getTextWidths(text, 0, len, widths);
    // java float sumWidth = 0.0F;
    // java
    // java for(int i = 0; i < len; ++i) {
    // java sumWidth += widths[i];
    // java }
    // java
    // java return sumWidth / 100.0F;
    // java }
    pub fn getCharWidth__text(
        text: &[u16],
        fontName: &str,
        size: f32,
        bold: bool,
        italic: bool,
    ) -> f32 {
        TEXT_PAINT.with(|cell| {
            let mut p = cell.borrow_mut();
            p.textSize = size * 100.0;
            p.setTypeface(&Self::getTypeface(fontName, bold, italic));
            let mut sum = 0.0f32;
            for width in p.getTextWidths(text) {
                sum += width;
            }
            sum / 100.0
        })
    }
    // java public static float getCharWidth(String text, SpanInfo span) {
    // java return getCharWidth(text, span.mFontName, span.mFontSize, span.mIsBold, span.mIsItalic);
    // java }
    pub fn getCharWidth__span(text: &[u16], s: &SpanInfo) -> f32 {
        Self::getCharWidth__text(text, &s.mFontName, s.mFontSize, s.mIsBold, s.mIsItalic)
    }
    // java public static void getFontMetricsInt(Paint.FontMetricsInt fontMetrics, SpanInfo span) {
    // java getFontMetricsInt(fontMetrics, span.mFontName, span.mFontSize, span.mIsBold, span.mIsItalic);
    // java }
    pub fn getFontMetricsInt(s: &SpanInfo) -> FontMetricsInt {
        Self::getFontMetricsInt__font(&s.mFontName, s.mFontSize, s.mIsBold, s.mIsItalic)
    }
    // java public static void getFontMetricsInt(Paint.FontMetricsInt fontMetrics, String fontName, float fontSize, boolean bold, boolean italic) {
    // java Typeface face = getTypeface(fontName, bold, italic);
    // java TextPaint textPaint = (TextPaint)sTextPaint.get();
    // java textPaint.setTextSize(fontSize);
    // java textPaint.setTypeface(face);
    // java textPaint.getFontMetricsInt(fontMetrics);
    // java }
    pub fn getFontMetricsInt__font(
        fontName: &str,
        size: f32,
        bold: bool,
        italic: bool,
    ) -> FontMetricsInt {
        TEXT_PAINT.with(|cell| {
            let mut p = cell.borrow_mut();
            p.textSize = size;
            p.setTypeface(&Self::getTypeface(fontName, bold, italic));
            p.getFontMetricsInt()
        })
    }
    // java public static void setNormalCharWidthTwip(KmoXFManager xfManager, Paint paint) {
    // java if (xfManager != null) {
    // java KmoFont font = xfManager.getNormalStyle().getFont();
    // java paint.reset();
    // java paint.setTextSize((float)font.getFontHeight());
    // java paint.setStrikeThruText(font.isStrikeout());
    // java paint.setUnderlineText(font.getUnderline() != 0);
    // java paint.setTypeface(getTypeface(font.getFontName(), font.getBoldWeight() == 700, font.isItalic()));
    // java paint.setFakeBoldText(font.getBoldWeight() == 700);
    // java xfManager.setNormalCharWidthTwip(getCharWidth(paint));
    // java }
    // java
    // java }
    pub fn setNormalCharWidthTwip(
        mgr: &crate::cn::wps::moss::app::xf::KmoXFManager::KmoXFManager,
        p: &mut Paint,
    ) {
        let font = mgr
            .getNormalStyle()
            .expect("Normal style")
            .getFont()
            .expect("Normal font");
        *p = Paint::default();
        p.textSize = font.getFontHeight() as f32;
        p.strike = font.isStrikeout();
        p.underline = font.getUnderline() != 0;
        p.useFontHost = true;
        p.fontName = font.getFontName();
        p.bold = font.getBoldWeight() == 700;
        p.italic = font.isItalic();
        p.fakeBold = p.bold;
        p.setTypeface(&Self::getTypeface(&p.fontName, p.bold, p.italic));
        mgr.setNormalCharWidthTwip(Self::getCharWidth(p));
    }
    // java public static float measureCharWidth(TextData data, int start, int end) {
    // java Paint paint = (Paint)sTextPaint.get();
    // java float sumWidth = 0.0F;
    // java int partEnd = 0;
    // java String text = data.mSourceText;
    // java int length = data.mOtherSpans.size();
    // java SpanInfo span = data.mFirstSpan;
    // java
    // java for(int i = 0; i <= length && start < end; ++i) {
    // java if (span.contains(start)) {
    // java setMeasurePaint(span, paint);
    // java partEnd = end > span.mEndPos ? span.mEndPos : end;
    // java sumWidth += paint.measureText(text, start, partEnd);
    // java char c = text.charAt(partEnd - 1);
    // java if (c == '\r' || c == '\n') {
    // java sumWidth -= paint.measureText("\n");
    // java }
    // java
    // java start = span.mEndPos;
    // java end = end > span.mEndPos ? end : span.mEndPos;
    // java }
    // java
    // java if (i < length) {
    // java span = (SpanInfo)data.mOtherSpans.get(i);
    // java }
    // java }
    // java
    // java return sumWidth;
    // java }
    pub fn measureCharWidth(
        data: &super::super::data::TextData::TextData,
        mut start: usize,
        mut end: usize,
    ) -> f32 {
        TEXT_PAINT.with(|cell| {
            let mut p = cell.borrow_mut();
            let mut sum = 0.0f32;
            let mut spans = vec![&data.mFirstSpan];
            spans.extend(data.mOtherSpans.iter());
            for span in spans {
                if start >= end {
                    break;
                }
                if span.contains(start as i32) {
                    Self::setMeasurePaint(span, &mut p);
                    let stop = end.min(span.mEndPos as usize);
                    sum += p.measureText(&data.source()[start..stop]);
                    let c = data.source()[stop - 1];
                    if c == 13 || c == 10 {
                        sum -= p.measureText(&[10]);
                    }
                    start = span.mEndPos as usize;
                    end = end.max(span.mEndPos as usize);
                }
            }
            sum
        })
    }
    // java public static void measureBoringMetrics(String text, BoringLayout.Metrics metrics, SpanInfo firstSpan, List<SpanInfo> otherSpans) {
    // java Paint paint = (Paint)sTextPaint.get();
    // java setMeasurePaint(firstSpan, paint);
    // java String fontName = firstSpan.mFontName;
    // java paint.getFontMetricsInt(metrics);
    // java float sumWidth = paint.measureText(text, firstSpan.mStartPos, firstSpan.mEndPos);
    // java if (firstSpan.mIsItalic) {
    // java sumWidth += firstSpan.mFontSize * 0.25F;
    // java }
    // java
    // java int length = otherSpans.size();
    // java float fontSize = firstSpan.mFontSize;
    // java
    // java for(int i = 0; i < length; ++i) {
    // java SpanInfo span = (SpanInfo)otherSpans.get(i);
    // java setMeasurePaint(span, paint);
    // java if (fontSize < span.mFontSize) {
    // java paint.getFontMetricsInt(metrics);
    // java fontSize = span.mFontSize;
    // java fontName = span.mFontName;
    // java }
    // java
    // java if (span.mIsItalic) {
    // java sumWidth += firstSpan.mFontSize * 0.25F;
    // java }
    // java
    // java sumWidth += paint.measureText(text, span.mStartPos, span.mEndPos);
    // java }
    // java
    // java metrics.width = (int)Math.ceil((double)sumWidth);
    // java if (TO_BOTTOM_FONT.indexOf(fontName) >= 0) {
    // java metrics.ascent = metrics.top;
    // java metrics.descent = metrics.bottom;
    // java }
    // java
    // java }
    pub fn measureBoringMetrics(
        text: &[u16],
        first: &SpanInfo,
        other: &[SpanInfo],
    ) -> FontMetricsInt {
        TEXT_PAINT.with(|cell| {
            let mut p = cell.borrow_mut();
            Self::setMeasurePaint(first, &mut p);
            let mut name = first.mFontName.clone();
            let mut fm = p.getFontMetricsInt();
            let mut sum = p.measureText(&text[first.mStartPos as usize..first.mEndPos as usize]);
            if first.mIsItalic {
                sum += first.mFontSize * 0.25;
            }
            let mut size = first.mFontSize;
            for span in other {
                Self::setMeasurePaint(span, &mut p);
                if size < span.mFontSize {
                    fm = p.getFontMetricsInt();
                    size = span.mFontSize;
                    name = span.mFontName.clone();
                }
                if span.mIsItalic {
                    sum += first.mFontSize * 0.25;
                }
                sum += p.measureText(&text[span.mStartPos as usize..span.mEndPos as usize]);
            }
            fm.width = sum.ceil() as i32;
            if [
                "Calibri",
                "Calibri Light",
                "Tahoma",
                "TH Charm of AU",
                "TH Charmonman",
                "TH NiramitIT๙ ",
                "TH Srisakdi",
                "TH Kodchasal",
                "TH Fah kwang",
                "TH Sarabun New",
                "lrisUPC",
                "EucrosiaUPC",
                "FreesiaUPC",
                "JasmineUPC",
                "KodchiangUPC",
                "DilleniaUPC",
            ]
            .contains(&name.as_str())
            {
                fm.ascent = fm.top;
                fm.descent = fm.bottom;
            }
            fm
        })
    }
    // java public static Typeface getTypeface(String fontName, boolean isBold, boolean isItalic) {
    // java int style = isBold ? 1 : 0;
    // java style += isItalic ? 2 : 0;
    // java IFontTable fontTable = FontHost.getFontTable();
    // java IFontFamily fontFamily = null;
    // java if (UILanguage.UILanguage_Thai == Platform.getLanguage()) {
    // java fontFamily = fontTable.get(fontName);
    // java if (null == fontFamily) {
    // java fontFamily = fontTable.getDefault(Platform.getLanguage());
    // java }
    // java } else {
    // java fontFamily = fontTable.get(fontName, true, false);
    // java }
    // java
    // java ITypeface typeface = fontFamily.getTypeface(style);
    // java return (Typeface)typeface.getNativeTypeface();
    // java }
    pub fn getTypeface(
        name: &str,
        bold: bool,
        italic: bool,
    ) -> crate::cn::wps::font::NativeTypeface::NativeTypeface {
        let style = if bold { 1 } else { 0 } + if italic { 2 } else { 0 };
        let table = crate::cn::wps::font::FontHost::FontHost::getFontTable().borrow();
        let family = table.get(name, true, false).expect("font family");
        family.getTypeface(style).expect("native typeface")
    }
}
