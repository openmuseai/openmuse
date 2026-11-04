// Java: import android.graphics.Paint
// Java: import android.graphics.Typeface
// Java: import android.graphics.Paint.Style
// Java: import android.text.BoringLayout
// Java: import android.text.TextPaint
// Java: import android.text.TextUtils
// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.font.FontHost
// Java: import cn.wps.font.IFontFamily
// Java: import cn.wps.font.IFontTable
// Java: import cn.wps.font.ITypeface
// Java: import cn.wps.moffice.define.UILanguage
// Java: import cn.wps.text.layout.data.SpanInfo
// Java: import cn.wps.text.layout.data.TextData
// Java: import java.text.Bidi
// Java: import java.util.List
use crate::cn::wps::text::layout::data::SpanInfo::SpanInfo;
use crate::cn::wps::text::layout::data::TextData::TextData;

pub struct Paint {
    pub(crate) flags: i32,
    pub(crate) text_size: f32,
    pub(crate) color: i32,
}
impl Paint {
    pub fn new() -> Self { Paint { flags: 0, text_size: 0.0, color: 0 } }
    pub fn setFlags(&mut self, flags: i32) { self.flags = flags; }
    pub fn setStyle(&mut self, _style: i32) {}
    pub fn setColor(&mut self, color: i32) { self.color = color; }
    pub fn setTextSize(&mut self, size: f32) { self.text_size = size; }
    pub fn setStrikeThruText(&mut self, _v: bool) {}
    pub fn setUnderlineText(&mut self, _v: bool) {}
    pub fn setTypeface(&mut self, _tf: Typeface) {}
    pub fn setFakeBoldText(&mut self, _v: bool) {}
    pub fn setTextSkewX(&mut self, _v: f32) {}
    pub fn measureText(&self, _text: &str) -> f32 { 0.0 }
    pub fn measureText_range(&self, _text: &str, _start: i32, _end: i32) -> f32 { 0.0 }
    pub fn getFontMetricsInt(&self, _metrics: &mut FontMetricsInt) {}
    pub fn getTextWidths(&self, _text: &[char], _start: i32, _len: i32, widths: &mut [f32]) {
        for w in widths.iter_mut() { *w = 0.0; }
    }
}

pub struct FontMetricsInt {
    pub ascent: i32,
    pub descent: i32,
    pub top: i32,
    pub bottom: i32,
    pub leading: i32,
    pub width: i32,
}
impl FontMetricsInt {
    pub fn new() -> Self { FontMetricsInt { ascent: 0, descent: 0, top: 0, bottom: 0, leading: 0, width: 0 } }
}

#[derive(Clone)]
pub struct Typeface;
pub struct TextPaint;
impl TextPaint {
    pub fn new() -> Self { TextPaint }
}

// Java field: int TEXT_PAINT_FLAG
pub const TEXT_PAINT_FLAG: i32 = 1;
// Java field: TextPaint INIT_TEXT_PAINT
static INIT_TEXT_PAINT: std::sync::OnceLock<TextPaint> = std::sync::OnceLock::new();
// Java field: LocalTextPaint sTextPaint
static S_TEXT_PAINT: std::sync::OnceLock<std::sync::Mutex<Paint>> = std::sync::OnceLock::new();
// Java field: float ITALIC_ANGLE
pub const ITALIC_ANGLE: f32 = 0.25;

fn get_paint() -> &'static std::sync::Mutex<Paint> {
    S_TEXT_PAINT.get_or_init(|| std::sync::Mutex::new(Paint::new()))
}

// Java: public class PaintFontHelper
pub struct PaintFontHelper {
}

// Java: public class LocalTextPaint
pub struct LocalTextPaint {
}

// Java: public class PaintFontHelper
impl PaintFontHelper {
    // Java: PaintFontHelper()
    pub fn new() -> Self {
      // Java: super();
      PaintFontHelper {}
    }

    // Java: static void initFont()
    pub fn initFont() {
    }

    // Java: static void initFontCache(String[] paths, String fontCacheFile)
    pub fn initFontCache(_paths: &[String], _fontCacheFile: &str) {
      // Java: FontHost.initCache(paths, fontCacheFile);
      // Java: FontHost.loadCache(fontCacheFile);
    }

    // Java: static void setPaintForText(SpanInfo spanInfo, Paint paint)
    pub fn setPaintForText(spanInfo: &SpanInfo, paint: &mut Paint) {
      // Java: paint.setFlags(1);
      paint.setFlags(1);
      // Java: paint.setStyle(Style.FILL);
      paint.setStyle(0);
      // Java: paint.setColor(spanInfo.mFontColor);
      paint.setColor(spanInfo.mFontColor);
      // Java: paint.setTextSize(spanInfo.mFontSize);
      paint.setTextSize(spanInfo.mFontSize);
      // Java: paint.setStrikeThruText(spanInfo.hasStrikethroughLine());
      paint.setStrikeThruText(spanInfo.hasStrikethroughLine());
      // Java: paint.setUnderlineText(spanInfo.hasUnderline());
      paint.setUnderlineText(spanInfo.hasUnderline());
      // Java: paint.setTypeface(getTypeface(spanInfo.mFontName, spanInfo.mIsBold, spanInfo.mIsItalic));
      paint.setTypeface(Self::getTypeface(&spanInfo.mFontName, spanInfo.mIsBold, spanInfo.mIsItalic));
      // Java: paint.setFakeBoldText(spanInfo.mIsBold);
      paint.setFakeBoldText(spanInfo.mIsBold);
      // Java: paint.setTextSkewX(spanInfo.mIsItalic ? -0.25F : 0.0F);
      paint.setTextSkewX(if spanInfo.mIsItalic { -0.25 } else { 0.0 });
    }

    // Java: static void setPaintUNoneText(SpanInfo spanInfo, Paint paint)
    pub fn setPaintUNoneText(spanInfo: &SpanInfo, paint: &mut Paint) {
      // Java: setPaintForText(spanInfo, paint);
      Self::setPaintForText(spanInfo, paint);
      // Java: paint.setUnderlineText(false);
      paint.setUnderlineText(false);
    }

    // Java: static void measureBoringMetrics(String text, BoringLayout.Metrics metrics, SpanInfo firstSpan, List<SpanInfo> otherSpans)
    pub fn measureBoringMetrics(text: &str, metrics: &mut FontMetricsInt, firstSpan: &SpanInfo, otherSpans: &[SpanInfo]) {
      // Java: Paint paint = (Paint)sTextPaint.get();
      let mut paint = get_paint().lock().unwrap();
      // Java: setMeasurePaint(firstSpan, paint);
      Self::setMeasurePaint_impl(firstSpan, &mut paint);
      // Java: paint.getFontMetricsInt(metrics);
      paint.getFontMetricsInt(metrics);
      // Java: float sumWidth = paint.measureText(text, firstSpan.mStartPos, firstSpan.mEndPos);
      let mut sumWidth = paint.measureText_range(text, firstSpan.mStartPos, firstSpan.mEndPos);
      // Java: if (firstSpan.mIsItalic)
      if firstSpan.mIsItalic {
         // Java: sumWidth += firstSpan.mFontSize * 0.25F;
         sumWidth += firstSpan.mFontSize * 0.25;
      }
      // Java: int length = otherSpans.size();
      let length = otherSpans.len();
      // Java: float fontSize = firstSpan.mFontSize;
      let mut fontSize = firstSpan.mFontSize;
      // Java: for(int i = 0; i < length; ++i)
      for i in 0..length {
         // Java: SpanInfo span = (SpanInfo)otherSpans.get(i);
         let span = &otherSpans[i];
         // Java: if (span.mStartPos >= text.length())
         if span.mStartPos >= text.len() as i32 {
            // Java: break;
            break;
         }
         // Java: if (span.mEndPos > text.length())
         // Java: setMeasurePaint(span, paint);
         Self::setMeasurePaint_impl(span, &mut paint);
         // Java: if (fontSize < span.mFontSize)
         if fontSize < span.mFontSize {
            // Java: paint.getFontMetricsInt(metrics);
            paint.getFontMetricsInt(metrics);
            // Java: fontSize = span.mFontSize;
            fontSize = span.mFontSize;
         }
         // Java: if (span.mIsItalic)
         if span.mIsItalic {
            // Java: sumWidth += span.mFontSize * 0.25F;
            sumWidth += span.mFontSize * 0.25;
         }
         // Java: sumWidth += paint.measureText(text, span.mStartPos, span.mEndPos);
         sumWidth += paint.measureText_range(text, span.mStartPos, span.mEndPos);
      }
      // Java: metrics.width = (int)Math.ceil((double)sumWidth);
      metrics.width = sumWidth.ceil() as i32;
    }

    // Java: static float measureCharWidth(TextData data, int start, int end)
    pub fn measureCharWidth(data: &TextData, start: i32, end: i32) -> f32 {
      // Java: Paint paint = (Paint)sTextPaint.get();
      let mut paint = get_paint().lock().unwrap();
      // Java: float sumWidth = 0.0F;
      let mut sumWidth: f32 = 0.0;
      // Java: int partEnd = 0;
      let mut partEnd: i32;
      // Java: String text = data.mTypoText;
      let text = data.mTypoText.as_deref().unwrap_or("");
      // Java: int length = data.mOtherSpans.size();
      let length = data.mOtherSpans.len();
      // Java: SpanInfo span = data.mFirstSpan;
      let mut span_idx: i32 = -1;
      let mut start = start;
      let mut end = end;
      // Java: for(int i = 0; i <= length && start < end; ++i)
      for i in 0..=length {
         if start >= end { break; }
         let span = if span_idx < 0 { &data.mFirstSpan } else { &data.mOtherSpans[span_idx as usize] };
         // Java: if (span.contains(start))
         if span.contains(start) {
            // Java: setMeasurePaint(span, paint);
            Self::setMeasurePaint_impl(span, &mut paint);
            // Java: partEnd = end > span.mEndPos ? span.mEndPos : end;
            partEnd = if end > span.mEndPos { span.mEndPos } else { end };
            // Java: sumWidth += paint.measureText(text, start, partEnd);
            sumWidth += paint.measureText_range(text, start, partEnd);
            // Java: char c = text.charAt(partEnd - 1);
            let c = text.chars().nth((partEnd - 1) as usize).unwrap_or('\0');
            // Java: if (c == '\r' || c == '\n')
            if c == '\r' || c == '\n' {
               // Java: sumWidth -= paint.measureText("\n");
               sumWidth -= paint.measureText("\n");
            }
            // Java: start = span.mEndPos;
            start = span.mEndPos;
            // Java: end = end > span.mEndPos ? end : span.mEndPos;
            end = if end > span.mEndPos { end } else { span.mEndPos };
         }
         // Java: if (i < length)
         if i < length {
            // Java: span = (SpanInfo)data.mOtherSpans.get(i);
            span_idx = i as i32;
         }
      }
      // Java: return sumWidth;
      sumWidth
    }

    // Java: static void setMeasurePaint(SpanInfo spanInfo, Paint paint)
    fn setMeasurePaint_impl(spanInfo: &SpanInfo, paint: &mut Paint) {
      // Java: paint.setFlags(1);
      paint.setFlags(1);
      // Java: paint.setTextSize(spanInfo.mFontSize);
      paint.setTextSize(spanInfo.mFontSize);
      // Java: paint.setTypeface(getTypeface(spanInfo.mFontName, spanInfo.mIsBold, spanInfo.mIsItalic));
      paint.setTypeface(Self::getTypeface(&spanInfo.mFontName, spanInfo.mIsBold, spanInfo.mIsItalic));
    }

    // Java: static int getDesiredWidth(CharSequence text, Paint paint)
    pub fn getDesiredWidth(text: &str, paint: &Paint) -> i32 {
      // Java: float need = 0.0F;
      let mut need: f32 = 0.0;
      // Java: int end = text.length();
      let end = text.len();
      // Java: int next;
      let mut next;
      // Java: for(int i = 0; i <= end; i = next)
      let mut i = 0;
      while i <= end {
         // Java: next = TextUtils.indexOf(text, '\n', i, end);
         next = text[i..end].find('\n').map_or(-1i32, |p| (p + i) as i32);
         // Java: if (next < 0)
         if next < 0 {
            // Java: next = end;
            next = end as i32;
         }
         // Java: float w = paint.measureText(text, i, next);
         let w = paint.measureText_range(text, i as i32, next);
         // Java: if (w > need)
         if w > need {
            // Java: need = w;
            need = w;
         }
         // Java: ++next;
         next += 1;
         i = next as usize;
      }
      // Java: return (int)Math.ceil((double)need);
      need.ceil() as i32
    }

    // Java: static float getCharWidth(String text, String fontName, float fontSize, boolean bold, boolean italic)
    pub fn getCharWidth_text(text: &str, fontName: &str, fontSize: f32, bold: bool, italic: bool) -> f32 {
      // Java: Typeface typeface = getTypeface(fontName, bold, italic);
      let _typeface = Self::getTypeface(fontName, bold, italic);
      // Java: Paint paint = (Paint)sTextPaint.get();
      let mut paint = get_paint().lock().unwrap();
      // Java: paint.setTypeface(typeface);
      paint.setTypeface(_typeface);
      // Java: paint.setTextSize(fontSize);
      paint.setTextSize(fontSize);
      // Java: return paint.measureText(text);
      paint.measureText(text)
    }

    // Java: static float[] getCharWidth(char[] text, int charsPos, int charsLen, String fontName, float fontSize, boolean bold, boolean italic, float fontSpace)
    pub fn getCharWidth_chars(text: &[char], charsPos: i32, charsLen: i32, fontName: &str, fontSize: f32, bold: bool, italic: bool, fontSpace: f32) -> Vec<f32> {
      // Java: Typeface typeface = getTypeface(fontName, bold, italic);
      let _typeface = Self::getTypeface(fontName, bold, italic);
      // Java: Paint paint = (Paint)sTextPaint.get();
      let mut paint = get_paint().lock().unwrap();
      // Java: paint.setTextSize(fontSize * 100.0F);
      paint.setTextSize(fontSize * 100.0);
      // Java: paint.setTypeface(typeface);
      paint.setTypeface(_typeface);
      // Java: float[] widths = new float[charsLen];
      let mut widths = vec![0.0f32; charsLen as usize];
      // Java: paint.getTextWidths(text, charsPos, charsLen, widths);
      paint.getTextWidths(text, charsPos, charsLen, &mut widths);
      // Java: for(int i = 0; i < charsLen; ++i)
      for i in 0..charsLen as usize {
         // Java: widths[i] /= 100.0F;
         widths[i] /= 100.0;
         // Java: widths[i] += fontSpace;
         widths[i] += fontSpace;
      }
      // Java: return widths;
      widths
    }

    // Java: static float getCharWidth(char[] text, SpanInfo span)
    pub fn getCharWidth_span(text: &[char], span: &SpanInfo) -> f32 {
      // Java: Typeface typeface = getTypeface(span.mFontName, span.mIsBold, span.mIsItalic);
      let _typeface = Self::getTypeface(&span.mFontName, span.mIsBold, span.mIsItalic);
      // Java: Paint paint = (Paint)sTextPaint.get();
      let mut paint = get_paint().lock().unwrap();
      // Java: paint.setTextSize(span.mFontSize);
      paint.setTextSize(span.mFontSize);
      // Java: paint.setTypeface(typeface);
      paint.setTypeface(_typeface);
      // Java: float[] widths = new float[text.length];
      let mut widths = vec![0.0f32; text.len()];
      // Java: paint.getTextWidths(text, 0, text.length, widths);
      paint.getTextWidths(text, 0, text.len() as i32, &mut widths);
      // Java: float sumWidth = 0.0F;
      let mut sumWidth: f32 = 0.0;
      // Java: for(int i = 0; i < text.length; ++i)
      for i in 0..text.len() {
         // Java: sumWidth += widths[i];
         sumWidth += widths[i];
      }
      // Java: return sumWidth;
      sumWidth
    }

    // Java: static Typeface getTypeface(String fontName, boolean isBold, boolean isItalic)
    pub fn getTypeface(_fontName: &str, _isBold: bool, _isItalic: bool) -> Typeface {
      // Java: ITypeface typeface = getITypeface(fontName, isBold, isItalic);
      // Java: return (Typeface)typeface.getNativeTypeface();
      Typeface
    }

    // Java: static ITypeface getITypeface(String fontName, boolean isBold, boolean isItalic)
    pub fn getITypeface(_fontName: &str, _isBold: bool, _isItalic: bool) -> Typeface {
      // Java: int style = isBold ? 1 : 0;
      // Java: style += isItalic ? 2 : 0;
      // Java: IFontTable fontTable = FontHost.getFontTable();
      // Java: IFontFamily fontFamily = null;
      // Java: if (UILanguage.UILanguage_Thai == Platform.getLanguage())
      // Java: else if (UILanguage.UILanguage_Arabic == Platform.getLanguage())
      // Java: else
      // Java: return fontFamily.getTypeface(style);
      Typeface
    }

    // Java: static void getFontMetricsInt(Paint.FontMetricsInt fontMetrics, SpanInfo span)
    pub fn getFontMetricsInt_span(fontMetrics: &mut FontMetricsInt, span: &SpanInfo) {
      // Java: getFontMetricsInt(fontMetrics, span.mFontName, span.mFontSize, span.mIsBold, span.mIsItalic);
      Self::getFontMetricsInt_full(fontMetrics, &span.mFontName, span.mFontSize, span.mIsBold, span.mIsItalic);
    }

    // Java: static void getFontMetricsInt(Paint.FontMetricsInt fontMetrics, String fontName, float fontSize, boolean bold, boolean italic)
    pub fn getFontMetricsInt_full(fontMetrics: &mut FontMetricsInt, _fontName: &str, fontSize: f32, _bold: bool, _italic: bool) {
      // Java: Typeface face = getTypeface(fontName, bold, italic);
      // Java: Paint paint = (Paint)sTextPaint.get();
      let mut paint = get_paint().lock().unwrap();
      // Java: paint.setTextSize(fontSize);
      paint.setTextSize(fontSize);
      // Java: paint.setTypeface(face);
      // Java: paint.getFontMetricsInt(fontMetrics);
      paint.getFontMetricsInt(fontMetrics);
    }

    // Java: static boolean isLeftToRight(String text)
    pub fn isLeftToRight(_text: &str) -> bool {
      // Java: return !Bidi.requiresBidi(text.toCharArray(), 0, text.length());
      true
    }

}

// Java: public class LocalTextPaint
impl LocalTextPaint {
    // Java: LocalTextPaint()
    pub fn new() -> Self {
         // Java: super();
         LocalTextPaint {}
    }

    // Java: TextPaint initialValue()
    pub fn initialValue(&self) -> TextPaint {
         // Java: return new TextPaint();
         TextPaint::new()
    }

}
