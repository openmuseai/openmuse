// Java: import cn.wps.text.layout.pool.ObjectPool
use crate::text::layout::pool::ObjectPool;
// Java: import java.lang.Character.UnicodeBlock
// TODO_RUST_CODE use ???; // java.lang.Character.UnicodeBlock
// Java: import java.text.Bidi
// TODO_RUST_CODE use ???; // java.text.Bidi
// Java: import java.util.ArrayList
// TODO_RUST_CODE use ???; // java.util.ArrayList

// Java field: String TEXTBODY_LINE_BREAK_STR
pub(crate) const TEXTBODY_LINE_BREAK_STR: String = /* TODO */;
// Java field: float ZERO_WIDTH_SCALE
pub(crate) const ZERO_WIDTH_SCALE: f32 = /* TODO */;

// Java: public class TextData
pub struct TextData {
    // Java field: boolean mArb
    pub(crate) mArb: bool,
    // Java field: ZeroWidthSpan mZeroWidthSpan
    pub(crate) mZeroWidthSpan: Arc<Mutex<ZeroWidthSpan>>,
    // Java field: String mSourceText
    pub(crate) mSourceText: Arc<Mutex<String>>,
    // Java field: String mTypoText
    pub(crate) mTypoText: Arc<Mutex<String>>,
    // Java field: CharSequence mTypedCharSequence
    pub(crate) mTypedCharSequence: Arc<Mutex<CharSequence>>,
    // Java field: int[] mReplacePositions
    pub(crate) mReplacePositions: Arc<Mutex<Vec<i32>>>,
    // Java field: ParaInfo mPara
    pub(crate) mPara: Arc<Mutex<ParaInfo>>,
    // Java field: SpanInfo mFirstSpan
    pub(crate) mFirstSpan: Arc<Mutex<SpanInfo>>,
    // Java field: ArrayList<SpanInfo> mOtherSpans
    pub(crate) mOtherSpans: Arc<Mutex<Vec<SpanInfo>>>,
    // Java field: boolean mIsRichText
    pub(crate) mIsRichText: bool,
    // Java field: boolean mIsTopBottomFont
    pub(crate) mIsTopBottomFont: bool,
    // Java field: Bidi mBidi
    pub(crate) mBidi: Arc<Mutex<Bidi>>,
    // Java field: boolean requiresBidi
    pub(crate) requiresBidi: bool,
    // Java field: int mVertical
    pub(crate) mVertical: i32,
    // Java field: int mSpanIndex
    pub(crate) mSpanIndex: i32,
    // Java field: boolean mIsSoftReturn
    pub(crate) mIsSoftReturn: bool,
}

// Java: public class ZeroWidthSpan
pub struct ZeroWidthSpan {
    // Java field: int start
    pub(crate) start: i32,
    // Java field: int end
    pub(crate) end: i32,
}

// Java: public class TextData
impl TextData {
    // Java: TextData()
    pub fn TextData(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: TextData(TextData data)
    pub fn TextData(&self, TextData data) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.mSourceText = data.mSourceText;
      // TODO_RUST_CODE
      // Java: this.mTypoText = data.mTypoText;
      // TODO_RUST_CODE
      // Java: this.mPara = data.mPara;
      // TODO_RUST_CODE
      // Java: this.mFirstSpan = data.mFirstSpan;
      // TODO_RUST_CODE
      // Java: this.mOtherSpans = data.mOtherSpans;
      // TODO_RUST_CODE
      // Java: this.mIsRichText = data.mIsRichText;
      // TODO_RUST_CODE
      // Java: this.mVertical = data.mVertical;
      // TODO_RUST_CODE
      // Java: this.mIsSoftReturn = data.mIsSoftReturn;
      // TODO_RUST_CODE
      // Java: this.mIsTopBottomFont = data.mIsTopBottomFont;
      // TODO_RUST_CODE
      // Java: this.mArb = data.mArb;
      // TODO_RUST_CODE
    }

    // Java: TextData(String sourceText, ParaInfo para, SpanInfo firstSpan, ArrayList<SpanInfo> otherSpans, boolean isRichText)
    pub fn TextData(&self, String sourceText, ParaInfo para, SpanInfo firstSpan, ArrayList<SpanInfo> otherSpans, boolean isRichText) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.mSourceText = sourceText;
      // TODO_RUST_CODE
      // Java: this.mPara = para;
      // TODO_RUST_CODE
      // Java: this.mFirstSpan = firstSpan;
      // TODO_RUST_CODE
      // Java: this.mOtherSpans = otherSpans;
      // TODO_RUST_CODE
      // Java: this.mIsRichText = isRichText;
      // TODO_RUST_CODE
    }

    // Java: float italicFontSize()
    pub fn italicFontSize(&self) -> f32 {
      // Java: float fontSize = this.mFirstSpan.mIsItalic ? this.mFirstSpan.mFontSize : 0.0F;
      // TODO_RUST_CODE
      // Java: int i = 0;
      // TODO_RUST_CODE
      // Java: for(int size = this.mOtherSpans.size(); i < size; ++i)
        // TODO_RUST_CODE
      {
         // Java: SpanInfo span = (SpanInfo)this.mOtherSpans.get(i);
         // TODO_RUST_CODE
         // Java: if (span.mIsItalic && fontSize < span.mFontSize)
            // TODO_RUST_CODE
         {
            // Java: fontSize = span.mFontSize;
            // TODO_RUST_CODE
         }
      }
      // Java: return fontSize;
      // TODO_RUST_CODE
    }

    // Java: float minFontSize()
    pub fn minFontSize(&self) -> f32 {
      // Java: float min = this.mFirstSpan.mFontSize;
      // TODO_RUST_CODE
      // Java: int i = 0;
      // TODO_RUST_CODE
      // Java: for(int size = this.mOtherSpans.size(); i < size; ++i)
        // TODO_RUST_CODE
      {
         // Java: SpanInfo span = (SpanInfo)this.mOtherSpans.get(i);
         // TODO_RUST_CODE
         // Java: if (min > span.mFontSize)
            // TODO_RUST_CODE
         {
            // Java: min = span.mFontSize;
            // TODO_RUST_CODE
         }
      }
      // Java: return min;
      // TODO_RUST_CODE
    }

    // Java: void concat(TextData textdata)
    pub fn concat(&self, TextData textdata) {
      // Java: int oldLength = this.mSourceText.length();
      // TODO_RUST_CODE
      // Java: if (this.mSourceText.equals(""))
        // TODO_RUST_CODE
      {
         // Java: this.mFirstSpan = textdata.mFirstSpan;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: SpanInfo var10000 = textdata.mFirstSpan;
         // TODO_RUST_CODE
         // Java: var10000.mStartPos += oldLength;
         // TODO_RUST_CODE
         // Java: var10000 = textdata.mFirstSpan;
         // TODO_RUST_CODE
         // Java: var10000.mEndPos += oldLength;
         // TODO_RUST_CODE
         // Java: textdata.mFirstSpan.adjustPos();
         // TODO_RUST_CODE
         // Java: this.mOtherSpans.add(textdata.mFirstSpan);
         // TODO_RUST_CODE
      }
      // Java: this.mSourceText = this.mSourceText.concat(textdata.mSourceText);
      // TODO_RUST_CODE
      // Java: if (textdata.mOtherSpans.size() > 0)
        // TODO_RUST_CODE
      {
         // Java: for(int i = 0; i < textdata.mOtherSpans.size(); ++i)
            // TODO_RUST_CODE
         {
            // Java: SpanInfo var5 = (SpanInfo)textdata.mOtherSpans.get(i);
            // TODO_RUST_CODE
            // Java: var5.mStartPos += oldLength;
            // TODO_RUST_CODE
            // Java: var5 = (SpanInfo)textdata.mOtherSpans.get(i);
            // TODO_RUST_CODE
            // Java: var5.mEndPos += oldLength;
            // TODO_RUST_CODE
            // Java: ((SpanInfo)textdata.mOtherSpans.get(i)).adjustPos();
            // TODO_RUST_CODE
            // Java: this.mOtherSpans.add((SpanInfo)textdata.mOtherSpans.get(i));
            // TODO_RUST_CODE
         }
      }
    }

    // Java: boolean hasArtFill()
    pub fn hasArtFill(&self) -> bool {
      // Java: if (this.mFirstSpan != null && this.mFirstSpan.hasArtFill())
        // TODO_RUST_CODE
      {
         // Java: return true;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: if (this.mOtherSpans != null && this.mOtherSpans.size() > 0)
            // TODO_RUST_CODE
         {
            // Java: for(int i = 0; i < this.mOtherSpans.size(); ++i)
                // TODO_RUST_CODE
            {
               // Java: SpanInfo span = (SpanInfo)this.mOtherSpans.get(i);
               // TODO_RUST_CODE
               // Java: if (span.hasArtFill())
                    // TODO_RUST_CODE
               {
                  // Java: return true;
                  // TODO_RUST_CODE
               }
            }
         }
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean hasFill()
    pub fn hasFill(&self) -> bool {
      // Java: if (this.mFirstSpan != null && this.mFirstSpan.hasFill())
        // TODO_RUST_CODE
      {
         // Java: return true;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: if (this.mOtherSpans != null && this.mOtherSpans.size() > 0)
            // TODO_RUST_CODE
         {
            // Java: for(int i = 0; i < this.mOtherSpans.size(); ++i)
                // TODO_RUST_CODE
            {
               // Java: SpanInfo span = (SpanInfo)this.mOtherSpans.get(i);
               // TODO_RUST_CODE
               // Java: if (span.hasFill())
                    // TODO_RUST_CODE
               {
                  // Java: return true;
                  // TODO_RUST_CODE
               }
            }
         }
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean hasOutline()
    pub fn hasOutline(&self) -> bool {
      // Java: if (this.mFirstSpan != null && this.mFirstSpan.hasOutline())
        // TODO_RUST_CODE
      {
         // Java: return true;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: if (this.mOtherSpans != null && this.mOtherSpans.size() > 0)
            // TODO_RUST_CODE
         {
            // Java: for(int i = 0; i < this.mOtherSpans.size(); ++i)
                // TODO_RUST_CODE
            {
               // Java: SpanInfo span = (SpanInfo)this.mOtherSpans.get(i);
               // TODO_RUST_CODE
               // Java: if (span.hasOutline())
                    // TODO_RUST_CODE
               {
                  // Java: return true;
                  // TODO_RUST_CODE
               }
            }
         }
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean hasBlur()
    pub fn hasBlur(&self) -> bool {
      // Java: if (this.mFirstSpan != null && this.mFirstSpan.hasBlur())
        // TODO_RUST_CODE
      {
         // Java: return true;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: if (this.mOtherSpans != null && this.mOtherSpans.size() > 0)
            // TODO_RUST_CODE
         {
            // Java: for(int i = 0; i < this.mOtherSpans.size(); ++i)
                // TODO_RUST_CODE
            {
               // Java: SpanInfo span = (SpanInfo)this.mOtherSpans.get(i);
               // TODO_RUST_CODE
               // Java: if (span.hasBlur())
                    // TODO_RUST_CODE
               {
                  // Java: return true;
                  // TODO_RUST_CODE
               }
            }
         }
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean hasTextImage()
    pub fn hasTextImage(&self) -> bool {
      // Java: if (this.mFirstSpan != null && this.mFirstSpan.hasTextImage())
        // TODO_RUST_CODE
      {
         // Java: return true;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: if (this.mOtherSpans != null && this.mOtherSpans.size() > 0)
            // TODO_RUST_CODE
         {
            // Java: for(int i = 0; i < this.mOtherSpans.size(); ++i)
                // TODO_RUST_CODE
            {
               // Java: SpanInfo span = (SpanInfo)this.mOtherSpans.get(i);
               // TODO_RUST_CODE
               // Java: if (span.hasTextImage())
                    // TODO_RUST_CODE
               {
                  // Java: return true;
                  // TODO_RUST_CODE
               }
            }
         }
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean hasOuterShdw()
    pub fn hasOuterShdw(&self) -> bool {
      // Java: if (this.mFirstSpan != null && this.mFirstSpan.hasOuterShdw())
        // TODO_RUST_CODE
      {
         // Java: return true;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: if (this.mOtherSpans != null && this.mOtherSpans.size() > 0)
            // TODO_RUST_CODE
         {
            // Java: for(int i = 0; i < this.mOtherSpans.size(); ++i)
                // TODO_RUST_CODE
            {
               // Java: SpanInfo span = (SpanInfo)this.mOtherSpans.get(i);
               // TODO_RUST_CODE
               // Java: if (span.hasOuterShdw())
                    // TODO_RUST_CODE
               {
                  // Java: return true;
                  // TODO_RUST_CODE
               }
            }
         }
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean hasInnerShdw()
    pub fn hasInnerShdw(&self) -> bool {
      // Java: if (this.mFirstSpan != null && this.mFirstSpan.hasInnerShdw())
        // TODO_RUST_CODE
      {
         // Java: return true;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: if (this.mOtherSpans != null && this.mOtherSpans.size() > 0)
            // TODO_RUST_CODE
         {
            // Java: for(int i = 0; i < this.mOtherSpans.size(); ++i)
                // TODO_RUST_CODE
            {
               // Java: SpanInfo span = (SpanInfo)this.mOtherSpans.get(i);
               // TODO_RUST_CODE
               // Java: if (span.hasInnerShdw())
                    // TODO_RUST_CODE
               {
                  // Java: return true;
                  // TODO_RUST_CODE
               }
            }
         }
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: void clear()
    pub fn clear(&self) {
      // Java: this.mZeroWidthSpan = null;
      // TODO_RUST_CODE
      // Java: this.mSourceText = null;
      // TODO_RUST_CODE
      // Java: this.mTypoText = null;
      // TODO_RUST_CODE
      // Java: this.mReplacePositions = null;
      // TODO_RUST_CODE
      // Java: this.mPara.clear();
      // TODO_RUST_CODE
      // Java: ObjectPool<SpanInfo> pool = DataObjectPool.spanInfoObjectPool;
      // TODO_RUST_CODE
      // Java: for(int i = this.mOtherSpans.size() - 1; i > -1; --i)
        // TODO_RUST_CODE
      {
         // Java: pool.returnObject((SpanInfo)this.mOtherSpans.get(i));
         // TODO_RUST_CODE
      }
      // Java: this.mFirstSpan.clear();
      // TODO_RUST_CODE
      // Java: this.mOtherSpans.clear();
      // TODO_RUST_CODE
      // Java: this.mIsRichText = true;
      // TODO_RUST_CODE
      // Java: this.mBidi = null;
      // TODO_RUST_CODE
      // Java: this.mVertical = 0;
      // TODO_RUST_CODE
      // Java: this.mSpanIndex = -1;
      // TODO_RUST_CODE
      // Java: this.mIsSoftReturn = false;
      // TODO_RUST_CODE
      // Java: this.mArb = false;
      // TODO_RUST_CODE
    }

    // Java: void copyFrom(TextData data)
    pub fn copyFrom(&self, TextData data) {
      // Java: this.mPara.copyFrom(data.mPara);
      // TODO_RUST_CODE
      // Java: this.mFirstSpan.copyFrom(data.mFirstSpan);
      // TODO_RUST_CODE
      // Java: int i = 0;
      // TODO_RUST_CODE
      // Java: for(int size = data.mOtherSpans.size(); i < size; ++i)
        // TODO_RUST_CODE
      {
         // Java: this.mOtherSpans.add(new SpanInfo((SpanInfo)data.mOtherSpans.get(i)));
         // TODO_RUST_CODE
      }
      // Java: this.mBidi = data.mBidi;
      // TODO_RUST_CODE
      // Java: this.mIsRichText = data.mIsRichText;
      // TODO_RUST_CODE
      // Java: this.mSourceText = data.mSourceText;
      // TODO_RUST_CODE
      // Java: this.mTypoText = data.mTypoText;
      // TODO_RUST_CODE
      // Java: this.mVertical = data.mVertical;
      // TODO_RUST_CODE
      // Java: this.mSpanIndex = data.mSpanIndex;
      // TODO_RUST_CODE
      // Java: this.mIsSoftReturn = data.mIsSoftReturn;
      // TODO_RUST_CODE
      // Java: this.mArb = data.mArb;
      // TODO_RUST_CODE
      // Java: if (data.mZeroWidthSpan != null)
        // TODO_RUST_CODE
      {
         // Java: this.mZeroWidthSpan = new ZeroWidthSpan(data.mZeroWidthSpan.start, data.mZeroWidthSpan.end);
         // TODO_RUST_CODE
      }
    }

    // Java: static boolean isChinese(char c)
    pub fn isChinese(&self, char c) -> bool {
      // Java: Character.UnicodeBlock ub = UnicodeBlock.of(c);
      // TODO_RUST_CODE
      // Java: return ub == UnicodeBlock.CJK_UNIFIED_IDEOGRAPHS || ub == UnicodeBlock.CJK_COMPATIBILITY_IDEOGRAPHS || ub == UnicodeBlock.CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A || ub == UnicodeBlock.CJK_SYMBOLS_AND_PUNCTUATION || ub == UnicodeBlock.HALFWIDTH_AND_FULLWIDTH_FORMS || ub == UnicodeBlock.KATAKANA || ub == UnicodeBlock.HIRAGANA || ub == UnicodeBlock.ENCLOSED_ALPHANUMERICS;
      // TODO_RUST_CODE
    }

    // Java: static boolean isEnclosedAlphanum(char c)
    pub fn isEnclosedAlphanum(&self, char c) -> bool {
      // Java: Character.UnicodeBlock ub = UnicodeBlock.of(c);
      // TODO_RUST_CODE
      // Java: return ub == UnicodeBlock.ENCLOSED_ALPHANUMERICS;
      // TODO_RUST_CODE
    }

    // Java: void insertSpanAft(SpanInfo span)
    pub fn insertSpanAft(&self, SpanInfo span) {
      // Java: if (this.mSpanIndex > this.mOtherSpans.size())
        // TODO_RUST_CODE
      {
         // Java: this.mSpanIndex = this.mOtherSpans.size();
         // TODO_RUST_CODE
         // Java: this.mOtherSpans.add(this.mSpanIndex, span);
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: if (this.mSpanIndex < -1)
            // TODO_RUST_CODE
         {
            // Java: this.mSpanIndex = -1;
            // TODO_RUST_CODE
         }
         // Java: this.mOtherSpans.add(this.mSpanIndex + 1, span);
         // TODO_RUST_CODE
      }
    }

    // Java: SpanInfo getCurrentSpan()
    pub fn getCurrentSpan(&self) -> SpanInfo {
      // Java: SpanInfo span = null;
      // TODO_RUST_CODE
      // Java: if (this.mSpanIndex < 0)
        // TODO_RUST_CODE
      {
         // Java: span = this.mFirstSpan;
         // TODO_RUST_CODE
      }
      // Java: else if (this.mSpanIndex < this.mOtherSpans.size())
        // TODO_RUST_CODE
      {
         // Java: span = (SpanInfo)this.mOtherSpans.get(this.mSpanIndex);
         // TODO_RUST_CODE
      }
      // Java: return span;
      // TODO_RUST_CODE
    }

    // Java: SpanInfo getNextSpan()
    pub fn getNextSpan(&self) -> SpanInfo {
      // Java: ++this.mSpanIndex;
      // TODO_RUST_CODE
      // Java: return this.getCurrentSpan();
      // TODO_RUST_CODE
    }

    // Java: void removeCurrentSpan()
    pub fn removeCurrentSpan(&self) {
      // Java: if (this.mSpanIndex >= 0)
        // TODO_RUST_CODE
      {
         // Java: if (this.mSpanIndex < this.mOtherSpans.size())
            // TODO_RUST_CODE
         {
            // Java: this.mOtherSpans.remove(this.mSpanIndex);
            // TODO_RUST_CODE
         }
         // Java: --this.mSpanIndex;
         // TODO_RUST_CODE
      }
    }

    // Java: SpanInfo getFirstSpan()
    pub fn getFirstSpan(&self) -> SpanInfo {
      // Java: this.mSpanIndex = -1;
      // TODO_RUST_CODE
      // Java: return this.mFirstSpan;
      // TODO_RUST_CODE
    }

    // Java: SpanInfo getLastSpan()
    pub fn getLastSpan(&self) -> SpanInfo {
      // Java: if (this.mOtherSpans.isEmpty())
        // TODO_RUST_CODE
      {
         // Java: this.mSpanIndex = -1;
         // TODO_RUST_CODE
         // Java: return this.mFirstSpan;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: this.mSpanIndex = this.mOtherSpans.size() - 1;
         // TODO_RUST_CODE
         // Java: return (SpanInfo)this.mOtherSpans.get(this.mSpanIndex);
         // TODO_RUST_CODE
      }
    }

    // Java: boolean isLastSpan()
    pub fn isLastSpan(&self) -> bool {
      // Java: return this.mSpanIndex == this.mOtherSpans.size() - 1;
      // TODO_RUST_CODE
    }

    // Java: void resetSpanIndex()
    pub fn resetSpanIndex(&self) {
      // Java: this.mSpanIndex = -1;
      // TODO_RUST_CODE
    }

    // Java: int getSpanIndex()
    pub fn getSpanIndex(&self) -> i32 {
      // Java: return this.mSpanIndex;
      // TODO_RUST_CODE
    }

    // Java: boolean hasText()
    pub fn hasText(&self) -> bool {
      // Java: return null != this.mSourceText && 0 != this.mSourceText.length() && !this.mSourceText.equals("\u000b") && !this.mSourceText.equals("\r") && !this.mSourceText.equals("\n");
      // TODO_RUST_CODE
    }

    // Java: boolean hasBullets()
    pub fn hasBullets(&self) -> bool {
      // Java: return this.mPara.mbulletsInfo.isVisible() && this.mPara.mbulletsInfo.getBulletType() != -1;
      // TODO_RUST_CODE
    }

    // Java: SpanInfo createSpan()
    pub fn createSpan(&self) -> SpanInfo {
      // Java: return DataObjectPool.spanInfoObjectPool.borrowObject();
      // TODO_RUST_CODE
    }

    // Java: SpanInfo getSpanByOffset(int offset)
    pub fn getSpanByOffset(&self, int offset) -> SpanInfo {
      // Java: if (offset >= this.mFirstSpan.mStartPos && offset < this.mFirstSpan.mEndPos)
        // TODO_RUST_CODE
      {
         // Java: return this.mFirstSpan;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: for(int i = 0; i < this.mOtherSpans.size(); ++i)
            // TODO_RUST_CODE
         {
            // Java: SpanInfo span = (SpanInfo)this.mOtherSpans.get(i);
            // TODO_RUST_CODE
            // Java: if (offset >= span.mStartPos && offset < span.mEndPos)
                // TODO_RUST_CODE
            {
               // Java: return span;
               // TODO_RUST_CODE
            }
         }
         // Java: return null;
         // TODO_RUST_CODE
      }
    }

    // Java: void setSoftReturn(boolean isSoftReturn)
    pub fn setSoftReturn(&self, boolean isSoftReturn) {
      // Java: this.mIsSoftReturn = isSoftReturn;
      // TODO_RUST_CODE
    }

    // Java: boolean isSoftReturn()
    pub fn isSoftReturn(&self) -> bool {
      // Java: return this.mIsSoftReturn;
      // TODO_RUST_CODE
    }

}

// Java: public class ZeroWidthSpan
impl ZeroWidthSpan {
    // Java: ZeroWidthSpan(int start, int end)
    pub fn ZeroWidthSpan(&self, int start, int end) {
         // Java: super();
         // TODO_RUST_CODE
         // Java: this.start = start;
         // TODO_RUST_CODE
         // Java: this.end = end;
         // TODO_RUST_CODE
    }

}
