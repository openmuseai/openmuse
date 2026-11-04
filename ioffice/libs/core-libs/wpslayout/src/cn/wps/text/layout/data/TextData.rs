// Java: import cn.wps.text.layout.pool.ObjectPool
// Java: import java.lang.Character.UnicodeBlock
// Java: import java.text.Bidi
// Java: import java.util.ArrayList
use super::ParaInfo::ParaInfo;
use super::SpanInfo::SpanInfo;

pub struct Bidi;

pub struct DataObjectPool;
impl DataObjectPool {
    pub fn borrow_span_info() -> SpanInfo { SpanInfo::new() }
    pub fn return_span_info(_span: SpanInfo) {}
}

// Java field: String TEXTBODY_LINE_BREAK_STR
pub const TEXTBODY_LINE_BREAK_STR: &str = "\u{000b}";
// Java field: float ZERO_WIDTH_SCALE
pub const ZERO_WIDTH_SCALE: f32 = 1.0E-5;

// Java: public class TextData
pub struct TextData {
    // Java field: boolean mArb
    pub mArb: bool,
    // Java field: ZeroWidthSpan mZeroWidthSpan
    pub mZeroWidthSpan: Option<ZeroWidthSpan>,
    // Java field: String mSourceText
    pub mSourceText: Option<String>,
    // Java field: String mTypoText
    pub mTypoText: Option<String>,
    // Java field: CharSequence mTypedCharSequence
    pub mTypedCharSequence: Option<String>,
    // Java field: int[] mReplacePositions
    pub mReplacePositions: Option<Vec<i32>>,
    // Java field: ParaInfo mPara
    pub mPara: ParaInfo,
    // Java field: SpanInfo mFirstSpan
    pub mFirstSpan: SpanInfo,
    // Java field: ArrayList<SpanInfo> mOtherSpans
    pub mOtherSpans: Vec<SpanInfo>,
    // Java field: boolean mIsRichText
    pub mIsRichText: bool,
    // Java field: boolean mIsTopBottomFont
    pub mIsTopBottomFont: bool,
    // Java field: Bidi mBidi
    pub mBidi: Option<Bidi>,
    // Java field: boolean requiresBidi
    pub requiresBidi: bool,
    // Java field: int mVertical
    pub mVertical: i32,
    // Java field: int mSpanIndex
    pub(crate) mSpanIndex: i32,
    // Java field: boolean mIsSoftReturn
    pub(crate) mIsSoftReturn: bool,
}

// Java: public class ZeroWidthSpan
pub struct ZeroWidthSpan {
    // Java field: int start
    pub start: i32,
    // Java field: int end
    pub end: i32,
}

// Java: public class TextData
impl TextData {
    // Java: TextData()
    pub fn new() -> Self {
      // Java: super();
      TextData {
        mArb: false,
        mZeroWidthSpan: None,
        mSourceText: None,
        mTypoText: None,
        mTypedCharSequence: None,
        mReplacePositions: None,
        mPara: ParaInfo::new(),
        mFirstSpan: SpanInfo::new(),
        mOtherSpans: Vec::new(),
        mIsRichText: false,
        mIsTopBottomFont: false,
        mBidi: None,
        requiresBidi: false,
        mVertical: 0,
        mSpanIndex: -1,
        mIsSoftReturn: false,
      }
    }

    // Java: TextData(TextData data)
    pub fn new_from(data: &TextData) -> Self {
      // Java: super();
      TextData {
        // Java: this.mSourceText = data.mSourceText;
        mSourceText: data.mSourceText.clone(),
        // Java: this.mTypoText = data.mTypoText;
        mTypoText: data.mTypoText.clone(),
        // Java: this.mPara = data.mPara;
        mPara: ParaInfo::new_from(&data.mPara),
        // Java: this.mFirstSpan = data.mFirstSpan;
        mFirstSpan: SpanInfo::new_from(&data.mFirstSpan),
        // Java: this.mOtherSpans = data.mOtherSpans;
        mOtherSpans: data.mOtherSpans.clone(),
        // Java: this.mIsRichText = data.mIsRichText;
        mIsRichText: data.mIsRichText,
        // Java: this.mVertical = data.mVertical;
        mVertical: data.mVertical,
        // Java: this.mIsSoftReturn = data.mIsSoftReturn;
        mIsSoftReturn: data.mIsSoftReturn,
        // Java: this.mIsTopBottomFont = data.mIsTopBottomFont;
        mIsTopBottomFont: data.mIsTopBottomFont,
        // Java: this.mArb = data.mArb;
        mArb: data.mArb,
        mZeroWidthSpan: None,
        mTypedCharSequence: None,
        mReplacePositions: None,
        mBidi: None,
        requiresBidi: false,
        mSpanIndex: -1,
      }
    }

    // Java: TextData(String sourceText, ParaInfo para, SpanInfo firstSpan, ArrayList<SpanInfo> otherSpans, boolean isRichText)
    pub fn new_with(sourceText: String, para: ParaInfo, firstSpan: SpanInfo, otherSpans: Vec<SpanInfo>, isRichText: bool) -> Self {
      // Java: super();
      TextData {
        // Java: this.mSourceText = sourceText;
        mSourceText: Some(sourceText),
        // Java: this.mPara = para;
        mPara: para,
        // Java: this.mFirstSpan = firstSpan;
        mFirstSpan: firstSpan,
        // Java: this.mOtherSpans = otherSpans;
        mOtherSpans: otherSpans,
        // Java: this.mIsRichText = isRichText;
        mIsRichText: isRichText,
        mArb: false,
        mZeroWidthSpan: None,
        mTypoText: None,
        mTypedCharSequence: None,
        mReplacePositions: None,
        mIsTopBottomFont: false,
        mBidi: None,
        requiresBidi: false,
        mVertical: 0,
        mSpanIndex: -1,
        mIsSoftReturn: false,
      }
    }

    // Java: float italicFontSize()
    pub fn italicFontSize(&self) -> f32 {
      // Java: float fontSize = this.mFirstSpan.mIsItalic ? this.mFirstSpan.mFontSize : 0.0F;
      let mut fontSize = if self.mFirstSpan.mIsItalic { self.mFirstSpan.mFontSize } else { 0.0 };
      // Java: int i = 0;
      // Java: for(int size = this.mOtherSpans.size(); i < size; ++i)
      for i in 0..self.mOtherSpans.len() {
         // Java: SpanInfo span = (SpanInfo)this.mOtherSpans.get(i);
         let span = &self.mOtherSpans[i];
         // Java: if (span.mIsItalic && fontSize < span.mFontSize)
         if span.mIsItalic && fontSize < span.mFontSize {
            // Java: fontSize = span.mFontSize;
            fontSize = span.mFontSize;
         }
      }
      // Java: return fontSize;
      fontSize
    }

    // Java: float minFontSize()
    pub fn minFontSize(&self) -> f32 {
      // Java: float min = this.mFirstSpan.mFontSize;
      let mut min = self.mFirstSpan.mFontSize;
      // Java: int i = 0;
      // Java: for(int size = this.mOtherSpans.size(); i < size; ++i)
      for i in 0..self.mOtherSpans.len() {
         // Java: SpanInfo span = (SpanInfo)this.mOtherSpans.get(i);
         let span = &self.mOtherSpans[i];
         // Java: if (min > span.mFontSize)
         if min > span.mFontSize {
            // Java: min = span.mFontSize;
            min = span.mFontSize;
         }
      }
      // Java: return min;
      min
    }

    // Java: void concat(TextData textdata)
    pub fn concat(&mut self, textdata: &mut TextData) {
      // Java: int oldLength = this.mSourceText.length();
      let oldLength = self.mSourceText.as_ref().map_or(0, |s| s.len()) as i32;
      // Java: if (this.mSourceText.equals(""))
      if self.mSourceText.as_ref().map_or(true, |s| s.is_empty()) {
         // Java: this.mFirstSpan = textdata.mFirstSpan;
         self.mFirstSpan = SpanInfo::new_from(&textdata.mFirstSpan);
      }
      // Java: else
      else {
         // Java: SpanInfo var10000 = textdata.mFirstSpan;
         // Java: var10000.mStartPos += oldLength;
         textdata.mFirstSpan.mStartPos += oldLength;
         // Java: var10000 = textdata.mFirstSpan;
         // Java: var10000.mEndPos += oldLength;
         textdata.mFirstSpan.mEndPos += oldLength;
         // Java: textdata.mFirstSpan.adjustPos();
         textdata.mFirstSpan.adjustPos();
         // Java: this.mOtherSpans.add(textdata.mFirstSpan);
         self.mOtherSpans.push(SpanInfo::new_from(&textdata.mFirstSpan));
      }
      // Java: this.mSourceText = this.mSourceText.concat(textdata.mSourceText);
      if let Some(ref mut s) = self.mSourceText {
          if let Some(ref other) = textdata.mSourceText {
              s.push_str(other);
          }
      }
      // Java: if (textdata.mOtherSpans.size() > 0)
      if textdata.mOtherSpans.len() > 0 {
         // Java: for(int i = 0; i < textdata.mOtherSpans.size(); ++i)
         for i in 0..textdata.mOtherSpans.len() {
            // Java: SpanInfo var5 = (SpanInfo)textdata.mOtherSpans.get(i);
            // Java: var5.mStartPos += oldLength;
            textdata.mOtherSpans[i].mStartPos += oldLength;
            // Java: var5 = (SpanInfo)textdata.mOtherSpans.get(i);
            // Java: var5.mEndPos += oldLength;
            textdata.mOtherSpans[i].mEndPos += oldLength;
            // Java: ((SpanInfo)textdata.mOtherSpans.get(i)).adjustPos();
            textdata.mOtherSpans[i].adjustPos();
            // Java: this.mOtherSpans.add((SpanInfo)textdata.mOtherSpans.get(i));
            self.mOtherSpans.push(SpanInfo::new_from(&textdata.mOtherSpans[i]));
         }
      }
    }

    // Java: boolean hasArtFill()
    pub fn hasArtFill(&self) -> bool {
      // Java: if (this.mFirstSpan != null && this.mFirstSpan.hasArtFill())
      if self.mFirstSpan.hasArtFill() {
         // Java: return true;
         return true;
      }
      // Java: else
      else {
         // Java: if (this.mOtherSpans != null && this.mOtherSpans.size() > 0)
         if self.mOtherSpans.len() > 0 {
            // Java: for(int i = 0; i < this.mOtherSpans.size(); ++i)
            for i in 0..self.mOtherSpans.len() {
               // Java: SpanInfo span = (SpanInfo)this.mOtherSpans.get(i);
               let span = &self.mOtherSpans[i];
               // Java: if (span.hasArtFill())
               if span.hasArtFill() {
                  // Java: return true;
                  return true;
               }
            }
         }
         // Java: return false;
         return false;
      }
    }

    // Java: boolean hasFill()
    pub fn hasFill(&self) -> bool {
      // Java: if (this.mFirstSpan != null && this.mFirstSpan.hasFill())
      if self.mFirstSpan.hasFill() {
         // Java: return true;
         return true;
      }
      // Java: else
      else {
         // Java: if (this.mOtherSpans != null && this.mOtherSpans.size() > 0)
         if self.mOtherSpans.len() > 0 {
            // Java: for(int i = 0; i < this.mOtherSpans.size(); ++i)
            for i in 0..self.mOtherSpans.len() {
               // Java: SpanInfo span = (SpanInfo)this.mOtherSpans.get(i);
               let span = &self.mOtherSpans[i];
               // Java: if (span.hasFill())
               if span.hasFill() {
                  // Java: return true;
                  return true;
               }
            }
         }
         // Java: return false;
         return false;
      }
    }

    // Java: boolean hasOutline()
    pub fn hasOutline(&self) -> bool {
      // Java: if (this.mFirstSpan != null && this.mFirstSpan.hasOutline())
      if self.mFirstSpan.hasOutline() {
         // Java: return true;
         return true;
      }
      // Java: else
      else {
         // Java: if (this.mOtherSpans != null && this.mOtherSpans.size() > 0)
         if self.mOtherSpans.len() > 0 {
            // Java: for(int i = 0; i < this.mOtherSpans.size(); ++i)
            for i in 0..self.mOtherSpans.len() {
               // Java: SpanInfo span = (SpanInfo)this.mOtherSpans.get(i);
               let span = &self.mOtherSpans[i];
               // Java: if (span.hasOutline())
               if span.hasOutline() {
                  // Java: return true;
                  return true;
               }
            }
         }
         // Java: return false;
         return false;
      }
    }

    // Java: boolean hasBlur()
    pub fn hasBlur(&self) -> bool {
      // Java: if (this.mFirstSpan != null && this.mFirstSpan.hasBlur())
      if self.mFirstSpan.hasBlur() {
         // Java: return true;
         return true;
      }
      // Java: else
      else {
         // Java: if (this.mOtherSpans != null && this.mOtherSpans.size() > 0)
         if self.mOtherSpans.len() > 0 {
            // Java: for(int i = 0; i < this.mOtherSpans.size(); ++i)
            for i in 0..self.mOtherSpans.len() {
               // Java: SpanInfo span = (SpanInfo)this.mOtherSpans.get(i);
               let span = &self.mOtherSpans[i];
               // Java: if (span.hasBlur())
               if span.hasBlur() {
                  // Java: return true;
                  return true;
               }
            }
         }
         // Java: return false;
         return false;
      }
    }

    // Java: boolean hasTextImage()
    pub fn hasTextImage(&self) -> bool {
      // Java: if (this.mFirstSpan != null && this.mFirstSpan.hasTextImage())
      if self.mFirstSpan.hasTextImage() {
         // Java: return true;
         return true;
      }
      // Java: else
      else {
         // Java: if (this.mOtherSpans != null && this.mOtherSpans.size() > 0)
         if self.mOtherSpans.len() > 0 {
            // Java: for(int i = 0; i < this.mOtherSpans.size(); ++i)
            for i in 0..self.mOtherSpans.len() {
               // Java: SpanInfo span = (SpanInfo)this.mOtherSpans.get(i);
               let span = &self.mOtherSpans[i];
               // Java: if (span.hasTextImage())
               if span.hasTextImage() {
                  // Java: return true;
                  return true;
               }
            }
         }
         // Java: return false;
         return false;
      }
    }

    // Java: boolean hasOuterShdw()
    pub fn hasOuterShdw(&self) -> bool {
      // Java: if (this.mFirstSpan != null && this.mFirstSpan.hasOuterShdw())
      if self.mFirstSpan.hasOuterShdw() {
         // Java: return true;
         return true;
      }
      // Java: else
      else {
         // Java: if (this.mOtherSpans != null && this.mOtherSpans.size() > 0)
         if self.mOtherSpans.len() > 0 {
            // Java: for(int i = 0; i < this.mOtherSpans.size(); ++i)
            for i in 0..self.mOtherSpans.len() {
               // Java: SpanInfo span = (SpanInfo)this.mOtherSpans.get(i);
               let span = &self.mOtherSpans[i];
               // Java: if (span.hasOuterShdw())
               if span.hasOuterShdw() {
                  // Java: return true;
                  return true;
               }
            }
         }
         // Java: return false;
         return false;
      }
    }

    // Java: boolean hasInnerShdw()
    pub fn hasInnerShdw(&self) -> bool {
      // Java: if (this.mFirstSpan != null && this.mFirstSpan.hasInnerShdw())
      if self.mFirstSpan.hasInnerShdw() {
         // Java: return true;
         return true;
      }
      // Java: else
      else {
         // Java: if (this.mOtherSpans != null && this.mOtherSpans.size() > 0)
         if self.mOtherSpans.len() > 0 {
            // Java: for(int i = 0; i < this.mOtherSpans.size(); ++i)
            for i in 0..self.mOtherSpans.len() {
               // Java: SpanInfo span = (SpanInfo)this.mOtherSpans.get(i);
               let span = &self.mOtherSpans[i];
               // Java: if (span.hasInnerShdw())
               if span.hasInnerShdw() {
                  // Java: return true;
                  return true;
               }
            }
         }
         // Java: return false;
         return false;
      }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.mZeroWidthSpan = null;
      self.mZeroWidthSpan = None;
      // Java: this.mSourceText = null;
      self.mSourceText = None;
      // Java: this.mTypoText = null;
      self.mTypoText = None;
      // Java: this.mReplacePositions = null;
      self.mReplacePositions = None;
      // Java: this.mPara.clear();
      self.mPara.clear();
      // Java: ObjectPool<SpanInfo> pool = DataObjectPool.spanInfoObjectPool;
      // Java: for(int i = this.mOtherSpans.size() - 1; i > -1; --i)
      for i in (0..self.mOtherSpans.len()).rev() {
         // Java: pool.returnObject((SpanInfo)this.mOtherSpans.get(i));
         let span = self.mOtherSpans.remove(i);
         DataObjectPool::return_span_info(span);
      }
      // Java: this.mFirstSpan.clear();
      self.mFirstSpan.clear();
      // Java: this.mOtherSpans.clear();
      self.mOtherSpans.clear();
      // Java: this.mIsRichText = true;
      self.mIsRichText = true;
      // Java: this.mBidi = null;
      self.mBidi = None;
      // Java: this.mVertical = 0;
      self.mVertical = 0;
      // Java: this.mSpanIndex = -1;
      self.mSpanIndex = -1;
      // Java: this.mIsSoftReturn = false;
      self.mIsSoftReturn = false;
      // Java: this.mArb = false;
      self.mArb = false;
    }

    // Java: void copyFrom(TextData data)
    pub fn copyFrom(&mut self, data: &TextData) {
      // Java: this.mPara.copyFrom(data.mPara);
      self.mPara.copyFrom(&data.mPara);
      // Java: this.mFirstSpan.copyFrom(data.mFirstSpan);
      self.mFirstSpan.copyFrom(&data.mFirstSpan);
      // Java: int i = 0;
      // Java: for(int size = data.mOtherSpans.size(); i < size; ++i)
      for i in 0..data.mOtherSpans.len() {
         // Java: this.mOtherSpans.add(new SpanInfo((SpanInfo)data.mOtherSpans.get(i)));
         self.mOtherSpans.push(SpanInfo::new_from(&data.mOtherSpans[i]));
      }
      // Java: this.mBidi = data.mBidi;
      self.mBidi = None;
      // Java: this.mIsRichText = data.mIsRichText;
      self.mIsRichText = data.mIsRichText;
      // Java: this.mSourceText = data.mSourceText;
      self.mSourceText = data.mSourceText.clone();
      // Java: this.mTypoText = data.mTypoText;
      self.mTypoText = data.mTypoText.clone();
      // Java: this.mVertical = data.mVertical;
      self.mVertical = data.mVertical;
      // Java: this.mSpanIndex = data.mSpanIndex;
      self.mSpanIndex = data.mSpanIndex;
      // Java: this.mIsSoftReturn = data.mIsSoftReturn;
      self.mIsSoftReturn = data.mIsSoftReturn;
      // Java: this.mArb = data.mArb;
      self.mArb = data.mArb;
      // Java: if (data.mZeroWidthSpan != null)
      if let Some(ref zws) = data.mZeroWidthSpan {
         // Java: this.mZeroWidthSpan = new ZeroWidthSpan(data.mZeroWidthSpan.start, data.mZeroWidthSpan.end);
         self.mZeroWidthSpan = Some(ZeroWidthSpan { start: zws.start, end: zws.end });
      }
    }

    // Java: static boolean isChinese(char c)
    pub fn isChinese(c: char) -> bool {
      // Java: Character.UnicodeBlock ub = UnicodeBlock.of(c);
      let cp = c as u32;
      // Java: return ub == UnicodeBlock.CJK_UNIFIED_IDEOGRAPHS || ub == UnicodeBlock.CJK_COMPATIBILITY_IDEOGRAPHS || ub == UnicodeBlock.CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A || ub == UnicodeBlock.CJK_SYMBOLS_AND_PUNCTUATION || ub == UnicodeBlock.HALFWIDTH_AND_FULLWIDTH_FORMS || ub == UnicodeBlock.KATAKANA || ub == UnicodeBlock.HIRAGANA || ub == UnicodeBlock.ENCLOSED_ALPHANUMERICS;
      (0x4E00..=0x9FFF).contains(&cp) ||
      (0xF900..=0xFAFF).contains(&cp) ||
      (0x3400..=0x4DBF).contains(&cp) ||
      (0x3000..=0x303F).contains(&cp) ||
      (0xFF00..=0xFFEF).contains(&cp) ||
      (0x30A0..=0x30FF).contains(&cp) ||
      (0x3040..=0x309F).contains(&cp) ||
      (0x2460..=0x24FF).contains(&cp)
    }

    // Java: static boolean isEnclosedAlphanum(char c)
    pub fn isEnclosedAlphanum(c: char) -> bool {
      // Java: Character.UnicodeBlock ub = UnicodeBlock.of(c);
      let cp = c as u32;
      // Java: return ub == UnicodeBlock.ENCLOSED_ALPHANUMERICS;
      (0x2460..=0x24FF).contains(&cp)
    }

    // Java: void insertSpanAft(SpanInfo span)
    pub fn insertSpanAft(&mut self, span: SpanInfo) {
      // Java: if (this.mSpanIndex > this.mOtherSpans.size())
      if self.mSpanIndex > self.mOtherSpans.len() as i32 {
         // Java: this.mSpanIndex = this.mOtherSpans.size();
         self.mSpanIndex = self.mOtherSpans.len() as i32;
         // Java: this.mOtherSpans.add(this.mSpanIndex, span);
         self.mOtherSpans.insert(self.mSpanIndex as usize, span);
      }
      // Java: else
      else {
         // Java: if (this.mSpanIndex < -1)
         if self.mSpanIndex < -1 {
            // Java: this.mSpanIndex = -1;
            self.mSpanIndex = -1;
         }
         // Java: this.mOtherSpans.add(this.mSpanIndex + 1, span);
         self.mOtherSpans.insert((self.mSpanIndex + 1) as usize, span);
      }
    }

    // Java: SpanInfo getCurrentSpan()
    pub fn getCurrentSpan(&self) -> Option<&SpanInfo> {
      // Java: SpanInfo span = null;
      // Java: if (this.mSpanIndex < 0)
      if self.mSpanIndex < 0 {
         // Java: span = this.mFirstSpan;
         Some(&self.mFirstSpan)
      }
      // Java: else if (this.mSpanIndex < this.mOtherSpans.size())
      else if (self.mSpanIndex as usize) < self.mOtherSpans.len() {
         // Java: span = (SpanInfo)this.mOtherSpans.get(this.mSpanIndex);
         Some(&self.mOtherSpans[self.mSpanIndex as usize])
      } else {
        // Java: return span;
        None
      }
    }

    // Java: SpanInfo getNextSpan()
    pub fn getNextSpan(&mut self) -> Option<&SpanInfo> {
      // Java: ++this.mSpanIndex;
      self.mSpanIndex += 1;
      // Java: return this.getCurrentSpan();
      if self.mSpanIndex < 0 {
         Some(&self.mFirstSpan)
      } else if (self.mSpanIndex as usize) < self.mOtherSpans.len() {
         Some(&self.mOtherSpans[self.mSpanIndex as usize])
      } else {
         None
      }
    }

    // Java: void removeCurrentSpan()
    pub fn removeCurrentSpan(&mut self) {
      // Java: if (this.mSpanIndex >= 0)
      if self.mSpanIndex >= 0 {
         // Java: if (this.mSpanIndex < this.mOtherSpans.size())
         if (self.mSpanIndex as usize) < self.mOtherSpans.len() {
            // Java: this.mOtherSpans.remove(this.mSpanIndex);
            self.mOtherSpans.remove(self.mSpanIndex as usize);
         }
         // Java: --this.mSpanIndex;
         self.mSpanIndex -= 1;
      }
    }

    // Java: SpanInfo getFirstSpan()
    pub fn getFirstSpan(&mut self) -> &SpanInfo {
      // Java: this.mSpanIndex = -1;
      self.mSpanIndex = -1;
      // Java: return this.mFirstSpan;
      &self.mFirstSpan
    }

    // Java: SpanInfo getLastSpan()
    pub fn getLastSpan(&mut self) -> &SpanInfo {
      // Java: if (this.mOtherSpans.isEmpty())
      if self.mOtherSpans.is_empty() {
         // Java: this.mSpanIndex = -1;
         self.mSpanIndex = -1;
         // Java: return this.mFirstSpan;
         &self.mFirstSpan
      }
      // Java: else
      else {
         // Java: this.mSpanIndex = this.mOtherSpans.size() - 1;
         self.mSpanIndex = self.mOtherSpans.len() as i32 - 1;
         // Java: return (SpanInfo)this.mOtherSpans.get(this.mSpanIndex);
         &self.mOtherSpans[self.mSpanIndex as usize]
      }
    }

    // Java: boolean isLastSpan()
    pub fn isLastSpan(&self) -> bool {
      // Java: return this.mSpanIndex == this.mOtherSpans.size() - 1;
      self.mSpanIndex == self.mOtherSpans.len() as i32 - 1
    }

    // Java: void resetSpanIndex()
    pub fn resetSpanIndex(&mut self) {
      // Java: this.mSpanIndex = -1;
      self.mSpanIndex = -1;
    }

    // Java: int getSpanIndex()
    pub fn getSpanIndex(&self) -> i32 {
      // Java: return this.mSpanIndex;
      self.mSpanIndex
    }

    // Java: boolean hasText()
    pub fn hasText(&self) -> bool {
      // Java: return null != this.mSourceText && 0 != this.mSourceText.length() && !this.mSourceText.equals("\u000b") && !this.mSourceText.equals("\r") && !this.mSourceText.equals("\n");
      match &self.mSourceText {
        Some(s) => !s.is_empty() && s != "\u{000b}" && s != "\r" && s != "\n",
        None => false,
      }
    }

    // Java: boolean hasBullets()
    pub fn hasBullets(&self) -> bool {
      // Java: return this.mPara.mbulletsInfo.isVisible() && this.mPara.mbulletsInfo.getBulletType() != -1;
      self.mPara.mbulletsInfo.isVisible() && self.mPara.mbulletsInfo.getBulletType() != -1
    }

    // Java: SpanInfo createSpan()
    pub fn createSpan(&self) -> SpanInfo {
      // Java: return DataObjectPool.spanInfoObjectPool.borrowObject();
      DataObjectPool::borrow_span_info()
    }

    // Java: SpanInfo getSpanByOffset(int offset)
    pub fn getSpanByOffset(&self, offset: i32) -> Option<&SpanInfo> {
      // Java: if (offset >= this.mFirstSpan.mStartPos && offset < this.mFirstSpan.mEndPos)
      if offset >= self.mFirstSpan.mStartPos && offset < self.mFirstSpan.mEndPos {
         // Java: return this.mFirstSpan;
         return Some(&self.mFirstSpan);
      }
      // Java: else
      else {
         // Java: for(int i = 0; i < this.mOtherSpans.size(); ++i)
         for i in 0..self.mOtherSpans.len() {
            // Java: SpanInfo span = (SpanInfo)this.mOtherSpans.get(i);
            let span = &self.mOtherSpans[i];
            // Java: if (offset >= span.mStartPos && offset < span.mEndPos)
            if offset >= span.mStartPos && offset < span.mEndPos {
               // Java: return span;
               return Some(span);
            }
         }
         // Java: return null;
         return None;
      }
    }

    // Java: void setSoftReturn(boolean isSoftReturn)
    pub fn setSoftReturn(&mut self, isSoftReturn: bool) {
      // Java: this.mIsSoftReturn = isSoftReturn;
      self.mIsSoftReturn = isSoftReturn;
    }

    // Java: boolean isSoftReturn()
    pub fn isSoftReturn(&self) -> bool {
      // Java: return this.mIsSoftReturn;
      self.mIsSoftReturn
    }

}

// Java: public class ZeroWidthSpan
impl ZeroWidthSpan {
    // Java: ZeroWidthSpan(int start, int end)
    pub fn new(start: i32, end: i32) -> Self {
         // Java: super();
         // Java: this.start = start;
         // Java: this.end = end;
         ZeroWidthSpan { start, end }
    }

}
