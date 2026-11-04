#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java field: Object[] EMPTY
// Java field: int START
// Java field: int END
// Java field: int FLAGS
// Java field: int COLUMNS

// Java: public class SpannableStringInternal
pub struct SpannableStringInternal {
    // Java field: String mText
    pub(crate) mText: String,
    // Java field: Object[] mSpans
    pub(crate) mSpans: Vec<i32>,
    // Java field: int[] mSpanData
    pub(crate) mSpanData: Vec<i32>,
    // Java field: int mSpanCount
    pub(crate) mSpanCount: i32,
}

// Java: public class SpannableStringInternal
impl SpannableStringInternal {
    // Java: void copySpans(Spanned src, int start, int end, boolean ignoreNoCopySpan)
    pub fn copySpans_spanned(&mut self, _src: i32, _start: i32, _end: i32, _ignoreNoCopySpan: bool) {
      // Java: Object[] spans = src.getSpans(start, end, Object.class);
      // Java: for(int i = 0; i < spans.length; ++i)
      {
         // Java: if (!ignoreNoCopySpan || !(spans[i] instanceof NoCopySpan))
         {
            // Java: int st = src.getSpanStart(spans[i]);
            // Java: int en = src.getSpanEnd(spans[i]);
            // Java: int fl = src.getSpanFlags(spans[i]);
            // Java: if (st < start)
            {
               // Java: st = start;
            }
            // Java: if (en > end)
            {
               // Java: en = end;
            }
            // Java: this.setSpan(spans[i], st - start, en - start, fl, false);
         }
      }
    }

    // Java: void copySpans(SpannableStringInternal src, int start, int end, boolean ignoreNoCopySpan)
    pub fn copySpans_internal(&mut self, _src: i32, _start: i32, _end: i32, _ignoreNoCopySpan: bool) {
      // Java: int count = 0;
      // Java: int[] srcData = src.mSpanData;
      // Java: Object[] srcSpans = src.mSpans;
      // Java: int limit = src.mSpanCount;
      // Java: boolean hasNoCopySpan = false;
      // Java: for(int i = 0; i < limit; ++i)
      {
         // Java: int spanStart = srcData[i * 3 + 0];
         // Java: int spanEnd = srcData[i * 3 + 1];
         // Java: if (!this.isOutOfCopyRange(start, end, spanStart, spanEnd))
         {
            // Java: if (srcSpans[i] instanceof NoCopySpan)
            {
               // Java: hasNoCopySpan = true;
               // Java: if (ignoreNoCopySpan)
               {
                  // Java: continue;
               }
            }
            // Java: ++count;
         }
      }
      // Java: if (count != 0)
      {
         // Java: if (!hasNoCopySpan && start == 0 && end == src.length())
         {
            // Java: this.mSpans = ArrayUtils.newUnpaddedObjectArray(src.mSpans.length);
            // Java: this.mSpanData = new int[src.mSpanData.length];
            // Java: this.mSpanCount = src.mSpanCount;
            // Java: System.arraycopy(src.mSpans, 0, this.mSpans, 0, src.mSpans.length);
            // Java: System.arraycopy(src.mSpanData, 0, this.mSpanData, 0, this.mSpanData.length);
         }
         // Java: else
         {
            // Java: this.mSpanCount = count;
            // Java: this.mSpans = ArrayUtils.newUnpaddedObjectArray(this.mSpanCount);
            // Java: this.mSpanData = new int[this.mSpans.length * 3];
            // Java: int i = 0;
            // Java: for(int j = 0; i < limit; ++i)
            {
               // Java: int spanStart = srcData[i * 3 + 0];
               // Java: int spanEnd = srcData[i * 3 + 1];
               // Java: if (!this.isOutOfCopyRange(start, end, spanStart, spanEnd) && (!ignoreNoCopySpan || !(srcSpans[i] instanceof NoCopySpan)))
               {
                  // Java: if (spanStart < start)
                  {
                     // Java: spanStart = start;
                  }
                  // Java: if (spanEnd > end)
                  {
                     // Java: spanEnd = end;
                  }
                  // Java: this.mSpans[j] = srcSpans[i];
                  // Java: this.mSpanData[j * 3 + 0] = spanStart - start;
                  // Java: this.mSpanData[j * 3 + 1] = spanEnd - start;
                  // Java: this.mSpanData[j * 3 + 2] = srcData[i * 3 + 2];
                  // Java: ++j;
               }
            }
         }
      }
    }

    // Java: boolean isOutOfCopyRange(int start, int end, int spanStart, int spanEnd)
    pub fn isOutOfCopyRange(&self, _start: i32, _end: i32, _spanStart: i32, _spanEnd: i32) -> bool {
      // Java: if (spanStart <= end && spanEnd >= start)
      if _spanStart <= _end && _spanEnd >= _start {
         // Java: return spanStart != spanEnd && start != end && (spanStart == end || spanEnd == start);
         return _spanStart != _spanEnd && _start != _end && (_spanStart == _end || _spanEnd == _start);
      }
      // Java: else
      else {
         // Java: return true;
         return true;
      }
    }

    // Java: int length()
    pub fn length(&self) -> i32 {
      // Java: return this.mText.length();
      self.mText.len() as i32
    }

    // Java: char charAt(int i)
    pub fn charAt(&self, _i: i32) -> char {
      // Java: return this.mText.charAt(i);
      '\0'
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return this.mText;
      self.mText.clone()
    }

    // Java: void getChars(int start, int end, char[] dest, int off)
    pub fn getChars(&self, _start: i32, _end: i32, _dest: &mut [char], _off: i32) {
      // Java: this.mText.getChars(start, end, dest, off);
    }

    // Java: void setSpan(Object what, int start, int end, int flags)
    pub fn setSpan(&mut self, _what: i32, _start: i32, _end: i32, _flags: i32) {
      // Java: this.setSpan(what, start, end, flags, true);
      self.setSpan_enforce(_what, _start, _end, _flags, true);
    }

    // Java: boolean isIndexFollowsNextLine(int index)
    pub fn isIndexFollowsNextLine(&self, _index: i32) -> bool {
      // Java: return index != 0 && index != this.length() && this.charAt(index - 1) != '\n';
      false
    }

    // Java: void setSpan(Object what, int start, int end, int flags, boolean enforceParagraph)
    pub fn setSpan_enforce(&mut self, _what: i32, _start: i32, _end: i32, _flags: i32, _enforceParagraph: bool) {
      // Java: this.checkRange("setSpan", start, end);
      // Java: if ((flags & 51) == 51)
      {
         // Java: if (this.isIndexFollowsNextLine(start))
         {
            // Java: if (!enforceParagraph)
            {
               // Java: return;
            }
            // Java: throw new RuntimeException("PARAGRAPH span must start at paragraph boundary (" + start + " follows " + this.charAt(start - 1) + ")");
         }
         // Java: if (this.isIndexFollowsNextLine(end))
         {
            // Java: if (!enforceParagraph)
            {
               // Java: return;
            }
            // Java: throw new RuntimeException("PARAGRAPH span must end at paragraph boundary (" + end + " follows " + this.charAt(end - 1) + ")");
         }
      }
      // Java: int count = this.mSpanCount;
      // Java: Object[] spans = this.mSpans;
      // Java: int[] data = this.mSpanData;
      // Java: for(int i = 0; i < count; ++i)
      {
         // Java: if (spans[i] == what)
         {
            // Java: int ostart = data[i * 3 + 0];
            // Java: int oend = data[i * 3 + 1];
            // Java: data[i * 3 + 0] = start;
            // Java: data[i * 3 + 1] = end;
            // Java: data[i * 3 + 2] = flags;
            // Java: this.sendSpanChanged(what, ostart, oend, start, end);
            // Java: return;
         }
      }
      // Java: if (this.mSpanCount + 1 >= this.mSpans.length)
      {
         // Java: Object[] newtags = ArrayUtils.newUnpaddedObjectArray(GrowingArrayUtils.growSize(this.mSpanCount));
         // Java: int[] newdata = new int[newtags.length * 3];
         // Java: System.arraycopy(this.mSpans, 0, newtags, 0, this.mSpanCount);
         // Java: System.arraycopy(this.mSpanData, 0, newdata, 0, this.mSpanCount * 3);
         // Java: this.mSpans = newtags;
         // Java: this.mSpanData = newdata;
      }
      // Java: this.mSpans[this.mSpanCount] = what;
      // Java: this.mSpanData[this.mSpanCount * 3 + 0] = start;
      // Java: this.mSpanData[this.mSpanCount * 3 + 1] = end;
      // Java: this.mSpanData[this.mSpanCount * 3 + 2] = flags;
      // Java: ++this.mSpanCount;
      // Java: if (this instanceof Spannable)
      {
         // Java: this.sendSpanAdded(what, start, end);
      }
    }

    // Java: void removeSpan(Object what)
    pub fn removeSpan(&mut self, _what: i32) {
      // Java: this.removeSpan(what, 0);
      self.removeSpan_flags(_what, 0);
    }

    // Java: void removeSpan(Object what, int flags)
    pub fn removeSpan_flags(&mut self, _what: i32, _flags: i32) {
      // Java: int count = this.mSpanCount;
      // Java: Object[] spans = this.mSpans;
      // Java: int[] data = this.mSpanData;
      // Java: for(int i = count - 1; i >= 0; --i)
      {
         // Java: if (spans[i] == what)
         {
            // Java: int ostart = data[i * 3 + 0];
            // Java: int oend = data[i * 3 + 1];
            // Java: int c = count - (i + 1);
            // Java: System.arraycopy(spans, i + 1, spans, i, c);
            // Java: System.arraycopy(data, (i + 1) * 3, data, i * 3, c * 3);
            // Java: --this.mSpanCount;
            // Java: if ((flags & 512) == 0)
            {
               // Java: this.sendSpanRemoved(what, ostart, oend);
            }
            // Java: return;
         }
      }
    }

    // Java: int getSpanStart(Object what)
    pub fn getSpanStart(&self, _what: i32) -> i32 {
      // Java: int count = this.mSpanCount;
      // Java: Object[] spans = this.mSpans;
      // Java: int[] data = this.mSpanData;
      // Java: for(int i = count - 1; i >= 0; --i)
      {
         // Java: if (spans[i] == what)
         {
            // Java: return data[i * 3 + 0];
         }
      }
      // Java: return -1;
      -1
    }

    // Java: int getSpanEnd(Object what)
    pub fn getSpanEnd(&self, _what: i32) -> i32 {
      // Java: int count = this.mSpanCount;
      // Java: Object[] spans = this.mSpans;
      // Java: int[] data = this.mSpanData;
      // Java: for(int i = count - 1; i >= 0; --i)
      {
         // Java: if (spans[i] == what)
         {
            // Java: return data[i * 3 + 1];
         }
      }
      // Java: return -1;
      -1
    }

    // Java: int getSpanFlags(Object what)
    pub fn getSpanFlags(&self, _what: i32) -> i32 {
      // Java: int count = this.mSpanCount;
      // Java: Object[] spans = this.mSpans;
      // Java: int[] data = this.mSpanData;
      // Java: for(int i = count - 1; i >= 0; --i)
      {
         // Java: if (spans[i] == what)
         {
            // Java: return data[i * 3 + 2];
         }
      }
      // Java: return 0;
      0
    }

    // Java: public <T> T[] getSpans(int queryStart, int queryEnd, Class<T> kind)
    pub fn getSpans(&self, _queryStart: i32, _queryEnd: i32, _kind: i32) -> Vec<i32> {
      // Java: int count = 0;
      // Java: int spanCount = this.mSpanCount;
      // Java: Object[] spans = this.mSpans;
      // Java: int[] data = this.mSpanData;
      // Java: Object[] ret = null;
      // Java: Object ret1 = null;
      // Java: for(int i = 0; i < spanCount; ++i)
      {
         // Java: int spanStart = data[i * 3 + 0];
         // Java: int spanEnd = data[i * 3 + 1];
         // Java: if (spanStart <= queryEnd && spanEnd >= queryStart && (spanStart == spanEnd || queryStart == queryEnd || spanStart != queryEnd && spanEnd != queryStart) && (kind == null || kind == Object.class || kind.isInstance(spans[i])))
         {
            // Java: if (count == 0)
            {
               // Java: ret1 = spans[i];
               // Java: ++count;
            }
            // Java: else
            {
               // Java: if (count == 1)
               {
                  // Java: ret = (Object[])Array.newInstance(kind, spanCount - i + 1);
                  // Java: ret[0] = ret1;
               }
               // Java: int prio = data[i * 3 + 2] & 16711680;
               // Java: if (prio == 0)
               {
                  // Java: ret[count++] = spans[i];
               }
               // Java: else
               {
                  // Java: int j;
                  // Java: for(j = 0; j < count; ++j)
                  {
                     // Java: int p = this.getSpanFlags(ret[j]) & 16711680;
                     // Java: if (prio > p)
                     {
                        // Java: break;
                     }
                  }
                  // Java: System.arraycopy(ret, j, ret, j + 1, count - j);
                  // Java: ret[j] = spans[i];
                  // Java: ++count;
               }
            }
         }
      }
      // Java: if (count == 0)
      {
         // Java: return (T[])ArrayUtils.emptyArray(kind);
      }
      // Java: else if (count == 1)
      {
         // Java: ret = (Object[])Array.newInstance(kind, 1);
         // Java: ret[0] = ret1;
         // Java: return (T[])ret;
      }
      // Java: else if (count == ret.length)
      {
         // Java: return (T[])ret;
      }
      // Java: else
      {
         // Java: Object[] nret = (Object[])Array.newInstance(kind, count);
         // Java: System.arraycopy(ret, 0, nret, 0, count);
         // Java: return (T[])nret;
      }
      Vec::new()
    }

    // Java: int nextSpanTransition(int start, int limit, Class kind)
    pub fn nextSpanTransition(&self, _start: i32, _limit: i32, _kind: i32) -> i32 {
      // Java: int count = this.mSpanCount;
      // Java: Object[] spans = this.mSpans;
      // Java: int[] data = this.mSpanData;
      // Java: if (kind == null)
      {
         // Java: kind = Object.class;
      }
      // Java: for(int i = 0; i < count; ++i)
      {
         // Java: int st = data[i * 3 + 0];
         // Java: int en = data[i * 3 + 1];
         // Java: if (st > start && st < limit && kind.isInstance(spans[i]))
         {
            // Java: limit = st;
         }
         // Java: if (en > start && en < limit && kind.isInstance(spans[i]))
         {
            // Java: limit = en;
         }
      }
      // Java: return limit;
      _limit
    }

    // Java: void sendSpanAdded(Object what, int start, int end)
    pub fn sendSpanAdded(&self, _what: i32, _start: i32, _end: i32) {
      // Java: SpanWatcher[] recip = (SpanWatcher[])this.getSpans(start, end, SpanWatcher.class);
      // Java: int n = recip.length;
      // Java: for(int i = 0; i < n; ++i)
      {
         // Java: recip[i].onSpanAdded((Spannable)this, what, start, end);
      }
    }

    // Java: void sendSpanRemoved(Object what, int start, int end)
    pub fn sendSpanRemoved(&self, _what: i32, _start: i32, _end: i32) {
      // Java: SpanWatcher[] recip = (SpanWatcher[])this.getSpans(start, end, SpanWatcher.class);
      // Java: int n = recip.length;
      // Java: for(int i = 0; i < n; ++i)
      {
         // Java: recip[i].onSpanRemoved((Spannable)this, what, start, end);
      }
    }

    // Java: void sendSpanChanged(Object what, int s, int e, int st, int en)
    pub fn sendSpanChanged(&self, _what: i32, _s: i32, _e: i32, _st: i32, _en: i32) {
      // Java: SpanWatcher[] recip = (SpanWatcher[])this.getSpans(Math.min(s, st), Math.max(e, en), SpanWatcher.class);
      // Java: int n = recip.length;
      // Java: for(int i = 0; i < n; ++i)
      {
         // Java: recip[i].onSpanChanged((Spannable)this, what, s, e, st, en);
      }
    }

    // Java: static String region(int start, int end)
    pub fn region(_start: i32, _end: i32) -> String {
      // Java: return "(" + start + " ... " + end + ")";
      format!("({} ... {})", _start, _end)
    }

    // Java: void checkRange(String operation, int start, int end)
    pub fn checkRange(&self, _operation: &str, _start: i32, _end: i32) {
      // Java: if (end < start)
      {
         // Java: throw new IndexOutOfBoundsException(operation + " " + region(start, end) + " has end before start");
      }
      // Java: else
      {
         // Java: int len = this.length();
         // Java: if (start <= len && end <= len)
         {
            // Java: if (start < 0 || end < 0)
            {
               // Java: throw new IndexOutOfBoundsException(operation + " " + region(start, end) + " starts before 0");
            }
         }
         // Java: else
         {
            // Java: throw new IndexOutOfBoundsException(operation + " " + region(start, end) + " ends beyond length " + len);
         }
      }
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, _o: i32) -> bool {
      // Java: if (o instanceof Spanned && this.toString().equals(o.toString()))
      {
         // Java: Spanned other = (Spanned)o;
         // Java: Object[] otherSpans = other.getSpans(0, other.length(), Object.class);
         // Java: if (this.mSpanCount == otherSpans.length)
         {
            // Java: for(int i = 0; i < this.mSpanCount; ++i)
            {
               // Java: Object thisSpan = this.mSpans[i];
               // Java: Object otherSpan = otherSpans[i];
               // Java: if (thisSpan == this)
               {
                  // Java: if (other != otherSpan || this.getSpanStart(thisSpan) != other.getSpanStart(otherSpan) || this.getSpanEnd(thisSpan) != other.getSpanEnd(otherSpan) || this.getSpanFlags(thisSpan) != other.getSpanFlags(otherSpan))
                  {
                     // Java: return false;
                  }
               }
               // Java: else if (!thisSpan.equals(otherSpan) || this.getSpanStart(thisSpan) != other.getSpanStart(otherSpan) || this.getSpanEnd(thisSpan) != other.getSpanEnd(otherSpan) || this.getSpanFlags(thisSpan) != other.getSpanFlags(otherSpan))
               {
                  // Java: return false;
               }
            }
            // Java: return true;
         }
      }
      // Java: return false;
      false
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int hash = this.toString().hashCode();
      let mut hash = 0i32;
      // Java: hash = hash * 31 + this.mSpanCount;
      hash = hash.wrapping_mul(31).wrapping_add(self.mSpanCount);
      // Java: for(int i = 0; i < this.mSpanCount; ++i)
      {
         // Java: Object span = this.mSpans[i];
         // Java: if (span != this)
         {
            // Java: hash = hash * 31 + span.hashCode();
         }
         // Java: hash = hash * 31 + this.getSpanStart(span);
         // Java: hash = hash * 31 + this.getSpanEnd(span);
         // Java: hash = hash * 31 + this.getSpanFlags(span);
      }
      // Java: return hash;
      hash
    }

    // Java: void copySpans(Spanned src, int start, int end)
    pub fn copySpans_spanned_3(&mut self, _src: i32, _start: i32, _end: i32) {
      // Java: this.copySpans(src, start, end, false);
      self.copySpans_spanned(_src, _start, _end, false);
    }

    // Java: void copySpans(SpannableStringInternal src, int start, int end)
    pub fn copySpans_internal_3(&mut self, _src: i32, _start: i32, _end: i32) {
      // Java: this.copySpans(src, start, end, false);
      self.copySpans_internal(_src, _start, _end, false);
    }
}
