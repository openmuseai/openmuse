// Java: import android.graphics.RectF
// Java: import android.graphics.Typeface
// Java: import android.os.Build.VERSION
// Java: import android.text.TextPaint
// Java: import android.text.TextUtils
// Java: import cn.wps.font.FontHost
// Java: import cn.wps.font.IFontTable
// Java: import cn.wps.font.ITypeface
// Java: import cn.wps.language.Arabic
// Java: import cn.wps.language.Thai
// Java: import cn.wps.text.layout.ITextLayout
// Java: import cn.wps.text.layout.data.ParaInfo
// Java: import cn.wps.text.layout.data.SpanInfo
// Java: import cn.wps.text.layout.data.TextData
// Java: import cn.wps.text.layout.layouter.AudioLayout
// Java: import cn.wps.text.layout.layouter.Emoji
// Java: import cn.wps.text.layout.layouter.UnsupportTextException
// Java: import cn.wps.text.layout.layouter.WpsLayout
// Java: import cn.wps.text.layout.typo.layout.Layout
// Java: import cn.wps.text.layout.typo.layout.Spannable
// Java: import cn.wps.text.layout.typo.layout.SpannableStringBuilder
// Java: import cn.wps.text.layout.typo.layout.style.AbsoluteSizeSpan
// Java: import cn.wps.text.layout.typo.layout.style.CharacterStyle
// Java: import cn.wps.text.layout.typo.layout.style.ForegroundColorSpan
// Java: import cn.wps.text.layout.typo.layout.style.LeadingMarginSpan
// Java: import cn.wps.text.layout.typo.layout.style.ParagraphStyle
// Java: import cn.wps.text.layout.typo.layout.style.ScaleXSpan
// Java: import cn.wps.text.layout.typo.layout.style.SubscriptSpan
// Java: import cn.wps.text.layout.typo.layout.style.SuperscriptSpan
// Java: import cn.wps.text.layout.util.BidiHelper
// Java: import java.text.Bidi
// Java: import java.util.Arrays
// Java: import java.util.HashMap
// Java: import java.util.Map
// Java: import java.util.Vector

use std::collections::HashMap;
use std::sync::OnceLock;

pub struct RectF { pub left: f32, pub top: f32, pub right: f32, pub bottom: f32 }
impl RectF { pub fn width(&self) -> f32 { self.right - self.left } }
pub struct TextPaint { pub text_size: f32, pub flags: i32 }
impl TextPaint {
    pub fn new() -> Self { TextPaint { text_size: 0.0, flags: 0 } }
    pub fn set(&mut self, _other: &TextPaint) {}
    pub fn setTypeface(&mut self, _tf: i32) {}
    pub fn setFlags(&mut self, flags: i32) { self.flags = flags; }
    pub fn setTextSize(&mut self, size: f32) { self.text_size = size; }
    pub fn measureText(&self, _text: &str) -> f32 { 0.0 }
}
pub struct Typeface;
impl Typeface { pub const DEFAULT: i32 = 0; pub fn isItalic(&self) -> bool { false } }
pub struct Layout { pub alignment: i32 }
impl Layout {
    pub fn new() -> Self { Layout { alignment: 0 } }
}
pub struct SpannableStringBuilder { pub text: String }
impl SpannableStringBuilder {
    pub fn new(text: &str) -> Self { SpannableStringBuilder { text: text.to_string() } }
    pub fn length(&self) -> i32 { self.text.len() as i32 }
    pub fn append(&mut self, s: &str) { self.text.push_str(s); }
    pub fn replace(&mut self, _start: i32, _end: i32, _text: &str) {}
    pub fn subSequence(&self, start: i32, end: i32) -> String { self.text[start as usize..end as usize].to_string() }
    pub fn setSpan(&mut self, _span: i32, _start: i32, _end: i32, _flags: i32) {}
    pub fn getSpans(&self, _start: i32, _end: i32) -> Vec<i32> { Vec::new() }
    pub fn getSpanStart(&self, _span: i32) -> i32 { 0 }
    pub fn getSpanEnd(&self, _span: i32) -> i32 { 0 }
    pub fn getSpanFlags(&self, _span: i32) -> i32 { 0 }
    pub fn toString(&self) -> &str { &self.text }
}
pub struct AudioLayout;
impl AudioLayout {
    pub fn new(_text: &str, _paint: &TextPaint, _width: i32, _align: i32, _mult: f32, _add: f32, _vert: i32) -> Self { AudioLayout }
}
pub struct WpsLayout;
impl WpsLayout {
    pub fn new(_source: &str, _start: i32, _end: i32, _paint: &TextPaint, _width: f32, _align: i32, _mult: f32, _add: f32, _trunc: i32, _max: i32, _tab: i32, _single: bool, _para: &ParaInfo, _vert: i32, _flags: i16) -> Layout { Layout::new() }
}
pub struct ParaInfo { pub mHAlignment: i32, pub mIsRightToLeft: bool, pub mSpcWithinPct: f64, pub mSpcWithinPts: f64, pub mDefTabSize: i32, pub mIsWrapText: bool, pub mVertical: i32 }
#[derive(Clone)]
pub struct SpanInfo { pub mStartPos: i32, pub mEndPos: i32, pub mFontSize: f32, pub mFontColor: i32, pub mFontName: String, pub mIsBold: bool, pub mIsItalic: bool, pub mIsSubScript: bool, pub mIsSuperScript: bool, pub mTextCaps: i32, pub mArb: bool }
impl SpanInfo {
    pub fn hasSpc(&self) -> bool { false }
    pub fn getSpc(&self) -> f32 { 0.0 }
    pub fn resetAscent(&mut self) {}
}
pub struct TextData { pub mIsRichText: bool, pub mTypoText: String, pub mFirstSpan: SpanInfo, pub mOtherSpans: Vec<SpanInfo>, pub mTypedCharSequence: String, pub mPara: ParaInfo, pub mBidi: Option<i32>, pub mVertical: i32, pub mArb: bool, pub mSourceText: String, pub mIsTopBottomFont: bool, pub mZeroWidthSpan: Option<ZeroWidthSpan> }
impl TextData {
    pub fn getFirstSpan(&self) -> Option<&SpanInfo> { Some(&self.mFirstSpan) }
    pub fn getNextSpan(&self) -> Option<&SpanInfo> { None }
    pub fn isChinese(c: char) -> bool { (c as u32) >= 0x4E00 && (c as u32) <= 0x9FFF }
}
pub struct ZeroWidthSpan { pub start: i32, pub end: i32 }
pub struct IFontTable;
impl IFontTable { pub fn get(&self, _name: &str) -> Option<i32> { Some(0) } }
pub struct FontHost;
impl FontHost { pub fn getFontTable() -> IFontTable { IFontTable } }
pub struct ITypeface;
pub struct ArabicArgs { pub srcBuff: Vec<char>, pub len: i32, pub desBuff: Vec<char>, pub srcPos: i32, pub desPos: i32 }
pub struct Arabic;
impl Arabic {
    pub fn getRenderGraphs(_args: &mut ArabicArgs) -> i32 { 0 }
}
pub struct Thai;
impl Thai { pub fn getWordPositions(_chars: &[char]) -> Option<Vec<i32>> { None } }
pub struct Emoji;
impl Emoji { pub fn isEmojiChar(_c: char) -> bool { false } }
pub struct BidiHelper;
impl BidiHelper { pub fn updateText(_source: &mut SpannableStringBuilder, _start: i32, _end: i32, _embed: &[bool]) {} }

// Java field: int ANDROID_VERSION
pub(crate) const ANDROID_VERSION: i32 = 21;
// Java field: int ANDROID_SDK_INT_4_1
pub(crate) const ANDROID_SDK_INT_4_1: i32 = 16;
// Java field: short MAX_BORING_TLEN
pub(crate) const MAX_BORING_TLEN: i16 = 5215;
// Java field: short MAX_Boring_TLEN_2_1
pub(crate) const MAX_Boring_TLEN_2_1: i16 = 3520;
// Java field: float SMALL_CAPS_SCALE_FACTOR
pub(crate) const SMALL_CAPS_SCALE_FACTOR: f32 = 0.8;
// Java field: float SUB_SCRIPT_SCALE_FACTOR
pub(crate) const SUB_SCRIPT_SCALE_FACTOR: f32 = 0.64;
// Java field: float SUPER_SCRIPT_SCALE_FACTOR
pub(crate) const SUPER_SCRIPT_SCALE_FACTOR: f32 = 0.7;
// Java field: char[] BRACKETS
static BRACKETS: OnceLock<Vec<char>> = OnceLock::new();
// Java field: char[] BRACKETS_VERT
static BRACKETS_VERT: OnceLock<Vec<char>> = OnceLock::new();
// Java field: Float DEFAULT_LINE_SCALE
static DEFAULT_LINE_SCALE: OnceLock<f32> = OnceLock::new();
// Java field: float LINESPACE_PCT_SCALE_DEFAULT
pub(crate) const LINESPACE_PCT_SCALE_DEFAULT: f32 = 1.2;
// Java field: float CALIBRI_NONECALIBRI_PCT_SCALE
pub(crate) const CALIBRI_NONECALIBRI_PCT_SCALE: f32 = 1.08;
// Java field: float CALIBRI_PMINGLIU_PCT_SCALE
pub(crate) const CALIBRI_PMINGLIU_PCT_SCALE: f32 = 1.168;
// Java field: String CALIBRI_NAME
pub(crate) const CALIBRI_NAME: &str = "Calibri";
// Java field: String PMINGLIU_NAME
pub(crate) const PMINGLIU_NAME: &str = "\u{65B0}\u{7D30}\u{660E}\u{9AD4}";
// Java field: String PUNCTUATIONS
pub(crate) const PUNCTUATIONS: &str = " ,?;'\"";

fn get_brackets() -> &'static Vec<char> {
    BRACKETS.get_or_init(|| vec!['〈','〉','《','》','「','」','『','』','【','】','〔','〕','〖','〗','（','）','［','］','｛','｝'])
}
fn get_brackets_vert() -> &'static Vec<char> {
    BRACKETS_VERT.get_or_init(|| vec!['︗','︘','︵','︶','︷','︸','︹','︺','︻','︼','︽','︾','︿','﹀','﹁','﹂','﹃','﹄'])
}
fn get_default_line_scale() -> f32 {
    *DEFAULT_LINE_SCALE.get_or_init(|| 1.0)
}

static FONT_TO_LINESPACE_PCT_SCALE: OnceLock<HashMap<&str, f32>> = OnceLock::new();
fn get_font_scale_map() -> &'static HashMap<&'static str, f32> {
    FONT_TO_LINESPACE_PCT_SCALE.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("Times New Roman", 1.05f32);
        m.insert("\u{5FAE}\u{8F6F}\u{96C5}\u{9ED1}", 0.9f32);
        m.insert("Arial", 1.06f32);
        m.insert("Arial Unicode MS", 0.9f32);
        m.insert("Arial Narrow", 1.06f32);
        m.insert("Verdana", 0.9f32);
        m.insert("\u{534E}\u{6587}\u{4E2D}\u{5B8B}", 0.9f32);
        m.insert("\u{6977}\u{4F53}_GB2312", 1.06f32);
        m.insert("\u{9ED1}\u{4F53}", 1.16f32);
        m.insert("MS Gothic", 1.01f32);
        m.insert("MS PGothic", 1.01f32);
        m.insert("\u{534E}\u{6587}\u{65B0}\u{9B4F}", 1.12f32);
        m.insert("\u{96B6}\u{4E66}", 1.1f32);
        m.insert("\u{534E}\u{6587}\u{6977}\u{4F53}", 0.93f32);
        m.insert("\u{82F9}\u{65B9}-\u{7B80}", 0.88f32);
        m.insert("\u{7ECF}\u{5178}\u{7279}\u{5B8B}\u{7B80}", 1.02f32);
        m.insert("Tahoma", 1.0f32);
        m.insert("Cambria", 0.95f32);
        m.insert("Microsoft YaHei UI", 0.9f32);
        m.insert("Microsoft YaHei UI Light", 1.04f32);
        m.insert("\u{534E}\u{6587}\u{7EC6}\u{9ED1}", 0.85f32);
        m.insert("Garamond", 1.05f32);
        m.insert("\u{5B8B}\u{4F53}", 1.0f32);
        m.insert("Vijaya", 1.15f32);
        m
    })
}

static TOP_BOTTOM_FONT: OnceLock<Vec<&str>> = OnceLock::new();
fn get_top_bottom_font() -> &'static Vec<&'static str> {
    TOP_BOTTOM_FONT.get_or_init(|| vec!["lrisUPC","EucrosiaUPC","FreesiaUPC","JasmineUPC","KodchiangUPC","TH Fah kwang","Angsana New"])
}

fn getLayoutAlignment(_hAlign: i32, _isRtl: bool) -> i32 { 0 }

// Java: public class TypoLayoutGen
pub struct TypoLayoutGen {
}

// Java: public class FontLineSpScale
pub struct FontLineSpScale {
}

// Java: public class TypoLayoutGen
impl TypoLayoutGen {
    // Java: TypoLayoutGen()
    pub fn new() -> Self {
      // Java: super();
      TypoLayoutGen {}
    }

    // Java: Layout getLayout(RectF rect, TextPaint textPaint, TextData data, int firstLead, int restLead, float maxWidth)
    pub fn getLayout(&self, rect: &RectF, textPaint: &mut TextPaint, data: &mut TextData, firstLead: i32, restLead: i32, maxWidth: f32) -> Layout {
      // Java: CharSequence layoutText;
      let layoutText: String;
      // Java: if (!data.mIsRichText)
      if !data.mIsRichText
      {
         // Java: layoutText = data.mTypoText;
         layoutText = data.mTypoText.clone();
         // Java: PaintFontHelper.setPaintUNoneText(data.mFirstSpan, textPaint);
      }
      // Java: else
      else
      {
         // Java: textPaint.set(PaintFontHelper.INIT_TEXT_PAINT);
         // Java: textPaint.setTypeface(Typeface.DEFAULT);
         textPaint.setTypeface(Typeface::DEFAULT);
         // Java: textPaint.setFlags(1);
         textPaint.setFlags(1);
         // Java: layoutText = this.createSpannable(data, firstLead, restLead);
         layoutText = self.createSpannable(data, firstLead, restLead);
         // Java: if (data.mOtherSpans.isEmpty())
         if data.mOtherSpans.is_empty()
         {
            // Java: textPaint.setTextSize(data.mFirstSpan.mFontSize);
            textPaint.setTextSize(data.mFirstSpan.mFontSize);
         }
      }
      // Java: data.mTypedCharSequence = layoutText;
      data.mTypedCharSequence = layoutText.clone();
      // Java: Layout.Alignment alignment = getLayoutAlignment(data.mPara.mHAlignment, data.mPara.mIsRightToLeft);
      let alignment = getLayoutAlignment(data.mPara.mHAlignment, data.mPara.mIsRightToLeft);
      // Java: float spcPct = (float)data.mPara.mSpcWithinPct;
      let spcPct = data.mPara.mSpcWithinPct as f32;
      // Java: float rectWidth = ITextLayout.TypoParam.isAutoAdjustSize(rect.width()) ? maxWidth : rect.width();
      let rectWidth = if rect.width() == f32::MAX { maxWidth } else { rect.width() };
      // Java: Layout layout = this.geneLayout(layoutText, data, rectWidth, alignment, textPaint, spcPct);
      let layout = self.geneLayout(&layoutText, data, rectWidth, alignment, textPaint, spcPct);
      // Java: return layout;
      layout
    }

    // Java: Layout geneLayout(CharSequence layoutText, TextData data, float rectWid, Layout.Alignment alignment, TextPaint textPaint, float spcPct)
    pub fn geneLayout(&self, layoutText: &str, data: &TextData, rectWid: f32, alignment: i32, textPaint: &TextPaint, spcPct: f32) -> Layout {
      // Java: boolean singleLine = !data.mPara.mIsWrapText && !data.mPara.mIsRightToLeft;
      let singleLine = !data.mPara.mIsWrapText && !data.mPara.mIsRightToLeft;
      // Java: return ANDROID_VERSION != 16 ? createTextLayout(layoutText, rectWid, alignment, textPaint, spcPct, (float)data.mPara.mSpcWithinPts, data.mPara.mDefTabSize, singleLine, null != data.mBidi, data.mPara, data.mVertical) : createTextLayoutFor4_1(layoutText, data, rectWid, alignment, textPaint, spcPct, (float)data.mPara.mSpcWithinPts, singleLine, data.mPara, data.mVertical);
      if ANDROID_VERSION != 16 {
          Self::createTextLayout(layoutText, rectWid, alignment, textPaint, spcPct, data.mPara.mSpcWithinPts as f32, data.mPara.mDefTabSize, singleLine, data.mBidi.is_some(), &data.mPara, data.mVertical)
      } else {
          Self::createTextLayoutFor4_1(layoutText, data, rectWid, alignment, textPaint, spcPct, data.mPara.mSpcWithinPts as f32, singleLine, &data.mPara, data.mVertical)
      }
    }

    // Java: static Layout createTextLayout(CharSequence text, float layoutWidth, Layout.Alignment alignment, TextPaint textPaint, float spacMult, float spacAdd, int tabSize, boolean singleLine, boolean needBidi, ParaInfo paraInfo, int vertical)
    pub fn createTextLayout(text: &str, mut layoutWidth: f32, alignment: i32, textPaint: &TextPaint, spacMult: f32, spacAdd: f32, tabSize: i32, mut singleLine: bool, _needBidi: bool, paraInfo: &ParaInfo, vertical: i32) -> Layout {
      // Java: try
      {
         // Java: if (layoutWidth == Float.MAX_VALUE)
         if layoutWidth == f32::MAX
         {
            // Java: singleLine = true;
            singleLine = true;
            // Java: layoutWidth = textPaint.measureText(text.toString()) + 0.5F;
            layoutWidth = textPaint.measureText(text) + 0.5;
         }
         // Java: if (BaseTextLayout.isShape(text.toString()))
         if false
         {
            // Java: return createAudioLayout(text, textPaint, (int)layoutWidth, alignment, spacMult, spacAdd, vertical);
            return Layout::new();
         }
         // Java: else
         else
         {
            // Java: CharSequence rawSource = text.subSequence(0, text.length());
            let rawSource = text;
            // Java: CharSequence source = processRightToLeftText(rawSource);
            let source = Self::processRightToLeftText(rawSource);
            // Java: return new WpsLayout(source, 0, source.length(), textPaint, layoutWidth, alignment, spacMult, spacAdd, (TextUtils.TruncateAt)null, 0, tabSize, singleLine, paraInfo, vertical, (short)0);
            return WpsLayout::new(&source, 0, source.len() as i32, textPaint, layoutWidth, alignment, spacMult, spacAdd, 0, 0, tabSize, singleLine, paraInfo, vertical, 0);
         }
      }
      // Java: catch (Throwable e)
      // {
         // Java: e.printStackTrace();
         // Java: return null;
      // }
    }

    // Java: static CharSequence processRightToLeftText(CharSequence text)
    pub fn processRightToLeftText(text: &str) -> String {
      // Java: int length = text.length();
      let length = text.len();
      // Java: char[] values = new char[length];
      let values: Vec<char> = text.chars().collect();
      // Java: boolean needBidi = Bidi.requiresBidi(text.toString().toCharArray(), 0, length);
      let needBidi = values.iter().any(|c| {
          let cp = *c as u32;
          (0x0590..=0x08FF).contains(&cp) || (0xFB50..=0xFDFF).contains(&cp) || (0xFE70..=0xFEFF).contains(&cp)
      });
      // Java: if (needBidi)
      if needBidi
      {
         // Java: char[] results = new char[length];
         let mut results: Vec<char> = vec!['\0'; length];
         // Java: TextUtils.getChars(text, 0, length, values, 0);
         // Java: SpannableStringBuilder result = new SpannableStringBuilder();
         let mut result = SpannableStringBuilder::new("");
         // Java: Arabic.Args mArgs = new Arabic.Args();
         let mut mArgs = ArabicArgs { srcBuff: values.clone(), len: length as i32, desBuff: results.clone(), srcPos: 0, desPos: 0 };
         // Java: mArgs.srcBuff = values;
         // Java: mArgs.len = length;
         // Java: mArgs.desBuff = results;
         // Java: while(mArgs.len > 0)
         while mArgs.len > 0
         {
            // Java: int oldSrcPos = mArgs.srcPos;
            let oldSrcPos = mArgs.srcPos;
            // Java: int oldDesPos = mArgs.desPos;
            let _oldDesPos = mArgs.desPos;
            // Java: int renderGraphLen = Arabic.getRenderGraphs(mArgs);
            let renderGraphLen = Arabic::getRenderGraphs(&mut mArgs);
            // Java: int srcLen = mArgs.srcPos - oldSrcPos;
            let srcLen = mArgs.srcPos - oldSrcPos;
            // Java: if (srcLen != renderGraphLen)
            if srcLen != renderGraphLen
            {
               // Java: char ZWNJ = 8204;
               let _zwnj: char = '\u{200C}';
               // Java: Arrays.fill(results, mArgs.desPos, mArgs.srcPos, ZWNJ);
               // Java: mArgs.desPos = mArgs.srcPos;
               mArgs.desPos = mArgs.srcPos;
            }
            // Java: String content = String.valueOf(mArgs.desBuff, oldDesPos, srcLen);
            let content: String = mArgs.desBuff[_oldDesPos as usize..((_oldDesPos + srcLen) as usize).min(mArgs.desBuff.len())].iter().collect();
            // Java: result.append(content);
            result.append(&content);
            mArgs.len -= srcLen;
         }
         // Java: if (text instanceof SpannableStringBuilder)
         {
            // Java: SpannableStringBuilder spanedText = (SpannableStringBuilder)text;
            // Java: CharacterStyle[] characterStyles = (CharacterStyle[])spanedText.getSpans(0, length, CharacterStyle.class);
            // Java: for(CharacterStyle style : characterStyles)
            {
               // Java: int start = spanedText.getSpanStart(style);
               // Java: int end = spanedText.getSpanEnd(style);
               // Java: int flag = spanedText.getSpanFlags(style);
               // Java: result.setSpan(style, start, end, flag);
            }
         }
         // Java: text = result;
         return result.text;
      }
      // Java: return text;
      text.to_string()
    }

    // Java: static AudioLayout createAudioLayout(CharSequence text, TextPaint textPaint, int layoutWidth, Layout.Alignment alignment, float spacMult, float spacAdd, int vertical)
    pub fn createAudioLayout(text: &str, textPaint: &TextPaint, layoutWidth: i32, alignment: i32, spacMult: f32, spacAdd: f32, vertical: i32) -> Layout {
      // Java: AudioLayout audioLayout = new AudioLayout(text, textPaint, layoutWidth, alignment, spacMult, spacAdd, vertical);
      let _audioLayout = AudioLayout::new(text, textPaint, layoutWidth, alignment, spacMult, spacAdd, vertical);
      // Java: return audioLayout;
      Layout::new()
    }

    // Java: static Layout createTextLayoutFor4_1(CharSequence text, TextData textData, float layoutWidth, Layout.Alignment alignment, TextPaint textPaint, float spacMult, float spacAdd, boolean singleLine, ParaInfo mPara, int vertical)
    pub fn createTextLayoutFor4_1(text: &str, textData: &TextData, layoutWidth: f32, alignment: i32, textPaint: &TextPaint, spacMult: f32, spacAdd: f32, singleLine: bool, mPara: &ParaInfo, vertical: i32) -> Layout {
      // Java: try
      {
         // Java: return createTextLayout(text, layoutWidth, alignment, textPaint, spacMult, spacAdd, textData.mPara.mDefTabSize, singleLine, null != textData.mBidi, mPara, vertical);
         return Self::createTextLayout(text, layoutWidth, alignment, textPaint, spacMult, spacAdd, textData.mPara.mDefTabSize, singleLine, textData.mBidi.is_some(), mPara, vertical);
      }
      // Java: catch (Throwable var11)
      // {
         // Java: return null;
      // }
    }

    // Java: CharSequence createSpannable(TextData data, int firstLead, int restLead)
    pub fn createSpannable(&self, data: &mut TextData, firstLead: i32, restLead: i32) -> String {
      // Java: SpannableStringBuilder mSpanBuilder = new SpannableStringBuilder(data.mTypoText);
      let mut mSpanBuilder = SpannableStringBuilder::new(&data.mTypoText);
      // Java: this.setSpan(mSpanBuilder, data, firstLead, restLead);
      self.setSpan(&mut mSpanBuilder, data, firstLead, restLead);
      // Java: this.convertArb(data, mSpanBuilder);
      self.convertArb(data, &mut mSpanBuilder);
      // Java: return mSpanBuilder;
      mSpanBuilder.text
    }

    // Java: void convertArb(TextData data, SpannableStringBuilder mSpanBuilder)
    pub fn convertArb(&self, data: &TextData, mSpanBuilder: &mut SpannableStringBuilder) {
      // Java: if (data.mArb && (new Bidi(mSpanBuilder.toString(), -2)).isLeftToRight())
      if data.mArb
      {
         // Java: int textLen = mSpanBuilder.length();
         let textLen = mSpanBuilder.length();
         // Java: boolean[] embed = new boolean[textLen];
         let mut embed: Vec<bool> = vec![false; textLen as usize];
         // Java: int start = -1;
         let mut start: i32 = -1;
         // Java: int end = -1;
         let mut end: i32 = -1;
         // Java: boolean needBidi = false;
         let mut needBidi = false;
         // Java: for(SpanInfo span = data.getFirstSpan(); span != null && span.mEndPos <= textLen; span = data.getNextSpan())
         let mut span_opt = data.getFirstSpan();
         while let Some(span) = span_opt {
             if span.mEndPos > textLen { break; }
         {
            // Java: needBidi |= span.mArb;
            needBidi |= span.mArb;
            // Java: if (span.mArb)
            if span.mArb
            {
               // Java: if (start == -1)
               if start == -1
               {
                  // Java: start = span.mStartPos;
                  start = span.mStartPos;
                  // Java: end = span.mEndPos;
                  end = span.mEndPos;
               }
               // Java: else if (span.mStartPos == end)
               else if span.mStartPos == end
               {
                  // Java: end = span.mEndPos;
                  end = span.mEndPos;
               }
               // Java: else
               else
               {
                  // Java: Arrays.fill(embed, start, end, true);
                  for i in start as usize..end as usize { embed[i] = true; }
                  // Java: start = span.mStartPos;
                  start = span.mStartPos;
                  // Java: end = span.mEndPos;
                  end = span.mEndPos;
               }
            }
            // Java: else if (start != -1)
            else if start != -1
            {
               // Java: Arrays.fill(embed, start, end, true);
               for i in start as usize..end as usize { embed[i] = true; }
               // Java: start = -1;
               start = -1;
               // Java: end = -1;
               end = -1;
            }
         }
             span_opt = data.getNextSpan();
         }
         // Java: if (start != -1)
         if start != -1
         {
            // Java: Arrays.fill(embed, start, end, true);
            for i in start as usize..end as usize { embed[i] = true; }
         }
         // Java: if (needBidi)
         if needBidi
         {
            // Java: BidiHelper.updateText(mSpanBuilder, 0, mSpanBuilder.length(), embed);
            BidiHelper::updateText(mSpanBuilder, 0, mSpanBuilder.length(), &embed);
         }
      }
    }

    // Java: boolean setSpan(SpannableStringBuilder spannable, TextData data, int firstLead, int restLead)
    pub fn setSpan(&self, spannable: &mut SpannableStringBuilder, data: &mut TextData, firstLead: i32, restLead: i32) -> bool {
      // Java: if (!data.mIsRichText)
      if !data.mIsRichText
      {
         // Java: return false;
         return false;
      }
      // Java: else
      else
      {
         // Java: CharacterStyle charStyle = null;
         // Java: ParagraphStyle paraStyle = null;
         // Java: int flag = 34;
         let flag: i32 = 34;
         // Java: int start = 0;
         let mut start: i32 = 0;
         // Java: int end = 0;
         let mut end: i32 = 0;
         // Java: int textLen = spannable.length();
         let textLen = spannable.length();
         // Java: data.mIsTopBottomFont = false;
         data.mIsTopBottomFont = false;
         // Java: for(SpanInfo span = data.getFirstSpan(); span != null && span.mEndPos <= textLen; span = data.getNextSpan())
         let mut span_opt = data.getFirstSpan().cloned();
         while let Some(ref mut span) = span_opt {
             if span.mEndPos > textLen { break; }
         {
            // Java: start = span.mStartPos;
            start = span.mStartPos;
            // Java: end = span.mEndPos;
            end = span.mEndPos;
            // Java: if (span.hasSpc())
            if span.hasSpc()
            {
               // Java: CharacterStyle var20 = new SpacingSpan(span.getSpc());
               // Java: spannable.setSpan(var20, start, end, flag);
               spannable.setSpan(0, start, end, flag);
            }
            // Java: if (span.mIsSubScript)
            if span.mIsSubScript
            {
               // Java: span.mFontSize *= 0.64F;
               span.mFontSize *= 0.64;
               // Java: span.resetAscent();
               span.resetAscent();
            }
            // Java: else if (span.mIsSuperScript)
            else if span.mIsSuperScript
            {
               // Java: span.mFontSize *= 0.7F;
               span.mFontSize *= 0.7;
               // Java: span.resetAscent();
               span.resetAscent();
            }
            // Java: int fontsize = (int)span.mFontSize;
            let fontsize = span.mFontSize as i32;
            // Java: switch (span.mTextCaps)
            match span.mTextCaps
            {
               // Java: case 0:
               0 => {
                  // Java: spannable.replace(start, end, spannable.subSequence(start, end).toString().toUpperCase());
                  let upper = spannable.subSequence(start, end).to_uppercase();
                  spannable.replace(start, end, &upper);
                  // Java: CharacterStyle var24 = new AbsoluteSizeSpan(fontsize);
                  // Java: spannable.setSpan(var24, start, end, flag);
                  spannable.setSpan(0, start, end, flag);
                  // Java: break;
               }
               // Java: case 1:
               1 => {
                  // Java: CharacterStyle var23 = new AbsoluteSizeSpan(fontsize);
                  // Java: spannable.setSpan(var23, start, end, flag);
                  spannable.setSpan(0, start, end, flag);
                  // Java: break;
               }
               // Java: case 2:
               2 => {
                  // Java: CharSequence subText = spannable.subSequence(start, end);
                  let subText = spannable.subSequence(start, end);
                  // Java: int subLen = subText.length();
                  let subLen = subText.len() as i32;
                  // Java: spannable.replace(start, end, subText.toString().toUpperCase());
                  let upper = subText.to_uppercase();
                  spannable.replace(start, end, &upper);
                  // Java: char c = subText.charAt(0);
                  let chars_sub: Vec<char> = subText.chars().collect();
                  let mut c = chars_sub[0];
                  // Java: boolean befIsLowercase = c >= 'a' && c <= 'z';
                  let mut befIsLowercase = c >= 'a' && c <= 'z';
                  // Java: int befInx = 0;
                  let mut befInx: i32 = 0;
                  // Java: int curInx;
                  // Java: for(curInx = 1; curInx < subLen; ++curInx)
                  let mut curInx: i32 = 1;
                  while curInx < subLen
                  {
                     // Java: c = subText.charAt(curInx);
                     c = chars_sub[curInx as usize];
                     // Java: boolean curIsLowercase = c >= 'a' && c <= 'z';
                     let curIsLowercase = c >= 'a' && c <= 'z';
                     // Java: if (curIsLowercase != befIsLowercase)
                     if curIsLowercase != befIsLowercase
                     {
                        // Java: AbsoluteSizeSpan var21;
                        let size;
                        // Java: if (befIsLowercase)
                        if befIsLowercase
                        {
                           // Java: var21 = new AbsoluteSizeSpan((int)((float)fontsize * 0.8F));
                           size = (fontsize as f32 * 0.8) as i32;
                        }
                        // Java: else
                        else
                        {
                           // Java: var21 = new AbsoluteSizeSpan(fontsize);
                           size = fontsize;
                        }
                        // Java: spannable.setSpan(var21, start + befInx, start + curInx, flag);
                        spannable.setSpan(size, start + befInx, start + curInx, flag);
                        // Java: befInx = curInx;
                        befInx = curInx;
                        // Java: befIsLowercase = curIsLowercase;
                        befIsLowercase = curIsLowercase;
                     }
                     curInx += 1;
                  }
                  // Java: if (befInx < curInx)
                  if befInx < curInx
                  {
                     // Java: AbsoluteSizeSpan var22;
                     let size;
                     // Java: if (befIsLowercase)
                     if befIsLowercase
                     {
                        // Java: var22 = new AbsoluteSizeSpan((int)((float)fontsize * 0.8F));
                        size = (fontsize as f32 * 0.8) as i32;
                     }
                     // Java: else
                     else
                     {
                        // Java: var22 = new AbsoluteSizeSpan(fontsize);
                        size = fontsize;
                     }
                     // Java: spannable.setSpan(var22, start + befInx, start + curInx, flag);
                     spannable.setSpan(size, start + befInx, start + curInx, flag);
                  }
               }
               _ => {}
            }
            // Java: boolean isTopBottomFont = TypoLayoutGen.FontLineSpScale.TOP_BOTTOM_FONT.indexOf(span.mFontName) >= 0;
            let isTopBottomFont = get_top_bottom_font().contains(&span.mFontName.as_str());
            // Java: data.mIsTopBottomFont = data.mIsTopBottomFont || isTopBottomFont;
            data.mIsTopBottomFont = data.mIsTopBottomFont || isTopBottomFont;
            // Java: ITypeface iTf = PaintFontHelper.getITypeface(span.mFontName, span.mIsBold, span.mIsItalic);
            // Java: Typeface tf = (Typeface)iTf.getNativeTypeface();
            // Java: CharacterStyle var25 = new WpsTypefaceSpan("", iTf.getFamily().getName(), span.mIsBold, span.mIsItalic, tf, isTopBottomFont);
            // Java: spannable.setSpan(var25, start, end, flag);
            spannable.setSpan(0, start, end, flag);
            // Java: int faceStyle = getFaceStyle(span);
            let mut faceStyle = Self::getFaceStyle(span);
            // Java: if (tf.isItalic())
            {
               // Java: faceStyle &= -3;
               faceStyle &= -3;
            }
            // Java: if (faceStyle != 0)
            if faceStyle != 0
            {
               // Java: CharacterStyle var26 = new WpsStyleSpan(faceStyle);
               // Java: spannable.setSpan(var26, start, end, flag);
               spannable.setSpan(0, start, end, flag);
            }
            // Java: CharacterStyle var27 = new ForegroundColorSpan(span.mFontColor);
            // Java: spannable.setSpan(var27, start, end, flag);
            spannable.setSpan(span.mFontColor, start, end, flag);
            // Java: if (span.mIsSuperScript)
            if span.mIsSuperScript
            {
               // Java: CharacterStyle var28 = new SuperscriptSpan();
               // Java: spannable.setSpan(var28, start, end, flag);
               spannable.setSpan(0, start, end, flag);
            }
            // Java: if (span.mIsSubScript)
            if span.mIsSubScript
            {
               // Java: CharacterStyle var29 = new SubscriptSpan();
               // Java: spannable.setSpan(var29, start, end, flag);
               spannable.setSpan(0, start, end, flag);
            }
            // Java: if (4 == data.mVertical)
            if 4 == data.mVertical
            {
               // Java: this.setChineseVertSpan(spannable, start, end, (float)fontsize, span.mFontColor, tf, faceStyle, span.mIsSuperScript, span.mIsSubScript);
               self.setChineseVertSpan(spannable, start, end, fontsize as f32, span.mFontColor, faceStyle, span.mIsSuperScript, span.mIsSubScript);
            }
            // Java: else if (3 != data.mVertical && 6 == data.mVertical)
            else if 3 != data.mVertical && 6 == data.mVertical
            {
            }
         }
             span_opt = None;
         }
         // Java: if (null != data.mZeroWidthSpan && spannable.length() > 0)
         if data.mZeroWidthSpan.is_some() && spannable.length() > 0
         {
            // Java: start = data.mZeroWidthSpan.start;
            start = data.mZeroWidthSpan.as_ref().unwrap().start;
            // Java: end = data.mZeroWidthSpan.end;
            end = data.mZeroWidthSpan.as_ref().unwrap().end;
            // Java: if (end > spannable.length())
            if end > spannable.length()
            {
               // Java: end = spannable.length();
               end = spannable.length();
            }
            // Java: if (end > start)
            if end > start
            {
               // Java: CharacterStyle var30 = new ScaleXSpan(1.0E-5F);
               // Java: spannable.setSpan(var30, start, end, flag);
               spannable.setSpan(0, start, end, flag);
            }
         }
         // Java: if (data.mBidi != null)
         if data.mBidi.is_some()
         {
            // Java: ParagraphStyle var31 = new LeadingMarginSpan.Standard(firstLead, restLead);
            // Java: spannable.setSpan(var31, 0, spannable.length(), flag);
            spannable.setSpan(0, 0, spannable.length(), flag);
         }
         // Java: int[] positions = Thai.getWordPositions(spannable.toString().toCharArray());
         let positions = Thai::getWordPositions(&spannable.toString().chars().collect::<Vec<char>>());
         // Java: if (null != positions)
         if positions.is_some()
         {
            // Java: WordSplitSpan splitSpan = new WordSplitSpan();
            // Java: splitSpan.positions = positions;
            // Java: spannable.setSpan(splitSpan, 0, spannable.length(), flag);
            spannable.setSpan(0, 0, spannable.length(), flag);
         }
         // Java: return true;
         return true;
      }
    }

    // Java: static int getFaceStyle(SpanInfo span)
    pub fn getFaceStyle(span: &SpanInfo) -> i32 {
      // Java: int style = 0;
      let mut style: i32 = 0;
      // Java: if (span.mIsItalic)
      if span.mIsItalic
      {
         // Java: style = 2;
         style = 2;
      }
      // Java: if (span.mIsBold)
      if span.mIsBold
      {
         // Java: ++style;
         style += 1;
      }
      // Java: return style;
      style
    }

    // Java: static boolean isRightToLeftTypo(String cellValue)
    pub fn isRightToLeftTypo(cellValue: &str) -> bool {
      // Java: if (cellValue.length() <= 0)
      if cellValue.is_empty()
      {
         // Java: return false;
         return false;
      }
      // Java: else
      else
      {
         // Java: int i = 0;
         let mut i: usize = 0;
         // Java: for(int count = cellValue.length(); i < count; ++i)
         let chars: Vec<char> = cellValue.chars().collect();
         let count = chars.len();
         while i < count
         {
            // Java: char c = cellValue.charAt(i);
            let c = chars[i];
            // Java: if (c != ' ' && c != 160 && (c < '0' || c > '9'))
            if c != ' ' && c != '\u{00A0}' && (c < '0' || c > '9')
            {
               // Java: return c >= 1424 && c <= 1439 || c >= 1536 && c <= 1791 || c >= 1872 && c <= 1919;
               let cp = c as u32;
               return (cp >= 1424 && cp <= 1439) || (cp >= 1536 && cp <= 1791) || (cp >= 1872 && cp <= 1919);
            }
            i += 1;
         }
         // Java: return false;
         return false;
      }
    }

    // Java: boolean isCNBracket(char c)
    pub fn isCNBracket(&self, c: char) -> bool {
      // Java: int index = Arrays.binarySearch(BRACKETS, c);
      let index = get_brackets().binary_search(&c);
      // Java: return index >= 0;
      index.is_ok()
    }

    // Java: boolean isVertBracket(char c)
    pub fn isVertBracket(&self, c: char) -> bool {
      // Java: int index = Arrays.binarySearch(BRACKETS_VERT, c);
      let index = get_brackets_vert().binary_search(&c);
      // Java: return index >= 0;
      index.is_ok()
    }

    // Java: void setChineseVertSpan(Spannable text, int start, int end, float fontSize, int fontColor, Typeface typeface, int fontStyle, boolean isSuperScript, boolean isSubScript)
    pub fn setChineseVertSpan(&self, text: &mut SpannableStringBuilder, start: i32, mut end: i32, fontSize: f32, fontColor: i32, fontStyle: i32, isSuperScript: bool, isSubScript: bool) {
      // Java: if (end > text.length())
      if end > text.length()
      {
         // Java: end = text.length();
         end = text.length();
      }
      // Java: int cs = end;
      let mut cs = end;
      // Java: int flags = 34;
      let _flags: i32 = 34;
      // Java: int i;
      // Java: for(i = start; i < end; ++i)
      let mut i = start;
      let text_chars: Vec<char> = text.text.chars().collect();
      while i < end
      {
         // Java: if ((!TextData.isChinese(text.charAt(i)) || this.isCNBracket(text.charAt(i))) && !this.isVertBracket(text.charAt(i)))
         let ch = text_chars[i as usize];
         if (!TextData::isChinese(ch) || self.isCNBracket(ch)) && !self.isVertBracket(ch)
         {
            // Java: if (Emoji.isEmojiChar(text.charAt(i)))
            if Emoji::isEmojiChar(ch)
            {
               // Java: int emoji = Character.codePointAt(text, i);
               let _emoji = ch as u32;
               // Java: int count = Character.charCount(emoji);
               let count = 1i32;
               // Java: if (end == cs)
               if end == cs
               {
                  // Java: cs = i;
                  cs = i;
               }
               // Java: i += count - 1;
               i += count - 1;
            }
            // Java: else if (i > cs)
            else if i > cs
            {
               // Java: ChineseVertSpan span = new ChineseVertSpan(cs, i, fontSize, fontColor, typeface, fontStyle, isSuperScript, isSubScript);
               // Java: text.setSpan(span, cs, i, 34);
               text.setSpan(0, cs, i, 34);
               // Java: CharacterStyle colorStyle = WpsHideSpan.Factory.create();
               // Java: text.setSpan(colorStyle, cs, i, 34);
               text.setSpan(0, cs, i, 34);
               // Java: cs = end;
               cs = end;
            }
         }
         // Java: else if (end == cs)
         else if end == cs
         {
            // Java: cs = i;
            cs = i;
         }
         i += 1;
      }
      // Java: if (i > cs)
      if i > cs
      {
         // Java: ChineseVertSpan span = new ChineseVertSpan(cs, i, fontSize, fontColor, typeface, fontStyle, isSuperScript, isSubScript);
         // Java: text.setSpan(span, cs, i, 34);
         text.setSpan(0, cs, i, 34);
         // Java: CharacterStyle colorStyle = WpsHideSpan.Factory.create();
         // Java: text.setSpan(colorStyle, cs, i, 34);
         text.setSpan(0, cs, i, 34);
      }
    }

}

// Java: public class FontLineSpScale
impl FontLineSpScale {
    // Java: FontLineSpScale()
    pub fn new() -> Self {
         // Java: super();
         FontLineSpScale {}
    }

    // Java: static float getLineSpacePctScale(TextData data)
    pub fn getLineSpacePctScale(data: &TextData) -> f32 {
         // Java: float scale = 1.2F;
         let mut scale: f32 = 1.2;
         // Java: Float val = null;
         let mut val: Option<f32> = None;
         // Java: SpanInfo span = data.getFirstSpan();
         let mut span_opt = data.getFirstSpan();
         // Java: boolean hasCalibri = false;
         let mut hasCalibri = false;
         // Java: boolean hasNoneCalibri = false;
         let mut hasNoneCalibri = false;
         // Java: boolean hasPmingLiu = false;
         let mut hasPmingLiu = false;
         // Java: for(IFontTable fontTable = FontHost.getFontTable(); null != span; span = data.getNextSpan())
         let fontTable = FontHost::getFontTable();
         while let Some(span) = span_opt
         {
            // Java: if (fontTable.get(span.mFontName) == null)
            if fontTable.get(&span.mFontName).is_none()
            {
               // Java: scale = DEFAULT_LINE_SCALE;
               scale = get_default_line_scale();
            }
            // Java: else
            else
            {
               // Java: if (span.mFontName.equals("Calibri"))
               if span.mFontName == "Calibri"
               {
                  // Java: hasCalibri = true;
                  hasCalibri = true;
               }
               // Java: else
               else
               {
                  // Java: hasNoneCalibri = true;
                  hasNoneCalibri = true;
               }
               // Java: hasPmingLiu = span.mFontName.equals("新細明體");
               hasPmingLiu = span.mFontName == PMINGLIU_NAME;
               // Java: if (hasCalibri && hasNoneCalibri)
               if hasCalibri && hasNoneCalibri
               {
                  // Java: val = 1.08F;
                  val = Some(1.08);
               }
               // Java: else
               else
               {
                  // Java: val = (Float)FONT_TO_LINESPACE_PCT_SCALE.get(span.mFontName);
                  val = get_font_scale_map().get(span.mFontName.as_str()).copied();
               }
               // Java: if (null != val && val < scale && !isPunctuationSpan(data, span))
               if let Some(v) = val {
                   if v < scale && !Self::isPunctuationSpan(data, span)
                   {
                       // Java: scale = val;
                       scale = v;
                   }
               }
            }
            span_opt = data.getNextSpan();
         }
         // Java: if (hasPmingLiu && 0 == Float.compare(scale, 1.08F))
         if hasPmingLiu && scale == 1.08
         {
            // Java: scale = 1.168F;
            scale = 1.168;
         }
         // Java: return scale;
         scale
    }

    // Java: static boolean isPunctuationSpan(TextData data, SpanInfo span)
    pub fn isPunctuationSpan(data: &TextData, span: &SpanInfo) -> bool {
         // Java: String text = data.mSourceText;
         let text = &data.mSourceText;
         // Java: int length = text.length();
         let length = text.len() as i32;
         // Java: for(int i = span.mStartPos; i < span.mEndPos; ++i)
         let chars: Vec<char> = text.chars().collect();
         let mut i = span.mStartPos;
         while i < span.mEndPos
         {
            // Java: if (i >= length || " ,?;'\"".indexOf(text.charAt(i)) <= -1)
            if i >= length || !PUNCTUATIONS.contains(chars[i as usize])
            {
               // Java: return false;
               return false;
            }
            i += 1;
         }
         // Java: return true;
         true
    }

}
