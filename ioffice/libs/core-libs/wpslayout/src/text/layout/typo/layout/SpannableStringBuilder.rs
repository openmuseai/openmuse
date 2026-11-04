// Java: import cn.wps.text.layout.typo.layout.style.NoCopySpan
use wpslayout::text::layout::typo::layout::style::NoCopySpan;
// Java: import java.lang.reflect.Array
// TODO_RUST_CODE use ???; // java.lang.reflect.Array
// Java: import java.util.IdentityHashMap
// TODO_RUST_CODE use ???; // java.util.IdentityHashMap

// Java field: String TAG
pub(crate) const TAG: String = /* TODO */;
// Java field: int[][] sCachedIntBuffer
static sCachedIntBuffer: std::sync::OnceLock<Vec<Vec<i32>>> = std::sync::OnceLock::new();
// Java field: int MARK
pub(crate) const MARK: i32 = /* TODO */;
// Java field: int POINT
pub(crate) const POINT: i32 = /* TODO */;
// Java field: int PARAGRAPH
pub(crate) const PARAGRAPH: i32 = /* TODO */;
// Java field: int START_MASK
pub(crate) const START_MASK: i32 = /* TODO */;
// Java field: int END_MASK
pub(crate) const END_MASK: i32 = /* TODO */;
// Java field: int START_SHIFT
pub(crate) const START_SHIFT: i32 = /* TODO */;
// Java field: int SPAN_ADDED
pub(crate) const SPAN_ADDED: i32 = /* TODO */;
// Java field: int SPAN_START_AT_START
pub(crate) const SPAN_START_AT_START: i32 = /* TODO */;
// Java field: int SPAN_START_AT_END
pub(crate) const SPAN_START_AT_END: i32 = /* TODO */;
// Java field: int SPAN_END_AT_START
pub(crate) const SPAN_END_AT_START: i32 = /* TODO */;
// Java field: int SPAN_END_AT_END
pub(crate) const SPAN_END_AT_END: i32 = /* TODO */;
// Java field: int SPAN_START_END_MASK
pub(crate) const SPAN_START_END_MASK: i32 = /* TODO */;

// Java: public class SpannableStringBuilder implements CharSequence, GetChars, Spannable, Editable, Appendable
pub struct SpannableStringBuilder {
    // Java field: char[] mText
    pub(crate) mText: Arc<Mutex<Vec<char>>>,
    // Java field: int mGapStart
    mGapStart: i32,
    // Java field: int mGapLength
    mGapLength: i32,
    // Java field: Object[] mSpans
    mSpans: Option<Vec<Object>>,
    // Java field: int[] mSpanStarts
    mSpanStarts: Option<Vec<i32>>,
    // Java field: int[] mSpanEnds
    mSpanEnds: Option<Vec<i32>>,
    // Java field: int[] mSpanMax
    mSpanMax: Option<Vec<i32>>,
    // Java field: int[] mSpanFlags
    mSpanFlags: Option<Vec<i32>>,
    // Java field: int[] mSpanOrder
    mSpanOrder: Option<Vec<i32>>,
    // Java field: int mSpanInsertCount
    mSpanInsertCount: i32,
    // Java field: int mSpanCount
    mSpanCount: i32,
    // Java field: IdentityHashMap<Object, Integer> mIndexOfSpan
    mIndexOfSpan: Option<IdentityHashMap<Object, Integer>>,
    // Java field: int mLowWaterMark
    mLowWaterMark: i32,
    // Java field: int mTextWatcherDepth
    mTextWatcherDepth: i32,
}

// Java: public class SpannableStringBuilder implements CharSequence, GetChars, Spannable, Editable, Appendable
impl SpannableStringBuilder {
    // Java: SpannableStringBuilder()
    pub fn SpannableStringBuilder(&self) {
      // Java: this("");
      // TODO_RUST_CODE
    }

    // Java: SpannableStringBuilder(CharSequence text)
    pub fn SpannableStringBuilder(&self, CharSequence text) {
      // Java: this(text, 0, text.length());
      // TODO_RUST_CODE
    }

    // Java: SpannableStringBuilder(CharSequence text, int start, int end)
    pub fn SpannableStringBuilder(&self, CharSequence text, int start, int end) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: int srclen = end - start;
      // TODO_RUST_CODE
      // Java: if (srclen < 0)
        // TODO_RUST_CODE
      {
         // Java: throw new StringIndexOutOfBoundsException();
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: this.mText = ArrayUtils.newUnpaddedCharArray(GrowingArrayUtils.growSize(srclen));
         // TODO_RUST_CODE
         // Java: this.mGapStart = srclen;
         // TODO_RUST_CODE
         // Java: this.mGapLength = this.mText.length - srclen;
         // TODO_RUST_CODE
         // Java: TextUtils.getChars(text, start, end, this.mText, 0);
         // TODO_RUST_CODE
         // Java: this.mSpanCount = 0;
         // TODO_RUST_CODE
         // Java: this.mSpanInsertCount = 0;
         // TODO_RUST_CODE
         // Java: this.mSpans = EmptyArray.OBJECT;
         // TODO_RUST_CODE
         // Java: this.mSpanStarts = EmptyArray.INT;
         // TODO_RUST_CODE
         // Java: this.mSpanEnds = EmptyArray.INT;
         // TODO_RUST_CODE
         // Java: this.mSpanFlags = EmptyArray.INT;
         // TODO_RUST_CODE
         // Java: this.mSpanMax = EmptyArray.INT;
         // TODO_RUST_CODE
         // Java: this.mSpanOrder = EmptyArray.INT;
         // TODO_RUST_CODE
         // Java: if (text instanceof Spanned)
            // TODO_RUST_CODE
         {
            // Java: Spanned sp = (Spanned)text;
            // TODO_RUST_CODE
            // Java: Object[] spans = sp.getSpans(start, end, Object.class);
            // TODO_RUST_CODE
            // Java: for(int i = 0; i < spans.length; ++i)
                // TODO_RUST_CODE
            {
               // Java: if (!(spans[i] instanceof NoCopySpan))
                    // TODO_RUST_CODE
               {
                  // Java: int st = sp.getSpanStart(spans[i]) - start;
                  // TODO_RUST_CODE
                  // Java: int en = sp.getSpanEnd(spans[i]) - start;
                  // TODO_RUST_CODE
                  // Java: int fl = sp.getSpanFlags(spans[i]);
                  // TODO_RUST_CODE
                  // Java: if (st < 0)
                        // TODO_RUST_CODE
                  {
                     // Java: st = 0;
                     // TODO_RUST_CODE
                  }
                  // Java: if (st > end - start)
                        // TODO_RUST_CODE
                  {
                     // Java: st = end - start;
                     // TODO_RUST_CODE
                  }
                  // Java: if (en < 0)
                        // TODO_RUST_CODE
                  {
                     // Java: en = 0;
                     // TODO_RUST_CODE
                  }
                  // Java: if (en > end - start)
                        // TODO_RUST_CODE
                  {
                     // Java: en = end - start;
                     // TODO_RUST_CODE
                  }
                  // Java: this.setSpan(false, spans[i], st, en, fl, false);
                  // TODO_RUST_CODE
               }
            }
            // Java: this.restoreInvariants();
            // TODO_RUST_CODE
         }
      }
    }

    // Java: static SpannableStringBuilder valueOf(CharSequence source)
    pub fn valueOf(&self, CharSequence source) -> SpannableStringBuilder {
      // Java: return source instanceof SpannableStringBuilder ? (SpannableStringBuilder)source : new SpannableStringBuilder(source);
      // TODO_RUST_CODE
    }

    // Java: char charAt(int where)
    pub fn charAt(&self, int where) -> char {
      // Java: int len = this.length();
      // TODO_RUST_CODE
      // Java: if (where < 0)
        // TODO_RUST_CODE
      {
         // Java: throw new IndexOutOfBoundsException("charAt: " + where + " < 0");
         // TODO_RUST_CODE
      }
      // Java: else if (where >= len)
        // TODO_RUST_CODE
      {
         // Java: throw new IndexOutOfBoundsException("charAt: " + where + " >= length " + len);
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return where >= this.mGapStart ? this.mText[where + this.mGapLength] : this.mText[where];
         // TODO_RUST_CODE
      }
    }

    // Java: int length()
    pub fn length(&self) -> i32 {
      // Java: return this.mText.length - this.mGapLength;
      // TODO_RUST_CODE
    }

    // Java: void resizeFor(int size)
    pub fn resizeFor(&self, int size) {
      // Java: int oldLength = this.mText.length;
      // TODO_RUST_CODE
      // Java: if (size + 1 > oldLength)
        // TODO_RUST_CODE
      {
         // Java: char[] newText = ArrayUtils.newUnpaddedCharArray(GrowingArrayUtils.growSize(size));
         // TODO_RUST_CODE
         // Java: System.arraycopy(this.mText, 0, newText, 0, this.mGapStart);
         // TODO_RUST_CODE
         // Java: int newLength = newText.length;
         // TODO_RUST_CODE
         // Java: int delta = newLength - oldLength;
         // TODO_RUST_CODE
         // Java: int after = oldLength - (this.mGapStart + this.mGapLength);
         // TODO_RUST_CODE
         // Java: System.arraycopy(this.mText, oldLength - after, newText, newLength - after, after);
         // TODO_RUST_CODE
         // Java: this.mText = newText;
         // TODO_RUST_CODE
         // Java: this.mGapLength += delta;
         // TODO_RUST_CODE
         // Java: if (this.mGapLength < 1)
            // TODO_RUST_CODE
         {
            // Java: (new Exception("mGapLength < 1")).printStackTrace();
            // TODO_RUST_CODE
         }
         // Java: if (this.mSpanCount != 0)
            // TODO_RUST_CODE
         {
            // Java: for(int i = 0; i < this.mSpanCount; ++i)
                // TODO_RUST_CODE
            {
               // Java: if (this.mSpanStarts[i] > this.mGapStart)
                    // TODO_RUST_CODE
               {
                  // Java: int[] var10000 = this.mSpanStarts;
                  // TODO_RUST_CODE
                  // Java: var10000[i] += delta;
                  // TODO_RUST_CODE
               }
               // Java: if (this.mSpanEnds[i] > this.mGapStart)
                    // TODO_RUST_CODE
               {
                  // Java: int[] var8 = this.mSpanEnds;
                  // TODO_RUST_CODE
                  // Java: var8[i] += delta;
                  // TODO_RUST_CODE
               }
            }
            // Java: this.calcMax(this.treeRoot());
            // TODO_RUST_CODE
         }
      }
    }

    // Java: void moveGapTo(int where)
    pub fn moveGapTo(&self, int where) {
      // Java: if (where != this.mGapStart)
        // TODO_RUST_CODE
      {
         // Java: boolean atEnd = where == this.length();
         // TODO_RUST_CODE
         // Java: if (where < this.mGapStart)
            // TODO_RUST_CODE
         {
            // Java: int overlap = this.mGapStart - where;
            // TODO_RUST_CODE
            // Java: System.arraycopy(this.mText, where, this.mText, this.mGapStart + this.mGapLength - overlap, overlap);
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: int overlap = where - this.mGapStart;
            // TODO_RUST_CODE
            // Java: System.arraycopy(this.mText, where + this.mGapLength - overlap, this.mText, this.mGapStart, overlap);
            // TODO_RUST_CODE
         }
         // Java: if (this.mSpanCount != 0)
            // TODO_RUST_CODE
         {
            // Java: for(int i = 0; i < this.mSpanCount; ++i)
                // TODO_RUST_CODE
            {
               // Java: int start = this.mSpanStarts[i];
               // TODO_RUST_CODE
               // Java: int end = this.mSpanEnds[i];
               // TODO_RUST_CODE
               // Java: if (start > this.mGapStart)
                    // TODO_RUST_CODE
               {
                  // Java: start -= this.mGapLength;
                  // TODO_RUST_CODE
               }
               // Java: if (start > where)
                    // TODO_RUST_CODE
               {
                  // Java: start += this.mGapLength;
                  // TODO_RUST_CODE
               }
               // Java: else if (start == where)
                    // TODO_RUST_CODE
               {
                  // Java: int flag = (this.mSpanFlags[i] & 240) >> 4;
                  // TODO_RUST_CODE
                  // Java: if (flag == 2 || atEnd && flag == 3)
                        // TODO_RUST_CODE
                  {
                     // Java: start += this.mGapLength;
                     // TODO_RUST_CODE
                  }
               }
               // Java: if (end > this.mGapStart)
                    // TODO_RUST_CODE
               {
                  // Java: end -= this.mGapLength;
                  // TODO_RUST_CODE
               }
               // Java: if (end > where)
                    // TODO_RUST_CODE
               {
                  // Java: end += this.mGapLength;
                  // TODO_RUST_CODE
               }
               // Java: else if (end == where)
                    // TODO_RUST_CODE
               {
                  // Java: int flag = this.mSpanFlags[i] & 15;
                  // TODO_RUST_CODE
                  // Java: if (flag == 2 || atEnd && flag == 3)
                        // TODO_RUST_CODE
                  {
                     // Java: end += this.mGapLength;
                     // TODO_RUST_CODE
                  }
               }
               // Java: this.mSpanStarts[i] = start;
               // TODO_RUST_CODE
               // Java: this.mSpanEnds[i] = end;
               // TODO_RUST_CODE
            }
            // Java: this.calcMax(this.treeRoot());
            // TODO_RUST_CODE
         }
         // Java: this.mGapStart = where;
         // TODO_RUST_CODE
      }
    }

    // Java: SpannableStringBuilder insert(int where, CharSequence tb, int start, int end)
    pub fn insert(&self, int where, CharSequence tb, int start, int end) -> SpannableStringBuilder {
      // Java: return this.replace(where, where, tb, start, end);
      // TODO_RUST_CODE
    }

    // Java: SpannableStringBuilder insert(int where, CharSequence tb)
    pub fn insert(&self, int where, CharSequence tb) -> SpannableStringBuilder {
      // Java: return this.replace(where, where, tb, 0, tb.length());
      // TODO_RUST_CODE
    }

    // Java: SpannableStringBuilder delete(int start, int end)
    pub fn delete(&self, int start, int end) -> SpannableStringBuilder {
      // Java: SpannableStringBuilder ret = this.replace(start, end, "", 0, 0);
      // TODO_RUST_CODE
      // Java: if (this.mGapLength > 2 * this.length())
        // TODO_RUST_CODE
      {
         // Java: this.resizeFor(this.length());
         // TODO_RUST_CODE
      }
      // Java: return ret;
      // TODO_RUST_CODE
    }

    // Java: void clear()
    pub fn clear(&self) {
      // Java: this.replace(0, this.length(), "", 0, 0);
      // TODO_RUST_CODE
      // Java: this.mSpanInsertCount = 0;
      // TODO_RUST_CODE
    }

    // Java: void clearSpans()
    pub fn clearSpans(&self) {
      // Java: for(int i = this.mSpanCount - 1; i >= 0; --i)
        // TODO_RUST_CODE
      {
         // Java: Object var10000 = this.mSpans[i];
         // TODO_RUST_CODE
         // Java: int ostart = this.mSpanStarts[i];
         // TODO_RUST_CODE
         // Java: int oend = this.mSpanEnds[i];
         // TODO_RUST_CODE
         // Java: if (ostart > this.mGapStart)
            // TODO_RUST_CODE
         {
            // Java: int var5 = ostart - this.mGapLength;
            // TODO_RUST_CODE
         }
         // Java: if (oend > this.mGapStart)
            // TODO_RUST_CODE
         {
            // Java: int var6 = oend - this.mGapLength;
            // TODO_RUST_CODE
         }
         // Java: this.mSpanCount = i;
         // TODO_RUST_CODE
         // Java: this.mSpans[i] = null;
         // TODO_RUST_CODE
      }
      // Java: if (this.mIndexOfSpan != null)
        // TODO_RUST_CODE
      {
         // Java: this.mIndexOfSpan.clear();
         // TODO_RUST_CODE
      }
      // Java: this.mSpanInsertCount = 0;
      // TODO_RUST_CODE
    }

    // Java: SpannableStringBuilder append(CharSequence text)
    pub fn append(&self, CharSequence text) -> SpannableStringBuilder {
      // Java: int length = this.length();
      // TODO_RUST_CODE
      // Java: return this.replace(length, length, text, 0, text.length());
      // TODO_RUST_CODE
    }

    // Java: SpannableStringBuilder append(CharSequence text, Object what, int flags)
    pub fn append(&self, CharSequence text, Object what, int flags) -> SpannableStringBuilder {
      // Java: int start = this.length();
      // TODO_RUST_CODE
      // Java: this.append(text);
      // TODO_RUST_CODE
      // Java: this.setSpan(what, start, this.length(), flags);
      // TODO_RUST_CODE
      // Java: return this;
      // TODO_RUST_CODE
    }

    // Java: SpannableStringBuilder append(CharSequence text, int start, int end)
    pub fn append(&self, CharSequence text, int start, int end) -> SpannableStringBuilder {
      // Java: int length = this.length();
      // TODO_RUST_CODE
      // Java: return this.replace(length, length, text, start, end);
      // TODO_RUST_CODE
    }

    // Java: SpannableStringBuilder append(char text)
    pub fn append(&self, char text) -> SpannableStringBuilder {
      // Java: return this.append(String.valueOf(text));
      // TODO_RUST_CODE
    }

    // Java: boolean removeSpansForChange(int start, int end, boolean textIsRemoved, int i)
    pub fn removeSpansForChange(&self, int start, int end, boolean textIsRemoved, int i) -> bool {
      // Java: if ((i & 1) != 0 && this.resolveGap(this.mSpanMax[i]) >= start && this.removeSpansForChange(start, end, textIsRemoved, leftChild(i)))
        // TODO_RUST_CODE
      {
         // Java: return true;
         // TODO_RUST_CODE
      }
      // Java: else if (i >= this.mSpanCount)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else if ((this.mSpanFlags[i] & 33) != 33 || this.mSpanStarts[i] < start || this.mSpanStarts[i] >= this.mGapStart + this.mGapLength || this.mSpanEnds[i] < start || this.mSpanEnds[i] >= this.mGapStart + this.mGapLength || !textIsRemoved && this.mSpanStarts[i] <= start && this.mSpanEnds[i] >= this.mGapStart)
        // TODO_RUST_CODE
      {
         // Java: return this.resolveGap(this.mSpanStarts[i]) <= end && (i & 1) != 0 && this.removeSpansForChange(start, end, textIsRemoved, rightChild(i));
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: this.mIndexOfSpan.remove(this.mSpans[i]);
         // TODO_RUST_CODE
         // Java: this.removeSpan(i, 0);
         // TODO_RUST_CODE
         // Java: return true;
         // TODO_RUST_CODE
      }
    }

    // Java: void change(int start, int end, CharSequence cs, int csStart, int csEnd)
    pub fn change(&self, int start, int end, CharSequence cs, int csStart, int csEnd) {
      // Java: int replacedLength = end - start;
      // TODO_RUST_CODE
      // Java: int replacementLength = csEnd - csStart;
      // TODO_RUST_CODE
      // Java: int nbNewChars = replacementLength - replacedLength;
      // TODO_RUST_CODE
      // Java: boolean changed = false;
      // TODO_RUST_CODE
      // Java: for(int i = this.mSpanCount - 1; i >= 0; --i)
        // TODO_RUST_CODE
      {
         // Java: int spanStart = this.mSpanStarts[i];
         // TODO_RUST_CODE
         // Java: if (spanStart > this.mGapStart)
            // TODO_RUST_CODE
         {
            // Java: spanStart -= this.mGapLength;
            // TODO_RUST_CODE
         }
         // Java: int spanEnd = this.mSpanEnds[i];
         // TODO_RUST_CODE
         // Java: if (spanEnd > this.mGapStart)
            // TODO_RUST_CODE
         {
            // Java: spanEnd -= this.mGapLength;
            // TODO_RUST_CODE
         }
         // Java: if ((this.mSpanFlags[i] & 51) == 51)
            // TODO_RUST_CODE
         {
            // Java: int ost = spanStart;
            // TODO_RUST_CODE
            // Java: int oen = spanEnd;
            // TODO_RUST_CODE
            // Java: int clen = this.length();
            // TODO_RUST_CODE
            // Java: if (spanStart > start && spanStart <= end)
                // TODO_RUST_CODE
            {
               // Java: for(spanStart = end; spanStart < clen && (spanStart <= end || this.charAt(spanStart - 1) != '\n'); ++spanStart)
                    // TODO_RUST_CODE
               {
               }
            }
            // Java: if (spanEnd > start && spanEnd <= end)
                // TODO_RUST_CODE
            {
               // Java: for(spanEnd = end; spanEnd < clen && (spanEnd <= end || this.charAt(spanEnd - 1) != '\n'); ++spanEnd)
                    // TODO_RUST_CODE
               {
               }
            }
            // Java: if (spanStart != ost || spanEnd != oen)
                // TODO_RUST_CODE
            {
               // Java: this.setSpan(false, this.mSpans[i], spanStart, spanEnd, this.mSpanFlags[i], true);
               // TODO_RUST_CODE
               // Java: changed = true;
               // TODO_RUST_CODE
            }
         }
         // Java: int flags = 0;
         // TODO_RUST_CODE
         // Java: if (spanStart == start)
            // TODO_RUST_CODE
         {
            // Java: flags |= 4096;
            // TODO_RUST_CODE
         }
         // Java: else if (spanStart == end + nbNewChars)
            // TODO_RUST_CODE
         {
            // Java: flags |= 8192;
            // TODO_RUST_CODE
         }
         // Java: if (spanEnd == start)
            // TODO_RUST_CODE
         {
            // Java: flags |= 16384;
            // TODO_RUST_CODE
         }
         // Java: else if (spanEnd == end + nbNewChars)
            // TODO_RUST_CODE
         {
            // Java: flags |= 32768;
            // TODO_RUST_CODE
         }
         // Java: int[] var10000 = this.mSpanFlags;
         // TODO_RUST_CODE
         // Java: var10000[i] |= flags;
         // TODO_RUST_CODE
      }
      // Java: if (changed)
        // TODO_RUST_CODE
      {
         // Java: this.restoreInvariants();
         // TODO_RUST_CODE
      }
      // Java: this.moveGapTo(end);
      // TODO_RUST_CODE
      // Java: if (nbNewChars >= this.mGapLength)
        // TODO_RUST_CODE
      {
         // Java: this.resizeFor(this.mText.length + nbNewChars - this.mGapLength);
         // TODO_RUST_CODE
      }
      // Java: boolean textIsRemoved = replacementLength == 0;
      // TODO_RUST_CODE
      // Java: if (replacedLength > 0)
        // TODO_RUST_CODE
      {
         // Java: while(this.mSpanCount > 0 && this.removeSpansForChange(start, end, textIsRemoved, this.treeRoot()))
            // TODO_RUST_CODE
         {
         }
      }
      // Java: this.mGapStart += nbNewChars;
      // TODO_RUST_CODE
      // Java: this.mGapLength -= nbNewChars;
      // TODO_RUST_CODE
      // Java: if (this.mGapLength < 1)
        // TODO_RUST_CODE
      {
         // Java: (new Exception("mGapLength < 1")).printStackTrace();
         // TODO_RUST_CODE
      }
      // Java: TextUtils.getChars(cs, csStart, csEnd, this.mText, start);
      // TODO_RUST_CODE
      // Java: if (replacedLength > 0)
        // TODO_RUST_CODE
      {
         // Java: boolean atEnd = this.mGapStart + this.mGapLength == this.mText.length;
         // TODO_RUST_CODE
         // Java: for(int i = 0; i < this.mSpanCount; ++i)
            // TODO_RUST_CODE
         {
            // Java: int startFlag = (this.mSpanFlags[i] & 240) >> 4;
            // TODO_RUST_CODE
            // Java: this.mSpanStarts[i] = this.updatedIntervalBound(this.mSpanStarts[i], start, nbNewChars, startFlag, atEnd, textIsRemoved);
            // TODO_RUST_CODE
            // Java: int endFlag = this.mSpanFlags[i] & 15;
            // TODO_RUST_CODE
            // Java: this.mSpanEnds[i] = this.updatedIntervalBound(this.mSpanEnds[i], start, nbNewChars, endFlag, atEnd, textIsRemoved);
            // TODO_RUST_CODE
         }
         // Java: this.restoreInvariants();
         // TODO_RUST_CODE
      }
      // Java: if (cs instanceof Spanned)
        // TODO_RUST_CODE
      {
         // Java: Spanned sp = (Spanned)cs;
         // TODO_RUST_CODE
         // Java: Object[] spans = sp.getSpans(csStart, csEnd, Object.class);
         // TODO_RUST_CODE
         // Java: for(int i = 0; i < spans.length; ++i)
            // TODO_RUST_CODE
         {
            // Java: int st = sp.getSpanStart(spans[i]);
            // TODO_RUST_CODE
            // Java: int en = sp.getSpanEnd(spans[i]);
            // TODO_RUST_CODE
            // Java: if (st < csStart)
                // TODO_RUST_CODE
            {
               // Java: st = csStart;
               // TODO_RUST_CODE
            }
            // Java: if (en > csEnd)
                // TODO_RUST_CODE
            {
               // Java: en = csEnd;
               // TODO_RUST_CODE
            }
            // Java: if (this.getSpanStart(spans[i]) < 0)
                // TODO_RUST_CODE
            {
               // Java: int copySpanStart = st - csStart + start;
               // TODO_RUST_CODE
               // Java: int copySpanEnd = en - csStart + start;
               // TODO_RUST_CODE
               // Java: int copySpanFlags = sp.getSpanFlags(spans[i]) | 2048;
               // TODO_RUST_CODE
               // Java: this.setSpan(false, spans[i], copySpanStart, copySpanEnd, copySpanFlags, false);
               // TODO_RUST_CODE
            }
         }
         // Java: this.restoreInvariants();
         // TODO_RUST_CODE
      }
    }

    // Java: int updatedIntervalBound(int offset, int start, int nbNewChars, int flag, boolean atEnd, boolean textIsRemoved)
    pub fn updatedIntervalBound(&self, int offset, int start, int nbNewChars, int flag, boolean atEnd, boolean textIsRemoved) -> i32 {
      // Java: if (offset >= start && offset < this.mGapStart + this.mGapLength)
        // TODO_RUST_CODE
      {
         // Java: if (flag == 2)
            // TODO_RUST_CODE
         {
            // Java: if (textIsRemoved || offset > start)
                // TODO_RUST_CODE
            {
               // Java: return this.mGapStart + this.mGapLength;
               // TODO_RUST_CODE
            }
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: if (flag != 3)
                // TODO_RUST_CODE
            {
               // Java: if (!textIsRemoved && offset >= this.mGapStart - nbNewChars)
                    // TODO_RUST_CODE
               {
                  // Java: return this.mGapStart;
                  // TODO_RUST_CODE
               }
               // Java: return start;
               // TODO_RUST_CODE
            }
            // Java: if (atEnd)
                // TODO_RUST_CODE
            {
               // Java: return this.mGapStart + this.mGapLength;
               // TODO_RUST_CODE
            }
         }
      }
      // Java: return offset;
      // TODO_RUST_CODE
    }

    // Java: void replaceChar(char c, int index)
    pub fn replaceChar(&self, char c, int index) {
      // Java: this.mText[index] = c;
      // TODO_RUST_CODE
    }

    // Java: void removeSpan(int i, int flags)
    pub fn removeSpan(&self, int i, int flags) {
      // Java: Object var10000 = this.mSpans[i];
      // TODO_RUST_CODE
      // Java: int start = this.mSpanStarts[i];
      // TODO_RUST_CODE
      // Java: int end = this.mSpanEnds[i];
      // TODO_RUST_CODE
      // Java: if (start > this.mGapStart)
        // TODO_RUST_CODE
      {
         // Java: int var7 = start - this.mGapLength;
         // TODO_RUST_CODE
      }
      // Java: if (end > this.mGapStart)
        // TODO_RUST_CODE
      {
         // Java: int var8 = end - this.mGapLength;
         // TODO_RUST_CODE
      }
      // Java: int count = this.mSpanCount - (i + 1);
      // TODO_RUST_CODE
      // Java: System.arraycopy(this.mSpans, i + 1, this.mSpans, i, count);
      // TODO_RUST_CODE
      // Java: System.arraycopy(this.mSpanStarts, i + 1, this.mSpanStarts, i, count);
      // TODO_RUST_CODE
      // Java: System.arraycopy(this.mSpanEnds, i + 1, this.mSpanEnds, i, count);
      // TODO_RUST_CODE
      // Java: System.arraycopy(this.mSpanFlags, i + 1, this.mSpanFlags, i, count);
      // TODO_RUST_CODE
      // Java: System.arraycopy(this.mSpanOrder, i + 1, this.mSpanOrder, i, count);
      // TODO_RUST_CODE
      // Java: --this.mSpanCount;
      // TODO_RUST_CODE
      // Java: this.invalidateIndex(i);
      // TODO_RUST_CODE
      // Java: this.mSpans[this.mSpanCount] = null;
      // TODO_RUST_CODE
      // Java: this.restoreInvariants();
      // TODO_RUST_CODE
    }

    // Java: SpannableStringBuilder replace(int start, int end, CharSequence tb)
    pub fn replace(&self, int start, int end, CharSequence tb) -> SpannableStringBuilder {
      // Java: return this.replace(start, end, tb, 0, tb.length());
      // TODO_RUST_CODE
    }

    // Java: SpannableStringBuilder replace(int start, int end, CharSequence tb, int tbstart, int tbend)
    pub fn replace(&self, int start, int end, CharSequence tb, int tbstart, int tbend) -> SpannableStringBuilder {
      // Java: this.checkRange("replace", start, end);
      // TODO_RUST_CODE
      // Java: int origLen = end - start;
      // TODO_RUST_CODE
      // Java: int newLen = tbend - tbstart;
      // TODO_RUST_CODE
      // Java: if (origLen == 0 && newLen == 0 && !hasNonExclusiveExclusiveSpanAt(tb, tbstart))
        // TODO_RUST_CODE
      {
         // Java: return this;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: this.change(start, end, tb, tbstart, tbend);
         // TODO_RUST_CODE
         // Java: this.sendToSpanWatchers(start, end, newLen - origLen);
         // TODO_RUST_CODE
         // Java: return this;
         // TODO_RUST_CODE
      }
    }

    // Java: static boolean hasNonExclusiveExclusiveSpanAt(CharSequence text, int offset)
    pub fn hasNonExclusiveExclusiveSpanAt(&self, CharSequence text, int offset) -> bool {
      // Java: if (text instanceof Spanned)
        // TODO_RUST_CODE
      {
         // Java: Spanned spanned = (Spanned)text;
         // TODO_RUST_CODE
         // Java: for(Object span : spanned.getSpans(offset, offset, Object.class))
            // TODO_RUST_CODE
         {
            // Java: int flags = spanned.getSpanFlags(span);
            // TODO_RUST_CODE
            // Java: if (flags != 33)
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

    // Java: void sendToSpanWatchers(int replaceStart, int replaceEnd, int nbNewChars)
    pub fn sendToSpanWatchers(&self, int replaceStart, int replaceEnd, int nbNewChars) {
      // Java: for(int i = 0; i < this.mSpanCount; ++i)
        // TODO_RUST_CODE
      {
         // Java: int spanFlags = this.mSpanFlags[i];
         // TODO_RUST_CODE
         // Java: if ((spanFlags & 2048) == 0)
            // TODO_RUST_CODE
         {
            // Java: int spanStart = this.mSpanStarts[i];
            // TODO_RUST_CODE
            // Java: int spanEnd = this.mSpanEnds[i];
            // TODO_RUST_CODE
            // Java: if (spanStart > this.mGapStart)
                // TODO_RUST_CODE
            {
               // Java: spanStart -= this.mGapLength;
               // TODO_RUST_CODE
            }
            // Java: if (spanEnd > this.mGapStart)
                // TODO_RUST_CODE
            {
               // Java: spanEnd -= this.mGapLength;
               // TODO_RUST_CODE
            }
            // Java: int newReplaceEnd = replaceEnd + nbNewChars;
            // TODO_RUST_CODE
            // Java: boolean spanChanged = false;
            // TODO_RUST_CODE
            // Java: if (spanStart > newReplaceEnd)
                // TODO_RUST_CODE
            {
               // Java: if (nbNewChars != 0)
                    // TODO_RUST_CODE
               {
                  // Java: int var10000 = spanStart - nbNewChars;
                  // TODO_RUST_CODE
                  // Java: spanChanged = true;
                  // TODO_RUST_CODE
               }
            }
            // Java: else if (spanStart >= replaceStart && (spanStart != replaceStart || (spanFlags & 4096) != 4096) && (spanStart != newReplaceEnd || (spanFlags & 8192) != 8192))
                // TODO_RUST_CODE
            {
               // Java: spanChanged = true;
               // TODO_RUST_CODE
            }
            // Java: if (spanEnd > newReplaceEnd)
                // TODO_RUST_CODE
            {
               // Java: if (nbNewChars != 0)
                    // TODO_RUST_CODE
               {
                  // Java: int var20 = spanEnd - nbNewChars;
                  // TODO_RUST_CODE
                  // Java: spanChanged = true;
                  // TODO_RUST_CODE
               }
            }
            // Java: else if (spanEnd >= replaceStart && (spanEnd != replaceStart || (spanFlags & 16384) != 16384) && (spanEnd != newReplaceEnd || (spanFlags & '耀') != 32768))
                // TODO_RUST_CODE
            {
               // Java: spanChanged = true;
               // TODO_RUST_CODE
            }
            // Java: int[] var21 = this.mSpanFlags;
            // TODO_RUST_CODE
            // Java: var21[i] &= -61441;
            // TODO_RUST_CODE
         }
      }
      // Java: for(int i = 0; i < this.mSpanCount; ++i)
        // TODO_RUST_CODE
      {
         // Java: int spanFlags = this.mSpanFlags[i];
         // TODO_RUST_CODE
         // Java: if ((spanFlags & 2048) != 0)
            // TODO_RUST_CODE
         {
            // Java: int[] var22 = this.mSpanFlags;
            // TODO_RUST_CODE
            // Java: var22[i] &= -2049;
            // TODO_RUST_CODE
            // Java: int spanStart = this.mSpanStarts[i];
            // TODO_RUST_CODE
            // Java: int spanEnd = this.mSpanEnds[i];
            // TODO_RUST_CODE
            // Java: if (spanStart > this.mGapStart)
                // TODO_RUST_CODE
            {
               // Java: int var23 = spanStart - this.mGapLength;
               // TODO_RUST_CODE
            }
            // Java: if (spanEnd > this.mGapStart)
                // TODO_RUST_CODE
            {
               // Java: int var24 = spanEnd - this.mGapLength;
               // TODO_RUST_CODE
            }
         }
      }
    }

    // Java: void setSpan(Object what, int start, int end, int flags)
    pub fn setSpan(&self, Object what, int start, int end, int flags) {
      // Java: this.setSpan(true, what, start, end, flags, true);
      // TODO_RUST_CODE
    }

    // Java: void setSpan(boolean send, Object what, int start, int end, int flags, boolean enforceParagraph)
    pub fn setSpan(&self, boolean send, Object what, int start, int end, int flags, boolean enforceParagraph) {
      // Java: this.checkRange("setSpan", start, end);
      // TODO_RUST_CODE
      // Java: int flagsStart = (flags & 240) >> 4;
      // TODO_RUST_CODE
      // Java: if (this.isInvalidParagraph(start, flagsStart))
        // TODO_RUST_CODE
      {
         // Java: if (enforceParagraph)
            // TODO_RUST_CODE
         {
            // Java: throw new RuntimeException("PARAGRAPH span must start at paragraph boundary (" + start + " follows " + this.charAt(start - 1) + ")");
            // TODO_RUST_CODE
         }
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: int flagsEnd = flags & 15;
         // TODO_RUST_CODE
         // Java: if (this.isInvalidParagraph(end, flagsEnd))
            // TODO_RUST_CODE
         {
            // Java: if (enforceParagraph)
                // TODO_RUST_CODE
            {
               // Java: throw new RuntimeException("PARAGRAPH span must end at paragraph boundary (" + end + " follows " + this.charAt(end - 1) + ")");
               // TODO_RUST_CODE
            }
         }
         // Java: else if (flagsStart == 2 && flagsEnd == 1 && start == end)
            // TODO_RUST_CODE
         {
            // Java: if (send)
                // TODO_RUST_CODE
            {
            }
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: if (start > this.mGapStart)
                // TODO_RUST_CODE
            {
               // Java: start += this.mGapLength;
               // TODO_RUST_CODE
            }
            // Java: else if (start == this.mGapStart && (flagsStart == 2 || flagsStart == 3 && start == this.length()))
                // TODO_RUST_CODE
            {
               // Java: start += this.mGapLength;
               // TODO_RUST_CODE
            }
            // Java: if (end > this.mGapStart)
                // TODO_RUST_CODE
            {
               // Java: end += this.mGapLength;
               // TODO_RUST_CODE
            }
            // Java: else if (end == this.mGapStart && (flagsEnd == 2 || flagsEnd == 3 && end == this.length()))
                // TODO_RUST_CODE
            {
               // Java: end += this.mGapLength;
               // TODO_RUST_CODE
            }
            // Java: if (this.mIndexOfSpan != null)
                // TODO_RUST_CODE
            {
               // Java: Integer index = (Integer)this.mIndexOfSpan.get(what);
               // TODO_RUST_CODE
               // Java: if (index != null)
                    // TODO_RUST_CODE
               {
                  // Java: int i = index;
                  // TODO_RUST_CODE
                  // Java: int ostart = this.mSpanStarts[i];
                  // TODO_RUST_CODE
                  // Java: int oend = this.mSpanEnds[i];
                  // TODO_RUST_CODE
                  // Java: if (ostart > this.mGapStart)
                        // TODO_RUST_CODE
                  {
                     // Java: int var10000 = ostart - this.mGapLength;
                     // TODO_RUST_CODE
                  }
                  // Java: if (oend > this.mGapStart)
                        // TODO_RUST_CODE
                  {
                     // Java: int var16 = oend - this.mGapLength;
                     // TODO_RUST_CODE
                  }
                  // Java: this.mSpanStarts[i] = start;
                  // TODO_RUST_CODE
                  // Java: this.mSpanEnds[i] = end;
                  // TODO_RUST_CODE
                  // Java: this.mSpanFlags[i] = flags;
                  // TODO_RUST_CODE
                  // Java: if (send)
                        // TODO_RUST_CODE
                  {
                     // Java: this.restoreInvariants();
                     // TODO_RUST_CODE
                  }
                  // Java: return;
                  // TODO_RUST_CODE
               }
            }
            // Java: this.mSpans = GrowingArrayUtils.append(this.mSpans, this.mSpanCount, what);
            // TODO_RUST_CODE
            // Java: this.mSpanStarts = GrowingArrayUtils.append(this.mSpanStarts, this.mSpanCount, start);
            // TODO_RUST_CODE
            // Java: this.mSpanEnds = GrowingArrayUtils.append(this.mSpanEnds, this.mSpanCount, end);
            // TODO_RUST_CODE
            // Java: this.mSpanFlags = GrowingArrayUtils.append(this.mSpanFlags, this.mSpanCount, flags);
            // TODO_RUST_CODE
            // Java: this.mSpanOrder = GrowingArrayUtils.append(this.mSpanOrder, this.mSpanCount, this.mSpanInsertCount);
            // TODO_RUST_CODE
            // Java: this.invalidateIndex(this.mSpanCount);
            // TODO_RUST_CODE
            // Java: ++this.mSpanCount;
            // TODO_RUST_CODE
            // Java: ++this.mSpanInsertCount;
            // TODO_RUST_CODE
            // Java: int sizeOfMax = 2 * this.treeRoot() + 1;
            // TODO_RUST_CODE
            // Java: if (this.mSpanMax.length < sizeOfMax)
                // TODO_RUST_CODE
            {
               // Java: this.mSpanMax = new int[sizeOfMax];
               // TODO_RUST_CODE
            }
            // Java: if (send)
                // TODO_RUST_CODE
            {
               // Java: this.restoreInvariants();
               // TODO_RUST_CODE
            }
         }
      }
    }

    // Java: boolean isInvalidParagraph(int index, int flag)
    pub fn isInvalidParagraph(&self, int index, int flag) -> bool {
      // Java: return flag == 3 && index != 0 && index != this.length() && this.charAt(index - 1) != '\n';
      // TODO_RUST_CODE
    }

    // Java: void removeSpan(Object what)
    pub fn removeSpan(&self, Object what) {
      // Java: this.removeSpan(what, 0);
      // TODO_RUST_CODE
    }

    // Java: void removeSpan(Object what, int flags)
    pub fn removeSpan(&self, Object what, int flags) {
      // Java: if (this.mIndexOfSpan != null)
        // TODO_RUST_CODE
      {
         // Java: Integer i = (Integer)this.mIndexOfSpan.remove(what);
         // TODO_RUST_CODE
         // Java: if (i != null)
            // TODO_RUST_CODE
         {
            // Java: this.removeSpan(i, flags);
            // TODO_RUST_CODE
         }
      }
    }

    // Java: int resolveGap(int i)
    pub fn resolveGap(&self, int i) -> i32 {
      // Java: return i > this.mGapStart ? i - this.mGapLength : i;
      // TODO_RUST_CODE
    }

    // Java: int getSpanStart(Object what)
    pub fn getSpanStart(&self, Object what) -> i32 {
      // Java: if (this.mIndexOfSpan == null)
        // TODO_RUST_CODE
      {
         // Java: return -1;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: Integer i = (Integer)this.mIndexOfSpan.get(what);
         // TODO_RUST_CODE
         // Java: return i == null ? -1 : this.resolveGap(this.mSpanStarts[i]);
         // TODO_RUST_CODE
      }
    }

    // Java: int getSpanEnd(Object what)
    pub fn getSpanEnd(&self, Object what) -> i32 {
      // Java: if (this.mIndexOfSpan == null)
        // TODO_RUST_CODE
      {
         // Java: return -1;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: Integer i = (Integer)this.mIndexOfSpan.get(what);
         // TODO_RUST_CODE
         // Java: return i == null ? -1 : this.resolveGap(this.mSpanEnds[i]);
         // TODO_RUST_CODE
      }
    }

    // Java: int getSpanFlags(Object what)
    pub fn getSpanFlags(&self, Object what) -> i32 {
      // Java: if (this.mIndexOfSpan == null)
        // TODO_RUST_CODE
      {
         // Java: return 0;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: Integer i = (Integer)this.mIndexOfSpan.get(what);
         // TODO_RUST_CODE
         // Java: return i == null ? 0 : this.mSpanFlags[i];
         // TODO_RUST_CODE
      }
    }

    // Java: public <T> T[] getSpans(int queryStart, int queryEnd, Class<T> kind)
    pub fn getSpans(&self, int queryStart, int queryEnd, Class<T> kind) -> public <T> T[] {
      // Java: return (T[])this.getSpans(queryStart, queryEnd, kind, true);
      // TODO_RUST_CODE
    }

    // Java: public <T> T[] getSpans(int queryStart, int queryEnd, Class<T> kind, boolean sortByInsertionOrder)
    pub fn getSpans(&self, int queryStart, int queryEnd, Class<T> kind, boolean sortByInsertionOrder) -> public <T> T[] {
      // Java: if (kind == null)
        // TODO_RUST_CODE
      {
         // Java: return (T[])ArrayUtils.emptyArray(Object.class);
         // TODO_RUST_CODE
      }
      // Java: else if (this.mSpanCount == 0)
        // TODO_RUST_CODE
      {
         // Java: return (T[])ArrayUtils.emptyArray(kind);
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: int count = this.countSpans(queryStart, queryEnd, kind, this.treeRoot());
         // TODO_RUST_CODE
         // Java: if (count == 0)
            // TODO_RUST_CODE
         {
            // Java: return (T[])ArrayUtils.emptyArray(kind);
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: T[] ret = (T[])((Object[])Array.newInstance(kind, count));
            // TODO_RUST_CODE
            // Java: int[] prioSortBuffer = sortByInsertionOrder ? obtain(count) : EmptyArray.INT;
            // TODO_RUST_CODE
            // Java: int[] orderSortBuffer = sortByInsertionOrder ? obtain(count) : EmptyArray.INT;
            // TODO_RUST_CODE
            // Java: this.getSpansRec(queryStart, queryEnd, kind, this.treeRoot(), ret, prioSortBuffer, orderSortBuffer, 0, sortByInsertionOrder);
            // TODO_RUST_CODE
            // Java: if (sortByInsertionOrder)
                // TODO_RUST_CODE
            {
               // Java: this.sort(ret, prioSortBuffer, orderSortBuffer);
               // TODO_RUST_CODE
               // Java: recycle(prioSortBuffer);
               // TODO_RUST_CODE
               // Java: recycle(orderSortBuffer);
               // TODO_RUST_CODE
            }
            // Java: return ret;
            // TODO_RUST_CODE
         }
      }
    }

    // Java: int countSpans(int queryStart, int queryEnd, Class kind, int i)
    pub fn countSpans(&self, int queryStart, int queryEnd, Class kind, int i) -> i32 {
      // Java: int count = 0;
      // TODO_RUST_CODE
      // Java: if ((i & 1) != 0)
        // TODO_RUST_CODE
      {
         // Java: int left = leftChild(i);
         // TODO_RUST_CODE
         // Java: int spanMax = this.mSpanMax[left];
         // TODO_RUST_CODE
         // Java: if (spanMax > this.mGapStart)
            // TODO_RUST_CODE
         {
            // Java: spanMax -= this.mGapLength;
            // TODO_RUST_CODE
         }
         // Java: if (spanMax >= queryStart)
            // TODO_RUST_CODE
         {
            // Java: count = this.countSpans(queryStart, queryEnd, kind, left);
            // TODO_RUST_CODE
         }
      }
      // Java: if (i < this.mSpanCount)
        // TODO_RUST_CODE
      {
         // Java: int spanStart = this.mSpanStarts[i];
         // TODO_RUST_CODE
         // Java: if (spanStart > this.mGapStart)
            // TODO_RUST_CODE
         {
            // Java: spanStart -= this.mGapLength;
            // TODO_RUST_CODE
         }
         // Java: if (spanStart <= queryEnd)
            // TODO_RUST_CODE
         {
            // Java: int spanEnd = this.mSpanEnds[i];
            // TODO_RUST_CODE
            // Java: if (spanEnd > this.mGapStart)
                // TODO_RUST_CODE
            {
               // Java: spanEnd -= this.mGapLength;
               // TODO_RUST_CODE
            }
            // Java: if (spanEnd >= queryStart && (spanStart == spanEnd || queryStart == queryEnd || spanStart != queryEnd && spanEnd != queryStart) && (Object.class == kind || kind.isInstance(this.mSpans[i])))
                // TODO_RUST_CODE
            {
               // Java: ++count;
               // TODO_RUST_CODE
            }
            // Java: if ((i & 1) != 0)
                // TODO_RUST_CODE
            {
               // Java: count += this.countSpans(queryStart, queryEnd, kind, rightChild(i));
               // TODO_RUST_CODE
            }
         }
      }
      // Java: return count;
      // TODO_RUST_CODE
    }

    // Java: private <T> int getSpansRec(int queryStart, int queryEnd, Class<T> kind, int i, T[] ret, int[] priority, int[] insertionOrder, int count, boolean sort)
    pub fn getSpansRec(&self, int queryStart, int queryEnd, Class<T> kind, int i, T[] ret, int[] priority, int[] insertionOrder, int count, boolean sort) -> private <T> int {
      // Java: if ((i & 1) != 0)
        // TODO_RUST_CODE
      {
         // Java: int left = leftChild(i);
         // TODO_RUST_CODE
         // Java: int spanMax = this.mSpanMax[left];
         // TODO_RUST_CODE
         // Java: if (spanMax > this.mGapStart)
            // TODO_RUST_CODE
         {
            // Java: spanMax -= this.mGapLength;
            // TODO_RUST_CODE
         }
         // Java: if (spanMax >= queryStart)
            // TODO_RUST_CODE
         {
            // Java: count = this.getSpansRec(queryStart, queryEnd, kind, left, ret, priority, insertionOrder, count, sort);
            // TODO_RUST_CODE
         }
      }
      // Java: if (i >= this.mSpanCount)
        // TODO_RUST_CODE
      {
         // Java: return count;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: int spanStart = this.mSpanStarts[i];
         // TODO_RUST_CODE
         // Java: if (spanStart > this.mGapStart)
            // TODO_RUST_CODE
         {
            // Java: spanStart -= this.mGapLength;
            // TODO_RUST_CODE
         }
         // Java: if (spanStart <= queryEnd)
            // TODO_RUST_CODE
         {
            // Java: int spanEnd = this.mSpanEnds[i];
            // TODO_RUST_CODE
            // Java: if (spanEnd > this.mGapStart)
                // TODO_RUST_CODE
            {
               // Java: spanEnd -= this.mGapLength;
               // TODO_RUST_CODE
            }
            // Java: if (spanEnd >= queryStart && (spanStart == spanEnd || queryStart == queryEnd || spanStart != queryEnd && spanEnd != queryStart) && (Object.class == kind || kind.isInstance(this.mSpans[i])))
                // TODO_RUST_CODE
            {
               // Java: int spanPriority = this.mSpanFlags[i] & 16711680;
               // TODO_RUST_CODE
               // Java: int target = count;
               // TODO_RUST_CODE
               // Java: if (sort)
                    // TODO_RUST_CODE
               {
                  // Java: priority[count] = spanPriority;
                  // TODO_RUST_CODE
                  // Java: insertionOrder[count] = this.mSpanOrder[i];
                  // TODO_RUST_CODE
               }
               // Java: else if (spanPriority != 0)
                    // TODO_RUST_CODE
               {
                  // Java: int j;
                  // TODO_RUST_CODE
                  // Java: for(j = 0; j < count; ++j)
                        // TODO_RUST_CODE
                  {
                     // Java: int p = this.getSpanFlags(ret[j]) & 16711680;
                     // TODO_RUST_CODE
                     // Java: if (spanPriority > p)
                            // TODO_RUST_CODE
                     {
                        // Java: break;
                        // TODO_RUST_CODE
                     }
                  }
                  // Java: System.arraycopy(ret, j, ret, j + 1, count - j);
                  // TODO_RUST_CODE
                  // Java: target = j;
                  // TODO_RUST_CODE
               }
               // Java: ret[target] = (T)this.mSpans[i];
               // TODO_RUST_CODE
               // Java: ++count;
               // TODO_RUST_CODE
            }
            // Java: if (count < ret.length && (i & 1) != 0)
                // TODO_RUST_CODE
            {
               // Java: count = this.getSpansRec(queryStart, queryEnd, kind, rightChild(i), ret, priority, insertionOrder, count, sort);
               // TODO_RUST_CODE
            }
         }
         // Java: return count;
         // TODO_RUST_CODE
      }
    }

    // Java: static int[] obtain(int elementCount)
    pub fn obtain(&self, int elementCount) -> int[] {
      // Java: int[] result = null;
      // TODO_RUST_CODE
      // Java: synchronized(sCachedIntBuffer)
        // TODO_RUST_CODE
      {
         // Java: int candidateIndex = -1;
         // TODO_RUST_CODE
         // Java: for(int i = sCachedIntBuffer.length - 1; i >= 0; --i)
            // TODO_RUST_CODE
         {
            // Java: if (sCachedIntBuffer[i] != null)
                // TODO_RUST_CODE
            {
               // Java: if (sCachedIntBuffer[i].length >= elementCount)
                    // TODO_RUST_CODE
               {
                  // Java: candidateIndex = i;
                  // TODO_RUST_CODE
                  // Java: break;
                  // TODO_RUST_CODE
               }
               // Java: if (candidateIndex == -1)
                    // TODO_RUST_CODE
               {
                  // Java: candidateIndex = i;
                  // TODO_RUST_CODE
               }
            }
         }
         // Java: if (candidateIndex != -1)
            // TODO_RUST_CODE
         {
            // Java: result = sCachedIntBuffer[candidateIndex];
            // TODO_RUST_CODE
            // Java: sCachedIntBuffer[candidateIndex] = null;
            // TODO_RUST_CODE
         }
      }
      // Java: result = checkSortBuffer(result, elementCount);
      // TODO_RUST_CODE
      // Java: return result;
      // TODO_RUST_CODE
    }

    // Java: static void recycle(int[] buffer)
    pub fn recycle(&self, int[] buffer) {
      // Java: synchronized(sCachedIntBuffer)
        // TODO_RUST_CODE
      {
         // Java: for(int i = 0; i < sCachedIntBuffer.length; ++i)
            // TODO_RUST_CODE
         {
            // Java: if (sCachedIntBuffer[i] == null || buffer.length > sCachedIntBuffer[i].length)
                // TODO_RUST_CODE
            {
               // Java: sCachedIntBuffer[i] = buffer;
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            }
         }
      }
    }

    // Java: static int[] checkSortBuffer(int[] buffer, int size)
    pub fn checkSortBuffer(&self, int[] buffer, int size) -> int[] {
      // Java: return buffer != null && size <= buffer.length ? buffer : ArrayUtils.newUnpaddedIntArray(GrowingArrayUtils.growSize(size));
      // TODO_RUST_CODE
    }

    // Java: private final <T> void sort(T[] array, int[] priority, int[] insertionOrder)
    pub fn sort(&self, T[] array, int[] priority, int[] insertionOrder) -> private final <T> void {
      // Java: int size = array.length;
      // TODO_RUST_CODE
      // Java: for(int i = size / 2 - 1; i >= 0; --i)
        // TODO_RUST_CODE
      {
         // Java: this.siftDown(i, array, size, priority, insertionOrder);
         // TODO_RUST_CODE
      }
      // Java: for(int i = size - 1; i > 0; --i)
        // TODO_RUST_CODE
      {
         // Java: T tmpSpan = (T)array[0];
         // TODO_RUST_CODE
         // Java: array[0] = array[i];
         // TODO_RUST_CODE
         // Java: array[i] = tmpSpan;
         // TODO_RUST_CODE
         // Java: int tmpPriority = priority[0];
         // TODO_RUST_CODE
         // Java: priority[0] = priority[i];
         // TODO_RUST_CODE
         // Java: priority[i] = tmpPriority;
         // TODO_RUST_CODE
         // Java: int tmpOrder = insertionOrder[0];
         // TODO_RUST_CODE
         // Java: insertionOrder[0] = insertionOrder[i];
         // TODO_RUST_CODE
         // Java: insertionOrder[i] = tmpOrder;
         // TODO_RUST_CODE
         // Java: this.siftDown(0, array, i, priority, insertionOrder);
         // TODO_RUST_CODE
      }
    }

    // Java: private final <T> void siftDown(int index, T[] array, int size, int[] priority, int[] insertionOrder)
    pub fn siftDown(&self, int index, T[] array, int size, int[] priority, int[] insertionOrder) -> private final <T> void {
      // Java: for(int left = 2 * index + 1; left < size; left = 2 * left + 1)
        // TODO_RUST_CODE
      {
         // Java: if (left < size - 1 && this.compareSpans(left, left + 1, priority, insertionOrder) < 0)
            // TODO_RUST_CODE
         {
            // Java: ++left;
            // TODO_RUST_CODE
         }
         // Java: if (this.compareSpans(index, left, priority, insertionOrder) >= 0)
            // TODO_RUST_CODE
         {
            // Java: break;
            // TODO_RUST_CODE
         }
         // Java: T tmpSpan = (T)array[index];
         // TODO_RUST_CODE
         // Java: array[index] = array[left];
         // TODO_RUST_CODE
         // Java: array[left] = tmpSpan;
         // TODO_RUST_CODE
         // Java: int tmpPriority = priority[index];
         // TODO_RUST_CODE
         // Java: priority[index] = priority[left];
         // TODO_RUST_CODE
         // Java: priority[left] = tmpPriority;
         // TODO_RUST_CODE
         // Java: int tmpOrder = insertionOrder[index];
         // TODO_RUST_CODE
         // Java: insertionOrder[index] = insertionOrder[left];
         // TODO_RUST_CODE
         // Java: insertionOrder[left] = tmpOrder;
         // TODO_RUST_CODE
         // Java: index = left;
         // TODO_RUST_CODE
      }
    }

    // Java: int compareSpans(int left, int right, int[] priority, int[] insertionOrder)
    pub fn compareSpans(&self, int left, int right, int[] priority, int[] insertionOrder) -> i32 {
      // Java: int priority1 = priority[left];
      // TODO_RUST_CODE
      // Java: int priority2 = priority[right];
      // TODO_RUST_CODE
      // Java: return priority1 == priority2 ? Integer.compare(insertionOrder[left], insertionOrder[right]) : Integer.compare(priority2, priority1);
      // TODO_RUST_CODE
    }

    // Java: int nextSpanTransition(int start, int limit, Class kind)
    pub fn nextSpanTransition(&self, int start, int limit, Class kind) -> i32 {
      // Java: if (this.mSpanCount == 0)
        // TODO_RUST_CODE
      {
         // Java: return limit;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: if (kind == null)
            // TODO_RUST_CODE
         {
            // Java: kind = Object.class;
            // TODO_RUST_CODE
         }
         // Java: return this.nextSpanTransitionRec(start, limit, kind, this.treeRoot());
         // TODO_RUST_CODE
      }
    }

    // Java: int nextSpanTransitionRec(int start, int limit, Class kind, int i)
    pub fn nextSpanTransitionRec(&self, int start, int limit, Class kind, int i) -> i32 {
      // Java: if ((i & 1) != 0)
        // TODO_RUST_CODE
      {
         // Java: int left = leftChild(i);
         // TODO_RUST_CODE
         // Java: if (this.resolveGap(this.mSpanMax[left]) > start)
            // TODO_RUST_CODE
         {
            // Java: limit = this.nextSpanTransitionRec(start, limit, kind, left);
            // TODO_RUST_CODE
         }
      }
      // Java: if (i < this.mSpanCount)
        // TODO_RUST_CODE
      {
         // Java: int st = this.resolveGap(this.mSpanStarts[i]);
         // TODO_RUST_CODE
         // Java: int en = this.resolveGap(this.mSpanEnds[i]);
         // TODO_RUST_CODE
         // Java: if (st > start && st < limit && kind.isInstance(this.mSpans[i]))
            // TODO_RUST_CODE
         {
            // Java: limit = st;
            // TODO_RUST_CODE
         }
         // Java: if (en > start && en < limit && kind.isInstance(this.mSpans[i]))
            // TODO_RUST_CODE
         {
            // Java: limit = en;
            // TODO_RUST_CODE
         }
         // Java: if (st < limit && (i & 1) != 0)
            // TODO_RUST_CODE
         {
            // Java: limit = this.nextSpanTransitionRec(start, limit, kind, rightChild(i));
            // TODO_RUST_CODE
         }
      }
      // Java: return limit;
      // TODO_RUST_CODE
    }

    // Java: CharSequence subSequence(int start, int end)
    pub fn subSequence(&self, int start, int end) -> CharSequence {
      // Java: return new SpannableStringBuilder(this, start, end);
      // TODO_RUST_CODE
    }

    // Java: void getChars(int start, int end, char[] dest, int destoff)
    pub fn getChars(&self, int start, int end, char[] dest, int destoff) {
      // Java: this.checkRange("getChars", start, end);
      // TODO_RUST_CODE
      // Java: if (end <= this.mGapStart)
        // TODO_RUST_CODE
      {
         // Java: System.arraycopy(this.mText, start, dest, destoff, end - start);
         // TODO_RUST_CODE
      }
      // Java: else if (start >= this.mGapStart)
        // TODO_RUST_CODE
      {
         // Java: System.arraycopy(this.mText, start + this.mGapLength, dest, destoff, end - start);
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: System.arraycopy(this.mText, start, dest, destoff, this.mGapStart - start);
         // TODO_RUST_CODE
         // Java: System.arraycopy(this.mText, this.mGapStart + this.mGapLength, dest, destoff + (this.mGapStart - start), end - this.mGapStart);
         // TODO_RUST_CODE
      }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: int len = this.length();
      // TODO_RUST_CODE
      // Java: char[] buf = new char[len];
      // TODO_RUST_CODE
      // Java: this.getChars(0, len, buf, 0);
      // TODO_RUST_CODE
      // Java: return new String(buf);
      // TODO_RUST_CODE
    }

    // Java: String substring(int start, int end)
    pub fn substring(&self, int start, int end) -> String {
      // Java: char[] buf = new char[end - start];
      // TODO_RUST_CODE
      // Java: this.getChars(start, end, buf, 0);
      // TODO_RUST_CODE
      // Java: return new String(buf);
      // TODO_RUST_CODE
    }

    // Java: static String region(int start, int end)
    pub fn region(&self, int start, int end) -> String {
      // Java: return "(" + start + " ... " + end + ")";
      // TODO_RUST_CODE
    }

    // Java: void checkRange(String operation, int start, int end)
    pub fn checkRange(&self, String operation, int start, int end) {
      // Java: if (end < start)
        // TODO_RUST_CODE
      {
         // Java: throw new IndexOutOfBoundsException(operation + " " + region(start, end) + " has end before start");
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: int len = this.length();
         // TODO_RUST_CODE
         // Java: if (start <= len && end <= len)
            // TODO_RUST_CODE
         {
            // Java: if (start < 0 || end < 0)
                // TODO_RUST_CODE
            {
               // Java: throw new IndexOutOfBoundsException(operation + " " + region(start, end) + " starts before 0");
               // TODO_RUST_CODE
            }
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: throw new IndexOutOfBoundsException(operation + " " + region(start, end) + " ends beyond length " + len);
            // TODO_RUST_CODE
         }
      }
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, Object o) -> bool {
      // Java: if (o instanceof Spanned && this.toString().equals(o.toString()))
        // TODO_RUST_CODE
      {
         // Java: Spanned other = (Spanned)o;
         // TODO_RUST_CODE
         // Java: Object[] otherSpans = other.getSpans(0, other.length(), Object.class);
         // TODO_RUST_CODE
         // Java: if (this.mSpanCount == otherSpans.length)
            // TODO_RUST_CODE
         {
            // Java: for(int i = 0; i < this.mSpanCount; ++i)
                // TODO_RUST_CODE
            {
               // Java: Object thisSpan = this.mSpans[i];
               // TODO_RUST_CODE
               // Java: Object otherSpan = otherSpans[i];
               // TODO_RUST_CODE
               // Java: if (thisSpan == this)
                    // TODO_RUST_CODE
               {
                  // Java: if (other != otherSpan || this.getSpanStart(thisSpan) != other.getSpanStart(otherSpan) || this.getSpanEnd(thisSpan) != other.getSpanEnd(otherSpan) || this.getSpanFlags(thisSpan) != other.getSpanFlags(otherSpan))
                        // TODO_RUST_CODE
                  {
                     // Java: return false;
                     // TODO_RUST_CODE
                  }
               }
               // Java: else if (!thisSpan.equals(otherSpan) || this.getSpanStart(thisSpan) != other.getSpanStart(otherSpan) || this.getSpanEnd(thisSpan) != other.getSpanEnd(otherSpan) || this.getSpanFlags(thisSpan) != other.getSpanFlags(otherSpan))
                    // TODO_RUST_CODE
               {
                  // Java: return false;
                  // TODO_RUST_CODE
               }
            }
            // Java: return true;
            // TODO_RUST_CODE
         }
      }
      // Java: return false;
      // TODO_RUST_CODE
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int hash = this.toString().hashCode();
      // TODO_RUST_CODE
      // Java: hash = hash * 31 + this.mSpanCount;
      // TODO_RUST_CODE
      // Java: for(int i = 0; i < this.mSpanCount; ++i)
        // TODO_RUST_CODE
      {
         // Java: Object span = this.mSpans[i];
         // TODO_RUST_CODE
         // Java: if (span != this)
            // TODO_RUST_CODE
         {
            // Java: hash = hash * 31 + span.hashCode();
            // TODO_RUST_CODE
         }
         // Java: hash = hash * 31 + this.getSpanStart(span);
         // TODO_RUST_CODE
         // Java: hash = hash * 31 + this.getSpanEnd(span);
         // TODO_RUST_CODE
         // Java: hash = hash * 31 + this.getSpanFlags(span);
         // TODO_RUST_CODE
      }
      // Java: return hash;
      // TODO_RUST_CODE
    }

    // Java: int treeRoot()
    pub fn treeRoot(&self) -> i32 {
      // Java: return Integer.highestOneBit(this.mSpanCount) - 1;
      // TODO_RUST_CODE
    }

    // Java: static int leftChild(int i)
    pub fn leftChild(&self, int i) -> i32 {
      // Java: return i - ((i + 1 & ~i) >> 1);
      // TODO_RUST_CODE
    }

    // Java: static int rightChild(int i)
    pub fn rightChild(&self, int i) -> i32 {
      // Java: return i + ((i + 1 & ~i) >> 1);
      // TODO_RUST_CODE
    }

    // Java: int calcMax(int i)
    pub fn calcMax(&self, int i) -> i32 {
      // Java: int max = 0;
      // TODO_RUST_CODE
      // Java: if ((i & 1) != 0)
        // TODO_RUST_CODE
      {
         // Java: max = this.calcMax(leftChild(i));
         // TODO_RUST_CODE
      }
      // Java: if (i < this.mSpanCount)
        // TODO_RUST_CODE
      {
         // Java: max = Math.max(max, this.mSpanEnds[i]);
         // TODO_RUST_CODE
         // Java: if ((i & 1) != 0)
            // TODO_RUST_CODE
         {
            // Java: max = Math.max(max, this.calcMax(rightChild(i)));
            // TODO_RUST_CODE
         }
      }
      // Java: this.mSpanMax[i] = max;
      // TODO_RUST_CODE
      // Java: return max;
      // TODO_RUST_CODE
    }

    // Java: void restoreInvariants()
    pub fn restoreInvariants(&self) {
      // Java: if (this.mSpanCount != 0)
        // TODO_RUST_CODE
      {
         // Java: for(int i = 1; i < this.mSpanCount; ++i)
            // TODO_RUST_CODE
         {
            // Java: if (this.mSpanStarts[i] < this.mSpanStarts[i - 1])
                // TODO_RUST_CODE
            {
               // Java: Object span = this.mSpans[i];
               // TODO_RUST_CODE
               // Java: int start = this.mSpanStarts[i];
               // TODO_RUST_CODE
               // Java: int end = this.mSpanEnds[i];
               // TODO_RUST_CODE
               // Java: int flags = this.mSpanFlags[i];
               // TODO_RUST_CODE
               // Java: int insertionOrder = this.mSpanOrder[i];
               // TODO_RUST_CODE
               // Java: int j = i;
               // TODO_RUST_CODE
               // Java: do
                    // TODO_RUST_CODE
               {
                  // Java: this.mSpans[j] = this.mSpans[j - 1];
                  // TODO_RUST_CODE
                  // Java: this.mSpanStarts[j] = this.mSpanStarts[j - 1];
                  // TODO_RUST_CODE
                  // Java: this.mSpanEnds[j] = this.mSpanEnds[j - 1];
                  // TODO_RUST_CODE
                  // Java: this.mSpanFlags[j] = this.mSpanFlags[j - 1];
                  // TODO_RUST_CODE
                  // Java: this.mSpanOrder[j] = this.mSpanOrder[j - 1];
                  // TODO_RUST_CODE
                  // Java: --j;
                  // TODO_RUST_CODE
               }
               // Java: this.mSpans[j] = span;
               // TODO_RUST_CODE
               // Java: this.mSpanStarts[j] = start;
               // TODO_RUST_CODE
               // Java: this.mSpanEnds[j] = end;
               // TODO_RUST_CODE
               // Java: this.mSpanFlags[j] = flags;
               // TODO_RUST_CODE
               // Java: this.mSpanOrder[j] = insertionOrder;
               // TODO_RUST_CODE
               // Java: this.invalidateIndex(j);
               // TODO_RUST_CODE
            }
         }
         // Java: this.calcMax(this.treeRoot());
         // TODO_RUST_CODE
         // Java: if (this.mIndexOfSpan == null)
            // TODO_RUST_CODE
         {
            // Java: this.mIndexOfSpan = new IdentityHashMap();
            // TODO_RUST_CODE
         }
         // Java: for(int i = this.mLowWaterMark; i < this.mSpanCount; ++i)
            // TODO_RUST_CODE
         {
            // Java: Integer existing = (Integer)this.mIndexOfSpan.get(this.mSpans[i]);
            // TODO_RUST_CODE
            // Java: if (existing == null || existing != i)
                // TODO_RUST_CODE
            {
               // Java: this.mIndexOfSpan.put(this.mSpans[i], i);
               // TODO_RUST_CODE
            }
         }
         // Java: this.mLowWaterMark = Integer.MAX_VALUE;
         // TODO_RUST_CODE
      }
    }

    // Java: void invalidateIndex(int i)
    pub fn invalidateIndex(&self, int i) {
      // Java: this.mLowWaterMark = Math.min(i, this.mLowWaterMark);
      // TODO_RUST_CODE
    }

}

impl CharSequence for SpannableStringBuilder {
}

impl GetChars for SpannableStringBuilder {
}

impl Spannable for SpannableStringBuilder {
}

impl Editable for SpannableStringBuilder {
}

impl Appendable for SpannableStringBuilder {
}
