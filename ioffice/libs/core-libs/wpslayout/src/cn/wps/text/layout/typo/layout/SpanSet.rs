#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: public class SpanSet
pub struct SpanSet {
    // Java field: Class<? extends E> classType
    classType: i32,
    // Java field: int numberOfSpans
    numberOfSpans: i32,
    // Java field: E[] spans
    spans: Vec<i32>,
    // Java field: int[] spanStarts
    spanStarts: Vec<i32>,
    // Java field: int[] spanEnds
    spanEnds: Vec<i32>,
    // Java field: int[] spanFlags
    spanFlags: Vec<i32>,
}

// Java: public class SpanSet
impl SpanSet {
    // Java: void init(Spanned spanned, int start, int limit)
    pub fn init(&mut self, _spanned: i32, _start: i32, _limit: i32) {
      // Java: E[] allSpans = (E[])spanned.getSpans(start, limit, this.classType);
      // Java: int length = allSpans.length;
      let _length = 0i32;
      // Java: if (length > 0 && (this.spans == null || this.spans.length < length))
      {
         // Java: this.spans = (E[])((Object[])Array.newInstance(this.classType, length));
         // Java: this.spanStarts = new int[length];
         // Java: this.spanEnds = new int[length];
         // Java: this.spanFlags = new int[length];
      }
      // Java: int prevNumberOfSpans = this.numberOfSpans;
      let _prevNumberOfSpans = self.numberOfSpans;
      // Java: this.numberOfSpans = 0;
      self.numberOfSpans = 0;
      // Java: for(int i = 0; i < length; ++i)
      {
         // Java: E span = (E)allSpans[i];
         // Java: int spanStart = spanned.getSpanStart(span);
         // Java: int spanEnd = spanned.getSpanEnd(span);
         // Java: if (spanStart != spanEnd)
         {
            // Java: int spanFlag = spanned.getSpanFlags(span);
            // Java: this.spans[this.numberOfSpans] = span;
            // Java: this.spanStarts[this.numberOfSpans] = spanStart;
            // Java: this.spanEnds[this.numberOfSpans] = spanEnd;
            // Java: this.spanFlags[this.numberOfSpans] = spanFlag;
            // Java: ++this.numberOfSpans;
         }
      }
      // Java: if (this.numberOfSpans < prevNumberOfSpans)
      {
         // Java: Arrays.fill(this.spans, this.numberOfSpans, prevNumberOfSpans, null);
      }
    }

    // Java: boolean hasSpansIntersecting(int start, int end)
    pub fn hasSpansIntersecting(&self, _start: i32, _end: i32) -> bool {
      // Java: for(int i = 0; i < this.numberOfSpans; ++i)
      for i in 0..self.numberOfSpans as usize
      {
         // Java: if (this.spanStarts[i] < end && this.spanEnds[i] > start)
         if self.spanStarts[i] < _end && self.spanEnds[i] > _start
         {
            // Java: return true;
            return true;
         }
      }
      // Java: return false;
      false
    }

    // Java: int getNextTransition(int start, int limit)
    pub fn getNextTransition(&self, _start: i32, mut _limit: i32) -> i32 {
      // Java: for(int i = 0; i < this.numberOfSpans; ++i)
      for i in 0..self.numberOfSpans as usize
      {
         // Java: int spanStart = this.spanStarts[i];
         let span_start = self.spanStarts[i];
         // Java: int spanEnd = this.spanEnds[i];
         let span_end = self.spanEnds[i];
         // Java: if (spanStart > start && spanStart < limit)
         if span_start > _start && span_start < _limit
         {
            // Java: limit = spanStart;
            _limit = span_start;
         }
         // Java: if (spanEnd > start && spanEnd < limit)
         if span_end > _start && span_end < _limit
         {
            // Java: limit = spanEnd;
            _limit = span_end;
         }
      }
      // Java: return limit;
      _limit
    }

    // Java: void recycle()
    pub fn recycle(&mut self) {
      // Java: if (this.spans != null)
      {
         // Java: Arrays.fill(this.spans, 0, this.numberOfSpans, null);
         self.spans.clear();
      }
    }
}
