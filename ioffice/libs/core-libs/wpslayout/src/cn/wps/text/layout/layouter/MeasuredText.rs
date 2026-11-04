use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

// Java: import android.graphics.Paint
// Java: import android.text.TextPaint
// Java: import android.text.TextUtils
// Java: import cn.wps.font.ITypeface
// Java: import cn.wps.language.Arabic
// Java: import cn.wps.text.layout.typo.PaintFontHelper
use crate::cn::wps::text::layout::typo::PaintFontHelper::PaintFontHelper;
// Java: import cn.wps.text.layout.typo.SpacingSpan
use crate::cn::wps::text::layout::typo::SpacingSpan::SpacingSpan;
// Java: import cn.wps.text.layout.typo.WpsTypefaceSpan
use crate::cn::wps::text::layout::typo::WpsTypefaceSpan::WpsTypefaceSpan;
// Java: import cn.wps.text.layout.typo.layout.ArrayUtils
use crate::cn::wps::text::layout::typo::layout::ArrayUtils::ArrayUtils;
// Java: import cn.wps.text.layout.typo.layout.Spanned
// Java: import cn.wps.text.layout.typo.layout.style.MetricAffectingSpan
// Java: import java.util.Arrays
// Java: import java.util.HashMap
// Java: import java.util.Map

// Java field: boolean localLOGV
pub(crate) const localLOGV: bool = false;
// Java field: float TOP_BOTTOM_SCALE
pub(crate) const TOP_BOTTOM_SCALE: f32 = 0.92;
// Java field: float CALC_FONT_SIZE
pub(crate) const CALC_FONT_SIZE: f32 = 100.0;
// Java field: String[] NOT_NEED_HEIGHT_PERCENT_FONTS
static NOT_NEED_HEIGHT_PERCENT_FONTS: OnceLock<Vec<String>> = OnceLock::new();

fn get_not_need_fonts() -> &'static Vec<String> {
    NOT_NEED_HEIGHT_PERCENT_FONTS.get_or_init(|| vec!["微软雅黑".to_string()])
}
fn get_cached() -> &'static Mutex<Vec<Option<MeasuredText>>> {
    sCached.get_or_init(|| Mutex::new(vec![None, None, None]))
}

static FONT_BIG_SIZE_MAP: OnceLock<HashMap<String, f32>> = OnceLock::new();
fn get_font_big_size_map() -> &'static HashMap<String, f32> {
    FONT_BIG_SIZE_MAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("Times New Roman".to_string(), 0.5);
        m
    })
}

static FONT_SCALE_MAP: OnceLock<HashMap<String, f32>> = OnceLock::new();
fn get_font_scale_map() -> &'static HashMap<String, f32> {
    FONT_SCALE_MAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("Lucida Sans Unicode".to_string(), 0.79);
        m.insert("Segoe Print".to_string(), 0.67);
        m.insert("宋体".to_string(), 1.07);
        m.insert("华文中宋".to_string(), 0.92);
        m.insert("黑体".to_string(), 1.09);
        m.insert("Times New Roman".to_string(), 1.05);
        m.insert("微软雅黑".to_string(), 0.96);
        m.insert("Arial".to_string(), 1.06);
        m.insert("Arial Unicode MS".to_string(), 0.9);
        m.insert("Arial Narrow".to_string(), 1.06);
        m.insert("Verdana".to_string(), 0.9);
        m.insert("楷体_GB2312".to_string(), 1.06);
        m.insert("MS Gothic".to_string(), 1.01);
        m.insert("MS PGothic".to_string(), 1.01);
        m.insert("华文新魏".to_string(), 1.12);
        m.insert("隶书".to_string(), 1.1);
        m.insert("华文楷体".to_string(), 0.93);
        m.insert("苹方-简".to_string(), 0.88);
        m.insert("经典特宋简".to_string(), 1.02);
        m.insert("Tahoma".to_string(), 1.0);
        m.insert("Cambria".to_string(), 1.065);
        m.insert("Microsoft YaHei UI".to_string(), 0.9);
        m.insert("Microsoft YaHei UI Light".to_string(), 1.04);
        m.insert("华文细黑".to_string(), 0.85);
        m.insert("Garamond".to_string(), 1.05);
        m.insert("Vijaya".to_string(), 1.15);
        m.insert("Tw Cen MT".to_string(), 1.1);
        m.insert("Comic Sans MS".to_string(), 0.9);
        m.insert("Calibri".to_string(), 1.2);
        m
    })
}

static LINE_SCALE_MAP: OnceLock<HashMap<String, f32>> = OnceLock::new();
fn get_line_scale_map() -> &'static HashMap<String, f32> {
    LINE_SCALE_MAP.get_or_init(|| HashMap::new())
}

// Java: public class MeasuredText
pub struct MeasuredText {
    // Java field: CharSequence mText
    pub(crate) mText: Option<String>,
    // Java field: int mTextStart
    pub(crate) mTextStart: i32,
    // Java field: float[] mWidths
    pub(crate) mWidths: Vec<f32>,
    // Java field: float[] mSpacing
    pub(crate) mSpacing: Vec<f32>,
    // Java field: char[] mChars
    pub(crate) mChars: Vec<char>,
    // Java field: byte[] mLevels
    pub(crate) mLevels: Vec<u8>,
    // Java field: int mDir
    pub(crate) mDir: i32,
    // Java field: boolean mEasy
    pub(crate) mEasy: bool,
    // Java field: int mLen
    pub(crate) mLen: i32,
    // Java field: int mPos
    mPos: i32,
    // Java field: TextPaint mWorkPaint
    mWorkPaint: TextPaint,
    // Java field: float spacingmult
    pub(crate) spacingmult: f32,
}

// Java field: Object[] sLock
#[allow(non_upper_case_globals)]
static sLock: OnceLock<Mutex<i32>> = OnceLock::new();
// Java field: MeasuredText[] sCached
#[allow(non_upper_case_globals)]
static sCached: OnceLock<Mutex<Vec<Option<MeasuredText>>>> = OnceLock::new();

pub struct Paint {
    text_size: f32,
}
impl Paint {
    pub fn new_from(p: &TextPaint) -> Self { Paint { text_size: p.text_size } }
    pub fn setTextSize(&mut self, size: f32) { self.text_size = size; }
    pub fn getTextWidths(&self, _chars: &[char], _start: usize, _len: usize, _widths: &mut [f32]) {}
    pub fn measureText(&self, _chars: &[char], _start: usize, _len: usize) -> f32 { 0.0 }
}

pub struct TextPaint {
    pub text_size: f32,
    pub baselineShift: i32,
}
impl TextPaint {
    pub fn new() -> Self { TextPaint { text_size: 12.0, baselineShift: 0 } }
    pub fn set(&mut self, other: &TextPaint) { self.text_size = other.text_size; self.baselineShift = other.baselineShift; }
    pub fn getTextSize(&self) -> f32 { self.text_size }
    pub fn isFakeBoldText(&self) -> bool { false }
    pub fn getFontMetricsInt(&self, fm: &mut FontMetricsInt) { fm.ascent = -12; fm.descent = 4; fm.top = -14; fm.bottom = 5; }
}

pub struct FontMetricsInt {
    pub ascent: i32,
    pub descent: i32,
    pub top: i32,
    pub bottom: i32,
}
impl FontMetricsInt {
    pub fn new() -> Self { FontMetricsInt { ascent: 0, descent: 0, top: 0, bottom: 0 } }
}

pub trait ITypefaceTrait {
    fn isCJKFont(&self) -> bool;
    fn getWppLayoutStakckedHeight(&self) -> f32;
    fn getWppLayoutHeight(&self) -> f32;
    fn getWriterLayoutEMSize(&self) -> f32;
}

struct Arabic;
impl Arabic {
    pub fn charIsArabic(_c: char) -> bool { false }
}

// Java: public class MeasuredText
impl MeasuredText {
    // Java: MeasuredText()
    pub fn new() -> Self {
      // Java: super();
      MeasuredText {
          mText: None, mTextStart: 0, mWidths: Vec::new(), mSpacing: Vec::new(),
          mChars: Vec::new(), mLevels: Vec::new(), mDir: 0, mEasy: false,
          mLen: 0, mPos: 0, mWorkPaint: TextPaint::new(), spacingmult: 1.0,
      }
    }

    // Java: static MeasuredText obtain()
    pub fn obtain() -> MeasuredText {
      // Java: synchronized(sLock)
      {
         let mut cached = get_cached().lock().unwrap();
         // Java: int i = sCached.length;
         let mut i = cached.len() as i32;
         // Java: while(true)
         loop
         {
            // Java: --i;
            i -= 1;
            // Java: if (i < 0)
            if i < 0
            {
               // Java: break;
               break;
            }
            // Java: if (sCached[i] != null)
            if cached[i as usize].is_some()
            {
               // Java: MeasuredText mt = sCached[i];
               // Java: sCached[i] = null;
               let mt = cached[i as usize].take().unwrap();
               // Java: return mt;
               return mt;
            }
         }
      }
      // Java: MeasuredText mt = new MeasuredText();
      let mt = MeasuredText::new();
      // Java: return mt;
      mt
    }

    // Java: static MeasuredText recycle(MeasuredText mt)
    pub fn recycle(mut mt: MeasuredText) -> Option<MeasuredText> {
      // Java: mt.mText = null;
      mt.mText = None;
      // Java: if (mt.mLen < 1000)
      if mt.mLen < 1000
      {
         // Java: synchronized(sLock)
         {
            let mut cached = get_cached().lock().unwrap();
            // Java: for(int i = 0; i < sCached.length; ++i)
            for i in 0..cached.len()
            {
               // Java: if (sCached[i] == null)
               if cached[i].is_none()
               {
                  // Java: sCached[i] = mt;
                  // Java: mt.mText = null;
                  cached[i] = Some(mt);
                  // Java: break;
                  return None;
               }
            }
         }
      }
      // Java: return null;
      None
    }

    // Java: void setPos(int pos)
    pub fn setPos(&mut self, pos: i32) {
      // Java: this.mPos = pos - this.mTextStart;
      self.mPos = pos - self.mTextStart;
    }

    // Java: void setPara(CharSequence text, int start, int end)
    pub fn setPara(&mut self, text: &str, start: i32, end: i32) {
      // Java: this.mText = text;
      self.mText = Some(text.to_string());
      // Java: this.mTextStart = start;
      self.mTextStart = start;
      // Java: int len = end - start;
      let len = (end - start) as usize;
      // Java: this.mLen = len;
      self.mLen = len as i32;
      // Java: this.mPos = 0;
      self.mPos = 0;
      // Java: if (this.mWidths == null || this.mWidths.length < len)
      if self.mWidths.len() < len
      {
         // Java: this.mWidths = new float[ArrayUtils.idealFloatArraySize(len)];
         self.mWidths = vec![0.0; ArrayUtils::idealFloatArraySize(len as i32) as usize];
      }
      // Java: if (this.mChars == null || this.mChars.length < len)
      if self.mChars.len() < len
      {
         // Java: this.mChars = new char[ArrayUtils.idealCharArraySize(len)];
         self.mChars = vec!['\0'; ArrayUtils::idealCharArraySize(len as i32) as usize];
      }
      // Java: TextUtils.getChars(text, start, end, this.mChars, 0);
      let chars: Vec<char> = text.chars().collect();
      for i in 0..len {
          if (start as usize + i) < chars.len() { self.mChars[i] = chars[start as usize + i]; }
      }
      if self.mSpacing.len() < len { self.mSpacing = vec![0.0; len]; }
      // Java: this.mDir = 1;
      self.mDir = 1;
      // Java: this.mEasy = true;
      self.mEasy = true;
    }

    // Java: float addStyleRun(TextPaint paint, int len, Paint.FontMetricsInt fm, ITypeface iTf)
    pub fn addStyleRun(&mut self, paint: &TextPaint, len: i32, fm: &mut FontMetricsInt, _iTf: Option<&dyn ITypefaceTrait>) -> f32 {
      // Java: if (fm != null)
      {
         // Java: paint.getFontMetricsInt(fm);
         paint.getFontMetricsInt(fm);
      }
      // Java: float fontSize = paint.getTextSize();
      let fontSize = paint.getTextSize();
      // Java: boolean isBold = paint.isFakeBoldText();
      let _isBold = paint.isFakeBoldText();
      // Java: int p = this.mPos;
      let p = self.mPos as usize;
      // Java: this.mPos = p + len;
      self.mPos = p as i32 + len;
      // Java: float[] widths = new float[len];
      let mut widths: Vec<f32> = vec![0.0; len as usize];
      // Java: Paint measurePaint = new Paint(paint);
      let mut measurePaint = Paint::new_from(paint);
      // Java: measurePaint.setTextSize(fontSize);
      measurePaint.setTextSize(fontSize);
      // Java: measurePaint.getTextWidths(this.mChars, p, len, widths);
      measurePaint.getTextWidths(&self.mChars, p, len as usize, &mut widths);
      // Java: for(int i = 1; i < len; ++i)
      for i in 1..(len as usize)
      {
         // Java: char cur = this.mChars[p + i];
         let cur = self.mChars[p + i];
         // Java: if (widths[i] == 0.0F && Arabic.charIsArabic(cur))
         if widths[i] == 0.0 && Arabic::charIsArabic(cur)
         {
            // Java: widths[i - 1] = measurePaint.measureText(this.mChars, p + i - 1, 1);
            widths[i - 1] = measurePaint.measureText(&self.mChars, p + i - 1, 1);
            // Java: widths[i] = measurePaint.measureText(this.mChars, p + i, 1);
            widths[i] = measurePaint.measureText(&self.mChars, p + i, 1);
         }
      }
      // Java: if (this.mText instanceof Spanned)
      {
         // Java: Spanned spanned = (Spanned)this.mText;
         // Java: SpacingSpan[] spacingSpans = (SpacingSpan[])spanned.getSpans(p, p + len, SpacingSpan.class);
         // Java: if (spacingSpans.length > 0)
         {
            // Java: for(SpacingSpan spacingSpan : spacingSpans)
            {
               // Java: int start = spanned.getSpanStart(spacingSpan);
               // Java: int end = spanned.getSpanEnd(spacingSpan);
               // Java: Arrays.fill(this.mSpacing, start, end, spacingSpan.getSpacing());
            }
         }
      }
      // Java: for(int i = 0; i < len; ++i)
      for i in 0..(len as usize)
      {
         // Java: float width = widths[i];
         let mut width = widths[i];
         // Java: if (width != 0.0F)
         if width != 0.0
         {
            // Java: width += this.mSpacing[i + p];
            if (i + p) < self.mSpacing.len() { width += self.mSpacing[i + p]; }
            // Java: if (this.mSpacing[i + p] < 0.0F && width <= 0.0F)
            if (i + p) < self.mSpacing.len() && self.mSpacing[i + p] < 0.0 && width <= 0.0
            {
               // Java: width = 0.01F;
               width = 0.01;
            }
         }
         // Java: this.mWidths[p + i] = width;
         if (p + i) < self.mWidths.len() { self.mWidths[p + i] = width; }
      }
      // Java: return (float)(fm.descent - fm.ascent);
      (fm.descent - fm.ascent) as f32
    }

    // Java: float addStyleRun(TextPaint paint, MetricAffectingSpan[] spans, int len, Paint.FontMetricsInt fm, boolean isVert)
    pub fn addStyleRun_with_spans(&mut self, paint: &TextPaint, spans: &[&WpsTypefaceSpan], len: i32, fm: &mut FontMetricsInt, isVert: bool) -> f32 {
      // Java: TextPaint workPaint = this.mWorkPaint;
      // Java: workPaint.set(paint);
      self.mWorkPaint.set(paint);
      // Java: workPaint.baselineShift = 0;
      self.mWorkPaint.baselineShift = 0;
      // Java: boolean needScale = false;
      let mut needScale = false;
      // Java: Float scale = 1.0F;
      let mut scale: f32 = 1.0;
      // Java: boolean isTopBottomFont = false;
      let mut isTopBottomFont = false;
      // Java: WpsTypefaceSpan wpsTypefaceSpan = null;
      let mut wpsTypefaceSpan: Option<usize> = None;
      // Java: float height = 0.0F;
      let mut height: f32 = 0.0;
      // Java: for(int i = 0; i < spans.length; ++i)
      for i in 0..spans.len()
      {
         // Java: MetricAffectingSpan span = spans[i];
         // Java: span.updateMeasureState(workPaint);
         // Java: if (span instanceof WpsTypefaceSpan)
         {
            // Java: wpsTypefaceSpan = (WpsTypefaceSpan)span;
            wpsTypefaceSpan = Some(i);
         }
      }
      // Java: ITypeface iTf = null;
      // Java: if (wpsTypefaceSpan != null)
      if let Some(idx) = wpsTypefaceSpan
      {
         let span = spans[idx];
         // Java: if (!wpsTypefaceSpan.getFontName().equals("serif"))
         if span.getFontName() != "serif"
         {
            // Java: iTf = PaintFontHelper.getITypeface(wpsTypefaceSpan.getFontName(), wpsTypefaceSpan.isBold(), wpsTypefaceSpan.isItalic());
            // Java: if (iTf.isCJKFont() && !isNotNeedHeightPercentFont(wpsTypefaceSpan.getFontName()) || isVert)
            {
               // Java: height = isVert ? iTf.getWppLayoutStakckedHeight() : iTf.getWppLayoutHeight();
               // Java: height = height * workPaint.getTextSize() / iTf.getWriterLayoutEMSize();
               // Java: height = (float)Math.floor((double)height);
            }
         }
         // Java: isTopBottomFont = wpsTypefaceSpan.isTopBottomFont();
         isTopBottomFont = span.isTopBottomFont();
         // Java: String fontName = wpsTypefaceSpan.getFontName();
         let fontName = span.getFontName();
         // Java: Float fontScale = (Float)fontScaleMap.get(fontName);
         let fontScale = get_font_scale_map().get(fontName);
         // Java: if (fontScale != null)
         if let Some(&fs) = fontScale
         {
            // Java: scale = fontScale;
            scale = fs;
            // Java: needScale = true;
            needScale = true;
         }
         // Java: fontScale = (Float)fontBigSizeMap.get(fontName);
         let fontScale2 = get_font_big_size_map().get(fontName);
         // Java: if (fontScale != null)
         if fontScale2.is_some()
         {
            // Java: if (workPaint.getTextSize() >= 60.0F)
            if self.mWorkPaint.getTextSize() >= 60.0
            {
               // Java: scale = scale * 0.6F;
               scale = scale * 0.6;
            }
            // Java: needScale = true;
            needScale = true;
         }
         // Java: if (this.spacingmult < 1.0F)
         if self.spacingmult < 1.0
         {
            // Java: Float lineScale = (Float)lineScaleMap.get(fontName);
            let lineScale = get_line_scale_map().get(fontName);
            // Java: if (lineScale != null)
            if let Some(&ls) = lineScale
            {
               // Java: scale = scale * lineScale;
               scale = scale * ls;
               // Java: needScale = true;
               needScale = true;
            }
         }
      }
      // Java: this.addStyleRun(workPaint, len, fm, iTf);
      let work_paint_copy = TextPaint { text_size: self.mWorkPaint.text_size, baselineShift: self.mWorkPaint.baselineShift };
      self.addStyleRun(&work_paint_copy, len, fm, None);
      // Java: if (fm != null)
      {
         // Java: if (workPaint.baselineShift < 0)
         if self.mWorkPaint.baselineShift < 0
         {
            // Java: fm.ascent += workPaint.baselineShift;
            fm.ascent += self.mWorkPaint.baselineShift;
            // Java: fm.top += workPaint.baselineShift;
            fm.top += self.mWorkPaint.baselineShift;
         }
         // Java: else
         else
         {
            // Java: fm.descent += workPaint.baselineShift;
            fm.descent += self.mWorkPaint.baselineShift;
            // Java: fm.bottom += workPaint.baselineShift;
            fm.bottom += self.mWorkPaint.baselineShift;
         }
         // Java: if (needScale && !isVert)
         if needScale && !isVert
         {
            // Java: if (isTopBottomFont)
            if isTopBottomFont
            {
               // Java: fm.descent = (int)((float)fm.bottom * 0.92F);
               fm.descent = (fm.bottom as f32 * 0.92) as i32;
               // Java: fm.ascent = (int)((float)fm.top * 0.92F);
               fm.ascent = (fm.top as f32 * 0.92) as i32;
            }
            // Java: else if (scale != null)
            else
            {
               // Java: fm.descent = (int)((float)fm.descent * scale);
               fm.descent = (fm.descent as f32 * scale) as i32;
               // Java: fm.ascent = (int)((float)fm.ascent * scale);
               fm.ascent = (fm.ascent as f32 * scale) as i32;
               // Java: fm.top = (int)((float)fm.top * scale);
               fm.top = (fm.top as f32 * scale) as i32;
               // Java: fm.bottom = (int)((float)fm.bottom * scale);
               fm.bottom = (fm.bottom as f32 * scale) as i32;
            }
         }
      }
      // Java: if (height == 0.0F)
      if height == 0.0
      {
         // Java: height = (float)(fm.descent - fm.ascent) / 1.2F;
         height = (fm.descent - fm.ascent) as f32 / 1.2;
      }
      // Java: return height;
      height
    }

    // Java: int breakText(int limit, boolean forwards, float width)
    pub fn breakText(&self, limit: i32, forwards: bool, mut width: f32) -> i32 {
      // Java: float[] w = this.mWidths;
      let w = &self.mWidths;
      // Java: if (forwards)
      if forwards
      {
         // Java: int i;
         let mut i: i32 = 0;
         // Java: for(i = 0; i < limit; ++i)
         while i < limit
         {
            // Java: width -= w[i];
            width -= w[i as usize];
            // Java: if (width < 0.0F)
            if width < 0.0
            {
               // Java: break;
               break;
            }
            i += 1;
         }
         // Java: while(i > 0 && this.mChars[i - 1] == ' ')
         while i > 0 && self.mChars[(i - 1) as usize] == ' '
         {
            // Java: --i;
            i -= 1;
         }
         // Java: return i;
         i
      }
      // Java: else
      else
      {
         // Java: int i;
         let mut i: i32;
         // Java: for(i = limit - 1; i >= 0; --i)
         i = limit - 1;
         while i >= 0
         {
            // Java: width -= w[i];
            width -= w[i as usize];
            // Java: if (width < 0.0F)
            if width < 0.0
            {
               // Java: break;
               break;
            }
            i -= 1;
         }
         // Java: while(i < limit - 1 && this.mChars[i + 1] == ' ')
         while i < limit - 1 && self.mChars[(i + 1) as usize] == ' '
         {
            // Java: ++i;
            i += 1;
         }
         // Java: return limit - i - 1;
         limit - i - 1
      }
    }

    // Java: float measure(int start, int limit)
    pub fn measure(&self, start: i32, limit: i32) -> f32 {
      // Java: float width = 0.0F;
      let mut width: f32 = 0.0;
      // Java: float[] w = this.mWidths;
      let w = &self.mWidths;
      // Java: for(int i = start; i < limit; ++i)
      for i in (start as usize)..(limit as usize)
      {
         // Java: width += w[i];
         width += w[i];
      }
      // Java: return width;
      width
    }

    // Java: static boolean isNotNeedHeightPercentFont(String fontName)
    pub fn isNotNeedHeightPercentFont(fontName: &str) -> bool {
      // Java: for(String font : NOT_NEED_HEIGHT_PERCENT_FONTS)
      for font in get_not_need_fonts().iter()
      {
         // Java: if (font.equals(fontName))
         if font == fontName
         {
            // Java: return true;
            return true;
         }
      }
      // Java: return false;
      false
    }

}
